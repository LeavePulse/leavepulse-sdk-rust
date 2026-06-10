// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// AdminDiscoveryNs procedure namespace.
pub struct AdminDiscoveryNs {
    client: Arc<LeavePulse>,
}

impl AdminDiscoveryNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.discovery.candidates
    pub async fn candidates(
        &self,
        params: models::AdminDiscoveryCandidatesParams,
    ) -> Result<Value, TransportError> {
        crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/admin/discovery/candidates",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("limit", params.limit.map(|v| v.to_string())),
                    ("status", params.status.map(|v| v.to_string())),
                    ("search", params.search.map(|v| v.to_string())),
                    ("source", params.source.map(|v| v.to_string())),
                    ("edition", params.edition.map(|v| v.to_string())),
                    ("region", params.region.map(|v| v.to_string())),
                    ("min_sources", params.min_sources.map(|v| v.to_string())),
                    ("min_mc_online", params.min_mc_online.map(|v| v.to_string())),
                    (
                        "min_discord_members",
                        params.min_discord_members.map(|v| v.to_string()),
                    ),
                    ("sort", params.sort.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await
    }

    /// admin.discovery.edit
    pub async fn edit(
        &self,
        candidate_id: i64,
        body: models::DiscoveryCandidateEditRequest,
    ) -> Result<Value, TransportError> {
        self.client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/admin/discovery/candidates/{}", candidate_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await
    }

    /// admin.discovery.approve
    pub async fn approve(
        &self,
        candidate_id: i64,
        params: models::AdminDiscoveryApproveParams,
    ) -> Result<models::DiscoveryApproveResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &resource::with_query(
                    &format!(
                        "/v1/admin/discovery/candidates/{}/actions/approve",
                        candidate_id
                    ),
                    &[
                        (
                            "show_in_public",
                            params.show_in_public.map(|v| v.to_string()),
                        ),
                        ("server_id", params.server_id.map(|v| v.to_string())),
                    ],
                ),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.discovery.ignore
    pub async fn ignore(
        &self,
        candidate_id: i64,
        params: models::AdminDiscoveryIgnoreParams,
    ) -> Result<models::DiscoveryIgnoreResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &resource::with_query(
                    &format!(
                        "/v1/admin/discovery/candidates/{}/actions/ignore",
                        candidate_id
                    ),
                    &[("reason", params.reason.map(|v| v.to_string()))],
                ),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.discovery.observations
    pub async fn observations(
        &self,
        candidate_id: i64,
        params: models::AdminDiscoveryObservationsParams,
    ) -> Result<Value, TransportError> {
        crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!(
                    "/v1/admin/discovery/candidates/{}/observations",
                    candidate_id
                ),
                &[("limit", params.limit.map(|v| v.to_string()))],
            ),
            Channel::Platform,
        )
        .await
    }

    /// admin.discovery.preview
    pub async fn preview(&self, candidate_id: i64) -> Result<Value, TransportError> {
        crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/admin/discovery/candidates/{}/preview", candidate_id),
            Channel::Platform,
        )
        .await
    }

    /// admin.discovery.sources
    pub async fn sources(&self) -> Result<Value, TransportError> {
        crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/admin/discovery/sources",
            Channel::Platform,
        )
        .await
    }
}

/// AdminOverridesNs procedure namespace.
pub struct AdminOverridesNs {
    client: Arc<LeavePulse>,
}

impl AdminOverridesNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.overrides.list
    pub async fn list(
        &self,
        server_id: i64,
        params: models::AdminOverridesListParams,
    ) -> Result<Vec<models::StatusOverrideItem>, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/admin/servers/{}/status-overrides", server_id),
                &[
                    ("start", params.start.map(|v| v.to_string())),
                    ("end", params.end.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.overrides.create
    pub async fn create(
        &self,
        server_id: i64,
        body: models::CreateStatusOverrideRequest,
    ) -> Result<models::StatusOverrideItem, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/admin/servers/{}/status-overrides", server_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.overrides.delete
    pub async fn delete(
        &self,
        server_id: i64,
        override_id: String,
    ) -> Result<models::DeleteStatusOverrideResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!(
                    "/v1/admin/servers/{}/status-overrides/{}",
                    server_id, override_id
                ),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminPlayersNs procedure namespace.
pub struct AdminPlayersNs {
    client: Arc<LeavePulse>,
}

impl AdminPlayersNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.players.search
    pub async fn search(
        &self,
        params: models::AdminPlayersSearchParams,
    ) -> Result<models::PlayerSearchPage, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/admin/players",
                &[
                    ("q", params.q.map(|v| v.to_string())),
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminProjectsNs procedure namespace.
pub struct AdminProjectsNs {
    client: Arc<LeavePulse>,
}

impl AdminProjectsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.projects.list
    pub async fn list(
        &self,
        params: models::AdminProjectsListParams,
    ) -> Result<models::AdminProjectListResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/admin/projects",
                &[
                    ("q", params.q.map(|v| v.to_string())),
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.projects.delete
    pub async fn delete(
        &self,
        project_id: i64,
    ) -> Result<models::AdminProjectDeleteResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/admin/projects/{}", project_id),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.projects.change_slug
    pub async fn change_slug(
        &self,
        project_id: i64,
        body: models::AdminChangeProjectSlugRequest,
    ) -> Result<models::AdminProject, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/admin/projects/{}/actions/change-slug", project_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.projects.rename
    pub async fn rename(
        &self,
        project_id: i64,
        body: models::AdminRenameProjectRequest,
    ) -> Result<models::AdminProject, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/admin/projects/{}/actions/rename", project_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.projects.set_online_strategy
    pub async fn set_online_strategy(
        &self,
        project_id: i64,
        body: models::AdminSetProjectOnlineStrategyRequest,
    ) -> Result<models::AdminProject, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!(
                    "/v1/admin/projects/{}/actions/set-online-strategy",
                    project_id
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.projects.set_rollout_mode
    pub async fn set_rollout_mode(
        &self,
        project_id: i64,
        body: models::AdminSetProjectRolloutModeRequest,
    ) -> Result<models::AdminProject, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/admin/projects/{}/actions/set-rollout-mode", project_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.projects.transfer_ownership
    pub async fn transfer_ownership(
        &self,
        project_id: i64,
        body: models::AdminTransferOwnershipRequest,
    ) -> Result<models::AdminProject, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!(
                    "/v1/admin/projects/{}/actions/transfer-ownership",
                    project_id
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminRolesNs procedure namespace.
pub struct AdminRolesNs {
    client: Arc<LeavePulse>,
}

impl AdminRolesNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.roles.list
    pub async fn list(&self) -> Result<models::AdminRoleListResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/admin/roles",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.roles.create
    pub async fn create(
        &self,
        body: models::AdminRoleRequest,
    ) -> Result<models::AdminRole, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/admin/roles",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.roles.delete
    pub async fn delete(
        &self,
        role_id: i64,
    ) -> Result<models::AdminRoleDeleteResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/admin/roles/{}", role_id),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.roles.update
    pub async fn update(
        &self,
        role_id: i64,
        body: models::AdminRoleRequest,
    ) -> Result<models::AdminRole, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/admin/roles/{}", role_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminServersNs procedure namespace.
pub struct AdminServersNs {
    client: Arc<LeavePulse>,
}

impl AdminServersNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.servers.list
    pub async fn list(
        &self,
        params: models::AdminServersListParams,
    ) -> Result<models::AdminServerListResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/admin/servers",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                    ("q", params.q.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.servers.create
    pub async fn create(
        &self,
        body: models::AdminForceCreateRequest,
    ) -> Result<models::AdminServerSummary, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/admin/servers",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.servers.stats
    pub async fn stats(&self) -> Result<models::GlobalServerStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/admin/servers/stats",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.servers.delete
    pub async fn delete(
        &self,
        server_id: i64,
    ) -> Result<models::AdminDeleteResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/admin/servers/{}", server_id),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.servers.update
    pub async fn update(
        &self,
        server_id: i64,
        body: models::AdminServerUpdateRequest,
    ) -> Result<models::AdminServerSummary, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/admin/servers/{}", server_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminSessionsNs procedure namespace.
pub struct AdminSessionsNs {
    client: Arc<LeavePulse>,
}

impl AdminSessionsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.sessions.revoke
    pub async fn revoke(
        &self,
        session_id: i64,
    ) -> Result<models::SessionRevokeResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/admin/sessions/{}/actions/revoke", session_id),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminSystemNs procedure namespace.
pub struct AdminSystemNs {
    client: Arc<LeavePulse>,
}

impl AdminSystemNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.system.health
    pub async fn health(&self) -> Result<models::ServicesHealthResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/admin/system/services-health",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminUsersNs procedure namespace.
pub struct AdminUsersNs {
    client: Arc<LeavePulse>,
}

impl AdminUsersNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// admin.users.list
    pub async fn list(
        &self,
        params: models::AdminUsersListParams,
    ) -> Result<models::AdminUserListResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/admin/users",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                    ("q", params.q.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.by_minecraft
    pub async fn by_minecraft(
        &self,
        uuid: String,
    ) -> Result<models::AdminUserDetail, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/admin/users/by-minecraft-uuid/{}", uuid),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.search
    pub async fn search(
        &self,
        params: models::AdminUsersSearchParams,
    ) -> Result<models::AdminUserListResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/admin/users/search",
                &[
                    ("q", params.q.map(|v| v.to_string())),
                    ("limit", params.limit.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.get
    pub async fn get(&self, user_id: i64) -> Result<models::AdminUserDetail, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/admin/users/{}", user_id),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.update
    pub async fn update(
        &self,
        user_id: i64,
        body: models::AdminUserUpdateRequest,
    ) -> Result<models::AdminUserDetail, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/admin/users/{}", user_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.set_discord
    pub async fn set_discord(
        &self,
        user_id: i64,
        body: models::AdminUserDiscordUpdateRequest,
    ) -> Result<models::AdminUserDetail, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/admin/users/{}/discord", user_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.create_offline_minecraft
    pub async fn create_offline_minecraft(
        &self,
        user_id: i64,
        body: models::AdminMinecraftAccountWriteRequest,
    ) -> Result<models::AdminMinecraftAccount, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/admin/users/{}/minecraft-accounts/offline", user_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.delete_minecraft
    pub async fn delete_minecraft(
        &self,
        user_id: i64,
        account_id: i64,
    ) -> Result<models::AdminMinecraftAccountDeleteResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!(
                    "/v1/admin/users/{}/minecraft-accounts/{}",
                    user_id, account_id
                ),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.update_minecraft
    pub async fn update_minecraft(
        &self,
        user_id: i64,
        account_id: i64,
        body: models::AdminMinecraftAccountWriteRequest,
    ) -> Result<models::AdminMinecraftAccount, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!(
                    "/v1/admin/users/{}/minecraft-accounts/{}",
                    user_id, account_id
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.roles
    pub async fn roles(&self, user_id: i64) -> Result<models::UserRolesResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/admin/users/{}/roles", user_id),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.remove_role
    pub async fn remove_role(
        &self,
        user_id: i64,
        role_slug: String,
    ) -> Result<models::UserRolesResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/admin/users/{}/roles/{}", user_id, role_slug),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.assign_role
    pub async fn assign_role(
        &self,
        user_id: i64,
        role_slug: String,
    ) -> Result<models::UserRolesResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/admin/users/{}/roles/{}", user_id, role_slug),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// admin.users.sessions
    pub async fn sessions(&self, user_id: i64) -> Result<models::SessionList, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/admin/users/{}/sessions", user_id),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AdminNs procedure namespace.
pub struct AdminNs {
    client: Arc<LeavePulse>,
}

impl AdminNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    pub fn discovery(&self) -> AdminDiscoveryNs {
        AdminDiscoveryNs::new(Arc::clone(&self.client))
    }

    pub fn overrides(&self) -> AdminOverridesNs {
        AdminOverridesNs::new(Arc::clone(&self.client))
    }

    pub fn players(&self) -> AdminPlayersNs {
        AdminPlayersNs::new(Arc::clone(&self.client))
    }

    pub fn projects(&self) -> AdminProjectsNs {
        AdminProjectsNs::new(Arc::clone(&self.client))
    }

    pub fn roles(&self) -> AdminRolesNs {
        AdminRolesNs::new(Arc::clone(&self.client))
    }

    pub fn servers(&self) -> AdminServersNs {
        AdminServersNs::new(Arc::clone(&self.client))
    }

    pub fn sessions(&self) -> AdminSessionsNs {
        AdminSessionsNs::new(Arc::clone(&self.client))
    }

    pub fn system(&self) -> AdminSystemNs {
        AdminSystemNs::new(Arc::clone(&self.client))
    }

    pub fn users(&self) -> AdminUsersNs {
        AdminUsersNs::new(Arc::clone(&self.client))
    }
}

/// AuthDeviceNs procedure namespace.
pub struct AuthDeviceNs {
    client: Arc<LeavePulse>,
}

impl AuthDeviceNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// auth.device.approve
    pub async fn approve(
        &self,
        body: models::DeviceApproveRequest,
    ) -> Result<models::DeviceApproveResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/auth/device/approve",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// auth.device.start
    pub async fn start(
        &self,
        body: models::DeviceStartRequest,
    ) -> Result<models::DeviceStartResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/auth/device/start",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// auth.device.token
    pub async fn token(
        &self,
        body: models::DeviceTokenRequest,
    ) -> Result<models::DeviceTokenResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/auth/device/token",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AuthOauthNs procedure namespace.
pub struct AuthOauthNs {
    client: Arc<LeavePulse>,
}

impl AuthOauthNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// auth.oauth.captcha_confirm
    pub async fn captcha_confirm(
        &self,
        body: models::OAuthCaptchaConfirmRequest,
    ) -> Result<Value, TransportError> {
        self.client
            .transport()
            .request(
                Method::Post,
                "/auth/oauth/captcha/confirm",
                Channel::Auth,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await
    }

    /// auth.oauth.totp_confirm
    pub async fn totp_confirm(
        &self,
        body: models::OAuthTotpConfirmRequest,
    ) -> Result<models::LoginResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/auth/oauth/totp/confirm",
                Channel::Auth,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// auth.oauth.callback
    pub async fn callback(
        &self,
        provider: String,
        body: models::OAuthCallbackRequest,
    ) -> Result<Value, TransportError> {
        self.client
            .transport()
            .request(
                Method::Post,
                &format!("/auth/oauth/{}/callback", provider),
                Channel::Auth,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await
    }

    /// auth.oauth.start
    pub async fn start(
        &self,
        provider: String,
    ) -> Result<models::OAuthStartResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/auth/oauth/{}/start", provider),
            Channel::Auth,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// AuthNs procedure namespace.
pub struct AuthNs {
    client: Arc<LeavePulse>,
}

impl AuthNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    pub fn device(&self) -> AuthDeviceNs {
        AuthDeviceNs::new(Arc::clone(&self.client))
    }

    pub fn oauth(&self) -> AuthOauthNs {
        AuthOauthNs::new(Arc::clone(&self.client))
    }

    /// auth.login
    pub async fn login(
        &self,
        body: models::UserLogin,
    ) -> Result<models::LoginResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/auth/login",
                Channel::Auth,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// auth.logout
    pub async fn logout(&self) -> Result<models::LogoutResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Post, "/auth/logout", Channel::Auth, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// auth.refresh
    pub async fn refresh(
        &self,
        body: models::RefreshTokenRequest,
    ) -> Result<models::LoginResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/auth/refresh",
                Channel::Auth,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// auth.register
    pub async fn register(
        &self,
        body: models::UserRegister,
    ) -> Result<models::UserPublic, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/auth/register",
                Channel::Auth,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// auth.session
    pub async fn session(&self) -> Result<models::LoginResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Post, "/auth/session", Channel::Auth, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// BillingOrdersNs procedure namespace.
pub struct BillingOrdersNs {
    client: Arc<LeavePulse>,
}

impl BillingOrdersNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// billing.orders.list
    pub async fn list(
        &self,
        params: models::BillingOrdersListParams,
    ) -> Result<models::OrderList, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/billing/orders",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("limit", params.limit.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// BillingProductsNs procedure namespace.
pub struct BillingProductsNs {
    client: Arc<LeavePulse>,
}

impl BillingProductsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// billing.products.list
    pub async fn list(&self) -> Result<Vec<models::Product>, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/billing/products",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// BillingSubscriptionsNs procedure namespace.
pub struct BillingSubscriptionsNs {
    client: Arc<LeavePulse>,
}

impl BillingSubscriptionsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// billing.subscriptions.list
    pub async fn list(
        &self,
        params: models::BillingSubscriptionsListParams,
    ) -> Result<models::SubscriptionList, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/billing/subscriptions",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("limit", params.limit.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// BillingNs procedure namespace.
pub struct BillingNs {
    client: Arc<LeavePulse>,
}

impl BillingNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    pub fn orders(&self) -> BillingOrdersNs {
        BillingOrdersNs::new(Arc::clone(&self.client))
    }

    pub fn products(&self) -> BillingProductsNs {
        BillingProductsNs::new(Arc::clone(&self.client))
    }

    pub fn subscriptions(&self) -> BillingSubscriptionsNs {
        BillingSubscriptionsNs::new(Arc::clone(&self.client))
    }

    /// billing.checkout
    pub async fn checkout(
        &self,
        body: models::CheckoutRequest,
    ) -> Result<models::CheckoutResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/billing/checkout",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// BuildsNs procedure namespace.
pub struct BuildsNs {
    client: Arc<LeavePulse>,
}

impl BuildsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// builds.create
    pub async fn create(
        &self,
        body: models::BuildCreateRequest,
    ) -> Result<models::Build, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/builds",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// builds.import
    pub async fn import(
        &self,
        body: models::ImportSharedBuildRequest,
    ) -> Result<models::Build, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/builds/import",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// builds.preview
    pub async fn preview(&self, share_token: String) -> Result<models::Build, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/builds/preview/{}", share_token),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// builds.shared_with_me
    pub async fn shared_with_me(&self) -> Result<models::BuildList, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/builds/shared-with-me",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// DiscordLinkNs procedure namespace.
pub struct DiscordLinkNs {
    client: Arc<LeavePulse>,
}

impl DiscordLinkNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// discord.link.complete
    pub async fn complete(
        &self,
        body: models::CompleteLinkRequest,
    ) -> Result<models::CompleteLinkResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/discord/link/complete",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// discord.link.session
    pub async fn session(
        &self,
        params: models::DiscordLinkSessionParams,
    ) -> Result<models::LinkSession, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/discord/link/session",
                &[("state", params.state.map(|v| v.to_string()))],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// discord.link.token
    pub async fn token(
        &self,
        body: models::CreateLinkTokenRequest,
    ) -> Result<models::LinkTokenResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/discord/link/token",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// DiscordNs procedure namespace.
pub struct DiscordNs {
    client: Arc<LeavePulse>,
}

impl DiscordNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    pub fn link(&self) -> DiscordLinkNs {
        DiscordLinkNs::new(Arc::clone(&self.client))
    }
}

/// MonitoringMeNs procedure namespace.
pub struct MonitoringMeNs {
    client: Arc<LeavePulse>,
}

impl MonitoringMeNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// monitoring.me.stats
    pub async fn stats(&self) -> Result<models::MyDashboardStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/monitoring/me/stats",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// MonitoringMeStatsNs procedure namespace.
pub struct MonitoringMeStatsNs {
    client: Arc<LeavePulse>,
}

impl MonitoringMeStatsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// monitoring.me.stats.unverified
    pub async fn unverified(&self) -> Result<models::MyDashboardStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/monitoring/me/stats/unverified",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// MonitoringNs procedure namespace.
pub struct MonitoringNs {
    client: Arc<LeavePulse>,
}

impl MonitoringNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    pub fn me(&self) -> MonitoringMeNs {
        MonitoringMeNs::new(Arc::clone(&self.client))
    }

    pub fn me_stats(&self) -> MonitoringMeStatsNs {
        MonitoringMeStatsNs::new(Arc::clone(&self.client))
    }

    /// monitoring.landing
    pub async fn landing(&self) -> Result<models::LandingStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/monitoring/landing",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// PasswordNs procedure namespace.
pub struct PasswordNs {
    client: Arc<LeavePulse>,
}

impl PasswordNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// password.reset_confirm
    pub async fn reset_confirm(
        &self,
        body: models::PasswordResetConfirmRequest,
    ) -> Result<models::PasswordResetResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/password/reset/confirm",
                Channel::PlatformPublic,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// password.reset_request
    pub async fn reset_request(
        &self,
        body: models::PasswordResetRequest,
    ) -> Result<models::PasswordResetResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/password/reset/request",
                Channel::PlatformPublic,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// ProjectsNs procedure namespace.
pub struct ProjectsNs {
    client: Arc<LeavePulse>,
}

impl ProjectsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// projects.stats
    pub async fn stats(
        &self,
        params: models::ProjectsStatsParams,
    ) -> Result<models::ProjectFilterStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/projects/stats",
                &[
                    ("q", params.q.map(|v| v.to_string())),
                    ("edition", params.edition.map(|v| v.to_string())),
                    ("access", params.access.map(|v| v.to_string())),
                    ("features", params.features.map(|v| v.to_string())),
                    ("region", params.region.map(|v| v.to_string())),
                    ("hosting", params.hosting.map(|v| v.to_string())),
                    ("verified", params.verified.map(|v| v.to_string())),
                ],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.bridge
    pub async fn bridge(&self, server_id: i64) -> Result<models::BridgeSettings, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/discord/servers/{}/bridge", server_id),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.bridge_update
    pub async fn bridge_update(
        &self,
        server_id: i64,
        body: models::BridgeSettingsUpdateRequest,
    ) -> Result<models::BridgeSettings, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/discord/servers/{}/bridge", server_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.bridge_import
    pub async fn bridge_import(
        &self,
        server_id: i64,
        body: models::ImportPullRequest,
    ) -> Result<models::ImportPull, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/discord/servers/{}/import-pull", server_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.bridge_roles
    pub async fn bridge_roles(
        &self,
        server_id: i64,
    ) -> Result<models::RoleCatalog, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/discord/servers/{}/roles-catalog", server_id),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.projects_list
    pub async fn projects_list(
        &self,
        params: models::ProjectsProjectsListParams,
    ) -> Result<models::WorkspaceListResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/me/projects",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.projects_resolve
    pub async fn projects_resolve(
        &self,
        project_ref: String,
    ) -> Result<models::WorkspaceResolveResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/me/projects/resolve/{}", project_ref),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.list
    pub async fn list(
        &self,
        params: models::ProjectsListParams,
    ) -> Result<models::ProjectListResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/projects",
                &[
                    ("q", params.q.map(|v| v.to_string())),
                    ("edition", params.edition.map(|v| v.to_string())),
                    ("access", params.access.map(|v| v.to_string())),
                    ("features", params.features.map(|v| v.to_string())),
                    ("region", params.region.map(|v| v.to_string())),
                    ("hosting", params.hosting.map(|v| v.to_string())),
                    ("verified", params.verified.map(|v| v.to_string())),
                    ("has_build", params.has_build.map(|v| v.to_string())),
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                    ("sort", params.sort.map(|v| v.to_string())),
                ],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.create
    pub async fn create(
        &self,
        body: models::ProjectCreateRequest,
    ) -> Result<models::ProjectCreateResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/projects",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// projects.resolve
    pub async fn resolve(
        &self,
        project_ref: String,
    ) -> Result<models::ProjectResolveResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/projects/resolve/{}", project_ref),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// RbacNs procedure namespace.
pub struct RbacNs {
    client: Arc<LeavePulse>,
}

impl RbacNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// rbac.batch_resolve
    pub async fn batch_resolve(
        &self,
        body: models::BatchResolveRequest,
    ) -> Result<models::BatchResolveResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/rbac/batch-resolve",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// ServersNs procedure namespace.
pub struct ServersNs {
    client: Arc<LeavePulse>,
}

impl ServersNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// servers.resolve
    pub async fn resolve(&self, server_ref: String) -> Result<models::ServerCard, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/servers/resolve/{}", server_ref),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// StatsNs procedure namespace.
pub struct StatsNs {
    client: Arc<LeavePulse>,
}

impl StatsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// stats.filter
    pub async fn filter(
        &self,
        params: models::StatsFilterParams,
    ) -> Result<models::FilterStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/stats/filter",
                &[
                    ("q", params.q.map(|v| v.to_string())),
                    ("edition", params.edition.map(|v| v.to_string())),
                    ("access", params.access.map(|v| v.to_string())),
                    ("features", params.features.map(|v| v.to_string())),
                    ("region", params.region.map(|v| v.to_string())),
                    ("hosting", params.hosting.map(|v| v.to_string())),
                    ("verified", params.verified.map(|v| v.to_string())),
                    ("role", params.role.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// stats.live
    pub async fn live(&self) -> Result<models::LiveDashboardStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/stats/live",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// TicketsNs procedure namespace.
pub struct TicketsNs {
    client: Arc<LeavePulse>,
}

impl TicketsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// tickets.create
    pub async fn create(
        &self,
        body: models::TicketCreateRequest,
    ) -> Result<models::TicketDetail, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/community/tickets",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// tickets.mine
    pub async fn mine(
        &self,
        params: models::TicketsMineParams,
    ) -> Result<models::TicketList, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/community/tickets/my",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("limit", params.limit.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// UpdatesNs procedure namespace.
pub struct UpdatesNs {
    client: Arc<LeavePulse>,
}

impl UpdatesNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// updates.manifest
    pub async fn manifest(
        &self,
        params: models::UpdatesManifestParams,
    ) -> Result<models::UpdateManifest, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/launcher/updates/manifest",
                &[
                    ("channel", params.channel.map(|v| v.to_string())),
                    ("platform", params.platform.map(|v| v.to_string())),
                    ("server_id", params.server_id.map(|v| v.to_string())),
                ],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// updates.manifest_upsert
    pub async fn manifest_upsert(
        &self,
        channel: String,
        platform: String,
        body: models::UpdateManifestUpsert,
    ) -> Result<models::UpdateManifest, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Put,
                &format!("/v1/launcher/updates/manifests/{}/{}", channel, platform),
                Channel::PlatformPublic,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// updates.manifest_delete
    pub async fn manifest_delete(
        &self,
        channel: String,
        platform: String,
    ) -> Result<models::DeleteAck, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!(
                    "/v1/launcher/updates/manifests/{}/{}/delete",
                    channel, platform
                ),
                Channel::PlatformPublic,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// updates.report
    pub async fn report(
        &self,
        body: models::UpdateReportInput,
    ) -> Result<models::UpdateReportAck, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/launcher/updates/report",
                Channel::PlatformPublic,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// UsersNs procedure namespace.
pub struct UsersNs {
    client: Arc<LeavePulse>,
}

impl UsersNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// users.batch
    pub async fn batch(
        &self,
        body: models::BatchPublicProfilesRequest,
    ) -> Result<models::BatchPublicProfilesResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/users/public-profiles",
                Channel::PlatformPublic,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// users.search
    pub async fn search(
        &self,
        params: models::UsersSearchParams,
    ) -> Result<models::BatchPublicProfilesResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/users/search",
                &[
                    ("q", params.q.map(|v| v.to_string())),
                    ("limit", params.limit.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// users.engagement
    pub async fn engagement(&self, user_id: i64) -> Result<models::UserEngagement, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/community/users/{}/engagement", user_id),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// users.activity_list
    pub async fn activity_list(
        &self,
        user_id: i64,
        params: models::UsersActivityListParams,
    ) -> Result<models::UserRecentActivity, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/community/users/{}/recent-activity", user_id),
                &[("limit", params.limit.map(|v| v.to_string()))],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// VerificationNs procedure namespace.
pub struct VerificationNs {
    client: Arc<LeavePulse>,
}

impl VerificationNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// verification.start_dns
    pub async fn start_dns(
        &self,
        body: models::VerificationStartRequest,
    ) -> Result<models::DnsVerification, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/servers/verification/dns",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// verification.check_dns
    pub async fn check_dns(
        &self,
        body: models::VerificationCheckRequest,
    ) -> Result<models::DnsVerification, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/servers/verification/dns/check",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// verification.start_motd
    pub async fn start_motd(
        &self,
        body: models::VerificationStartRequest,
    ) -> Result<models::MotdVerification, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/servers/verification/motd",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// verification.check_motd
    pub async fn check_motd(
        &self,
        body: models::VerificationCheckRequest,
    ) -> Result<models::MotdVerification, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/servers/verification/motd/check",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// verification.start_plugin
    pub async fn start_plugin(
        &self,
        body: models::PluginVerificationStartRequest,
    ) -> Result<models::PluginVerification, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/servers/verification/plugin",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// verification.check_plugin
    pub async fn check_plugin(
        &self,
        server_id: i64,
    ) -> Result<models::PluginVerification, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/servers/verification/plugin/{}", server_id),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// WhitelistBindingsNs procedure namespace.
pub struct WhitelistBindingsNs {
    client: Arc<LeavePulse>,
}

impl WhitelistBindingsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// whitelist.bindings.create
    pub async fn create(
        &self,
        body: models::WhitelistBindingWriteRequest,
    ) -> Result<models::WhitelistBindingDetail, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/whitelist/bindings",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// WhitelistFormsNs procedure namespace.
pub struct WhitelistFormsNs {
    client: Arc<LeavePulse>,
}

impl WhitelistFormsNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// whitelist.forms.list
    pub async fn list(
        &self,
        params: models::WhitelistFormsListParams,
    ) -> Result<models::WhitelistFormPage, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/whitelist/forms",
                &[
                    ("project_id", params.project_id.map(|v| v.to_string())),
                    ("search", params.search.map(|v| v.to_string())),
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// whitelist.forms.create
    pub async fn create(
        &self,
        body: models::WhitelistFormCreateRequest,
    ) -> Result<models::WhitelistFormDetail, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/whitelist/forms",
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}

/// WhitelistNs procedure namespace.
pub struct WhitelistNs {
    client: Arc<LeavePulse>,
}

impl WhitelistNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    pub fn bindings(&self) -> WhitelistBindingsNs {
        WhitelistBindingsNs::new(Arc::clone(&self.client))
    }

    pub fn forms(&self) -> WhitelistFormsNs {
        WhitelistFormsNs::new(Arc::clone(&self.client))
    }
}
