//! LeavePulse SDK — credential-driven bearer transport.
//!
//! [`AuthenticatedTransport`] is the general bearer adapter: before every
//! request it asks a [`CredentialProvider`] for the current token; on a `401`,
//! if the provider [`can_refresh`](CredentialProvider::can_refresh) it refreshes
//! **once** and retries the request **once** — covering device-flow / OAuth2 /
//! launcher tokens that rotate. It reuses the same 429 / 5xx retry+backoff as
//! [`BearerTransport`], which is now a thin wrapper over it with a
//! [`StaticCredential`].
//!
//! [`StaticCredential`]: crate::credentials::StaticCredential

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::Value;

use crate::credentials::CredentialProvider;
use crate::errors::{HttpError, HttpErrorKind, LeavePulseError};
use crate::transport::{
    Channel, ConditionalOutcome, Method, RetryOptions, Transport, TransportError,
};

/// The body to send with a request. Kinds are mutually exclusive: `Json`
/// serializes to `application/json`; `Form` to `application/x-www-form-urlencoded`
/// (required by the OAuth2 `/auth/oauth/token` endpoint); `None` sends no body.
#[derive(Debug, Clone, Default)]
pub enum RequestBody {
    /// No request body.
    #[default]
    None,
    /// A JSON body.
    Json(Value),
    /// A form-urlencoded body (ordered key/value pairs).
    Form(Vec<(String, String)>),
}

impl RequestBody {
    fn from_json(body: Option<Value>) -> Self {
        match body {
            Some(value) => RequestBody::Json(value),
            None => RequestBody::None,
        }
    }
}

/// Parse a `Retry-After` header value (delta-seconds) into seconds.
fn parse_retry_after(value: Option<&str>) -> Option<f64> {
    value.and_then(|v| v.trim().parse::<f64>().ok())
}

/// Bearer transport driven by a [`CredentialProvider`] rather than a fixed
/// token. Uses reqwest; retries 429 (honouring `Retry-After`) and 5xx
/// (exponential backoff) up to [`RetryOptions::max_retries`], and refreshes the
/// credential once on a `401` before retrying.
#[derive(Clone)]
pub struct AuthenticatedTransport {
    base_url: String,
    /// Auth-service base URL for the `Auth` channel; defaults to `base_url`.
    auth_base_url: String,
    provider: Arc<dyn CredentialProvider>,
    client: reqwest::Client,
    retry: RetryOptions,
    /// When set, every request carries `X-On-Behalf-Of: <subject>`. Only a bot
    /// account may use this: the bot's own credential still authenticates the
    /// call, and the platform resolves `subject` (`<source>:<id>`, e.g.
    /// `discord:123` or `leavepulse:42`) to the human the bot acts for. The
    /// effective permissions are the intersection of the bot's and the human's
    /// — on-behalf never escalates. Set via [`on_behalf_of`](Self::on_behalf_of).
    on_behalf_subject: Option<String>,
}

impl AuthenticatedTransport {
    pub fn new(base_url: impl Into<String>, provider: Arc<dyn CredentialProvider>) -> Self {
        let base = base_url.into().trim_end_matches('/').to_string();
        Self {
            auth_base_url: base.clone(),
            base_url: base,
            provider,
            client: reqwest::Client::new(),
            retry: RetryOptions::default(),
            on_behalf_subject: None,
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

    /// Return a transport that sends every request on behalf of `subject`,
    /// sharing this transport's credential and config. `subject` is
    /// `<source>:<id>` — `discord:<id>`, `telegram:<id>`, or
    /// `leavepulse:<user_id>`. Intended for bot accounts; the platform ignores
    /// the header for non-bot callers.
    pub fn on_behalf_of(mut self, subject: impl Into<String>) -> Self {
        self.on_behalf_subject = Some(subject.into());
        self
    }

    fn base_for(&self, channel: Channel) -> &str {
        match channel {
            Channel::Auth => &self.auth_base_url,
            Channel::Platform | Channel::PlatformPublic => &self.base_url,
        }
    }

    /// Apply auth + on-behalf headers for an authenticated channel. Bearer comes
    /// from the provider; the on-behalf header rides alongside when set.
    async fn apply_auth(
        &self,
        mut req: reqwest::RequestBuilder,
        channel: Channel,
    ) -> Result<reqwest::RequestBuilder, TransportError> {
        if channel.authenticated() {
            let token = self.provider.token().await?;
            req = req.bearer_auth(token);
            if let Some(subject) = &self.on_behalf_subject {
                req = req.header("x-on-behalf-of", subject);
            }
        }
        Ok(req)
    }

    /// Apply the body kind to a request builder.
    fn apply_body(req: reqwest::RequestBuilder, body: &RequestBody) -> reqwest::RequestBuilder {
        match body {
            RequestBody::None => req,
            RequestBody::Json(value) => req.json(value),
            RequestBody::Form(pairs) => req.form(pairs),
        }
    }

    /// Compute the backoff wait for a retriable error at `attempt`.
    fn backoff_wait(&self, error: &HttpError, attempt: u32) -> Duration {
        match &error.kind {
            HttpErrorKind::RateLimited {
                retry_after: Some(secs),
            } => Duration::from_secs_f64(secs.max(0.0)),
            _ => self
                .retry
                .backoff_base
                .saturating_mul(2u32.saturating_pow(attempt))
                .min(self.retry.backoff_max),
        }
    }

    /// Dispatch a request with the given body kind (JSON or form), decoding a
    /// JSON value (or `Null` on 204). Public so the OAuth2 facade can POST a
    /// form-encoded grant without the generated `auth.oauth2.*` methods.
    pub async fn send_request(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: RequestBody,
    ) -> Result<Value, TransportError> {
        let base = self.base_for(channel);
        let url = format!("{base}{path}");
        let reqwest_method = reqwest::Method::from_bytes(method.as_str().as_bytes())
            .map_err(|e| LeavePulseError::Transport(e.into()))?;

        let mut attempt = 0u32;
        let mut refreshed = false;
        loop {
            let mut req = self.client.request(reqwest_method.clone(), &url);
            req = self.apply_auth(req, channel).await?;
            req = Self::apply_body(req, &body);

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

            if error.is_unauthorized() && !refreshed && self.provider.can_refresh() {
                refreshed = true;
                self.provider.refresh().await?;
                continue;
            }

            let retriable = error.is_rate_limited() || error.is_server();
            if retriable && attempt < self.retry.max_retries {
                let wait = self.backoff_wait(&error, attempt);
                attempt += 1;
                tokio::time::sleep(wait).await;
                continue;
            }
            return Err(LeavePulseError::from(error));
        }
    }

    /// Form-encoded POST/PUT/etc. convenience over [`send_request`].
    ///
    /// [`send_request`]: AuthenticatedTransport::send_request
    pub async fn form_request(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        form: Vec<(String, String)>,
    ) -> Result<Value, TransportError> {
        self.send_request(method, path, channel, RequestBody::Form(form))
            .await
    }

    /// Conditional GET (`If-None-Match`) treating 304/404 as outcomes. Refreshes
    /// once on 401 and retries 429/5xx like [`send_request`].
    ///
    /// [`send_request`]: AuthenticatedTransport::send_request
    pub async fn conditional_request(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        prior_etag: Option<&str>,
    ) -> Result<ConditionalOutcome, TransportError> {
        let base = self.base_for(channel);
        let url = format!("{base}{path}");
        let reqwest_method = reqwest::Method::from_bytes(method.as_str().as_bytes())
            .map_err(|e| LeavePulseError::Transport(e.into()))?;

        let mut attempt = 0u32;
        let mut refreshed = false;
        loop {
            let mut req = self.client.request(reqwest_method.clone(), &url);
            req = self.apply_auth(req, channel).await?;
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

            if error.is_unauthorized() && !refreshed && self.provider.can_refresh() {
                refreshed = true;
                self.provider.refresh().await?;
                continue;
            }

            let retriable = error.is_rate_limited() || error.is_server();
            if retriable && attempt < self.retry.max_retries {
                let wait = self.backoff_wait(&error, attempt);
                attempt += 1;
                tokio::time::sleep(wait).await;
                continue;
            }
            return Err(LeavePulseError::from(error));
        }
    }

    /// Like [`send_request`] but sends an `If-Match` header for optimistic
    /// concurrency on writes; a precondition failure surfaces as `Conflict`.
    ///
    /// [`send_request`]: AuthenticatedTransport::send_request
    pub async fn request_with_if_match(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
        if_match: Option<&str>,
    ) -> Result<Value, TransportError> {
        let base = self.base_for(channel);
        let url = format!("{base}{path}");
        let reqwest_method = reqwest::Method::from_bytes(method.as_str().as_bytes())
            .map_err(|e| LeavePulseError::Transport(e.into()))?;
        let body = RequestBody::from_json(body);

        let mut attempt = 0u32;
        let mut refreshed = false;
        loop {
            let mut req = self.client.request(reqwest_method.clone(), &url);
            req = self.apply_auth(req, channel).await?;
            if let Some(etag) = if_match {
                req = req.header("if-match", etag);
            }
            req = Self::apply_body(req, &body);

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

            if error.is_unauthorized() && !refreshed && self.provider.can_refresh() {
                refreshed = true;
                self.provider.refresh().await?;
                continue;
            }

            let retriable = error.is_rate_limited() || error.is_server();
            if retriable && attempt < self.retry.max_retries {
                let wait = self.backoff_wait(&error, attempt);
                attempt += 1;
                tokio::time::sleep(wait).await;
                continue;
            }
            return Err(LeavePulseError::from(error));
        }
    }
}

#[async_trait]
impl Transport for AuthenticatedTransport {
    async fn request(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
    ) -> Result<Value, TransportError> {
        self.send_request(method, path, channel, RequestBody::from_json(body))
            .await
    }

    async fn conditional(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        prior_etag: Option<&str>,
    ) -> Result<ConditionalOutcome, TransportError> {
        self.conditional_request(method, path, channel, prior_etag)
            .await
    }

    async fn request_with_if_match(
        &self,
        method: Method,
        path: &str,
        channel: Channel,
        body: Option<Value>,
        if_match: Option<&str>,
    ) -> Result<Value, TransportError> {
        AuthenticatedTransport::request_with_if_match(self, method, path, channel, body, if_match)
            .await
    }
}
