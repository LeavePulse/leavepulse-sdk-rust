// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::resources::Application;
use crate::resources::Build;
use crate::resources::Server;
use crate::resources::Session;
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

    /// me.issue_ws_token
    pub async fn issue_ws_token(&self) -> Result<models::WsTokenResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Post, "/v1/auth/ws-token", Channel::Platform, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// builds.list
    pub async fn builds_list(&self) -> Result<Vec<Build>, TransportError> {
        let data = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/builds",
            Channel::Platform,
        )
        .await?;
        let items = if data.is_array() {
            data
        } else {
            data.get("items").cloned().unwrap_or(Value::Null)
        };
        Ok(self.client.hydrate_many::<Build>("Build", items))
    }

    /// me.engagement
    pub async fn engagement(&self) -> Result<models::UserEngagement, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/community/projects/me/engagement",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.account.change_email
    pub async fn account_change_email(&self) -> Result<models::EmailChangeResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/account/actions/change-email",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.account.delete
    pub async fn account_delete(&self) -> Result<models::AccountDeletionResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/account/actions/delete",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.account.export
    pub async fn account_export(&self) -> Result<models::AccountExport, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/me/account/export",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.avatar.remove
    pub async fn avatar_remove(&self) -> Result<models::UserProfile, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Delete, "/v1/me/avatar", Channel::Platform, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.avatar.upload
    pub async fn avatar_upload(&self) -> Result<models::UserProfile, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Post, "/v1/me/avatar", Channel::Platform, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.avatar.set
    pub async fn avatar_set(&self) -> Result<models::UserProfile, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Put, "/v1/me/avatar", Channel::Platform, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.minecraft.unlink
    pub async fn minecraft_unlink(&self) -> Result<models::MinecraftUnlinkResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/me/minecraft/accounts/{}", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.minecraft.link_code
    pub async fn minecraft_link_code(
        &self,
    ) -> Result<models::MinecraftLinkCodeResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/minecraft/link-code",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.minecraft.complete_link
    pub async fn minecraft_complete_link(
        &self,
    ) -> Result<models::LinkCompletionResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/minecraft/link/complete",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.minecraft.official_link_start
    pub async fn minecraft_official_link_start(
        &self,
    ) -> Result<models::MinecraftOfficialLinkStart, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/minecraft/official/start",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.minecraft.resolve
    pub async fn minecraft_resolve(
        &self,
    ) -> Result<models::MinecraftCandidateAccount, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/minecraft/resolve",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.minecraft.state
    pub async fn minecraft_state(
        &self,
        params: models::MeMinecraftStateParams,
    ) -> Result<Vec<models::MinecraftVerificationState>, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/me/minecraft/state",
                &[("project_id", params.project_id.map(|v| v.to_string()))],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.notifications.get
    pub async fn notifications_get(
        &self,
    ) -> Result<models::NotificationPreferences, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/me/notifications",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.notifications.update
    pub async fn notifications_update(
        &self,
    ) -> Result<models::NotificationPreferences, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Patch,
                "/v1/me/notifications",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.oauth.list
    pub async fn oauth_list(&self) -> Result<models::OAuthProvidersResponse, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/me/oauth/providers",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.oauth.unlink
    pub async fn oauth_unlink(&self) -> Result<models::OAuthUnlinkResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/me/oauth/{}", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.oauth.link_start
    pub async fn oauth_link_start(&self) -> Result<models::OAuthLinkStartResponse, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/me/oauth/{}/link/start", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.password.status
    pub async fn password_status(&self) -> Result<models::PasswordStatus, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/me/password",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.password.set
    pub async fn password_set(&self) -> Result<models::PasswordMutationResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Put, "/v1/me/password", Channel::Platform, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.password.change
    pub async fn password_change(&self) -> Result<models::PasswordMutationResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/password/actions/change",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.profile.update
    pub async fn profile_update(&self) -> Result<models::UserProfile, TransportError> {
        let value = self
            .client
            .transport()
            .request(Method::Post, "/v1/me/profile", Channel::Platform, None)
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.sessions.list
    pub async fn sessions_list(&self) -> Result<Session, TransportError> {
        let data = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/me/sessions",
            Channel::Platform,
        )
        .await?;
        Ok(self
            .client
            .hydrate::<Session>("Session", data, Some("sessions")))
    }

    /// me.sessions.revoke_others
    pub async fn sessions_revoke_others(
        &self,
    ) -> Result<models::RevokeOtherSessionsResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/sessions/actions/revoke-others",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.stats
    pub async fn stats(
        &self,
        params: models::MeStatsParams,
    ) -> Result<models::MyPlayerStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/me/stats",
                &[("estimated", params.estimated.map(|v| v.to_string()))],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.stats_unverified
    pub async fn stats_unverified(&self) -> Result<models::MyPlayerStats, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/me/stats/unverified",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.totp.status
    pub async fn totp_status(&self) -> Result<models::TotpStatus, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            "/v1/me/totp",
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.totp.begin
    pub async fn totp_begin(&self) -> Result<models::TotpBeginResult, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/totp/actions/begin",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.totp.confirm
    pub async fn totp_confirm(&self) -> Result<models::TotpStatus, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/totp/actions/confirm",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.totp.disable
    pub async fn totp_disable(&self) -> Result<models::TotpStatus, TransportError> {
        let value = self
            .client
            .transport()
            .request(
                Method::Post,
                "/v1/me/totp/actions/disable",
                Channel::Platform,
                None,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// me.whitelist.applications
    pub async fn whitelist_applications(
        &self,
        params: models::MeWhitelistApplicationsParams,
    ) -> Result<Vec<Application>, TransportError> {
        let data = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/me/whitelist/applications",
                &[
                    ("status", params.status.map(|v| v.to_string())),
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        let items = if data.is_array() {
            data
        } else {
            data.get("items").cloned().unwrap_or(Value::Null)
        };
        Ok(self
            .client
            .hydrate_many::<Application>("Application", items))
    }

    /// me.servers.list
    pub async fn servers_list(
        &self,
        params: models::MeServersListParams,
    ) -> Result<Vec<Server>, TransportError> {
        let data = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/servers/mine",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        let items = if data.is_array() {
            data
        } else {
            data.get("items").cloned().unwrap_or(Value::Null)
        };
        Ok(self.client.hydrate_many::<Server>("Server", items))
    }

    /// me.servers.issues
    pub async fn servers_issues(
        &self,
        params: models::MeServersIssuesParams,
    ) -> Result<Vec<models::MyServerIssuesPage>, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                "/v1/servers/mine/issues",
                &[
                    ("page", params.page.map(|v| v.to_string())),
                    ("per_page", params.per_page.map(|v| v.to_string())),
                    ("include_ok", params.include_ok.map(|v| v.to_string())),
                ],
            ),
            Channel::Platform,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}
