//! LeavePulse SDK — OAuth device-flow (RFC 8628) polling helper.
//!
//! `auth.device.start` / `approve` / `token` are the raw generated calls. This
//! wraps the token poll loop: call `token` every `interval` seconds, back off on
//! `slow_down`, and resolve once the user approves (or fail on expiry/denial).
//! Framework-agnostic — the caller passes the poll closure and the device_code.
//!
//! [`begin_device_flow`] is the higher-level headless facade: it runs `start`,
//! surfaces the user-facing URL + code, and exposes `.poll()` which honours the
//! returned interval and maps the approved grant into a [`RefreshingCredential`].

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use serde::Deserialize;

use crate::credentials::{RefreshFn, RefreshingCredential, RefreshingCredentialInit};
use crate::transport::TransportError;

/// The poll status the token endpoint returns (RFC 8628 §3.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DevicePollStatus {
    Approved,
    Pending,
    SlowDown,
    Expired,
    Denied,
}

/// Minimal shape of an `auth.device.token` response the poller needs.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DeviceTokenResponse {
    pub status: Option<DevicePollStatus>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_in: Option<u64>,
}

/// Why a device authorization could not complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceFlowError {
    Expired,
    Denied,
}

impl std::fmt::Display for DeviceFlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match self {
            DeviceFlowError::Expired => "expired",
            DeviceFlowError::Denied => "denied",
        };
        write!(f, "device authorization {what}")
    }
}

impl std::error::Error for DeviceFlowError {}

/// Options controlling the device-token poll loop.
#[derive(Debug, Clone, Copy)]
pub struct DevicePollOptions {
    /// Initial seconds between polls (the `interval` from `start`). Default 5.
    pub interval_seconds: u64,
    /// Seconds added to the interval on each `slow_down`. Default 5.
    pub slow_down_step_seconds: u64,
}

impl Default for DevicePollOptions {
    fn default() -> Self {
        Self {
            interval_seconds: 5,
            slow_down_step_seconds: 5,
        }
    }
}

/// A boxed async closure that performs one `auth.device.token` call.
pub type PollFn = Box<
    dyn Fn() -> Pin<Box<dyn Future<Output = Result<DeviceTokenResponse, TransportError>> + Send>>
        + Send
        + Sync,
>;

/// A boxed async closure that performs one `auth.device.start` call.
pub type StartFn = Box<
    dyn FnOnce()
            -> Pin<Box<dyn Future<Output = Result<DeviceStartResponse, TransportError>> + Send>>
        + Send,
>;

/// Outcome of [`poll_device_token`]: either the approved grant or a typed
/// [`DeviceFlowError`]; transport failures surface as [`TransportError`].
pub type PollResult = Result<Result<DeviceTokenResponse, DeviceFlowError>, TransportError>;

/// Poll `token` until the user approves the device. Returns the approved token
/// response, `Err(DeviceFlowError)` on expiry/denial, or a `TransportError` if a
/// poll call itself fails.
///
/// `token` performs one `auth.device.token` call per invocation.
pub async fn poll_device_token(token: PollFn, options: DevicePollOptions) -> PollResult {
    let step = Duration::from_secs(options.slow_down_step_seconds);
    let mut interval = Duration::from_secs(options.interval_seconds);
    loop {
        let response = token().await?;
        match response.status {
            Some(DevicePollStatus::Approved) => return Ok(Ok(response)),
            Some(DevicePollStatus::Pending) | None => {}
            Some(DevicePollStatus::SlowDown) => interval = interval.saturating_add(step),
            Some(DevicePollStatus::Expired) => return Ok(Err(DeviceFlowError::Expired)),
            Some(DevicePollStatus::Denied) => return Ok(Err(DeviceFlowError::Denied)),
        }
        tokio::time::sleep(interval).await;
    }
}

/// Shape of an `auth.device.start` response (RFC 8628 §3.2, wire snake_case).
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceStartResponse {
    /// Opaque code the client polls `token` with.
    pub device_code: String,
    /// Short code the user enters in the frontend (`device.vue`).
    pub user_code: String,
    /// URL the user opens to approve.
    pub verification_uri: String,
    /// `verification_uri` with the `user_code` pre-filled, when provided.
    pub verification_uri_complete: Option<String>,
    /// Seconds until the device code expires.
    pub expires_in: u64,
    /// Recommended seconds between polls.
    pub interval: Option<u64>,
}

/// Options for [`begin_device_flow`]'s `.poll()`.
#[derive(Default)]
pub struct BeginDeviceFlowOptions {
    /// Performs the refresh-token exchange for the credential returned by
    /// `.poll()`. Omit to leave wiring to the integration layer — the returned
    /// credential then errors if a refresh is attempted. Keep transport-agnostic.
    pub refresh_fn: Option<RefreshFn>,
    /// Refresh this many seconds before the access token expires (default 30).
    pub leeway_seconds: Option<u64>,
    /// Override the `slow_down` interval step (default 5s).
    pub slow_down_step_seconds: Option<u64>,
}

/// A started device authorization: user-facing fields plus a `poll()` that
/// blocks until approval and yields a refreshing credential.
pub struct DeviceFlowHandle {
    /// Short code the user confirms in the frontend.
    pub user_code: String,
    /// URL the user opens to approve the device.
    pub verification_uri: String,
    /// `verification_uri` with the code pre-filled, when the server supplied it.
    pub verification_uri_complete: Option<String>,
    /// Seconds until the device code expires.
    pub expires_in: u64,
    /// The `device_code` to poll with (exposed for advanced callers).
    pub device_code: String,
    interval: Option<u64>,
    poll: PollFn,
    options: BeginDeviceFlowOptions,
}

impl DeviceFlowHandle {
    /// Poll `token` (honouring `interval`/`slow_down`/`expires_in`) until the
    /// user approves, then return a [`RefreshingCredential`] seeded from the
    /// grant. Returns `Err(DeviceFlowError)` on expiry/denial, or a
    /// `TransportError` if a poll call fails.
    pub async fn poll(
        self,
    ) -> Result<Result<RefreshingCredential, DeviceFlowError>, TransportError> {
        let mut poll_opts = DevicePollOptions::default();
        if let Some(interval) = self.interval {
            poll_opts.interval_seconds = interval;
        }
        if let Some(step) = self.options.slow_down_step_seconds {
            poll_opts.slow_down_step_seconds = step;
        }

        let approved = match poll_device_token(self.poll, poll_opts).await? {
            Ok(response) => response,
            Err(err) => return Ok(Err(err)),
        };

        let (Some(access_token), Some(refresh_token)) =
            (approved.access_token, approved.refresh_token)
        else {
            return Ok(Err(DeviceFlowError::Denied));
        };

        let refresh_fn = self.options.refresh_fn.unwrap_or_else(missing_refresh_fn);
        Ok(Ok(RefreshingCredential::new(RefreshingCredentialInit {
            access_token,
            refresh_token,
            expires_in: approved.expires_in,
            refresh_fn,
            leeway_seconds: self.options.leeway_seconds,
        })))
    }
}

/// Begin RFC 8628 device authorization headlessly. Calls `start()`, returns the
/// user-facing URL + code immediately, and yields a [`DeviceFlowHandle`] whose
/// `.poll()` runs the [`poll_device_token`] loop with the server-advised
/// `interval` and maps the approved grant into a [`RefreshingCredential`].
///
/// `start` performs one `auth.device.start` call; `poll` performs one
/// `auth.device.token` call per invocation.
pub async fn begin_device_flow(
    start: StartFn,
    poll: PollFn,
    options: BeginDeviceFlowOptions,
) -> Result<DeviceFlowHandle, TransportError> {
    let started = start().await?;
    Ok(DeviceFlowHandle {
        user_code: started.user_code,
        verification_uri: started.verification_uri,
        verification_uri_complete: started.verification_uri_complete,
        expires_in: started.expires_in,
        device_code: started.device_code,
        interval: started.interval,
        poll,
        options,
    })
}

/// A `refresh_fn` placeholder that errors: device-flow credentials need one
/// supplied via [`BeginDeviceFlowOptions::refresh_fn`].
fn missing_refresh_fn() -> RefreshFn {
    Box::new(|_token| {
        Box::pin(async {
            Err(TransportError::Transport(anyhow::anyhow!(
                "device-flow credential has no refresh_fn; provide one in begin_device_flow options"
            )))
        })
    })
}
