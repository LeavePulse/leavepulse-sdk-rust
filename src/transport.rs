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

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::Value;

use crate::authenticated_transport::AuthenticatedTransport;
use crate::credentials::StaticCredential;
use crate::errors::LeavePulseError;

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
    pub(crate) fn authenticated(self) -> bool {
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

    /// Conditional GET for the ETag cache: sends `If-None-Match` when
    /// `prior_etag` is given and treats `304`/`404` as outcomes rather than
    /// errors. Lives on the trait so the generated client (which holds a
    /// `Box<dyn Transport>`) can reach it; `fetch_cached` dispatches through it.
    async fn conditional(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        prior_etag: Option<&str>,
    ) -> Result<ConditionalOutcome, TransportError>;

    /// Write with an optimistic-concurrency `If-Match` validator. On the trait so
    /// the generated client can reach it through `Box<dyn Transport>`; the
    /// default ignores the validator for transports that don't support it.
    async fn request_with_if_match(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
        _if_match: Option<&str>,
    ) -> Result<Value, TransportError> {
        self.request(method, path, channel, body).await
    }
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

/// Bearer-token transport for external consumers (no cookies), over reqwest.
///
/// A thin specialization of [`AuthenticatedTransport`] over a
/// [`StaticCredential`], kept for back-compat: its public API
/// (`new`/`with_auth_base_url`/`with_retry`, the inherent `conditional` and
/// `request_with_if_match`, and the [`Transport`] impl) is unchanged. For tokens
/// that rotate (device flow, OAuth2, launcher) use [`AuthenticatedTransport`]
/// with a refreshing credential instead.
pub struct BearerTransport {
    inner: AuthenticatedTransport,
}

impl BearerTransport {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        let provider = Arc::new(StaticCredential::new(token));
        Self {
            inner: AuthenticatedTransport::new(base_url, provider),
        }
    }

    /// Set a distinct base URL for the `Auth` channel (auth core not co-hosted).
    pub fn with_auth_base_url(mut self, auth_base_url: impl Into<String>) -> Self {
        self.inner = self.inner.with_auth_base_url(auth_base_url);
        self
    }

    /// Override the automatic-retry tuning (429 / 5xx).
    pub fn with_retry(mut self, retry: RetryOptions) -> Self {
        self.inner = self.inner.with_retry(retry);
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
        self.inner.request(method, path, channel, body).await
    }

    async fn conditional(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        prior_etag: Option<&str>,
    ) -> Result<ConditionalOutcome, TransportError> {
        // Disambiguate from this trait method: call the inherent one.
        BearerTransport::conditional(self, method, path, channel, prior_etag).await
    }

    async fn request_with_if_match(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
        if_match: Option<&str>,
    ) -> Result<Value, TransportError> {
        BearerTransport::request_with_if_match(self, method, path, channel, body, if_match).await
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
    /// Conditional GET: sends `If-None-Match` when `prior_etag` is given and
    /// treats `304`/`404` as outcomes rather than errors, surfacing the response
    /// ETag. Retries 429/5xx like `request`. Kept as an inherent method so
    /// callers holding a concrete `BearerTransport` and wanting the
    /// `ConditionalOutcome` variant (e.g. the launcher's launch-manifest fetch,
    /// which must observe `NotModified`) reach it directly; the `Transport`
    /// trait method delegates here for `Box<dyn Transport>` callers.
    pub async fn conditional(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        prior_etag: Option<&str>,
    ) -> Result<ConditionalOutcome, TransportError> {
        self.inner
            .conditional_request(method, path, channel, prior_etag)
            .await
    }

    /// Like [`Transport::request`] but sends an `If-Match` header for optimistic
    /// concurrency on writes. A precondition failure surfaces as the usual
    /// `Conflict` (409) `HttpError`, so callers can prompt the user to sync.
    pub async fn request_with_if_match(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
        if_match: Option<&str>,
    ) -> Result<Value, TransportError> {
        self.inner
            .request_with_if_match(method, path, channel, body, if_match)
            .await
    }
}
