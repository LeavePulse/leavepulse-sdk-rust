//! LeavePulse SDK — OAuth2 authorization-code + PKCE facade for third-party apps
//! acting on behalf of a LeavePulse *user*.
//!
//! [`build_authorize_url`] is a pure helper: it mints a PKCE S256 pair and
//! assembles the authorize URL. The app sends the user to that URL; a *frontend*
//! page renders the visual consent (the SDK never drives a browser). After the
//! redirect-back with `?code=`, [`exchange_code`] trades the code for tokens at
//! `/auth/oauth/token` (form-urlencoded) and returns an auto-refreshing
//! [`OAuth2Credential`]. The exchange is driven through the caller's
//! [`AuthenticatedTransport`], so it works without the generated `auth.oauth2.*`
//! methods existing yet.

use std::sync::Arc;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::authenticated_transport::AuthenticatedTransport;
use crate::credentials::{OAuth2Credential, RefreshFn, RefreshingCredentialInit, TokenPair};
use crate::transport::{Channel, Method, TransportError};

/// Inputs for [`build_authorize_url`].
pub struct BuildAuthorizeUrlInit<'a> {
    /// OAuth2 client id of the third-party app.
    pub client_id: &'a str,
    /// Redirect URI registered for the app (must match at exchange).
    pub redirect_uri: &'a str,
    /// Requested scopes; joined with spaces per OAuth2.
    pub scope: &'a [&'a str],
    /// Base URL of the authorize page (a frontend URL).
    pub authorize_base_url: &'a str,
    /// CSRF/anti-forgery state; a random one is generated when `None`.
    pub state: Option<String>,
}

/// Result of [`build_authorize_url`]: the URL to send the user to, plus the PKCE
/// verifier and state the app must keep until [`exchange_code`].
#[derive(Debug, Clone)]
pub struct AuthorizeUrl {
    /// The authorize URL to open in the frontend (visual consent).
    pub url: String,
    /// PKCE `code_verifier` — keep secret, pass back to [`exchange_code`].
    pub code_verifier: String,
    /// The `state` echoed back on redirect — verify it matches.
    pub state: String,
}

/// Build an OAuth2 authorize URL with a fresh PKCE (S256) challenge. Pure (no
/// network). `response_type=code`, `code_challenge_method=S256`.
pub fn build_authorize_url(init: BuildAuthorizeUrlInit<'_>) -> AuthorizeUrl {
    let code_verifier = random_url_safe(64);
    let state = init.state.unwrap_or_else(|| random_url_safe(32));
    let code_challenge = s256_challenge(&code_verifier);

    let scope = init.scope.join(" ");
    let pairs = [
        ("response_type", "code"),
        ("client_id", init.client_id),
        ("redirect_uri", init.redirect_uri),
        ("scope", scope.as_str()),
        ("state", state.as_str()),
        ("code_challenge", code_challenge.as_str()),
        ("code_challenge_method", "S256"),
    ];
    let query = serde_urlencoded::to_string(pairs).unwrap_or_default();
    let base = init.authorize_base_url.trim_end_matches('/');
    AuthorizeUrl {
        url: format!("{base}?{query}"),
        code_verifier,
        state,
    }
}

/// Inputs for [`exchange_code`].
pub struct ExchangeCodeInit<'a> {
    pub client_id: &'a str,
    /// Authorization code returned to the redirect URI.
    pub code: &'a str,
    /// Same redirect URI used to build the authorize URL.
    pub redirect_uri: &'a str,
    /// The PKCE `code_verifier` from [`build_authorize_url`].
    pub code_verifier: &'a str,
}

/// `/auth/oauth/token` response shape (wire snake_case). Local to the runtime so
/// the facade does not depend on generated code.
#[derive(Debug, Clone, Deserialize)]
struct TokenExchangeResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    #[allow(dead_code)]
    token_type: Option<String>,
}

/// Exchange an authorization code for tokens at `/auth/oauth/token`
/// (form-urlencoded, `grant_type=authorization_code`) and return an
/// auto-refreshing [`OAuth2Credential`]. Driven through the supplied
/// `transport` (channel `Auth`), so it does not depend on generated code.
pub async fn exchange_code(
    init: ExchangeCodeInit<'_>,
    transport: Arc<AuthenticatedTransport>,
) -> Result<OAuth2Credential, TransportError> {
    let tokens = post_token(
        &transport,
        vec![
            ("grant_type".into(), "authorization_code".into()),
            ("code".into(), init.code.to_owned()),
            ("client_id".into(), init.client_id.to_owned()),
            ("redirect_uri".into(), init.redirect_uri.to_owned()),
            ("code_verifier".into(), init.code_verifier.to_owned()),
        ],
    )
    .await?;

    // Auto-refresh via the same /token endpoint (refresh_token grant).
    let client_id = init.client_id.to_owned();
    let refresh_transport = transport.clone();
    let refresh_fn: RefreshFn = Box::new(move |refresh_token: String| {
        let transport = refresh_transport.clone();
        let client_id = client_id.clone();
        Box::pin(async move {
            let tokens = post_token(
                &transport,
                vec![
                    ("grant_type".into(), "refresh_token".into()),
                    ("refresh_token".into(), refresh_token),
                    ("client_id".into(), client_id),
                ],
            )
            .await?;
            Ok(TokenPair {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                expires_in: tokens.expires_in,
            })
        })
    });

    Ok(OAuth2Credential::new(RefreshingCredentialInit {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token.unwrap_or_default(),
        expires_in: tokens.expires_in,
        refresh_fn,
        leeway_seconds: None,
    }))
}

/// POST a form-encoded grant request to `/auth/oauth/token` on the auth channel.
async fn post_token(
    transport: &AuthenticatedTransport,
    form: Vec<(String, String)>,
) -> Result<TokenExchangeResponse, TransportError> {
    let value = transport
        .form_request(Method::Post, "/auth/oauth/token", Channel::Auth, form)
        .await?;
    serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
}

/// Compute the PKCE S256 `code_challenge` (base64url, no padding) from a verifier.
fn s256_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}

/// A cryptographically-random URL-safe string with `byte_length` of entropy.
fn random_url_safe(byte_length: usize) -> String {
    let mut bytes = vec![0u8; byte_length];
    getrandom::fill(&mut bytes).expect("OS RNG unavailable");
    URL_SAFE_NO_PAD.encode(bytes)
}
