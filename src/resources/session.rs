// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Session resource.
#[derive(Clone)]
pub struct Session {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Session {
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

    /// Whether the current user may revoke (RFC §4).
    pub fn can_revoke(&self) -> bool {
        resource::has_capability(&self.data, "session.revoke")
    }

    /// session.revoke
    pub async fn revoke(&self) -> Result<models::SessionRevokeResult, TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/me/sessions/{}/actions/revoke", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(data).map_err(|e| TransportError::Transport(e.into()))
    }
}
