// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Server resource.
#[derive(Clone)]
pub struct Server {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Server {
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

    /// Re-fetch this Server and hydrate in place.
    pub async fn refresh(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Get,
                &format!("/v1/servers/{}", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// Whether the current user may change_address (RFC §4).
    pub fn can_change_address(&self) -> bool {
        resource::has_capability(&self.data, "server.change_address")
    }

    /// Whether the current user may change_slug (RFC §4).
    pub fn can_change_slug(&self) -> bool {
        resource::has_capability(&self.data, "server.change_slug")
    }

    /// Whether the current user may force_ping (RFC §4).
    pub fn can_force_ping(&self) -> bool {
        resource::has_capability(&self.data, "server.force_ping")
    }

    /// Whether the current user may rename (RFC §4).
    pub fn can_rename(&self) -> bool {
        resource::has_capability(&self.data, "server.rename")
    }

    /// Whether the current user may set_bedrock_port (RFC §4).
    pub fn can_set_bedrock_port(&self) -> bool {
        resource::has_capability(&self.data, "server.set_bedrock_port")
    }

    /// Whether the current user may set_description (RFC §4).
    pub fn can_set_description(&self) -> bool {
        resource::has_capability(&self.data, "server.set_description")
    }

    /// Whether the current user may set_parent (RFC §4).
    pub fn can_set_parent(&self) -> bool {
        resource::has_capability(&self.data, "server.set_parent")
    }

    /// Whether the current user may set_ping_port (RFC §4).
    pub fn can_set_ping_port(&self) -> bool {
        resource::has_capability(&self.data, "server.set_ping_port")
    }

    /// Whether the current user may set_regions (RFC §4).
    pub fn can_set_regions(&self) -> bool {
        resource::has_capability(&self.data, "server.set_regions")
    }

    /// Whether the current user may set_role (RFC §4).
    pub fn can_set_role(&self) -> bool {
        resource::has_capability(&self.data, "server.set_role")
    }

    /// Whether the current user may set_show_description (RFC §4).
    pub fn can_set_show_description(&self) -> bool {
        resource::has_capability(&self.data, "server.set_show_description")
    }

    /// Whether the current user may set_show_in_public (RFC §4).
    pub fn can_set_show_in_public(&self) -> bool {
        resource::has_capability(&self.data, "server.set_show_in_public")
    }

    /// Whether the current user may set_team_enabled (RFC §4).
    pub fn can_set_team_enabled(&self) -> bool {
        resource::has_capability(&self.data, "server.set_team_enabled")
    }

    /// Whether the current user may set_version_override (RFC §4).
    pub fn can_set_version_override(&self) -> bool {
        resource::has_capability(&self.data, "server.set_version_override")
    }

    /// Whether the current user may manage_bot (RFC §4).
    pub fn can_manage_bot(&self) -> bool {
        resource::has_capability(&self.data, "server.manage_bot")
    }

    /// Whether the current user may manage_voting (RFC §4).
    pub fn can_manage_voting(&self) -> bool {
        resource::has_capability(&self.data, "server.manage_voting")
    }

    /// Whether the current user may set_maintenance (RFC §4).
    pub fn can_set_maintenance(&self) -> bool {
        resource::has_capability(&self.data, "server.set_maintenance")
    }

    /// Whether the current user may set_motd (RFC §4).
    pub fn can_set_motd(&self) -> bool {
        resource::has_capability(&self.data, "server.set_motd")
    }

    /// Whether the current user may manage_social (RFC §4).
    pub fn can_manage_social(&self) -> bool {
        resource::has_capability(&self.data, "server.manage_social")
    }

    /// Whether the current user may import (RFC §4).
    pub fn can_import(&self) -> bool {
        resource::has_capability(&self.data, "server.import")
    }

    /// server.change_address
    pub async fn change_address(
        &self,
        body: models::ServerChangeAddressRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/change-address", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.change_slug
    pub async fn change_slug(
        &self,
        body: models::ServerChangeSlugRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/change-slug", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.force_ping
    pub async fn force_ping(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/force-ping", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.rename
    pub async fn rename(&self, body: models::ServerRenameRequest) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/rename", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_bedrock_port
    pub async fn set_bedrock_port(
        &self,
        body: models::ServerSetBedrockPortRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-bedrock-port", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_description
    pub async fn set_description(
        &self,
        body: models::ServerSetDescriptionRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-description", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_parent
    pub async fn set_parent(
        &self,
        body: models::ServerSetParentRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-parent", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_ping_port
    pub async fn set_ping_port(
        &self,
        body: models::ServerSetPingPortRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-ping-port", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_regions
    pub async fn set_regions(
        &self,
        body: models::ServerSetRegionsRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-regions", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_role
    pub async fn set_role(&self, body: models::ServerSetRoleRequest) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-role", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_show_description
    pub async fn set_show_description(
        &self,
        body: models::ServerSetShowDescriptionRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-show-description", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_show_in_public
    pub async fn set_show_in_public(
        &self,
        body: models::ServerSetShowInPublicRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-show-in-public", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_team_enabled
    pub async fn set_team_enabled(
        &self,
        body: models::ServerSetTeamEnabledRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-team-enabled", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.set_version_override
    pub async fn set_version_override(
        &self,
        body: models::ServerSetVersionOverrideRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/actions/set-version-override", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.bot.update
    pub async fn bot_update(
        &self,
        body: models::ServerBotUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/bot", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.issue_gateway_token
    pub async fn issue_gateway_token(
        &self,
        body: models::GatewayTokenRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/gateway-token", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.icons.upload
    pub async fn icons_upload(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/icon", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.icons.select
    pub async fn icons_select(
        &self,
        body: models::IconSelectRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/icon/select", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.voting.update
    pub async fn voting_update(
        &self,
        body: models::VotingLinksUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/integrations/voting", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.maintenance.update
    pub async fn maintenance_update(
        &self,
        body: models::ServerMaintenanceUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/maintenance", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.motd.update
    pub async fn motd_update(
        &self,
        body: models::ServerMotdUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/motd", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.discord.unlink
    pub async fn discord_unlink(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/servers/{}/social/discord", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.discord.update
    pub async fn discord_update(
        &self,
        body: models::DiscordLinkUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/social/discord", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.discord.verify
    pub async fn discord_verify(
        &self,
        body: models::DiscordVerifyRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/social/discord/verify", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.social.update
    pub async fn social_update(
        &self,
        body: models::SocialLinksUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/social/links", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.social.verify
    pub async fn social_verify(
        &self,
        body: models::SocialLinkVerifyRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/social/verify", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.subservers.issue_link_code
    pub async fn subservers_issue_link_code(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/subservers/auto-link-code", self.id()),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.members.create
    pub async fn members_create(
        &self,
        body: models::TeamMemberCreateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/team/members", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.members.delete
    pub async fn members_delete(&self, member_id: String) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/servers/{}/team/members/{}", self.id(), member_id),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.members.update
    pub async fn members_update(
        &self,
        member_id: String,
        body: models::TeamMemberUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/team/members/{}", self.id(), member_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.roles.create
    pub async fn roles_create(
        &self,
        body: models::TeamRoleCreateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/team/roles", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.roles.delete
    pub async fn roles_delete(&self, role_id: String) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/servers/{}/team/roles/{}", self.id(), role_id),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.roles.update
    pub async fn roles_update(
        &self,
        role_id: String,
        body: models::TeamRoleUpdateRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!("/v1/servers/{}/team/roles/{}", self.id(), role_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.translations.delete
    pub async fn translations_delete(
        &self,
        field: String,
        locale: String,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!(
                    "/v1/servers/{}/translations/{}/{}",
                    self.id(),
                    field,
                    locale
                ),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.translations.set
    pub async fn translations_set(
        &self,
        field: String,
        locale: String,
        body: models::ServerTranslationUpsertRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Patch,
                &format!(
                    "/v1/servers/{}/translations/{}/{}",
                    self.id(),
                    field,
                    locale
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.whitelist.add_direct
    pub async fn whitelist_add_direct(
        &self,
        body: models::WhitelistDirectAddRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/whitelist/direct", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.whitelist.remove_direct
    pub async fn whitelist_remove_direct(&self, entry_id: String) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!("/v1/servers/{}/whitelist/direct/{}", self.id(), entry_id),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.whitelist.create_import
    pub async fn whitelist_create_import(
        &self,
        body: models::WhitelistImportRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/whitelist/imports", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }

    /// server.whitelist.pull_minecraft_import
    pub async fn whitelist_pull_minecraft_import(
        &self,
        body: models::WhitelistMinecraftPullRequest,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/servers/{}/whitelist/imports/pull-minecraft", self.id()),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Server = self.client.hydrate("Server", data, None);
        Ok(())
    }
}
