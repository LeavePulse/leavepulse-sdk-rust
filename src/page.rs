//! LeavePulse SDK — pagination.
//!
//! `list`-style operations (`x-sdk-paginated`) return a [`Page<T>`]: a snapshot
//! of one page that preserves the response envelope's `total`/`page`/`per_page`
//! metadata, which a bare `Vec<T>` would silently drop. Callers read totals
//! directly and advance by re-calling the list method with the next page.
//!
//! Unlike the TypeScript `Page`, this is a pure snapshot with no `next()`: the
//! Rust transport/client isn't trivially clonable into a self-contained fetcher
//! closure, and threading it would force the snapshot to borrow the client.
//! Keeping `Page` a value type makes it `Send`/`Sync`/`Clone` and keeps the
//! generated method signatures simple. The priority is not losing the metadata;
//! the caller walks pages with `if page.has_next() { list(page.page + 1, ...) }`.

use serde_json::Value;

/// A single page of a paginated list, plus the envelope's pagination metadata.
///
/// `T` is the already-hydrated item type (a resource or a typed DTO). Build one
/// with [`page_data_from`], which coerces the canonical
/// `{items, total, page, per_page}` envelope and hydrates the items.
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// The hydrated items on this page.
    pub items: Vec<T>,
    /// 1-based index of this page.
    pub page: i64,
    /// Page size requested for this page.
    pub per_page: i64,
    /// Total number of items across all pages.
    pub total: i64,
}

impl<T> Page<T> {
    /// Whether a further page exists after this one (`page * per_page < total`).
    pub fn has_next(&self) -> bool {
        self.page.saturating_mul(self.per_page) < self.total
    }

    /// The 1-based index of the next page, or `None` when this is the last page.
    /// Callers pass this back to the list method to fetch the following page.
    pub fn next_page(&self) -> Option<i64> {
        if self.has_next() {
            Some(self.page + 1)
        } else {
            None
        }
    }

    /// Number of items on this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether this page carries no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> IntoIterator for Page<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Page<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

/// Read a number out of a JSON envelope field, falling back when it is absent or
/// not numeric. Mirrors the TS `pageDataFrom`'s `num` fallback.
fn num(body: &Value, key: &str, fallback: i64) -> i64 {
    body.get(key)
        .and_then(|v| v.as_i64().or_else(|| v.as_f64().map(|f| f as i64)))
        .unwrap_or(fallback)
}

/// Coerce a list-envelope `Value` into a [`Page<T>`], hydrating the `items`
/// array via `hydrate` and reading the canonical `{total, page, per_page}`
/// fields (with sensible fallbacks: `total` → item count, `page`/`per_page` →
/// the requested values). Generated list methods supply the per-endpoint
/// `hydrate` closure; the field plumbing lives here so it stays identical across
/// every paginated method, mirroring the TS `pageDataFrom`.
pub fn page_data_from<T>(
    body: Value,
    hydrate: impl FnOnce(Value) -> Vec<T>,
    requested_page: i64,
    requested_per_page: i64,
) -> Page<T> {
    let raw_items = match body.get("items") {
        Some(Value::Array(_)) => body.get("items").cloned().unwrap_or(Value::Null),
        _ if body.is_array() => body.clone(),
        _ => Value::Array(Vec::new()),
    };
    let items = hydrate(raw_items);
    let total = num(&body, "total", items.len() as i64);
    let page = num(&body, "page", requested_page);
    let per_page = num(&body, "per_page", requested_per_page);
    Page {
        items,
        page,
        per_page,
        total,
    }
}
