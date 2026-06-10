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
    /// Stable machine-readable error code, UPPER_SNAKE (e.g. `RESOURCE_NOT_FOUND`).
    pub code: Option<String>,
    pub timestamp: Option<String>,
    /// Correlation id for support / log lookup.
    #[serde(alias = "requestId")]
    pub request_id: Option<String>,
    /// Originating service name.
    pub service: Option<String>,
    /// Extra structured context. For validation failures it holds
    /// `{ "errors": [{ "key", "message", "source" }], "path" }`.
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

/// A single per-field validation failure, normalized from the backend's
/// `details.errors` array (Litestar's `{ key, message, source }` shape).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldError {
    /// The offending field path (e.g. `email`, `body.address.city`).
    pub field: String,
    /// Human-readable validation message.
    pub message: String,
    /// Where it came from: `body` | `query` | `path` | …
    pub source: Option<String>,
}

/// Extract normalized per-field validation errors from a problem's `details`.
/// The backend reports them as `details.errors = [{ key, message, source }]`;
/// legacy/other shapes (`details.fields` object, pydantic `{ loc, msg }`) are
/// tolerated.
pub fn field_errors_of(problem: Option<&ProblemDetails>) -> Vec<FieldError> {
    let Some(details) = problem
        .and_then(|p| p.details.as_ref())
        .and_then(Value::as_object)
    else {
        return Vec::new();
    };

    let from_entry = |entry: &Value| -> Option<FieldError> {
        let obj = entry.as_object()?;
        let loc = obj
            .get("key")
            .or_else(|| obj.get("field"))
            .or_else(|| obj.get("path"))
            .or_else(|| obj.get("loc"));
        let field = match loc {
            Some(Value::Array(parts)) => parts
                .iter()
                .map(|p| {
                    p.as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| p.to_string())
                })
                .collect::<Vec<_>>()
                .join("."),
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => String::new(),
        };
        let message = obj
            .get("message")
            .or_else(|| obj.get("msg"))
            .or_else(|| obj.get("detail"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        if field.is_empty() && message.is_empty() {
            return None;
        }
        let source = obj.get("source").and_then(Value::as_str).map(str::to_owned);
        Some(FieldError {
            field,
            message,
            source,
        })
    };

    if let Some(Value::Array(errors)) = details.get("errors") {
        return errors.iter().filter_map(from_entry).collect();
    }
    // Legacy `details.fields = { email: "msg" }` object shape.
    if let Some(Value::Object(fields)) = details.get("fields") {
        return fields
            .iter()
            .map(|(field, msg)| FieldError {
                field: field.clone(),
                message: msg.as_str().unwrap_or_default().to_owned(),
                source: None,
            })
            .collect();
    }
    Vec::new()
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
    /// 422 — semantic validation failure (the backend's primary validation
    /// status, carrying `details.errors`).
    Validation,
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
            422 => Self::Validation,
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

    /// Whether the server's stable error code matches `code` — a transport-
    /// agnostic check that survives status remapping (`err.is_code("SESSION_EXPIRED")`).
    pub fn is_code(&self, code: &str) -> bool {
        self.code() == Some(code)
    }

    /// Correlation id for support, when present.
    pub fn request_id(&self) -> Option<&str> {
        self.problem.as_ref().and_then(|p| p.request_id.as_deref())
    }

    /// Normalized per-field validation errors, when the backend reported them
    /// (populated for 400/422 validation failures).
    pub fn field_errors(&self) -> Vec<FieldError> {
        field_errors_of(self.problem.as_ref())
    }

    pub fn is_bad_request(&self) -> bool {
        self.kind == HttpErrorKind::BadRequest
    }
    pub fn is_validation(&self) -> bool {
        self.kind == HttpErrorKind::Validation
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The backend's real validation body: 422 + `details.errors` array of
    /// `{key, message, source}`. The SDK must classify it as `Validation` and
    /// normalize the field errors.
    #[test]
    fn parses_backend_validation_422() {
        let raw = r#"{
            "type": "urn:platform-api:error:validation_failed",
            "title": "Request validation failed",
            "status": 422,
            "detail": "Request validation failed",
            "code": "VALIDATION_FAILED",
            "request_id": "abc-123",
            "details": {
                "errors": [
                    {"key": "email", "message": "value is not a valid email", "source": "body"},
                    {"key": "age", "message": "must be >= 0", "source": "body"}
                ],
                "path": null
            },
            "service": "platform-api"
        }"#;
        let err = HttpError::new(422, "POST", "/v1/users", raw.to_owned(), None);
        assert!(err.is_validation(), "422 → Validation kind");
        assert_eq!(err.code(), Some("VALIDATION_FAILED"));
        assert!(err.is_code("VALIDATION_FAILED"));
        assert_eq!(err.request_id(), Some("abc-123"));

        let fields = err.field_errors();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].field, "email");
        assert_eq!(fields[0].message, "value is not a valid email");
        assert_eq!(fields[0].source.as_deref(), Some("body"));
        assert_eq!(fields[1].field, "age");
    }

    /// A legacy `details.fields = { field: "msg" }` object is still tolerated.
    #[test]
    fn tolerates_legacy_fields_object() {
        let raw =
            r#"{"status":400,"code":"INVALID_INPUT","details":{"fields":{"name":"required"}}}"#;
        let err = HttpError::new(400, "POST", "/x", raw.to_owned(), None);
        let fields = err.field_errors();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].field, "name");
        assert_eq!(fields[0].message, "required");
    }

    /// A non-validation error yields no field errors and the right kind.
    #[test]
    fn classifies_not_found_without_fields() {
        let raw = r#"{"status":404,"code":"RESOURCE_NOT_FOUND","detail":"gone"}"#;
        let err = HttpError::new(404, "GET", "/x", raw.to_owned(), None);
        assert!(err.is_not_found());
        assert!(err.is_code("RESOURCE_NOT_FOUND"));
        assert!(err.field_errors().is_empty());
    }
}
