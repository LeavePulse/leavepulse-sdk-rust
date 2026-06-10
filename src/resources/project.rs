// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::resources::Binding;
use crate::resources::Comment;
use crate::resources::Form;
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

    /// project.comments.list
    pub async fn comments_list(
        &self,
        params: models::ProjectCommentsListParams,
    ) -> Result<Vec<Comment>, TransportError> {
        let data = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/community/projects/{}/comments", self.id()),
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("limit", params.limit.map(|v| v.to_string())),
                    ("target_locale", params.target_locale.map(|v| v.to_string())),
                ],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        let items = if data.is_array() {
            data
        } else {
            data.get("items").cloned().unwrap_or(Value::Null)
        };
        Ok(self.client.hydrate_many::<Comment>("Comment", items))
    }

    /// project.comments.liked
    pub async fn comments_liked(&self) -> Result<models::LikedCommentIds, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/community/projects/{}/comments/liked", self.id()),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.comments.mine
    pub async fn comments_mine(
        &self,
        params: models::ProjectCommentsMineParams,
    ) -> Result<models::MyComment, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/community/projects/{}/comments/me", self.id()),
                &[("target_locale", params.target_locale.map(|v| v.to_string()))],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.engagement
    pub async fn engagement(&self) -> Result<models::ProjectEngagement, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/community/projects/{}/engagement", self.id()),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.engagement.status
    pub async fn engagement_status(
        &self,
    ) -> Result<models::ProjectEngagementStatus, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/community/projects/{}/engagement/status", self.id()),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.votes.list
    pub async fn votes_list(
        &self,
        params: models::ProjectVotesListParams,
    ) -> Result<models::RecentVotes, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/community/projects/{}/votes", self.id()),
                &[("limit", params.limit.map(|v| v.to_string()))],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.projects.get
    pub async fn me_projects_get(&self) -> Result<models::WorkspaceDetail, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/me/projects/{}", self.id()),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.history.list
    pub async fn history_list(
        &self,
        params: models::ProjectHistoryListParams,
    ) -> Result<models::HistoryResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/monitoring/projects/{}/history", self.id()),
                &[("period", params.period.map(|v| v.to_string()))],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.live
    pub async fn live(&self) -> Result<models::ProjectLiveStatus, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/monitoring/projects/{}/live", self.id()),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.servers.create
    pub async fn servers_create(&self) -> Result<Server, TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/projects/{}/servers", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        Ok(self.client.hydrate::<Server>("Server", data, None))
    }

    /// project.stats
    pub async fn stats(
        &self,
        params: models::ProjectStatsParams,
    ) -> Result<models::ProjectStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/projects/{}/stats", self.id()),
                &[("period", params.period.map(|v| v.to_string()))],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.team_sync.targets
    pub async fn team_sync_targets(
        &self,
        params: models::ProjectTeamSyncTargetsParams,
    ) -> Result<models::DiscordRoleTargets, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/projects/{}/team-sync/discord-targets", self.id()),
                &[("role_id", params.role_id.map(|v| v.to_string()))],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.whitelist.config
    pub async fn whitelist_config(
        &self,
    ) -> Result<Vec<models::ProjectWhitelistConfigItem>, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/projects/{}/whitelist/config", self.id()),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// project.whitelist.forms
    pub async fn whitelist_forms(&self) -> Result<Vec<Form>, TransportError> {
        let data = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/projects/{}/whitelist/forms", self.id()),
            Channel::Platform,
        )
        .await?;
        let items = if data.is_array() {
            data
        } else {
            data.get("items").cloned().unwrap_or(Value::Null)
        };
        Ok(self.client.hydrate_many::<Form>("Form", items))
    }

    /// project.policies
    pub async fn policies(&self) -> Result<Vec<Binding>, TransportError> {
        let data = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/projects/{}/whitelist/policies", self.id()),
            Channel::Platform,
        )
        .await?;
        let items = if data.is_array() {
            data
        } else {
            data.get("items").cloned().unwrap_or(Value::Null)
        };
        Ok(self.client.hydrate_many::<Binding>("Binding", items))
    }
}
