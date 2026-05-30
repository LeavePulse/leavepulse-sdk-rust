// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Build resource.
#[derive(Clone)]
pub struct Build {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Build {
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

    /// Re-fetch this Build and hydrate in place.
    pub async fn refresh(&self) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Get, &format!("/v1/builds/{}", self.id()), Channel::Platform, None).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// Whether the current user may delete (RFC §4).
    pub fn can_delete(&self) -> bool {
        resource::has_capability(&self.data, "build.delete")
    }

    /// Whether the current user may update (RFC §4).
    pub fn can_update(&self) -> bool {
        resource::has_capability(&self.data, "build.update")
    }

    /// Whether the current user may add_collaborator (RFC §4).
    pub fn can_add_collaborator(&self) -> bool {
        resource::has_capability(&self.data, "build.add_collaborator")
    }

    /// Whether the current user may remove_collaborator (RFC §4).
    pub fn can_remove_collaborator(&self) -> bool {
        resource::has_capability(&self.data, "build.remove_collaborator")
    }

    /// Whether the current user may confirm_config (RFC §4).
    pub fn can_confirm_config(&self) -> bool {
        resource::has_capability(&self.data, "build.confirm_config")
    }

    /// Whether the current user may upload_config (RFC §4).
    pub fn can_upload_config(&self) -> bool {
        resource::has_capability(&self.data, "build.upload_config")
    }

    /// Whether the current user may unshare (RFC §4).
    pub fn can_unshare(&self) -> bool {
        resource::has_capability(&self.data, "build.unshare")
    }

    /// Whether the current user may share (RFC §4).
    pub fn can_share(&self) -> bool {
        resource::has_capability(&self.data, "build.share")
    }

    /// build.delete
    pub async fn delete(&self) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Delete, &format!("/v1/builds/{}", self.id()), Channel::Platform, None).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// build.update
    pub async fn update(&self, body: Value) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Patch, &format!("/v1/builds/{}", self.id()), Channel::Platform, Some(body)).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// build.collaborators.add
    pub async fn collaborators_add(&self, body: Value) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Post, &format!("/v1/builds/{}/collaborators", self.id()), Channel::Platform, Some(body)).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// build.collaborators.remove
    pub async fn collaborators_remove(&self, build_id: String) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Delete, &format!("/v1/builds/{}/collaborators/{}", build_id, self.id()), Channel::Platform, None).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// build.config.confirm
    pub async fn config_confirm(&self, body: Value) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Post, &format!("/v1/builds/{}/config/confirm", self.id()), Channel::Platform, Some(body)).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// build.config.upload
    pub async fn config_upload(&self, body: Value) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Post, &format!("/v1/builds/{}/config/upload", self.id()), Channel::Platform, Some(body)).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// build.unshare
    pub async fn unshare(&self) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Delete, &format!("/v1/builds/{}/share", self.id()), Channel::Platform, None).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }

    /// build.share
    pub async fn share(&self) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Post, &format!("/v1/builds/{}/share", self.id()), Channel::Platform, None).await?;
        let _: Build = self.client.hydrate("Build", data, None);
        Ok(())
    }
}
