//! LeavePulse Snowflake id type.
//!
//! Rust can keep Snowflakes as i64; TypeScript exposes the same domain type as
//! a string to avoid JavaScript safe-integer loss.

pub type Snowflake = i64;
