// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Me resource.
#[derive(Clone)]
pub struct Me {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Me {
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

    /// Re-fetch this Me and hydrate in place.
    pub async fn refresh(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(Method::Get, "/v1/me", Channel::Platform, None)
            .await?;
        let _: Me = self.client.hydrate("Me", data, None);
        Ok(())
    }
}
