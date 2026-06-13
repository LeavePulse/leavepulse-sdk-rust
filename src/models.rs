// Generated from the LeavePulse contract. Do not edit.
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

/// AccountDeletionResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeletionResult {
    pub scheduled_at: String,
    pub status: String,
}

/// AccountExport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<AccountExportIdentity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<AccountExportSession>>,
    pub user: UserProfile,
}

/// AccountExportIdentity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExportIdentity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<String>,
    pub provider: String,
}

/// AccountExportSession
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExportSession {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    #[serde(rename = "vote")]
    Vote,
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "favorite")]
    Favorite,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Vote => "vote",
            Self::Comment => "comment",
            Self::Favorite => "favorite",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// AdminChangeProjectSlugRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminChangeProjectSlugRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// AdminDeleteResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminDeleteResponse {
    pub ok: bool,
}

/// AdminDiscordSubjectResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminDiscordSubjectResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_subject: Option<String>,
    pub status: String,
}

/// AdminForceCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminForceCreateRequest {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    pub name: String,
    pub owner_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    pub server_role: ServerRole,
}

/// AdminMinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccount {
    pub account_type: MinecraftAccountType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_scope_id: Option<String>,
    pub identity_scope_type: MinecraftIdentityScopeType,
    pub link_source: MinecraftLinkSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_server_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid_type: Option<MinecraftUuidType>,
    pub verification_status: MinecraftVerificationStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

/// AdminMinecraftAccountCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountCreateRequest {
    pub minecraft_nick: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
}

/// AdminMinecraftAccountDeleteResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountDeleteResult {
    pub account_id: String,
    pub status: String,
}

/// AdminMinecraftAccountDetailed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountDetailed {
    pub account_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_scope_id: Option<i64>,
    pub identity_scope_type: String,
    pub link_source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_server_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid_type: Option<String>,
    pub verification_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

/// AdminMinecraftAccountUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountUpdateRequest {
    pub minecraft_nick: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
}

/// AdminMinecraftAccountWriteRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountWriteRequest {
    pub minecraft_nick: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
}

/// AdminProject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminProject {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_server_id: Option<Snowflake>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<LifecycleState>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_server_id: Option<Snowflake>,
    pub online_strategy: OnlineStrategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_rollout_mode: Option<RolloutMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_rollout_state: Option<RolloutState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_server_count: Option<i64>,
}

/// AdminProjectDeleteResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminProjectDeleteResponse {
    pub ok: bool,
}

/// AdminProjectListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminProjectListResponse {
    pub items: Vec<AdminProject>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// AdminRenameProjectRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRenameProjectRequest {
    pub name: String,
}

/// AdminRole
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRole {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// AdminRoleDeleteResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleDeleteResponse {
    pub role_id: String,
}

/// AdminRoleListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleListResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AdminRole>>,
}

/// AdminRoleRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleRequest {
    pub key: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// AdminServerListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminServerListResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AdminServerSummary>>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// AdminServerSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminServerSummary {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    pub id: String,
    pub ip_or_domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    pub server_role: ServerRole,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_compatibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_last_seen_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_protocol_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_rollout_state: Option<RolloutState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_version: Option<String>,
}

/// AdminServerUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminServerUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<serde_json::Value>,
}

/// AdminSetProjectOnlineStrategyRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSetProjectOnlineStrategyRequest {
    pub online_strategy: OnlineStrategy,
}

/// AdminSetProjectRolloutModeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSetProjectRolloutModeRequest {
    pub verified_plugin_rollout_mode: RolloutMode,
}

/// AdminTransferOwnershipRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminTransferOwnershipRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
}

/// AdminUserDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shadow: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_accounts: Option<Vec<AdminMinecraftAccount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub status: UserStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub username: String,
}

/// AdminUserDiscordUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserDiscordUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_subject: Option<String>,
}

/// AdminUserListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserListResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AdminUserSummary>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    pub total: i64,
}

/// AdminUserSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub status: UserStatus,
    pub username: String,
}

/// AdminUserUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub status: UserStatus,
    pub username: String,
}

/// AuthStatusRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    pub username: String,
}

/// AuthStatusResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatusResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub exists: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs_password_change: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trusted_login: Option<bool>,
}

/// AvatarUrlRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUrlRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

/// BatchPublicProfilesRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPublicProfilesRequest {
    pub user_ids: Vec<Snowflake>,
}

/// BatchPublicProfilesResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPublicProfilesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PublicProfileCard>>,
}

/// BatchResolveRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResolveRequest {
    pub user_ids: Vec<Snowflake>,
}

/// BatchResolveResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResolveResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PlatformPermsRow>>,
}

/// BridgeSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_channel_id: Option<String>,
    pub discord_to_mc_enabled: bool,
    pub discord_to_mc_mode: String,
    pub discord_to_mc_plain_format: String,
    pub discord_to_mc_spoof_content_format: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_target_channel: Option<String>,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_chat_routing: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub mc_to_discord_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_notifications: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub mc_to_discord_use_webhook: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_chat_filter: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub nickname_sync_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_notifications: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// BridgeSettingsUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSettingsUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_plain_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_spoof_content_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_to_mc_target_channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_chat_routing: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_notifications: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc_to_discord_use_webhook: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_chat_filter: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname_sync_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_notifications: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub access: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_blob_sha256: Option<String>,
    pub created_at: String,
    pub has_config_blob: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: String,
    pub is_public: bool,
    pub manifest: BuildManifest,
    pub name: String,
    pub owner_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_token: Option<String>,
    pub summary: String,
    pub updated_at: String,
    pub updated_revision: i64,
}

/// BuildCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<BuildManifest>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// BuildList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildList {
    pub items: Vec<BuildSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    pub total: i64,
}

/// BuildManifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jvm_args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loader_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loader_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_max_mb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_min_mb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mods: Option<Vec<BuildManifestMod>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
}

/// BuildManifestMod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifestMod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// BuildSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSummary {
    pub access: String,
    pub created_at: String,
    pub has_config_blob: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: String,
    pub is_public: bool,
    pub name: String,
    pub owner_id: String,
    pub summary: String,
    pub updated_at: String,
    pub updated_revision: i64,
}

/// BuildUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<BuildManifest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// CheckoutRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_subscription: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_pull: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
}

/// CheckoutResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    pub order: Order,
}

/// Collaborator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    pub build_id: String,
    pub can_edit: bool,
    pub user_id: String,
}

/// CollaboratorAddRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorAddRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    pub user_id: String,
}

/// CollaboratorList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorList {
    pub items: Vec<Collaborator>,
}

/// Comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub author: CommentAuthor,
    pub content: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<String>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub likes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<CommentReply>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<TextTranslation>,
}

/// CommentAuthor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAuthor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// CommentCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCreateRequest {
    pub content: String,
}

/// CommentLikeResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentLikeResult {
    pub comment_id: Snowflake,
    pub liked: bool,
    pub likes: i64,
}

/// CommentList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentList {
    pub items: Vec<Comment>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// CommentReply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    pub author: CommentAuthor,
    pub content: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<String>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<TextTranslation>,
}

/// CompleteLinkRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteLinkRequest {
    pub project_id: Snowflake,
    pub state: String,
}

/// CompleteLinkResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteLinkResult {
    pub guild_id: String,
    pub project_id: Snowflake,
    pub status: LinkResultStatus,
}

/// ConfigBlobConfirmRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBlobConfirmRequest {
    pub sha256: String,
}

/// ConfigBlobRef
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBlobRef {
    pub download_url: String,
    pub sha256: String,
}

/// ConfigBlobUpload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBlobUpload {
    pub expires_in_seconds: i64,
    pub object_key: String,
    pub upload_url: String,
}

/// ConfigBlobUploadRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBlobUploadRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

/// CreateLinkTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLinkTokenRequest {
    pub project_id: Snowflake,
}

/// CreateProjectServerRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectServerRequest {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// CreateStatusOverrideRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStatusOverrideRequest {
    pub ends_at: String,
    pub mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub starts_at: String,
}

/// DashboardAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// DashboardDailyActivityPoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardDailyActivityPoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    pub date: String,
}

/// DashboardServerItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardServerItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playtime_seconds: Option<i64>,
    pub server_id: Snowflake,
}

/// DeleteAck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAck {
    pub deleted: bool,
}

/// DeleteBuildResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBuildResult {
    pub build_id: String,
}

/// DeleteCommentResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommentResult {
    pub comment_id: Snowflake,
}

/// DeleteStatusOverrideResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStatusOverrideResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
}

/// DeviceApproveRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceApproveRequest {
    pub user_code: String,
}

/// DeviceApproveResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceApproveResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevicePollStatus {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "slow_down")]
    SlowDown,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for DevicePollStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Approved => "approved",
            Self::Pending => "pending",
            Self::SlowDown => "slow_down",
            Self::Expired => "expired",
            Self::Denied => "denied",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// DeviceStartRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStartRequest {
    pub client_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

/// DeviceStartResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStartResult {
    pub device_code: String,
    pub expires_in: i64,
    pub interval: i64,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
}

/// DeviceTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenRequest {
    pub device_code: String,
}

/// DeviceTokenResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token_expires_in: Option<i64>,
    pub status: DevicePollStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

/// DiscordLink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordLink {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    pub server_id: Snowflake,
    pub verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

/// DiscordLinkUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordLinkUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscordMembershipMode {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "on_apply")]
    OnApply,
    #[serde(rename = "on_join")]
    OnJoin,
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for DiscordMembershipMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Disabled => "disabled",
            Self::OnApply => "on_apply",
            Self::OnJoin => "on_join",
            Self::Both => "both",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// DiscordRoleTarget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRoleTarget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_discord_user_ids: Option<Vec<String>>,
    pub discord_role_id: String,
    pub role_id: Snowflake,
    pub role_key: String,
    pub role_name: String,
}

/// DiscordRoleTargets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRoleTargets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DiscordRoleTarget>>,
    pub project_id: Snowflake,
}

/// DiscordVerifyRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordVerifyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_url: Option<String>,
}

/// DiscoveryApproveResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryApproveResult {
    pub candidate_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<i64>,
    pub status: String,
}

/// DiscoveryCandidateEditRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryCandidateEditRequest {
    pub edits: std::collections::HashMap<String, serde_json::Value>,
}

/// DiscoveryIgnoreResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryIgnoreResult {
    pub candidate_id: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoverySort {
    #[serde(rename = "sources")]
    Sources,
    #[serde(rename = "discord_members")]
    DiscordMembers,
    #[serde(rename = "mc_online")]
    McOnline,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for DiscoverySort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Sources => "sources",
            Self::DiscordMembers => "discord_members",
            Self::McOnline => "mc_online",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// DnsVerification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsVerification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checked_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub record_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    pub record_value: String,
    pub server: VerificationServerSummary,
    pub status: VerificationStatus,
    pub token: String,
}

/// EmailChangeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailChangeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
}

/// EmailChangeResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailChangeResult {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementMode {
    #[serde(rename = "kick")]
    Kick,
    #[serde(rename = "restrict")]
    Restrict,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for EnforcementMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Kick => "kick",
            Self::Restrict => "restrict",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// FilterCount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCount {
    pub key: String,
    pub value: i64,
}

/// FilterStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editions: Option<std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosting: Option<std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<std::collections::HashMap<String, i64>>,
    pub total: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<std::collections::HashMap<String, i64>>,
}

/// ForcePingResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForcePingResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooldown_seconds: Option<i64>,
    pub enqueued: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_hourly: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FreshnessState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for FreshnessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEdition {
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "bedrock")]
    Bedrock,
    #[serde(rename = "hybrid")]
    Hybrid,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for GameEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Java => "java",
            Self::Bedrock => "bedrock",
            Self::Hybrid => "hybrid",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// GatewayToken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayToken {
    pub audience: String,
    pub expires_in: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    pub server_id: Snowflake,
    pub session_id: String,
    pub token: String,
    pub token_type: String,
}

/// GatewayTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayTokenRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<i64>,
}

/// GlobalServerStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalServerStats {
    pub total_networks: i64,
    pub total_servers: i64,
    pub total_subservers: i64,
}

/// HistoryPoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_online: Option<f64>,
    pub collected_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_score: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_online: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_online: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OnlineState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_source: Option<TrustState>,
}

/// HistoryResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub period: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<HistoryPoint>>,
}

/// IconEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconEntry {
    pub created_at: String,
    pub key: String,
    pub source: IconSource,
    pub url: String,
}

/// IconHistory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IconEntry>>,
}

/// IconSelectRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSelectRequest {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IconSource {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for IconSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Manual => "manual",
            Self::Auto => "auto",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// ImportPull
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPull {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<String>,
    pub guild_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_name: Option<String>,
    pub scanned_messages: i64,
    pub source: String,
}

/// ImportPullRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_message_id: Option<String>,
    pub channel_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// ImportSharedBuildRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSharedBuildRequest {
    pub share_token: String,
}

/// InternalBatchIdentitiesRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalBatchIdentitiesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    pub user_ids: Vec<i64>,
}

/// InternalBatchIdentitiesResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalBatchIdentitiesResponse {
    pub items: Vec<InternalBatchIdentityItem>,
}

/// InternalBatchIdentityItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalBatchIdentityItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    pub user_id: i64,
}

/// InternalDiscordEnsureUserResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalDiscordEnsureUserResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shadow: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// InternalDiscordIdentityLookupResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalDiscordIdentityLookupResponse {
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// InternalMinecraftIdentityLookupResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftIdentityLookupResponse {
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// InternalMinecraftLinkCodeCandidateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftLinkCodeCandidateRequest {
    pub minecraft_nick: String,
    pub user_id: i64,
}

/// InternalMinecraftLinkCodeCompleteRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftLinkCodeCompleteRequest {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    pub server_id: i64,
}

/// InternalMinecraftLinkCodeCompleteResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftLinkCodeCompleteResponse {
    pub account: MinecraftAccountResponse,
    pub status: String,
    pub user_id: i64,
}

/// InternalMinecraftLinkCodeIssueRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftLinkCodeIssueRequest {
    pub minecraft_nick: String,
    pub user_id: i64,
}

/// InternalMinecraftLinkCodeIssueResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftLinkCodeIssueResponse {
    pub account: MinecraftAccountResolveResponse,
    pub code: String,
    pub expires_in: i64,
    pub status: String,
}

/// InternalMinecraftLinkCodeStoreRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftLinkCodeStoreRequest {
    pub account_type: String,
    pub code: String,
    pub expires_in: i64,
    pub minecraft_nick: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    pub user_id: i64,
}

/// InternalMinecraftLinkCodeStoreResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftLinkCodeStoreResponse {
    pub status: String,
}

/// InternalMinecraftProfilesStatsResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftProfilesStatsResponse {
    pub registered_profiles: i64,
}

/// InternalUserContactResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalUserContactResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// InternalUserDiscordSubjectResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalUserDiscordSubjectResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_subject: Option<String>,
    pub found: bool,
}

/// IntrospectRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectRequest {
    pub token: String,
}

/// IntrospectResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectResponse {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// IssuedTokenResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuedTokenResponse {
    pub audience: String,
    pub expires_in: i64,
    pub roles: Vec<String>,
    pub scope: Vec<String>,
    pub session_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    pub token: String,
    pub token_type: String,
}

/// JWKSResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWKSResponse {
    pub keys: Vec<PublicJWK>,
}

/// LandingStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingStats {
    pub players_with_profile: i64,
    pub registered_profiles: i64,
    pub total_playtime_hours: i64,
}

/// LaunchManifestInlineContent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestInlineContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mods: Option<Vec<LaunchManifestModRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourcepacks: Option<Vec<LaunchManifestPackRef>>,
}

/// LaunchManifestJvm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestJvm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ram_mb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_ram_mb: Option<i64>,
}

/// LaunchManifestMinecraft
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestMinecraft {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loader: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loader_version: Option<String>,
    pub version: String,
}

/// LaunchManifestModRef
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestModRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    pub filename: String,
    pub project_id: String,
    pub provider: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    pub version_id: String,
}

/// LaunchManifestMrpackContent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestMrpackContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    pub url: String,
}

/// LaunchManifestPackRef
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestPackRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    pub filename: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    pub provider: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// LaunchManifestServerEndpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestServerEndpoint {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// LaunchManifestWhitelist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchManifestWhitelist {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleState {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// LikedCommentIds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikedCommentIds {
    pub comment_ids: Vec<Snowflake>,
}

/// LinkCompletionRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCompletionRequest {
    pub code: String,
    pub minecraft_nick: String,
    pub minecraft_uuid: String,
    pub project_id: Snowflake,
    pub server_id: Snowflake,
}

/// LinkCompletionResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCompletionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<LinkedMinecraftAccount>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkResultStatus {
    #[serde(rename = "linked")]
    Linked,
    #[serde(rename = "already_linked")]
    AlreadyLinked,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for LinkResultStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Linked => "linked",
            Self::AlreadyLinked => "already_linked",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// LinkSession
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSession {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub already_linked_project_id: Option<Snowflake>,
    pub expires_at: String,
    pub guild_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_name: Option<String>,
    pub status: LinkSessionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkSessionStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "used")]
    Used,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for LinkSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Pending => "pending",
            Self::Used => "used",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// LinkTokenResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenResult {
    pub expires_at: String,
    pub token: String,
}

/// LinkedMinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedMinecraftAccount {
    pub account_type: MinecraftAccountType,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_scope_id: Option<Snowflake>,
    pub identity_scope_type: MinecraftIdentityScopeType,
    pub link_source: MinecraftLinkSource,
    pub minecraft_nick: String,
    pub minecraft_uuid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_server_id: Option<Snowflake>,
    pub verification_status: MinecraftVerificationStatus,
}

/// LiveDashboardStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDashboardStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collected_at: Option<String>,
    pub players_online: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub players_with_profile: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_profiles: Option<i64>,
    pub servers_online: i64,
    pub servers_with_agent: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_playtime_hours: Option<i64>,
}

/// LiveStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStatus {
    pub collected_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<OnlineState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    pub freshness_state: FreshnessState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_reason: Option<String>,
    pub online_state: OnlineState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<String>>,
    pub server_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<TrustState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// LoginResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_jwt: String,
    pub audience: String,
    pub expires_in: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token_expires_in: Option<i64>,
    pub roles: Vec<String>,
    pub scope: Vec<String>,
    pub sid: String,
    pub user: UserPublic,
}

/// LogoutResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub status: String,
}

/// ManifestComponent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestComponent {
    pub path: String,
    pub sha256: String,
}

/// ManifestDelta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestDelta {
    pub patch_path: String,
    pub patch_sha256: String,
    pub result_sha256: String,
}

/// MeResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_accounts: Option<Vec<MinecraftAccount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub user_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for MemberState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// MinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccount {
    pub account_type: MinecraftAccountType,
    pub minecraft_nick: String,
    pub minecraft_uuid: String,
}

/// MinecraftAccountLinkResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccountLinkResponse {
    pub account: MinecraftAccountResponse,
    pub status: String,
}

/// MinecraftAccountListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccountListResponse {
    pub items: Vec<MinecraftAccountResponse>,
}

/// MinecraftAccountResolveRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccountResolveRequest {
    pub minecraft_nick: String,
}

/// MinecraftAccountResolveResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccountResolveResponse {
    pub account_type: String,
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// MinecraftAccountResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccountResponse {
    pub account_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_scope_id: Option<i64>,
    pub identity_scope_type: String,
    pub link_source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_server_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid_type: Option<String>,
    pub verification_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinecraftAccountType {
    #[serde(rename = "official")]
    Official,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for MinecraftAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Official => "official",
            Self::Offline => "offline",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// MinecraftCandidateAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftCandidateAccount {
    pub account_type: MinecraftAccountType,
    pub found: bool,
    pub minecraft_nick: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// MinecraftGroupTarget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftGroupTarget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_minecraft_uuids: Option<Vec<String>>,
    pub luckperms_group: String,
    pub role_id: Snowflake,
    pub role_key: String,
    pub role_name: String,
    pub server_id: Snowflake,
}

/// MinecraftGroupTargets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftGroupTargets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MinecraftGroupTarget>>,
    pub project_id: Snowflake,
    pub server_id: Snowflake,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinecraftIdentityScopeType {
    #[serde(rename = "global_minecraft")]
    GlobalMinecraft,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for MinecraftIdentityScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::GlobalMinecraft => "global_minecraft",
            Self::Project => "project",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinecraftIdentityState {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "unverified_candidate")]
    UnverifiedCandidate,
    #[serde(rename = "approved_pending_proof")]
    ApprovedPendingProof,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for MinecraftIdentityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Verified => "verified",
            Self::UnverifiedCandidate => "unverified_candidate",
            Self::ApprovedPendingProof => "approved_pending_proof",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// MinecraftLinkCodeIssueRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftLinkCodeIssueRequest {
    pub minecraft_nick: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<serde_json::Value>,
}

/// MinecraftLinkCodeIssueResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftLinkCodeIssueResponse {
    pub account: MinecraftAccountResolveResponse,
    pub code: String,
    pub expires_in: i64,
    pub status: String,
}

/// MinecraftLinkCodeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftLinkCodeRequest {
    pub minecraft_nick: String,
}

/// MinecraftLinkCodeResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftLinkCodeResponse {
    pub account: MinecraftCandidateAccount,
    pub challenge_id: Snowflake,
    pub code: String,
    pub expires_in: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinecraftLinkSource {
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "nickname_proof")]
    NicknameProof,
    #[serde(rename = "offline_manual")]
    OfflineManual,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for MinecraftLinkSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Microsoft => "microsoft",
            Self::NicknameProof => "nickname_proof",
            Self::OfflineManual => "offline_manual",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// MinecraftOfficialLinkStart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftOfficialLinkStart {
    pub authorization_url: String,
    pub expires_in: i64,
    pub state: String,
}

/// MinecraftPendingChallenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftPendingChallenge {
    pub account_type: MinecraftAccountType,
    pub challenge_id: Snowflake,
    pub code: String,
    pub expires_at: String,
    pub expires_in: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
}

/// MinecraftResolveRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftResolveRequest {
    pub minecraft_nick: String,
}

/// MinecraftUnlinkResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftUnlinkResult {
    pub account: UnlinkedMinecraftAccount,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinecraftUuidType {
    #[serde(rename = "mojang")]
    Mojang,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "xuid")]
    Xuid,
    #[serde(rename = "floodgate")]
    Floodgate,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for MinecraftUuidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Mojang => "mojang",
            Self::Offline => "offline",
            Self::Xuid => "xuid",
            Self::Floodgate => "floodgate",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// MinecraftVerificationAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVerificationAccount {
    pub account_type: MinecraftAccountType,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    pub verification_status: MinecraftVerificationStatus,
}

/// MinecraftVerificationState
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVerificationState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MinecraftVerificationAccount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_challenge: Option<MinecraftPendingChallenge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinecraftVerificationStatus {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "legacy_unverified")]
    LegacyUnverified,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for MinecraftVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Verified => "verified",
            Self::LegacyUnverified => "legacy_unverified",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// MotdVerification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotdVerification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checked_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub server: VerificationServerSummary,
    pub status: VerificationStatus,
    pub token: String,
}

/// MyComment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyComment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_moderate_comments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_reply_to_comments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

/// MyDashboardStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyDashboardStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<DashboardAccount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_activity: Option<Vec<DashboardDailyActivityPoint>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_breakdown: Option<std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<DashboardServerItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub total_playtime_seconds: i64,
}

/// MyPlayerStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyPlayerStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_activity: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_breakdown: Option<std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub total_playtime_seconds: i64,
}

/// MyServerIssuesPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyServerIssuesPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServerIssuesItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_issues: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_servers: Option<i64>,
}

/// MyServersPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyServersPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServerCard>>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// NotificationPreferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist_applicant: Option<NotificationTopicPreferences>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist_staff: Option<NotificationTopicPreferences>,
}

/// NotificationPreferencesUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferencesUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist_applicant: Option<NotificationTopicPreferences>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist_staff: Option<NotificationTopicPreferences>,
}

/// NotificationTopicPreferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTopicPreferences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_channels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_order: Option<Vec<String>>,
}

/// OAuthAuthorizationTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAuthorizationTokenRequest {
    pub client_id: String,
    pub code: String,
    pub code_verifier: String,
    pub grant_type: String,
    pub redirect_uri: String,
}

/// OAuthAuthorizationTokenResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAuthorizationTokenResponse {
    pub access_token: String,
    pub audience: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_token_expires_in: i64,
    pub roles: Vec<String>,
    pub scope: String,
    pub sid: String,
    pub token_type: String,
    pub user: UserPublic,
}

/// OAuthCallbackRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCallbackRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
}

/// OAuthCaptchaChallengeResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCaptchaChallengeResponse {
    pub challenge_id: String,
    pub expires_in: i64,
    pub status: String,
}

/// OAuthCaptchaConfirmRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCaptchaConfirmRequest {
    pub captcha_token: String,
    pub challenge_id: String,
}

/// OAuthLinkStartResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthLinkStartResponse {
    pub authorization_url: String,
    pub expires_in: i64,
    pub state: String,
}

/// OAuthProviderListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderListResponse {
    pub providers: Vec<OAuthProviderStatus>,
}

/// OAuthProviderStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub connected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub provider: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// OAuthProvidersResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvidersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<OAuthProviderStatus>>,
}

/// OAuthStartResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthStartResponse {
    pub authorization_url: String,
    pub expires_in: i64,
    pub state: String,
}

/// OAuthTotpChallengeResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTotpChallengeResponse {
    pub challenge_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub expires_in: i64,
    pub status: String,
}

/// OAuthTotpConfirmRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTotpConfirmRequest {
    pub challenge_id: String,
    pub totp_code: String,
}

/// OAuthUnlinkResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUnlinkResult {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnlineSource {
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "aggregate")]
    Aggregate,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for OnlineSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Proxy => "proxy",
            Self::Aggregate => "aggregate",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnlineState {
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for OnlineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Live => "live",
            Self::Offline => "offline",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnlineStrategy {
    #[serde(rename = "proxy_preferred")]
    ProxyPreferred,
    #[serde(rename = "aggregate")]
    Aggregate,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for OnlineStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::ProxyPreferred => "proxy_preferred",
            Self::Aggregate => "aggregate",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// Order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub amount_minor: i64,
    pub ccy: i64,
    pub created_at: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub external_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<Payment>>,
    pub product_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refunded_at: Option<String>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    pub updated_at: String,
    pub user_id: String,
}

/// OrderList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderList {
    pub items: Vec<Order>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// PasswordChangeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
}

/// PasswordMutationResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordMutationResult {
    pub status: String,
}

/// PasswordResetConfirmRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetConfirmRequest {
    pub password: String,
    pub token: String,
}

/// PasswordResetRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

/// PasswordResetResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetResult {
    pub status: String,
}

/// PasswordSetRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordSetRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    pub new_password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
}

/// PasswordStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordStatus {
    pub has_password: bool,
}

/// Payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub created_at: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
    pub provider: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_payment_id: Option<String>,
    pub status: String,
    pub updated_at: String,
}

/// PersonalAccessTokenCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalAccessTokenCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<i64>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<i64>,
}

/// PersonalAccessTokenCreateResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalAccessTokenCreateResponse {
    pub expires_in: i64,
    pub item: PersonalAccessTokenItem,
    pub token: String,
    pub token_type: String,
}

/// PersonalAccessTokenItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalAccessTokenItem {
    pub audience: String,
    pub created_at: String,
    pub expires_at: String,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    pub scope: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    pub token_hint: String,
}

/// PersonalAccessTokenListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalAccessTokenListResponse {
    pub items: Vec<PersonalAccessTokenItem>,
}

/// PlatformInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub canonical_base_url: String,
    pub docs_url: String,
    pub openapi_url: String,
    pub service: String,
    pub version: String,
}

/// PlatformPermsRow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformPermsRow {
    pub bits: i64,
    pub max_role_weight: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub user_id: Snowflake,
    pub version: i64,
}

/// PlatformRoleListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRoleListResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PlatformRoleResponse>>,
}

/// PlatformRoleRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRoleRequest {
    pub key: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// PlatformRoleResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRoleResponse {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// PlayerSearchPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSearchPage {
    pub items: Vec<PlayerSearchResult>,
}

/// PlayerSearchResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSearchResult {
    pub last_seen_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_server_id: Option<Snowflake>,
    pub total_playtime_seconds: i64,
    pub username: String,
    pub uuid: String,
}

/// PlayerStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_server_seconds: Option<std::collections::HashMap<String, i64>>,
    pub server_playtime_seconds: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub total_playtime_seconds: i64,
}

/// PluginVerification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVerification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_code: Option<String>,
    pub server: VerificationServerSummary,
    pub status: VerificationStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

/// PluginVerificationStartRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVerificationStartRequest {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<ServerRole>,
}

/// Product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub active: bool,
    pub amount_minor: i64,
    pub ccy: i64,
    pub code: String,
    pub country_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub geo_source: String,
    pub id: String,
    pub interval: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_country_code: Option<String>,
    pub product_type: String,
    pub provider_supported: bool,
    pub used_fallback: bool,
}

/// ProfileActivityHeatmap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileActivityHeatmap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<ProfileActivityHeatmapDay>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_events: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_days: Option<i64>,
}

/// ProfileActivityHeatmapDay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileActivityHeatmapDay {
    pub date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<i64>,
}

/// ProfileGameplaySummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileGameplaySummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_accounts: Option<Vec<ProfileLinkedMinecraftAccount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_played_server: Option<ProfileMostPlayedServer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streak_current: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streak_longest: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_playtime_seconds: Option<i64>,
}

/// ProfileLinkedMinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileLinkedMinecraftAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// ProfileMostPlayedServer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMostPlayedServer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playtime_seconds: Option<i64>,
    pub server_id: String,
}

/// ProfileOwnedProject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOwnedProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_server: Option<ProfileOwnedProjectDisplayServer>,
    pub id: Snowflake,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// ProfileOwnedProjectDisplayServer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOwnedProjectDisplayServer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_or_domain: Option<String>,
}

/// ProfileOwnedServer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOwnedServer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_project_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_project_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_or_domain: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_profile_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// ProfileOwnershipSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOwnershipSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProfileOwnedProject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ProfileOwnedServer>>,
}

/// ProfilePrivacyUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePrivacyUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_activity_stats: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_bio: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_join_date: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_linked_accounts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_ownership: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_streak: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_top_server: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_user_id: Option<bool>,
}

/// ProfileUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<ProfilePrivacyUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_format_preference: Option<String>,
    pub username: String,
}

/// Project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub catalog_mode: String,
    pub created_at: String,
    pub display_server: ServerSummary,
    pub display_server_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_state: Option<FreshnessState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_editions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hearts: Option<i64>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_status_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i64>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_source: Option<OnlineSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_state: Option<OnlineState>,
    pub online_strategy: OnlineStrategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint_state: Option<PublicEntrypointState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_profile_kind: Option<PublicProfileKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_breakdown: Option<ScoreBreakdown>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_state: Option<TrustState>,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime_24h_percentage: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime_7d_percentage: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub votes_monthly: Option<i64>,
}

/// ProjectCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreateRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// ProjectCreateResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreateResponse {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<LifecycleState>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// ProjectDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDetail {
    pub project: Project,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerSummary>>,
}

/// ProjectEngagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEngagement {
    pub comments: i64,
    pub hearts: i64,
    pub project_id: Snowflake,
    pub thumbs: i64,
}

/// ProjectEngagementStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEngagementStatus {
    pub hearted: bool,
    pub project_id: Snowflake,
    pub thumbed: bool,
}

/// ProjectFilterStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFilterStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<FilterCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editions: Option<Vec<FilterCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<FilterCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosting: Option<Vec<FilterCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<FilterCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<FilterCount>>,
    pub total: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<Vec<FilterCount>>,
}

/// ProjectHeartResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectHeartResult {
    pub hearted: bool,
    pub hearts: i64,
    pub project_id: Snowflake,
}

/// ProjectListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub items: Vec<Project>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// ProjectLiveStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLiveStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collected_at: Option<String>,
    pub freshness_state: FreshnessState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LiveStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_source: Option<String>,
    pub online_state: OnlineState,
    pub project_id: Snowflake,
}

/// ProjectResolveResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResolveResponse {
    pub project_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectSort {
    #[serde(rename = "players")]
    Players,
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "score")]
    Score,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for ProjectSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Players => "players",
            Self::Newest => "newest",
            Self::Verified => "verified",
            Self::Score => "score",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// ProjectStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub active_servers: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_online: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chats: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_event_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub joins: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_event_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_online: Option<i64>,
    pub period: String,
    pub server_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_events: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_players: Option<i64>,
}

/// ProjectThumbResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectThumbResult {
    pub project_id: Snowflake,
    pub thumbed: bool,
    pub thumbs: i64,
}

/// ProjectWhitelistApplicationPreview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWhitelistApplicationPreview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_approved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<Snowflake>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_account_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_identity_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<Snowflake>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,
}

/// ProjectWhitelistBindingPreview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWhitelistBindingPreview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_membership_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub enforcement_mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_role_ids: Option<Vec<Snowflake>>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict_chat: Option<bool>,
    pub scope_type: String,
    pub server_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_server_ids: Option<Vec<Snowflake>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// ProjectWhitelistConfigItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWhitelistConfigItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<ProjectWhitelistApplicationPreview>,
    pub apply_server_id: Snowflake,
    pub binding: ProjectWhitelistBindingPreview,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforcement_servers: Option<Vec<WhitelistTargetServerRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<WhitelistFormCard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_target_servers: Option<Vec<WhitelistTargetServerRef>>,
    pub project_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_entry: Option<WhitelistProofEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicEntrypointState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "multiple")]
    Multiple,
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for PublicEntrypointState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Multiple => "multiple",
            Self::Primary => "primary",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// PublicJWK
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicJWK {
    pub alg: String,
    pub e: String,
    pub kid: String,
    pub kty: String,
    pub n: String,
    #[serde(rename = "use")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,
}

/// PublicProfile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<UsersProfilePrivacy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub user_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// PublicProfileCard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfileCard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub user_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicProfileKind {
    #[serde(rename = "canonical_project_only")]
    CanonicalProjectOnly,
    #[serde(rename = "project_child_visible")]
    ProjectChildVisible,
    #[serde(rename = "standalone_server")]
    StandaloneServer,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for PublicProfileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::CanonicalProjectOnly => "canonical_project_only",
            Self::ProjectChildVisible => "project_child_visible",
            Self::StandaloneServer => "standalone_server",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// PublicProfileResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfileResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<UserProfilePrivacy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// PublicProfilesBatchItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfilesBatchItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// PublicProfilesBatchRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfilesBatchRequest {
    pub user_ids: Vec<serde_json::Value>,
}

/// PublicProfilesBatchResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfilesBatchResponse {
    pub profiles: Vec<PublicProfilesBatchItem>,
}

/// PublicTeamMember
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicTeamMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub user_id: Snowflake,
}

/// RecentActivityItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivityItem {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    pub project_id: Snowflake,
    #[serde(rename = "type")]
    pub r#type: ActivityType,
}

/// RecentVotes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentVotes {
    pub project_id: Snowflake,
    pub votes: Vec<VoteItem>,
}

/// RefreshTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<i64>,
}

/// ReportUserRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportUserRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    pub reason: String,
}

/// ReportUserResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportUserResult {
    pub id: Snowflake,
    pub status: String,
}

/// RevokeOtherSessionsResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeOtherSessionsResult {
    pub revoked_count: i64,
}

/// RoleCandidate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCandidate {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

/// RoleCatalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCatalog {
    pub available: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleCandidate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RolloutMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "cutover_enforced")]
    CutoverEnforced,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for RolloutMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Auto => "auto",
            Self::CutoverEnforced => "cutover_enforced",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RolloutState {
    #[serde(rename = "legacy_grace")]
    LegacyGrace,
    #[serde(rename = "mixed_fleet")]
    MixedFleet,
    #[serde(rename = "cutover_ready")]
    CutoverReady,
    #[serde(rename = "cutover_enforced")]
    CutoverEnforced,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for RolloutState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::LegacyGrace => "legacy_grace",
            Self::MixedFleet => "mixed_fleet",
            Self::CutoverReady => "cutover_ready",
            Self::CutoverEnforced => "cutover_enforced",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// ScoreBreakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_online: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hearts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbs: Option<i64>,
    pub total: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_bonus: Option<f64>,
}

/// ServerBot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_guilds: Option<Vec<ServerBotLinkedGuild>>,
}

/// ServerBotLinkedGuild
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBotLinkedGuild {
    pub guild_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_at: Option<String>,
}

/// ServerBotUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBotUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_link: Option<String>,
}

/// ServerCard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCard {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_edition: Option<GameEdition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub id: Snowflake,
    pub ip_or_domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_max_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_online_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintenance_enabled: Option<bool>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    pub role: ServerRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_in_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: String,
}

/// ServerCardTranslations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCardTranslations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<TextTranslation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintenance_message: Option<TextTranslation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motd: Option<TextTranslation>,
}

/// ServerChangeAddressRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerChangeAddressRequest {
    pub address: String,
}

/// ServerChangeSlugRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerChangeSlugRequest {
    pub slug: String,
}

/// ServerDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bedrock_port: Option<i64>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_edition: Option<GameEdition>,
    pub id: Snowflake,
    pub ip_or_domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping_ip_or_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_online_strategy: Option<OnlineStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    pub role: ServerRole,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_source: Option<VerificationSource>,
}

/// ServerEventPoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEventPoint {
    pub collected_at: String,
    pub event_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_delta: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub player_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// ServerEvents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEvents {
    pub items: Vec<ServerEventPoint>,
    pub page: i64,
    pub per_page: i64,
    pub period: String,
    pub total: i64,
}

/// ServerHostRiskEvidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHostRiskEvidence {
    pub filename: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// ServerIssuesItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerIssuesItem {
    pub ip_or_domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<ServerServiceIssue>>,
    pub server_id: Snowflake,
    pub server_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_slug: Option<String>,
}

/// ServerLaunchManifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerLaunchManifest {
    pub content: ServerLaunchManifestContent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jvm: Option<LaunchManifestJvm>,
    pub minecraft: LaunchManifestMinecraft,
    pub schema_version: i64,
    pub server: LaunchManifestServerEndpoint,
    pub server_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<LaunchManifestWhitelist>,
}

/// ServerMaintenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMaintenance {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// ServerMaintenanceUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMaintenanceUpdateRequest {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// ServerMediaSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMediaSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub icon_version: i64,
    pub server_id: Snowflake,
}

/// ServerMotdSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMotdSummary {
    pub server: VerificationServerSummary,
}

/// ServerMotdUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMotdUpdateRequest {
    pub motd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

/// ServerOwnership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerOwnership {
    pub can_edit: bool,
    pub is_owner: bool,
    pub server_id: Snowflake,
}

/// ServerRenameRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRenameRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerRole {
    #[serde(rename = "standalone")]
    Standalone,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "sub")]
    Sub,
    #[serde(rename = "hub")]
    Hub,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for ServerRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Standalone => "standalone",
            Self::Proxy => "proxy",
            Self::Sub => "sub",
            Self::Hub => "hub",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// ServerRoot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRoot {
    pub project_id: Snowflake,
    pub root_server_id: Snowflake,
    pub server_id: Snowflake,
}

/// ServerServiceIssue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerServiceIssue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub service: String,
    pub severity: String,
    pub title: String,
}

/// ServerSetBedrockPortRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetBedrockPortRequest {
    pub port: i64,
}

/// ServerSetDescriptionRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetDescriptionRequest {
    pub description: String,
}

/// ServerSetParentRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetParentRequest {
    pub parent_id: Snowflake,
}

/// ServerSetPingPortRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetPingPortRequest {
    pub port: i64,
}

/// ServerSetRegionsRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetRegionsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// ServerSetRoleRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetRoleRequest {
    pub role: ServerRole,
}

/// ServerSetShowDescriptionRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetShowDescriptionRequest {
    pub value: bool,
}

/// ServerSetShowInPublicRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetShowInPublicRequest {
    pub value: bool,
}

/// ServerSetTeamEnabledRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetTeamEnabledRequest {
    pub value: bool,
}

/// ServerSetVersionOverrideRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetVersionOverrideRequest {
    pub version_override: String,
}

/// ServerStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    pub achievements: i64,
    pub activity: i64,
    pub chats: i64,
    pub commands: i64,
    pub deaths: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_event_at: Option<String>,
    pub joins: i64,
    pub kicks: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_event_at: Option<String>,
    pub leaves: i64,
    pub period: String,
    pub total_events: i64,
    pub unique_players: i64,
    pub world: i64,
}

/// ServerSubservers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSubservers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subserver_ids: Option<Vec<Snowflake>>,
}

/// ServerSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSummary {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_edition: Option<GameEdition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_build: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub id: Snowflake,
    pub ip_or_domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    pub role: ServerRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_description: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translations: Option<ServerCardTranslations>,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_source: Option<VerificationSource>,
}

/// ServerTeamManage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTeamManage {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TeamMemberItem>>,
    pub project_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_servers: Option<Vec<TeamScopeServer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<TeamRoleItem>>,
    pub root_server_id: Snowflake,
    pub server_id: Snowflake,
}

/// ServerTeamPublic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTeamPublic {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited_from_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<PublicTeamMember>>,
    pub project_id: Snowflake,
    pub server_id: Snowflake,
}

/// ServerTelemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTelemetry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collected_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<ServerTelemetryMetric>>,
    pub period: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// ServerTelemetryMetric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTelemetryMetric {
    pub avg: f64,
    pub cadence_hint_seconds: i64,
    pub key: String,
    pub kind: TelemetryMetricKind,
    pub last: f64,
    pub max: f64,
    pub min: f64,
    pub samples: i64,
}

/// ServerTranslation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTranslation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    pub field: String,
    pub locale: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TranslationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translated_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// ServerTranslationUpsertRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTranslationUpsertRequest {
    pub translated_text: String,
}

/// ServerTranslations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTranslations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServerTranslation>>,
}

/// ServerWhitelistPublicConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerWhitelistPublicConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<WhitelistApplication>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<WhitelistBindingDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforcement_servers: Option<Vec<WhitelistTargetServerRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<WhitelistFormDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_target_servers: Option<Vec<WhitelistTargetServerRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_entry: Option<WhitelistProofEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceHealth {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for ServiceHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// ServiceHealthEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthEntry {
    pub checked_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i64>,
    pub name: String,
    pub status: ServiceHealth,
}

/// ServicesHealthResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesHealthResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ServiceHealthEntry>>,
}

/// SessionInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    pub issued_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// SessionList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<SessionInfo>>,
}

/// SessionListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<SessionInfo>,
}

/// SessionRevokeOthersResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRevokeOthersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked_count: Option<i64>,
    pub status: String,
}

/// SessionRevokeResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRevokeResponse {
    pub status: String,
}

/// SessionRevokeResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRevokeResult {
    pub session_id: String,
    pub status: String,
}

/// ShareLink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLink {
    pub build_id: String,
    pub share_token: String,
}

/// SocialLinkVerification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLinkVerification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub platform: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub verified: bool,
}

/// SocialLinkVerifyRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLinkVerifyRequest {
    pub platform: String,
    pub url: String,
}

/// SocialLinks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instagram_url: Option<String>,
    pub server_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tiktok_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitch_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_data: Option<std::collections::HashMap<String, SocialLinkVerification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}

/// SocialLinksUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLinksUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instagram_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tiktok_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitch_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}

/// StatusOverrideItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOverrideItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i64>,
    pub ends_at: String,
    pub id: String,
    pub mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub server_id: Snowflake,
    pub starts_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<i64>,
}

/// StatusResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
}

/// Subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_charge_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_charge_at: Option<String>,
    pub product_id: String,
    pub provider: String,
    pub pull_mode: String,
    pub status: String,
    pub updated_at: String,
    pub user_id: String,
}

/// SubscriptionList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionList {
    pub items: Vec<Subscription>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// TeamMemberCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_state: Option<MemberState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Vec<TeamMemberRoleAssignmentInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
    pub user_id: String,
}

/// TeamMemberDeleteResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberDeleteResponse {
    pub ok: bool,
}

/// TeamMemberItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberItem {
    pub created_at: String,
    pub id: Snowflake,
    pub is_owner: bool,
    pub is_public: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_state: Option<MemberState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Vec<TeamMemberRoleAssignment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    pub sort_order: i64,
    pub updated_at: String,
    pub user_id: Snowflake,
}

/// TeamMemberRoleAssignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberRoleAssignment {
    pub role_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<TeamScopeType>,
}

/// TeamMemberRoleAssignmentInput
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberRoleAssignmentInput {
    pub role_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<TeamScopeType>,
}

/// TeamMemberUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_state: Option<MemberState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Vec<TeamMemberRoleAssignmentInput>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

/// TeamRoleCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRoleCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luckperms_group: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

/// TeamRoleDeleteResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRoleDeleteResponse {
    pub ok: bool,
}

/// TeamRoleItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRoleItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luckperms_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions_bits: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
    pub updated_at: String,
}

/// TeamRoleUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRoleUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luckperms_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

/// TeamScopeServer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamScopeServer {
    pub created_at: String,
    pub id: Snowflake,
    pub name: String,
    pub server_role: ServerRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamScopeType {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "network_all")]
    NetworkAll,
    #[serde(rename = "network_selected")]
    NetworkSelected,
    #[serde(rename = "whitelist_policy")]
    WhitelistPolicy,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TeamScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Project => "project",
            Self::Server => "server",
            Self::NetworkAll => "network_all",
            Self::NetworkSelected => "network_selected",
            Self::WhitelistPolicy => "whitelist_policy",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryMetricKind {
    #[serde(rename = "hot")]
    Hot,
    #[serde(rename = "warm")]
    Warm,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TelemetryMetricKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Hot => "hot",
            Self::Warm => "warm",
            Self::Cold => "cold",
            Self::Static => "static",
            Self::Event => "event",
            Self::Other => "other",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// TextTranslation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTranslation {
    pub engine: String,
    pub original_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_locale: Option<String>,
    pub status: TranslationStatus,
    pub target_locale: String,
    pub translated_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketAuthorType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "staff")]
    Staff,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TicketAuthorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::User => "user",
            Self::Staff => "staff",
            Self::System => "system",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// TicketCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCreateRequest {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<TicketPriority>,
    pub server_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<TicketSource>,
    pub subject: String,
}

/// TicketDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketDetail {
    pub creator_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<TicketMessage>>,
    pub summary: TicketSummary,
}

/// TicketList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketList {
    pub items: Vec<TicketSummary>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// TicketMessage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessage {
    pub author_id: Snowflake,
    pub author_type: TicketAuthorType,
    pub content: String,
    pub created_at: String,
    pub id: Snowflake,
    pub source: TicketSource,
}

/// TicketMessageCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessageCreateRequest {
    pub content: String,
}

/// TicketMessageList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessageList {
    pub messages: Vec<TicketMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "urgent")]
    Urgent,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TicketPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Urgent => "urgent",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketSource {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "minecraft")]
    Minecraft,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TicketSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Web => "web",
            Self::Discord => "discord",
            Self::Minecraft => "minecraft",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Open => "open",
            Self::Pending => "pending",
            Self::Resolved => "resolved",
            Self::Closed => "closed",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// TicketStatusUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketStatusUpdateRequest {
    pub status: TicketStatus,
}

/// TicketSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSummary {
    pub created_at: String,
    pub id: Snowflake,
    pub priority: TicketPriority,
    pub server_id: Snowflake,
    pub status: TicketStatus,
    pub subject: String,
    pub updated_at: String,
}

/// TokenExchangeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenExchangeRequest {
    pub audience: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_minutes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<i64>,
}

/// TotpBeginResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpBeginResult {
    pub otpauth_url: String,
    pub secret: String,
}

/// TotpCodeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpCodeRequest {
    pub code: String,
}

/// TotpStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranslationStatus {
    #[serde(rename = "translated")]
    Translated,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TranslationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Translated => "translated",
            Self::Skipped => "skipped",
            Self::Pending => "pending",
            Self::Unavailable => "unavailable",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustState {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "gateway")]
    Gateway,
    #[serde(rename = "unverified")]
    Unverified,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for TrustState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Verified => "verified",
            Self::Gateway => "gateway",
            Self::Unverified => "unverified",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// UnlinkedMinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedMinecraftAccount {
    pub account_type: String,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_scope_id: Option<Snowflake>,
    pub identity_scope_type: String,
    pub link_source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid_type: Option<String>,
    pub verification_status: String,
}

/// UpdateManifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub artifact_id: String,
    pub channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<std::collections::HashMap<String, ManifestComponent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deltas:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, ManifestDelta>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_urls: Option<Vec<String>>,
    pub file_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modrinth_url: Option<String>,
    pub platform: String,
    pub product: String,
    pub sha256: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_version: Option<String>,
    pub version: String,
}

/// UpdateManifestUpsert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifestUpsert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<std::collections::HashMap<String, ManifestComponent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deltas:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, ManifestDelta>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_urls: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modrinth_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollout_allow_servers: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollout_deny_servers: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollout_percent: Option<i64>,
    pub sha256: String,
    pub version: String,
}

/// UpdateReportAck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReportAck {
    pub ok: bool,
}

/// UpdateReportInput
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReportInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
}

/// UserDetailed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetailed {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub status: String,
    pub username: String,
}

/// UserEngagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEngagement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_project: Option<UserEngagementTopProject>,
    pub total_comments: i64,
    pub total_favorites: i64,
    pub total_votes: i64,
}

/// UserEngagementTopProject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEngagementTopProject {
    pub project_id: Snowflake,
    pub votes: i64,
}

/// UserLogin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captcha_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    pub password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
    pub username: String,
}

/// UserProfile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<MeProfilePrivacy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_format_preference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub username: String,
}

/// UserProfilePrivacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePrivacy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_activity_stats: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_bio: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_join_date: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_linked_accounts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_ownership: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_streak: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_top_server: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_user_id: Option<bool>,
}

/// UserPublic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<UserProfilePrivacy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_format_preference: Option<UserTimeFormatPreference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub username: String,
}

/// UserRecentActivity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecentActivity {
    pub items: Vec<RecentActivityItem>,
}

/// UserRegister
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegister {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captcha_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub password: String,
    pub username: String,
}

/// UserRolesResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRolesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    pub user_id: String,
}

/// UserSearchResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchResponse {
    pub items: Vec<UserDetailed>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "banned")]
    Banned,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Disabled => "disabled",
            Self::Banned => "banned",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserTimeFormatPreference {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "12h")]
    _12h,
    #[serde(rename = "24h")]
    _24h,
}

impl std::fmt::Display for UserTimeFormatPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Auto => "auto",
            Self::_12h => "12h",
            Self::_24h => "24h",
        };
        f.write_str(wire)
    }
}

/// VerificationCheckRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCheckRequest {
    pub token: String,
}

/// VerificationServerSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationServerSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_edition: Option<GameEdition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub id: Snowflake,
    pub ip_or_domain: String,
    pub is_verified: bool,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    pub proxy_type: String,
    pub role: ServerRole,
    pub server_role: ServerRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_source: Option<VerificationSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationSource {
    #[serde(rename = "plugin")]
    Plugin,
    #[serde(rename = "motd")]
    Motd,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for VerificationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Plugin => "plugin",
            Self::Motd => "motd",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// VerificationStartRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStartRequest {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    #[serde(rename = "issued")]
    Issued,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Issued => "issued",
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// VoteItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub user_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub voted_at: String,
}

/// VotingLinks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allmc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disflip_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leavepulse_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monicore_url: Option<String>,
    pub server_id: Snowflake,
}

/// VotingLinksUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingLinksUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allmc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disflip_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leavepulse_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monicore_url: Option<String>,
}

/// WebsocketTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketTokenRequest {
    pub audience: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

/// WhitelistApplicantNotificationSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplicantNotificationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_dm: Option<WhitelistNotificationChannelSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<WhitelistNotificationChannelSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_direct: Option<WhitelistNotificationChannelSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram: Option<WhitelistNotificationChannelSettings>,
}

/// WhitelistApplication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_approved: Option<bool>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<Snowflake>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_account_type: Option<MinecraftAccountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_identity_state: Option<MinecraftIdentityState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    pub server_id: Snowflake,
    pub status: WhitelistApplicationStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_alias: Option<String>,
    pub updated_at: String,
    pub user_id: Snowflake,
}

/// WhitelistApplicationList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplicationList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WhitelistApplication>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhitelistApplicationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "approved_pending_proof")]
    ApprovedPendingProof,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "revision")]
    Revision,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for WhitelistApplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::ApprovedPendingProof => "approved_pending_proof",
            Self::Denied => "denied",
            Self::Revision => "revision",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// WhitelistApplyRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answers: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<Snowflake>,
    pub minecraft_account_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
}

/// WhitelistBindingDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistBindingDetail {
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_membership_mode: Option<DiscordMembershipMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub enforcement_mode: EnforcementMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_role_ids: Option<Vec<Snowflake>>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<WhitelistBindingMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<WhitelistNotificationSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict_chat: Option<bool>,
    pub scope_type: TeamScopeType,
    pub server_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_server_ids: Option<Vec<Snowflake>>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhitelistBindingMode {
    #[serde(rename = "form")]
    Form,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for WhitelistBindingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Form => "form",
            Self::Direct => "direct",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// WhitelistBindingTestResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistBindingTestResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub sent: bool,
}

/// WhitelistBindingWriteRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistBindingWriteRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_membership_mode: Option<DiscordMembershipMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub enforcement_mode: EnforcementMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_role_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<WhitelistBindingMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<WhitelistNotificationSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict_chat: Option<bool>,
    pub scope_type: TeamScopeType,
    pub server_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_server_ids: Option<Vec<String>>,
}

/// WhitelistConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_server_id: Option<Snowflake>,
    pub enabled: bool,
    pub enforcement_mode: EnforcementMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<WhitelistEntry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_fields: Option<Vec<WhitelistFormField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_name: Option<String>,
    pub restrict_chat: bool,
    pub scope_type: TeamScopeType,
    pub server_id: Snowflake,
}

/// WhitelistDecisionRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDecisionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// WhitelistDirectAddRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDirectAddRequest {
    pub minecraft_nick: String,
}

/// WhitelistDirectEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDirectEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by_user_id: Option<Snowflake>,
    pub created_at: String,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_account_type: Option<MinecraftAccountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    pub server_id: Snowflake,
    pub updated_at: String,
}

/// WhitelistDirectEntryPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDirectEntryPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WhitelistDirectEntry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    pub total: i64,
}

/// WhitelistDirectRemoval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDirectRemoval {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    pub removed: bool,
}

/// WhitelistEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_account_type: Option<MinecraftAccountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,
}

/// WhitelistFieldConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFieldConfig {
    pub field_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    pub key: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhitelistFieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "textarea")]
    Textarea,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for WhitelistFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wire = match self {
            Self::Text => "text",
            Self::Textarea => "textarea",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Select => "select",
            Self::Unknown => "unknown",
        };
        f.write_str(wire)
    }
}

/// WhitelistFormCard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormCard {
    pub auto_approve_enabled: bool,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: Snowflake,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    pub require_discord: bool,
    pub require_minecraft_nick: bool,
    pub updated_at: String,
}

/// WhitelistFormCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_approve_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_approve_rules: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<WhitelistFieldConfig>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_mapping: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_discord: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_minecraft_nick: Option<bool>,
}

/// WhitelistFormDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_approve_rules: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<WhitelistFieldConfig>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_mapping: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub summary: WhitelistFormCard,
}

/// WhitelistFormDetailPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormDetailPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WhitelistFormDetail>>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// WhitelistFormField
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormField {
    pub field_type: WhitelistFieldType,
    pub key: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    pub required: bool,
}

/// WhitelistFormImportMappingRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormImportMappingRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_mapping: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// WhitelistFormPatch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_approve_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_approve_rules: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<WhitelistFieldConfig>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_mapping: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_discord: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_minecraft_nick: Option<bool>,
}

/// WhitelistFormUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormUpdateRequest {
    pub patch: WhitelistFormPatch,
}

/// WhitelistImportJob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistImportJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications_added: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batches_expected: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batches_received: Option<i64>,
    pub binding_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    pub conflict_policy: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<i64>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries_added: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries_updated: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub id: Snowflake,
    pub include_history: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    pub server_id: Snowflake,
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    pub status: String,
    pub token: String,
    pub updated_at: String,
}

/// WhitelistImportJobPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistImportJobPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WhitelistImportJob>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// WhitelistImportOptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistImportOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_priority: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_overrides:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_priority: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// WhitelistImportRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistImportRequest {
    pub binding_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_account_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_history: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<WhitelistImportOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// WhitelistMinecraftPullRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistMinecraftPullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    pub binding_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_account_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_history: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// WhitelistNotificationChannelSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistNotificationChannelSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_scope: Option<String>,
}

/// WhitelistNotificationSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistNotificationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicant: Option<WhitelistApplicantNotificationSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staff: Option<WhitelistStaffNotificationSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// WhitelistProofEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistProofEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_hint_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_command_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// WhitelistStaffNotificationSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistStaffNotificationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_channel: Option<WhitelistNotificationChannelSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_dm: Option<WhitelistNotificationChannelSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<WhitelistNotificationChannelSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minecraft_broadcast: Option<WhitelistNotificationChannelSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram: Option<WhitelistNotificationChannelSettings>,
}

/// WhitelistStatusRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
}

/// WhitelistTargetServerRef
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistTargetServerRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_server_id: Option<Snowflake>,
    pub server_id: Snowflake,
}

/// Workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_draft: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_manage: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_verify: Option<bool>,
    pub catalog_mode: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_server: Option<ServerSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_state: Option<FreshnessState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_editions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hearts: Option<i64>,
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_status_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<LifecycleState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i64>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_players: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_source: Option<OnlineSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_state: Option<OnlineState>,
    pub online_strategy: OnlineStrategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_entrypoint_state: Option<PublicEntrypointState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_server_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_state: Option<TrustState>,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_rollout_mode: Option<RolloutMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_plugin_rollout_state: Option<RolloutState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_server_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub votes_monthly: Option<i64>,
}

/// WorkspaceChangeSlugRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceChangeSlugRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// WorkspaceDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerSummary>>,
    pub workspace: Workspace,
}

/// WorkspaceListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceListResponse {
    pub items: Vec<Workspace>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// WorkspaceRenameRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRenameRequest {
    pub name: String,
}

/// WorkspaceResolveResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceResolveResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub workspace_id: Snowflake,
}

/// WorkspaceSetOnlineStrategyRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSetOnlineStrategyRequest {
    pub online_strategy: OnlineStrategy,
}

/// WorkspaceSetRolloutModeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSetRolloutModeRequest {
    pub verified_plugin_rollout_mode: RolloutMode,
}

/// WsTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsTokenRequest {
    pub audience: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

/// WsTokenResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsTokenResponse {
    pub audience: String,
    pub expires_in: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    pub session_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    pub token: String,
    pub token_type: String,
}

/// _AvatarUploadForm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUploadForm {
    pub file: String,
}

/// _IconUploadForm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconUploadForm {
    pub file: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<serde_json::Value>,
}

/// ProfilePrivacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeProfilePrivacy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_activity_stats: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_bio: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_join_date: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_linked_accounts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_ownership: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_streak: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_top_server: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_user_id: Option<bool>,
}

/// ProfilePrivacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersProfilePrivacy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_activity_stats: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_bio: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_join_date: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_linked_accounts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_ownership: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_streak: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_top_server: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_user_id: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerLaunchManifestContent {
    #[serde(rename = "inline")]
    Inline(LaunchManifestInlineContent),
    #[serde(rename = "mrpack")]
    Mrpack(LaunchManifestMrpackContent),
}

/// Query parameters for `binding.entries.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BindingEntriesListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `binding.test`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BindingTestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
}

/// Query parameters for `comment.reply`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentReplyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locale: Option<String>,
}

/// Query parameters for `me.minecraft.state`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeMinecraftStateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
}

/// Query parameters for `me.stats`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
}

/// Query parameters for `me.whitelist.applications`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeWhitelistApplicationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `me.servers.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeServersListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `me.servers.issues`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeServersIssuesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_ok: Option<bool>,
}

/// Query parameters for `project.comments.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectCommentsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locale: Option<String>,
}

/// Query parameters for `project.comments.mine`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectCommentsMineParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locale: Option<String>,
}

/// Query parameters for `project.votes.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectVotesListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `project.history.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectHistoryListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Query parameters for `project.stats`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Query parameters for `project.team_sync.targets`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectTeamSyncTargetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i64>,
}

/// Query parameters for `project.comments.create`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectCommentsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locale: Option<String>,
}

/// Query parameters for `project.policies.test`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectPoliciesTestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
}

/// Query parameters for `server.tickets.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerTicketsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Query parameters for `server.player_stats`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerPlayerStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_server_id: Option<i64>,
}

/// Query parameters for `server.events.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerEventsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player: Option<String>,
}

/// Query parameters for `server.history.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerHistoryListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Query parameters for `server.stats`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Query parameters for `server.team_sync.targets`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerTeamSyncTargetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i64>,
}

/// Query parameters for `server.telemetry`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerTelemetryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Query parameters for `server.whitelist.public`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerWhitelistPublicParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// Query parameters for `server.whitelist.applications`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerWhitelistApplicationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `server.whitelist.direct`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerWhitelistDirectParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `server.whitelist.imports`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerWhitelistImportsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `user.heatmap`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserHeatmapParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}

/// Query parameters for `admin.discovery.candidates`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminDiscoveryCandidatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<GameEdition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_sources: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_online: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discord_members: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<DiscoverySort>,
}

/// Query parameters for `admin.discovery.approve`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminDiscoveryApproveParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<i64>,
}

/// Query parameters for `admin.discovery.ignore`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminDiscoveryIgnoreParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Query parameters for `admin.discovery.observations`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminDiscoveryObservationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `admin.players.search`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminPlayersSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `admin.projects.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminProjectsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `admin.servers.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminServersListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

/// Query parameters for `admin.overrides.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminOverridesListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Query parameters for `admin.users.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminUsersListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

/// Query parameters for `admin.users.search`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminUsersSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `billing.orders.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BillingOrdersListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `billing.subscriptions.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BillingSubscriptionsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `tickets.mine`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketsMineParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `discord.link.session`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscordLinkSessionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Query parameters for `updates.manifest`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatesManifestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<i64>,
}

/// Query parameters for `projects.stats`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectsStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<GameEdition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

/// Query parameters for `stats.filter`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatsFilterParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<GameEdition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Query parameters for `users.search`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsersSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `whitelist.forms.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhitelistFormsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `auth.oauth2.authorize`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthOauth2AuthorizeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge_method: Option<String>,
}

/// Query parameters for `projects.projects_list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectsProjectsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `projects.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<GameEdition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ProjectSort>,
}

/// Query parameters for `users.activity_list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsersActivityListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
