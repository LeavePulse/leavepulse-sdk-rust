// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Subscription resource.
#[derive(Clone)]
pub struct Subscription {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Subscription {
    pub(crate) fn new(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Self { data, client }
    }

    /// This resource's id.
    pub fn id(&self) -> String {
        resource::read_id(&self.data)
    }

    /// Snapshot of the backing data (always from memory).
    pub fn snapshot(&self) -> Value {
        resource::snapshot(&self.data)
    }

    /// Whether the current user may cancel (RFC §4).
    pub fn can_cancel(&self) -> bool {
        resource::has_capability(&self.data, "subscription.cancel")
    }

    /// subscription.cancel
    pub async fn cancel(&self) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Patch, &format!("/v1/billing/subscriptions/{}/cancel", self.id()), Channel::Platform, None).await?;
        let _: Subscription = self.client.hydrate("Subscription", data, None);
        Ok(())
    }
}
