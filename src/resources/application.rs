// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Application resource.
#[derive(Clone)]
pub struct Application {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Application {
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

    /// Whether the current user may apply (RFC §4).
    pub fn can_apply(&self) -> bool {
        resource::has_capability(&self.data, "application.apply")
    }

    /// Whether the current user may set_status (RFC §4).
    pub fn can_set_status(&self) -> bool {
        resource::has_capability(&self.data, "application.set_status")
    }

    /// Whether the current user may approve (RFC §4).
    pub fn can_approve(&self) -> bool {
        resource::has_capability(&self.data, "application.approve")
    }

    /// Whether the current user may deny (RFC §4).
    pub fn can_deny(&self) -> bool {
        resource::has_capability(&self.data, "application.deny")
    }

    /// Whether the current user may resubmit (RFC §4).
    pub fn can_resubmit(&self) -> bool {
        resource::has_capability(&self.data, "application.resubmit")
    }

    /// server.whitelist.apply
    pub async fn whitelist_apply(
        &self,
        server_id: String,
        body: models::WhitelistApplyRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/whitelist/applications", server_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Application = self.client.hydrate("Application", data, None);
        Ok(())
    }

    /// application.set_status
    pub async fn set_status(
        &self,
        server_id: String,
        body: models::WhitelistStatusRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!(
                    "/v1/servers/{}/whitelist/applications/{}",
                    server_id,
                    self.id()
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Application = self.client.hydrate("Application", data, None);
        Ok(())
    }

    /// application.approve
    pub async fn approve(
        &self,
        server_id: String,
        body: models::WhitelistDecisionRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!(
                    "/v1/servers/{}/whitelist/applications/{}/approve",
                    server_id,
                    self.id()
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Application = self.client.hydrate("Application", data, None);
        Ok(())
    }

    /// application.deny
    pub async fn deny(
        &self,
        server_id: String,
        body: models::WhitelistDecisionRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!(
                    "/v1/servers/{}/whitelist/applications/{}/deny",
                    server_id,
                    self.id()
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Application = self.client.hydrate("Application", data, None);
        Ok(())
    }

    /// application.resubmit
    pub async fn resubmit(
        &self,
        server_id: String,
        body: models::WhitelistApplyRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!(
                    "/v1/servers/{}/whitelist/applications/{}/resubmit",
                    server_id,
                    self.id()
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Application = self.client.hydrate("Application", data, None);
        Ok(())
    }
}
