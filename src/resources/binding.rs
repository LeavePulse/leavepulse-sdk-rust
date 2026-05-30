// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Binding resource.
#[derive(Clone)]
pub struct Binding {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Binding {
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

    /// Whether the current user may delete (RFC §4).
    pub fn can_delete(&self) -> bool {
        resource::has_capability(&self.data, "binding.delete")
    }

    /// Whether the current user may update (RFC §4).
    pub fn can_update(&self) -> bool {
        resource::has_capability(&self.data, "binding.update")
    }

    /// Whether the current user may test (RFC §4).
    pub fn can_test(&self) -> bool {
        resource::has_capability(&self.data, "binding.test")
    }

    /// binding.delete
    pub async fn delete(&self, binding_id: String) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Delete, &format!("/v1/whitelist/bindings/{}", binding_id), Channel::Platform, None).await?;
        let _: Binding = self.client.hydrate("Binding", data, None);
        Ok(())
    }

    /// binding.update
    pub async fn update(&self, binding_id: String, body: Value) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Patch, &format!("/v1/whitelist/bindings/{}", binding_id), Channel::Platform, Some(body)).await?;
        let _: Binding = self.client.hydrate("Binding", data, None);
        Ok(())
    }

    /// binding.test
    pub async fn test(&self, binding_id: String, audience: Option<String>) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Post, &resource::with_query(&format!("/v1/whitelist/bindings/{}/actions/test-notifications", binding_id), &[("audience", audience.map(|v| v.to_string()))]), Channel::Platform, None).await?;
        let _: Binding = self.client.hydrate("Binding", data, None);
        Ok(())
    }

    /// binding.entries.add
    pub async fn entries_add(&self, binding_id: String, body: Value) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Post, &format!("/v1/whitelist/bindings/{}/direct/entries", binding_id), Channel::Platform, Some(body)).await?;
        let _: Binding = self.client.hydrate("Binding", data, None);
        Ok(())
    }

    /// binding.entries.remove
    pub async fn entries_remove(&self, binding_id: String) -> Result<(), TransportError> {
        let data = self.client.transport().request(Method::Delete, &format!("/v1/whitelist/bindings/{}/direct/entries/{}", binding_id, self.id()), Channel::Platform, None).await?;
        let _: Binding = self.client.hydrate("Binding", data, None);
        Ok(())
    }
}
