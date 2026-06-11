//! LeavePulse Snowflake id type.
//!
//! A 64-bit Snowflake identifier. The contract carries it as an `integer` on the
//! wire, and Rust keeps it as an `i64` in memory — no precision is ever lost, so
//! you work with it like a normal integer (`From<i64>`, `as_i64`, `Ord`, masks).
//!
//! What this newtype adds over a bare `i64` is **safe re-serialization**: it
//! serializes as a JSON *string* and deserializes from either a string or a
//! number. That matters at any boundary a Snowflake crosses on its way to a
//! safe-integer-limited consumer (JavaScript / a Tauri IPC payload), where a
//! plain `number` past 2^53 would silently lose its low digits. The backend's
//! `integer` body still deserializes fine; only the *outgoing* representation is
//! a string, so the value survives the trip regardless of who reads it.

use std::fmt;

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};

/// A 64-bit Snowflake id. `i64` in memory, JSON string on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Snowflake(pub i64);

impl Snowflake {
    /// The underlying `i64`. Use this wherever an integer id is needed (path
    /// params, snowflake bit math, comparisons).
    pub const fn as_i64(self) -> i64 {
        self.0
    }
}

impl From<i64> for Snowflake {
    fn from(value: i64) -> Self {
        Snowflake(value)
    }
}

impl From<Snowflake> for i64 {
    fn from(value: Snowflake) -> Self {
        value.0
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Snowflake {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

/// Accepts a Snowflake as a JSON string (`"123"`) or number (`123`): the backend
/// emits an integer, but anything that round-tripped it through a JS layer hands
/// it back as a string.
struct SnowflakeVisitor;

impl Visitor<'_> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a Snowflake as an integer or a string")
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Snowflake, E> {
        Ok(Snowflake(value))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Snowflake, E> {
        i64::try_from(value)
            .map(Snowflake)
            .map_err(|_| E::custom("Snowflake out of i64 range"))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Snowflake, E> {
        value
            .parse::<i64>()
            .map(Snowflake)
            .map_err(|_| E::custom(format!("invalid Snowflake string: {value:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_as_string() {
        let id = Snowflake(8765432109876543210);
        assert_eq!(
            serde_json::to_string(&id).unwrap(),
            "\"8765432109876543210\""
        );
    }

    #[test]
    fn deserializes_from_number_or_string() {
        let from_num: Snowflake = serde_json::from_str("8765432109876543210").unwrap();
        let from_str: Snowflake = serde_json::from_str("\"8765432109876543210\"").unwrap();
        assert_eq!(from_num, Snowflake(8765432109876543210));
        assert_eq!(from_str, from_num);
    }

    #[test]
    fn round_trips_through_json() {
        let id = Snowflake(1);
        let json = serde_json::to_value(id).unwrap();
        assert_eq!(json, serde_json::json!("1"));
        let back: Snowflake = serde_json::from_value(json).unwrap();
        assert_eq!(back, id);
    }
}
