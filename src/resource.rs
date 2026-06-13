//! LeavePulse SDK — resource helpers.
//!
//! Generated resource structs wrap a [`DataCell`] (shared, in-place-updatable
//! data) plus a client handle. These free functions read fields out of the
//! cell so generated code stays terse. Accessors never touch the network.

use serde_json::Value;

use crate::cache::DataCell;

/// Extract an instance id from a JSON object as a string. Resources key on
/// different fields: most use `id`, some a domain id (`user_id`). Falls back to
/// the first `*_id` field so identity-mapping works regardless of schema.
pub fn extract_id(data: &Value) -> String {
    let scalar = |v: &Value| match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    };
    if let Some(id) = data.get("id").and_then(scalar) {
        return id;
    }
    if let Value::Object(map) = data {
        for (key, value) in map {
            if key.ends_with("_id") {
                if let Some(id) = scalar(value) {
                    return id;
                }
            }
        }
    }
    String::new()
}

/// Read the resource id from a data cell as a string (ids may be int or str).
pub fn read_id(cell: &DataCell) -> String {
    let data = cell.lock().expect("cell poisoned");
    extract_id(&data)
}

/// Rewrite integer id fields (`id` and any `*_id`) into JSON strings in place.
///
/// Domain ids are Snowflakes — `integer` on the wire, but past 2^53 a JSON
/// `number` silently loses its low digits in any safe-integer-limited consumer
/// (JavaScript, a Tauri IPC payload). The typed [`crate::snowflake::Snowflake`]
/// already serializes as a string, but a raw [`snapshot`] bypasses the typed
/// models and would hand out the original number. Normalizing the cell at
/// hydration time means every snapshot is IPC-safe, matching how [`read_id`]
/// and the resource id getters already coerce ids to strings. Runs one level
/// deep into nested objects/arrays so envelope ids (e.g. `summary.id`) and
/// embedded resource lists are covered too.
pub fn stringify_ids(value: &mut Value) {
    fn is_id_key(key: &str) -> bool {
        key == "id" || key.ends_with("_id")
    }
    fn rewrite_object(map: &mut serde_json::Map<String, Value>) {
        for (key, val) in map.iter_mut() {
            if is_id_key(key) {
                if let Value::Number(n) = val {
                    if n.is_i64() || n.is_u64() {
                        *val = Value::String(n.to_string());
                    }
                }
            }
        }
    }
    if let Value::Object(map) = value {
        rewrite_object(map);
        for val in map.values_mut() {
            match val {
                Value::Object(nested) => rewrite_object(nested),
                Value::Array(items) => {
                    for item in items {
                        if let Value::Object(nested) = item {
                            rewrite_object(nested);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Append a query string to a path, dropping `None` values and URL-encoding
/// each value. Returns the path unchanged when no params are present. Generated
/// SDK methods pass their optional query args here before issuing the request.
pub fn with_query(path: &str, params: &[(&str, Option<String>)]) -> String {
    let mut pairs: Vec<String> = Vec::new();
    for (key, value) in params {
        if let Some(v) = value {
            pairs.push(format!("{key}={}", encode_component(v)));
        }
    }
    if pairs.is_empty() {
        return path.to_string();
    }
    format!("{path}?{}", pairs.join("&"))
}

/// Minimal percent-encoding for query values (RFC 3986 unreserved set passes
/// through; everything else is `%XX`). Avoids a dependency for the few chars
/// that appear in filters.
fn encode_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Deserialize the whole cell into a typed value (`T: DeserializeOwned`).
pub fn read_into<T: serde::de::DeserializeOwned>(cell: &DataCell) -> Result<T, serde_json::Error> {
    let data = cell.lock().expect("cell poisoned");
    serde_json::from_value(data.clone())
}

/// Snapshot the cell as a raw JSON value (always from memory).
pub fn snapshot(cell: &DataCell) -> Value {
    cell.lock().expect("cell poisoned").clone()
}

/// Pull a named array field out of the cell (for links/siblings).
pub fn field(cell: &DataCell, name: &str) -> Value {
    cell.lock()
        .expect("cell poisoned")
        .get(name)
        .cloned()
        .unwrap_or(Value::Null)
}

/// Runtime capability check: whether `permission_code` is in the resource's
/// `user_permissions` array (RFC 0001 §4).
pub fn has_capability(cell: &DataCell, permission_code: &str) -> bool {
    let data = cell.lock().expect("cell poisoned");
    data.get("user_permissions")
        .and_then(Value::as_array)
        .is_some_and(|perms| perms.iter().any(|p| p.as_str() == Some(permission_code)))
}

/// Normalize a (possibly wrapped) payload into a flat object: unwrap the
/// `data_root` field and merge sibling fields onto it (RFC §3.1 x-sdk-data-root).
pub fn normalize(payload: Value, data_root: Option<&str>) -> Value {
    let Some(root) = data_root else {
        return payload;
    };
    let Value::Object(mut map) = payload else {
        return payload;
    };
    let Some(Value::Object(core)) = map.remove(root) else {
        return Value::Object(map);
    };
    let mut merged = core;
    for (k, v) in map {
        merged.insert(k, v);
    }
    Value::Object(merged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stringifies_top_level_ids_and_leaves_other_numbers() {
        let mut v = json!({
            "id": 149076749580242940i64,
            "project_id": 12345,
            "schema_version": 1,
            "name": "frog"
        });
        stringify_ids(&mut v);
        assert_eq!(v["id"], json!("149076749580242940"));
        assert_eq!(v["project_id"], json!("12345"));
        // not an id field — must stay numeric
        assert_eq!(v["schema_version"], json!(1));
        assert_eq!(v["name"], json!("frog"));
    }

    #[test]
    fn preserves_value_beyond_safe_integer() {
        let big = 149076749580242940i64;
        let mut v = json!({ "id": big });
        stringify_ids(&mut v);
        let s = v["id"].as_str().unwrap();
        assert_eq!(s, "149076749580242940");
        assert_eq!(s.parse::<i64>().unwrap(), big);
    }

    #[test]
    fn stringifies_nested_envelope_and_embedded_lists() {
        let mut v = json!({
            "summary": { "id": 7, "owner_id": 8 },
            "servers": [{ "id": 9 }, { "id": 10 }]
        });
        stringify_ids(&mut v);
        assert_eq!(v["summary"]["id"], json!("7"));
        assert_eq!(v["summary"]["owner_id"], json!("8"));
        assert_eq!(v["servers"][0]["id"], json!("9"));
        assert_eq!(v["servers"][1]["id"], json!("10"));
    }

    #[test]
    fn leaves_string_ids_untouched() {
        let mut v = json!({ "id": "already-a-string" });
        stringify_ids(&mut v);
        assert_eq!(v["id"], json!("already-a-string"));
    }
}
