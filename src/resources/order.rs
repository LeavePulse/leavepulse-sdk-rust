// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::client::LeavePulse;
use crate::transport::{Channel, Method, TransportError};

/// Order — a scope namespace (no instance identity).
pub struct Order {
    client: Arc<LeavePulse>,
}

impl Order {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// order.get
    pub async fn get(&self, order_id: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/billing/orders/{}", order_id), Channel::Platform, None).await
    }
}
