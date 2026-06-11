//! LeavePulse SDK (Rust) — generated resource client for the platform contract.
//!
//! `cache`, `transport`, `resource` are the hand-written runtime. `client` and
//! `resources` are generated from the contract by leavepulse-sdk.

pub mod authenticated_transport;
pub mod cache;
pub mod credentials;
pub mod device;
pub mod errors;
pub mod etag_store;
pub mod oauth2;
pub mod page;
pub mod resource;
pub mod snowflake;
pub mod transport;

pub mod client;
pub mod models;
pub mod procedures;
pub mod resources;

pub use authenticated_transport::{AuthenticatedTransport, RequestBody};
pub use client::LeavePulse;
pub use credentials::{
    CredentialProvider, OAuth2Credential, RefreshFn, RefreshingCredential,
    RefreshingCredentialInit, StaticCredential, TokenPair,
};
pub use device::{
    begin_device_flow, poll_device_token, BeginDeviceFlowOptions, DeviceFlowError,
    DeviceFlowHandle, DevicePollOptions, DevicePollStatus, DeviceStartResponse,
    DeviceTokenResponse,
};
pub use errors::{
    field_errors_of, FieldError, HttpError, HttpErrorKind, LeavePulseError, ProblemDetails,
};
pub use etag_store::{
    default_cache_key, fetch_cached, fetch_cached_or_throw, EtagEntry, EtagStore, FileEtagStore,
    MemoryEtagStore,
};
pub use oauth2::{
    build_authorize_url, exchange_code, AuthorizeUrl, BuildAuthorizeUrlInit, ExchangeCodeInit,
};
pub use page::{page_data_from, Page};
pub use snowflake::Snowflake;
pub use transport::{
    BearerTransport, Channel, ConditionalOutcome, Method, RetryOptions, Transport, TransportError,
};
