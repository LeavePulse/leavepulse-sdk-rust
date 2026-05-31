// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Ticket resource.
#[derive(Clone)]
pub struct Ticket {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Ticket {
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

    /// Whether the current user may set_status (RFC §4).
    pub fn can_set_status(&self) -> bool {
        resource::has_capability(&self.data, "ticket.set_status")
    }

    /// Whether the current user may reply (RFC §4).
    pub fn can_reply(&self) -> bool {
        resource::has_capability(&self.data, "ticket.reply")
    }

    /// ticket.set_status
    pub async fn set_status(
        &self,
        body: models::TicketStatusUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/community/tickets/{}", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Ticket = self.client.hydrate("Ticket", data, None);
        Ok(())
    }

    /// ticket.reply
    pub async fn reply(
        &self,
        body: models::TicketMessageCreateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/community/tickets/{}/messages", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Ticket = self.client.hydrate("Ticket", data, None);
        Ok(())
    }
}
