//! LeavePulse SDK (Rust) — generated resource client for the platform contract.
//!
//! `cache`, `transport`, `resource` are the hand-written runtime. `client` and
//! `resources` are generated from the contract by leavepulse-sdk.

pub mod cache;
pub mod errors;
pub mod etag_store;
pub mod resource;
pub mod snowflake;
pub mod transport;

pub mod client;
pub mod models;
pub mod procedures;
pub mod resources;

pub use client::LeavePulse;
pub use errors::{
    field_errors_of, FieldError, HttpError, HttpErrorKind, LeavePulseError, ProblemDetails,
};
pub use etag_store::{
    default_cache_key, fetch_cached, fetch_cached_or_throw, EtagEntry, EtagStore, FileEtagStore,
    MemoryEtagStore,
};
pub use snowflake::Snowflake;
pub use transport::{
    BearerTransport, Channel, ConditionalOutcome, Method, RetryOptions, Transport, TransportError,
};
