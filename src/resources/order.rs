// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use crate::client::LeavePulse;
use crate::models;
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
    pub async fn get(&self, order_id: String) -> Result<models::Order, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Get,
                &format!("/v1/billing/orders/{}", order_id),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}
