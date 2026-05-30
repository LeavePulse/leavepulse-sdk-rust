//! LeavePulse SDK — error model (discord.py-style, Rust idiom).
//!
//! Every failure is a [`LeavePulseError`]. HTTP failures carry an [`HttpError`]
//! whose [`HttpErrorKind`] is chosen by status code, so callers can match:
//! `match err { LeavePulseError::Http(h) if h.is_not_found() => ... }` or
//! `if let Some(h) = err.http() { matches!(h.kind, HttpErrorKind::NotFound) }`.
//! The backend speaks RFC 7807 problem+json (service-toolkit / awesome_errors),
//! so the parsed [`ProblemDetails`] carries the machine-readable `code`, human
//! `detail`, validation `fields`, and `request_id` for support.

use serde::Deserialize;
use serde_json::Value;

/// RFC 7807 problem details as emitted by the LeavePulse backend.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProblemDetails {
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub status: Option<u16>,
    /// Human-readable explanation specific to this occurrence.
    pub detail: Option<String>,
    pub instance: Option<String>,
    /// Stable machine-readable error code (e.g. `whitelist.not_found`).
    pub code: Option<String>,
    pub timestamp: Option<String>,
    /// Correlation id for support / log lookup.
    #[serde(alias = "requestId")]
    pub request_id: Option<String>,
    /// Originating service name.
    pub service: Option<String>,
    /// Extra structured context, including per-field validation errors.
    pub details: Option<Value>,
}

impl ProblemDetails {
    /// Parse a response body as RFC 7807 problem+json; `None` if it isn't JSON.
    pub fn parse(raw: &str) -> Option<Self> {
        if raw.is_empty() {
            return None;
        }
        serde_json::from_str(raw).ok()
    }
}

/// The status-derived category of an [`HttpError`].
#[derive(Debug, Clone, PartialEq)]
pub enum HttpErrorKind {
    /// 400 — malformed request / failed validation.
    BadRequest,
    /// 401 — authentication required or failed.
    Unauthorized,
    /// 403 — authenticated but not permitted.
    Forbidden,
    /// 404 — resource not found.
    NotFound,
    /// 409 — state conflict (e.g. duplicate, already-exists).
    Conflict,
    /// 429 — rate limited; `retry_after` is the server-advised wait (s).
    RateLimited { retry_after: Option<f64> },
    /// 5xx — the server failed to fulfil a valid request.
    Server,
    /// Any other non-2xx status.
    Other,
}

impl HttpErrorKind {
    fn from_status(status: u16, retry_after: Option<f64>) -> Self {
        match status {
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            409 => Self::Conflict,
            429 => Self::RateLimited { retry_after },
            s if s >= 500 => Self::Server,
            _ => Self::Other,
        }
    }
}

/// A non-2xx HTTP response, classified by status into [`HttpErrorKind`].
#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: u16,
    pub kind: HttpErrorKind,
    pub method: String,
    pub path: String,
    pub problem: Option<ProblemDetails>,
    /// Raw response body text when it wasn't valid problem+json.
    pub raw: String,
}

impl HttpError {
    /// Build the classified error from a failed response.
    pub fn new(
        status: u16,
        method: impl Into<String>,
        path: impl Into<String>,
        raw: String,
        retry_after: Option<f64>,
    ) -> Self {
        let problem = ProblemDetails::parse(&raw);
        Self {
            status,
            kind: HttpErrorKind::from_status(status, retry_after),
            method: method.into(),
            path: path.into(),
            problem,
            raw,
        }
    }

    /// Machine-readable error code, when the server supplied one.
    pub fn code(&self) -> Option<&str> {
        self.problem.as_ref().and_then(|p| p.code.as_deref())
    }

    /// Correlation id for support, when present.
    pub fn request_id(&self) -> Option<&str> {
        self.problem.as_ref().and_then(|p| p.request_id.as_deref())
    }

    /// Per-field validation errors (400), when the backend reported them.
    pub fn fields(&self) -> Option<&serde_json::Map<String, Value>> {
        let details = self.problem.as_ref()?.details.as_ref()?.as_object()?;
        details
            .get("fields")
            .or_else(|| details.get("errors"))
            .and_then(Value::as_object)
    }

    pub fn is_bad_request(&self) -> bool {
        self.kind == HttpErrorKind::BadRequest
    }
    pub fn is_unauthorized(&self) -> bool {
        self.kind == HttpErrorKind::Unauthorized
    }
    pub fn is_forbidden(&self) -> bool {
        self.kind == HttpErrorKind::Forbidden
    }
    pub fn is_not_found(&self) -> bool {
        self.kind == HttpErrorKind::NotFound
    }
    pub fn is_conflict(&self) -> bool {
        self.kind == HttpErrorKind::Conflict
    }
    pub fn is_rate_limited(&self) -> bool {
        matches!(self.kind, HttpErrorKind::RateLimited { .. })
    }
    pub fn is_server(&self) -> bool {
        self.kind == HttpErrorKind::Server
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code().map(|c| format!(" [{c}]")).unwrap_or_default();
        let detail = self
            .problem
            .as_ref()
            .and_then(|p| p.detail.as_deref().or(p.title.as_deref()))
            .unwrap_or(&self.raw);
        if detail.is_empty() {
            write!(f, "{} {} -> {}{code}", self.method, self.path, self.status)
        } else {
            write!(
                f,
                "{} {} -> {}{code}: {detail}",
                self.method, self.path, self.status
            )
        }
    }
}

/// Every error the SDK raises.
#[derive(Debug, thiserror::Error)]
pub enum LeavePulseError {
    /// A non-2xx HTTP response, classified by status. Boxed to keep the enum
    /// small (the variants otherwise differ widely in size).
    #[error("{0}")]
    Http(Box<HttpError>),
    /// The request succeeded but the body is unusable (e.g. a resource with no
    /// id to identity-map on).
    #[error("{message}")]
    Malformed { message: String, payload: Value },
    /// A transport / IO failure before a status was seen.
    #[error(transparent)]
    Transport(#[from] anyhow::Error),
}

impl From<HttpError> for LeavePulseError {
    fn from(error: HttpError) -> Self {
        LeavePulseError::Http(Box::new(error))
    }
}

impl LeavePulseError {
    /// The classified HTTP error, if this is an HTTP failure.
    pub fn http(&self) -> Option<&HttpError> {
        match self {
            LeavePulseError::Http(h) => Some(h),
            _ => None,
        }
    }
}
