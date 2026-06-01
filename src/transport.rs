//! LeavePulse SDK — transport abstraction.
//!
//! The SDK never knows how requests are authenticated or sent; it only calls
//! `transport.request(...)`. Adapters supply the mechanism — `BearerTransport`
//! (reqwest, `Authorization: Bearer`) for external use, or a custom impl.
//!
//! HTTP failures surface as the typed [`crate::errors::LeavePulseError`]
//! hierarchy (classified by status); `BearerTransport` additionally retries 429
//! (honouring `Retry-After`) and 5xx (exponential backoff) up to
//! `RetryOptions::max_retries`.

use std::time::Duration;

use async_trait::async_trait;
use serde_json::Value;

use crate::errors::{HttpError, HttpErrorKind, LeavePulseError};

/// The error type returned by every transport call. Alias kept for the
/// generated client/resources, which name `transport::TransportError`.
pub type TransportError = LeavePulseError;

/// HTTP method for a transport request.
#[derive(Debug, Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Method {
    pub fn as_str(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
        }
    }
}

/// Which backend a request targets. `Platform` is the BFF (`/v1`); `Auth` is
/// the auth-service core (`/auth`) carrying login/refresh/oauth. The adapter
/// maps each channel to a base URL and to its own auth mechanism; the SDK never
/// learns *how* a channel authenticates, only which one to hit.
///
/// `PlatformPublic` is the BFF too, but for routes the contract marks as public
/// (`exclude_from_auth`): the generator selects it for operations with no
/// `security` block. Adapters must not attach a credential — a stale bearer
/// would turn an otherwise-open route into a 401.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Channel {
    #[default]
    Platform,
    PlatformPublic,
    Auth,
}

impl Channel {
    /// Whether a credential should be attached on this channel.
    fn authenticated(self) -> bool {
        !matches!(self, Channel::PlatformPublic)
    }
}

/// Anything that can dispatch a request and return decoded JSON.
#[async_trait]
pub trait Transport: Send + Sync {
    async fn request(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
    ) -> Result<Value, TransportError>;
}

/// Tuning for a transport's automatic retry behaviour.
#[derive(Debug, Clone, Copy)]
pub struct RetryOptions {
    /// Max automatic retries on 429 / 5xx. Set 0 to disable.
    pub max_retries: u32,
    /// Base backoff for 5xx exponential backoff.
    pub backoff_base: Duration,
    /// Cap on any single backoff wait.
    pub backoff_max: Duration,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            max_retries: 2,
            backoff_base: Duration::from_millis(250),
            backoff_max: Duration::from_secs(10),
        }
    }
}

/// Parse a `Retry-After` header value (delta-seconds) into seconds.
fn parse_retry_after(value: Option<&str>) -> Option<f64> {
    value.and_then(|v| v.trim().parse::<f64>().ok())
}

/// Bearer-token transport for external consumers (no cookies), over reqwest.
pub struct BearerTransport {
    base_url: String,
    /// Auth-service base URL for the `Auth` channel; defaults to `base_url`.
    auth_base_url: String,
    token: String,
    client: reqwest::Client,
    retry: RetryOptions,
}

impl BearerTransport {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        let base = base_url.into().trim_end_matches('/').to_string();
        Self {
            auth_base_url: base.clone(),
            base_url: base,
            token: token.into(),
            client: reqwest::Client::new(),
            retry: RetryOptions::default(),
        }
    }

    /// Set a distinct base URL for the `Auth` channel (auth core not co-hosted).
    pub fn with_auth_base_url(mut self, auth_base_url: impl Into<String>) -> Self {
        self.auth_base_url = auth_base_url.into().trim_end_matches('/').to_string();
        self
    }

    /// Override the automatic-retry tuning (429 / 5xx).
    pub fn with_retry(mut self, retry: RetryOptions) -> Self {
        self.retry = retry;
        self
    }
}

#[async_trait]
impl Transport for BearerTransport {
    async fn request(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
    ) -> Result<Value, TransportError> {
        let base = match channel {
            Channel::Auth => &self.auth_base_url,
            Channel::Platform | Channel::PlatformPublic => &self.base_url,
        };
        let url = format!("{base}{path}");
        let reqwest_method = reqwest::Method::from_bytes(method.as_str().as_bytes())
            .map_err(|e| LeavePulseError::Transport(e.into()))?;

        let mut attempt = 0u32;
        loop {
            let mut req = self.client.request(reqwest_method.clone(), &url);
            if channel.authenticated() {
                req = req.bearer_auth(&self.token);
            }
            if let Some(body) = &body {
                req = req.json(body);
            }
            let response = req
                .send()
                .await
                .map_err(|e| LeavePulseError::Transport(e.into()))?;
            let status = response.status();

            if status.is_success() {
                if status.as_u16() == 204 {
                    return Ok(Value::Null);
                }
                return response
                    .json()
                    .await
                    .map_err(|e| LeavePulseError::Transport(e.into()));
            }

            let retry_after = parse_retry_after(
                response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok()),
            );
            let raw = response.text().await.unwrap_or_default();
            let error = HttpError::new(status.as_u16(), method.as_str(), path, raw, retry_after);

            let retriable = error.is_rate_limited() || error.is_server();
            if retriable && attempt < self.retry.max_retries {
                let wait = match &error.kind {
                    HttpErrorKind::RateLimited {
                        retry_after: Some(secs),
                    } => Duration::from_secs_f64(secs.max(0.0)),
                    _ => {
                        let backoff = self
                            .retry
                            .backoff_base
                            .saturating_mul(2u32.saturating_pow(attempt));
                        backoff.min(self.retry.backoff_max)
                    }
                };
                attempt += 1;
                tokio::time::sleep(wait).await;
                continue;
            }
            return Err(LeavePulseError::from(error));
        }
    }
}

/// Outcome of a conditional (`If-None-Match`) request, mirroring the launcher's
/// `FetchOutcome`: a fresh body with its new ETag, an unchanged `304`, or a
/// `404` (e.g. nothing published yet).
#[derive(Debug, Clone)]
pub enum ConditionalOutcome {
    /// The body changed (or there was no prior ETag); `etag` is the new
    /// validator to cache alongside it.
    Modified { data: Value, etag: Option<String> },
    /// Server returned `304` — reuse the cached copy.
    NotModified { etag: Option<String> },
    /// Server returned `404`.
    NotFound,
}

impl BearerTransport {
    /// Like [`Transport::request`] but for caching: sends `If-None-Match` when
    /// `prior_etag` is given and treats `304`/`404` as outcomes rather than
    /// errors, surfacing the response ETag. Retries 429/5xx like `request`.
    pub async fn conditional(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        prior_etag: Option<&str>,
    ) -> Result<ConditionalOutcome, TransportError> {
        let base = match channel {
            Channel::Auth => &self.auth_base_url,
            Channel::Platform | Channel::PlatformPublic => &self.base_url,
        };
        let url = format!("{base}{path}");
        let reqwest_method = reqwest::Method::from_bytes(method.as_str().as_bytes())
            .map_err(|e| LeavePulseError::Transport(e.into()))?;

        let mut attempt = 0u32;
        loop {
            let mut req = self.client.request(reqwest_method.clone(), &url);
            if channel.authenticated() {
                req = req.bearer_auth(&self.token);
            }
            if let Some(etag) = prior_etag {
                req = req.header("if-none-match", etag);
            }
            let response = req
                .send()
                .await
                .map_err(|e| LeavePulseError::Transport(e.into()))?;
            let status = response.status();
            let etag = response
                .headers()
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);

            if status.as_u16() == 404 {
                return Ok(ConditionalOutcome::NotFound);
            }
            if status.as_u16() == 304 {
                return Ok(ConditionalOutcome::NotModified { etag });
            }
            if status.is_success() {
                let data = if status.as_u16() == 204 {
                    Value::Null
                } else {
                    response
                        .json()
                        .await
                        .map_err(|e| LeavePulseError::Transport(e.into()))?
                };
                return Ok(ConditionalOutcome::Modified { data, etag });
            }

            let retry_after = parse_retry_after(
                response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok()),
            );
            let raw = response.text().await.unwrap_or_default();
            let error = HttpError::new(status.as_u16(), method.as_str(), path, raw, retry_after);

            let retriable = error.is_rate_limited() || error.is_server();
            if retriable && attempt < self.retry.max_retries {
                let wait = match &error.kind {
                    HttpErrorKind::RateLimited {
                        retry_after: Some(secs),
                    } => Duration::from_secs_f64(secs.max(0.0)),
                    _ => self
                        .retry
                        .backoff_base
                        .saturating_mul(2u32.saturating_pow(attempt))
                        .min(self.retry.backoff_max),
                };
                attempt += 1;
                tokio::time::sleep(wait).await;
                continue;
            }
            return Err(LeavePulseError::from(error));
        }
    }
}
