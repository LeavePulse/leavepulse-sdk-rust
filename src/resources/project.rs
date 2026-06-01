// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::resources::Server;
use crate::transport::{Channel, Method, TransportError};

/// Project resource.
#[derive(Clone)]
pub struct Project {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Project {
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

    /// Re-fetch this Project and hydrate in place.
    pub async fn refresh(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Get,
                &format!("/v1/projects/{}", self.id()),
                Channel::PlatformPublic,
                None,
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, Some("project"));
        Ok(())
    }

    /// Whether the current user may heart (RFC §4).
    pub fn can_heart(&self) -> bool {
        resource::has_capability(&self.data, "project.heart")
    }

    /// Whether the current user may thumb (RFC §4).
    pub fn can_thumb(&self) -> bool {
        resource::has_capability(&self.data, "project.thumb")
    }

    /// Whether the current user may manage_bridge (RFC §4).
    pub fn can_manage_bridge(&self) -> bool {
        resource::has_capability(&self.data, "project.manage_bridge")
    }

    /// Whether the current user may change_slug (RFC §4).
    pub fn can_change_slug(&self) -> bool {
        resource::has_capability(&self.data, "project.change_slug")
    }

    /// Whether the current user may rename (RFC §4).
    pub fn can_rename(&self) -> bool {
        resource::has_capability(&self.data, "project.rename")
    }

    /// Whether the current user may set_online_strategy (RFC §4).
    pub fn can_set_online_strategy(&self) -> bool {
        resource::has_capability(&self.data, "project.set_online_strategy")
    }

    /// Whether the current user may set_rollout_mode (RFC §4).
    pub fn can_set_rollout_mode(&self) -> bool {
        resource::has_capability(&self.data, "project.set_rollout_mode")
    }

    /// Related Server from cache (no network).
    pub fn servers(&self) -> Vec<Server> {
        self.client
            .hydrate_many::<Server>("Server", resource::field(&self.data, "servers"))
    }

    /// Fetch related Server from the server.
    pub async fn get_servers(&self) -> Result<Vec<Server>, TransportError> {
        self.refresh().await?;
        Ok(self
            .client
            .hydrate_many::<Server>("Server", resource::field(&self.data, "servers")))
    }

    /// project.comments.create
    pub async fn comments_create(
        &self,
        body: models::CommentCreateRequest,
        params: models::ProjectCommentsCreateParams,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &resource::with_query(
                    &format!("/v1/community/projects/{}/comments", self.id()),
                    &[("target_locale", params.target_locale.map(|v| v.to_string()))],
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.heart
    pub async fn heart(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/community/projects/{}/heart", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.thumb
    pub async fn thumb(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/community/projects/{}/thumb", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.bridge.update
    pub async fn bridge_update(
        &self,
        server_id: String,
        body: models::BridgeSettingsUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/discord/servers/{}/bridge", server_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.bridge.import
    pub async fn bridge_import(
        &self,
        server_id: String,
        body: models::ImportPullRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/discord/servers/{}/import-pull", server_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.change_slug
    pub async fn change_slug(
        &self,
        body: models::WorkspaceChangeSlugRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/me/projects/{}/actions/change-slug", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, Some("workspace"));
        Ok(())
    }

    /// project.rename
    pub async fn rename(&self, body: models::WorkspaceRenameRequest) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/me/projects/{}/actions/rename", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, Some("workspace"));
        Ok(())
    }

    /// project.set_online_strategy
    pub async fn set_online_strategy(
        &self,
        body: models::WorkspaceSetOnlineStrategyRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/me/projects/{}/actions/set-online-strategy", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, Some("workspace"));
        Ok(())
    }

    /// project.set_rollout_mode
    pub async fn set_rollout_mode(
        &self,
        body: models::WorkspaceSetRolloutModeRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/me/projects/{}/actions/set-rollout-mode", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, Some("workspace"));
        Ok(())
    }

    /// project.policies.create
    pub async fn policies_create(
        &self,
        body: models::WhitelistBindingWriteRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/projects/{}/whitelist/policies", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.policies.delete
    pub async fn policies_delete(&self, policy_id: String) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!(
                    "/v1/projects/{}/whitelist/policies/{}",
                    self.id(),
                    policy_id
                ),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.policies.update
    pub async fn policies_update(
        &self,
        policy_id: String,
        body: models::WhitelistBindingWriteRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!(
                    "/v1/projects/{}/whitelist/policies/{}",
                    self.id(),
                    policy_id
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }

    /// project.policies.test
    pub async fn policies_test(
        &self,
        policy_id: String,
        params: models::ProjectPoliciesTestParams,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &resource::with_query(
                    &format!(
                        "/v1/projects/{}/whitelist/policies/{}/actions/test-notifications",
                        self.id(),
                        policy_id
                    ),
                    &[("audience", params.audience.map(|v| v.to_string()))],
                ),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Project = self.client.hydrate("Project", data, None);
        Ok(())
    }
}
