// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Form resource.
#[derive(Clone)]
pub struct Form {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Form {
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

    /// Re-fetch this Form and hydrate in place.
    pub async fn refresh(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Get,
                &format!("/v1/whitelist/forms/{}", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Form = self.client.hydrate("Form", data, None);
        Ok(())
    }

    /// Whether the current user may delete (RFC §4).
    pub fn can_delete(&self) -> bool {
        resource::has_capability(&self.data, "form.delete")
    }

    /// Whether the current user may update (RFC §4).
    pub fn can_update(&self) -> bool {
        resource::has_capability(&self.data, "form.update")
    }

    /// form.delete
    pub async fn delete(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/whitelist/forms/{}", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Form = self.client.hydrate("Form", data, None);
        Ok(())
    }

    /// form.update
    pub async fn update(
        &self,
        body: models::WhitelistFormUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/whitelist/forms/{}", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Form = self.client.hydrate("Form", data, None);
        Ok(())
    }

    /// form.set_import_mapping
    pub async fn set_import_mapping(
        &self,
        body: models::WhitelistFormImportMappingRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/whitelist/forms/{}/import-mapping", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Form = self.client.hydrate("Form", data, None);
        Ok(())
    }
}
