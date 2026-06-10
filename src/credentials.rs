//! LeavePulse SDK — credential providers.
//!
//! A [`CredentialProvider`] is the seam between *acquiring* a token (PAT, device
//! flow, OAuth2, service token) and *sending* it: [`AuthenticatedTransport`] asks
//! the provider for a bearer before each request and, on a `401`, for a refresh.
//! Providers are transport-agnostic — anything that performs a network call (e.g.
//! a token refresh) is injected, never hardcoded — so the same provider works
//! from a CLI, a service, or the launcher.
//!
//! [`AuthenticatedTransport`]: crate::transport::AuthenticatedTransport

use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tokio::sync::Mutex as AsyncMutex;

use crate::transport::TransportError;

/// Supplies the bearer token a transport sends, and optionally rotates it.
///
/// - `token()` returns the **current** access token to send.
/// - `can_refresh()` reports whether `refresh()` does anything; the transport
///   only attempts a refresh-and-retry on a `401` when it returns `true`.
/// - `refresh()` rotates the credential (e.g. exchanges a refresh token); the
///   default is a no-op for non-rotating credentials (PAT, service token).
#[async_trait]
pub trait CredentialProvider: Send + Sync {
    /// The current bearer token to send on the next request.
    async fn token(&self) -> Result<String, TransportError>;

    /// Whether this credential can rotate (so a `401` retry is worth trying).
    /// Defaults to `false`; rotating credentials override it.
    fn can_refresh(&self) -> bool {
        false
    }

    /// Rotate the credential (e.g. exchange a refresh token). Default no-op so
    /// fixed credentials need not implement it.
    async fn refresh(&self) -> Result<(), TransportError> {
        Ok(())
    }
}

/// A fixed, non-rotating credential: `token()` always returns the same value and
/// there is no refresh. Use for Personal Access Tokens and out-of-band service
/// tokens — [`BearerTransport`] is exactly this.
///
/// [`BearerTransport`]: crate::transport::BearerTransport
#[derive(Debug, Clone)]
pub struct StaticCredential(pub String);

impl StaticCredential {
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }
}

#[async_trait]
impl CredentialProvider for StaticCredential {
    async fn token(&self) -> Result<String, TransportError> {
        Ok(self.0.clone())
    }
}

/// The token pair a refresh call yields (wire shape: snake_case).
#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: Option<String>,
    /// Access-token lifetime in seconds, used to compute the local expiry.
    pub expires_in: Option<u64>,
}

/// Performs the actual refresh exchange. Injected so the credential stays
/// transport-agnostic (no hardcoded URL/HTTP client): the caller decides whether
/// it hits the transport, a raw client, or a mock. Takes the current refresh
/// token and yields a fresh [`TokenPair`].
pub type RefreshFn = Box<
    dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<TokenPair, TransportError>> + Send>>
        + Send
        + Sync,
>;

/// Seed tokens for a [`RefreshingCredential`].
pub struct RefreshingCredentialInit {
    pub access_token: String,
    pub refresh_token: String,
    /// Access-token lifetime in seconds, from the issuing response.
    pub expires_in: Option<u64>,
    /// Performs the refresh-token exchange.
    pub refresh_fn: RefreshFn,
    /// Refresh this many seconds before the computed expiry (default 30).
    pub leeway_seconds: Option<u64>,
}

/// Mutable token state guarded by a sync `Mutex` (only ever held briefly).
struct TokenState {
    access_token: String,
    refresh_token: String,
    /// When the access token expires, or `None` if unknown.
    expires_at: Option<Instant>,
}

/// Holds an access + refresh token locally and rotates them on demand.
/// `token()` returns the access token, transparently refreshing first if it is
/// known to be expired (within the leeway window); `refresh()` forces an
/// exchange (called by the transport on a `401`). The network exchange is
/// delegated to the injected `refresh_fn`, so this works for the launcher
/// (refresh-in-body), device flow, and any other rotating credential without
/// knowing the URL or transport.
///
/// Concurrent refreshes are coalesced into one in-flight exchange via the
/// async `refresh_lock` (a `401` retry racing an expiry check share the result).
pub struct RefreshingCredential {
    state: Mutex<TokenState>,
    refresh_fn: RefreshFn,
    leeway: Duration,
    /// Serializes refreshes so concurrent callers coalesce onto one exchange.
    refresh_lock: AsyncMutex<()>,
}

impl RefreshingCredential {
    pub fn new(init: RefreshingCredentialInit) -> Self {
        let leeway = Duration::from_secs(init.leeway_seconds.unwrap_or(30));
        Self {
            state: Mutex::new(TokenState {
                access_token: init.access_token,
                refresh_token: init.refresh_token,
                expires_at: expiry_from_seconds(init.expires_in),
            }),
            refresh_fn: init.refresh_fn,
            leeway,
            refresh_lock: AsyncMutex::new(()),
        }
    }

    /// The current refresh token (for persisting to a local store).
    pub fn current_refresh_token(&self) -> String {
        self.state.lock().unwrap().refresh_token.clone()
    }

    /// Whether the access token is at/within the leeway window of expiring.
    fn is_expiring(&self) -> bool {
        let state = self.state.lock().unwrap();
        match state.expires_at {
            Some(at) => Instant::now() + self.leeway >= at,
            None => false,
        }
    }

    /// Adopt a freshly-issued token pair (keeps the old refresh token if the
    /// server rotated only the access token).
    fn apply_token_pair(&self, pair: TokenPair) {
        let mut state = self.state.lock().unwrap();
        state.access_token = pair.access_token;
        if let Some(refresh) = pair.refresh_token {
            state.refresh_token = refresh;
        }
        state.expires_at = expiry_from_seconds(pair.expires_in);
    }
}

#[async_trait]
impl CredentialProvider for RefreshingCredential {
    async fn token(&self) -> Result<String, TransportError> {
        // Refresh `leeway` early so the token is still valid for the request.
        if self.is_expiring() {
            self.refresh().await?;
        }
        Ok(self.state.lock().unwrap().access_token.clone())
    }

    fn can_refresh(&self) -> bool {
        true
    }

    async fn refresh(&self) -> Result<(), TransportError> {
        // Coalesce concurrent callers: the first through the lock refreshes; the
        // rest, once it releases, see the token is no longer expiring and skip.
        let _guard = self.refresh_lock.lock().await;
        if !self.is_expiring() && self.state.lock().unwrap().expires_at.is_some() {
            // A racing caller already refreshed while we waited on the lock.
            return Ok(());
        }
        let refresh_token = self.state.lock().unwrap().refresh_token.clone();
        let pair = (self.refresh_fn)(refresh_token).await?;
        self.apply_token_pair(pair);
        Ok(())
    }
}

/// A credential seeded from an OAuth2 authorization-code token exchange that
/// auto-refreshes via the same `/auth/oauth/token` endpoint. Behaviour is
/// identical to [`RefreshingCredential`]; it exists as a named type so
/// `oauth2::exchange_code` can return a self-describing credential. The refresh
/// exchange is still injected (`refresh_fn`) to stay transport-agnostic.
pub struct OAuth2Credential(RefreshingCredential);

impl OAuth2Credential {
    pub fn new(init: RefreshingCredentialInit) -> Self {
        Self(RefreshingCredential::new(init))
    }

    /// The current refresh token (for persisting to a local store).
    pub fn current_refresh_token(&self) -> String {
        self.0.current_refresh_token()
    }
}

#[async_trait]
impl CredentialProvider for OAuth2Credential {
    async fn token(&self) -> Result<String, TransportError> {
        self.0.token().await
    }

    fn can_refresh(&self) -> bool {
        true
    }

    async fn refresh(&self) -> Result<(), TransportError> {
        self.0.refresh().await
    }
}

/// Convert a seconds-from-now lifetime into an absolute monotonic expiry.
fn expiry_from_seconds(expires_in: Option<u64>) -> Option<Instant> {
    expires_in.map(|secs| Instant::now() + Duration::from_secs(secs))
}
