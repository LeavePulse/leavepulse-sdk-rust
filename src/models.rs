// Generated from the LeavePulse contract. Do not edit.
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// AccountDeletionResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeletionResult {
    pub scheduled_at: String,
    pub status: String,
}

/// AccountExport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExport {
    #[serde(default)]
    pub identities: Option<Vec<AccountExportIdentity>>,
    #[serde(default)]
    pub sessions: Option<Vec<AccountExportSession>>,
    pub user: UserProfile,
}

/// AccountExportIdentity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExportIdentity {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub email_verified: Option<bool>,
    #[serde(default)]
    pub last_login_at: Option<String>,
    pub provider: String,
}

/// AccountExportSession
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExportSession {
    #[serde(default)]
    pub expires_at: Option<String>,
    pub id: String,
    #[serde(default)]
    pub issued_at: Option<String>,
    #[serde(default)]
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

/// AdminChangeProjectSlugRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminChangeProjectSlugRequest {
    #[serde(default)]
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
    #[serde(default)]
    pub discord_subject: Option<String>,
    pub status: String,
}

/// AdminForceCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminForceCreateRequest {
    pub address: String,
    #[serde(default)]
    pub is_verified: Option<bool>,
    pub name: String,
    pub owner_id: i64,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
    pub server_role: ServerRole,
}

/// AdminMinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccount {
    pub account_type: MinecraftAccountType,
    #[serde(default)]
    pub created_at: Option<String>,
    pub id: String,
    #[serde(default)]
    pub identity_scope_id: Option<String>,
    pub identity_scope_type: MinecraftIdentityScopeType,
    pub link_source: MinecraftLinkSource,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub proof_server_id: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub uuid_type: Option<MinecraftUuidType>,
    pub verification_status: MinecraftVerificationStatus,
    #[serde(default)]
    pub verified_at: Option<String>,
}

/// AdminMinecraftAccountCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountCreateRequest {
    pub minecraft_nick: String,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
}

/// AdminMinecraftAccountDetailed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountDetailed {
    pub account_type: String,
    #[serde(default)]
    pub created_at: Option<String>,
    pub id: i64,
    #[serde(default)]
    pub identity_scope_id: Option<i64>,
    pub identity_scope_type: String,
    pub link_source: String,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub proof_server_id: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub uuid_type: Option<String>,
    pub verification_status: String,
    #[serde(default)]
    pub verified_at: Option<String>,
}

/// AdminMinecraftAccountUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMinecraftAccountUpdateRequest {
    pub minecraft_nick: String,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
}

/// AdminProject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminProject {
    #[serde(default)]
    pub creator_user_id: Option<i64>,
    #[serde(default)]
    pub display_server_id: Option<i64>,
    pub id: i64,
    #[serde(default)]
    pub lifecycle_state: Option<LifecycleState>,
    pub name: String,
    #[serde(default)]
    pub online_server_id: Option<i64>,
    pub online_strategy: OnlineStrategy,
    #[serde(default)]
    pub owner_id: Option<i64>,
    #[serde(default)]
    pub public_server_count: Option<i64>,
    #[serde(default)]
    pub root_server_id: Option<i64>,
    #[serde(default)]
    pub server_count: Option<i64>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub verified_plugin_rollout_mode: Option<RolloutMode>,
    #[serde(default)]
    pub verified_plugin_rollout_state: Option<RolloutState>,
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
    pub items: Option<Vec<AdminRole>>,
}

/// AdminRoleRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleRequest {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub permissions: Option<Vec<String>>,
}

/// AdminServerListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminServerListResponse {
    #[serde(default)]
    pub items: Option<Vec<AdminServerSummary>>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// AdminServerSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminServerSummary {
    #[serde(default)]
    pub creator_user_id: Option<String>,
    pub id: String,
    pub ip_or_domain: String,
    #[serde(default)]
    pub is_hidden: Option<bool>,
    #[serde(default)]
    pub is_verified: Option<bool>,
    pub name: String,
    #[serde(default)]
    pub owner_id: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    pub server_role: ServerRole,
    #[serde(default)]
    pub verified_plugin_compatibility: Option<String>,
    #[serde(default)]
    pub verified_plugin_last_seen_at: Option<String>,
    #[serde(default)]
    pub verified_plugin_platform: Option<String>,
    #[serde(default)]
    pub verified_plugin_protocol_generation: Option<i64>,
    #[serde(default)]
    pub verified_plugin_rollout_state: Option<RolloutState>,
    #[serde(default)]
    pub verified_plugin_version: Option<String>,
}

/// AdminServerUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminServerUpdateRequest {
    #[serde(default)]
    pub is_hidden: Option<serde_json::Value>,
    #[serde(default)]
    pub is_verified: Option<serde_json::Value>,
    #[serde(default)]
    pub owner_id: Option<i64>,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
    #[serde(default)]
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
    #[serde(default)]
    pub owner_id: Option<i64>,
}

/// AdminUserDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserDetail {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub discord_subject: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub id: String,
    #[serde(default)]
    pub is_shadow: Option<bool>,
    #[serde(default)]
    pub minecraft_accounts: Option<Vec<AdminMinecraftAccount>>,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    pub status: UserStatus,
    #[serde(default)]
    pub updated_at: Option<String>,
    pub username: String,
}

/// AdminUserDiscordUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserDiscordUpdateRequest {
    #[serde(default)]
    pub discord_subject: Option<String>,
}

/// AdminUserListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserListResponse {
    #[serde(default)]
    pub items: Option<Vec<AdminUserSummary>>,
    pub total: i64,
}

/// AdminUserSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserSummary {
    #[serde(default)]
    pub avatar_url: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub email: Option<String>,
    pub id: String,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    pub status: UserStatus,
    pub username: String,
}

/// AdminUserUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserUpdateRequest {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub status: UserStatus,
    pub username: String,
}

/// AuthStatusRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatusRequest {
    #[serde(default)]
    pub ip_address: Option<String>,
    pub username: String,
}

/// AuthStatusResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatusResponse {
    #[serde(default)]
    pub created_at: Option<String>,
    pub exists: bool,
    #[serde(default)]
    pub last_login_at: Option<String>,
    #[serde(default)]
    pub needs_password_change: Option<bool>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub trusted_login: Option<bool>,
}

/// AvatarUrlRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUrlRequest {
    #[serde(default)]
    pub avatar_url: Option<String>,
}

/// BatchPublicProfilesRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPublicProfilesRequest {
    pub user_ids: Vec<i64>,
}

/// BatchPublicProfilesResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPublicProfilesResponse {
    #[serde(default)]
    pub items: Option<Vec<PublicProfileCard>>,
}

/// BatchResolveRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResolveRequest {
    pub user_ids: Vec<i64>,
}

/// BatchResolveResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResolveResponse {
    #[serde(default)]
    pub items: Option<Vec<PlatformPermsRow>>,
}

/// BridgeSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSettings {
    #[serde(default)]
    pub discord_to_mc_channel_id: Option<String>,
    pub discord_to_mc_enabled: bool,
    pub discord_to_mc_mode: String,
    pub discord_to_mc_plain_format: String,
    pub discord_to_mc_spoof_content_format: String,
    #[serde(default)]
    pub discord_to_mc_target_channel: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub mc_to_discord_channel_id: Option<String>,
    #[serde(default)]
    pub mc_to_discord_chat_routing: Option<serde_json::Value>,
    pub mc_to_discord_enabled: bool,
    #[serde(default)]
    pub mc_to_discord_notifications: Option<serde_json::Value>,
    pub mc_to_discord_use_webhook: bool,
    #[serde(default)]
    pub minecraft_chat_filter: Option<serde_json::Value>,
    pub nickname_sync_enabled: bool,
    #[serde(default)]
    pub status_notifications: Option<serde_json::Value>,
}

/// BridgeSettingsUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSettingsUpdateRequest {
    #[serde(default)]
    pub discord_to_mc_channel_id: Option<String>,
    #[serde(default)]
    pub discord_to_mc_enabled: Option<bool>,
    #[serde(default)]
    pub discord_to_mc_mode: Option<String>,
    #[serde(default)]
    pub discord_to_mc_plain_format: Option<String>,
    #[serde(default)]
    pub discord_to_mc_spoof_content_format: Option<String>,
    #[serde(default)]
    pub discord_to_mc_target_channel: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub mc_to_discord_channel_id: Option<String>,
    #[serde(default)]
    pub mc_to_discord_chat_routing: Option<serde_json::Value>,
    #[serde(default)]
    pub mc_to_discord_enabled: Option<bool>,
    #[serde(default)]
    pub mc_to_discord_notifications: Option<serde_json::Value>,
    #[serde(default)]
    pub mc_to_discord_use_webhook: Option<bool>,
    #[serde(default)]
    pub minecraft_chat_filter: Option<serde_json::Value>,
    #[serde(default)]
    pub nickname_sync_enabled: Option<bool>,
    #[serde(default)]
    pub status_notifications: Option<serde_json::Value>,
}

/// Build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub access: String,
    #[serde(default)]
    pub config_blob_sha256: Option<String>,
    pub created_at: String,
    pub has_config_blob: bool,
    #[serde(default)]
    pub icon: Option<String>,
    pub id: String,
    pub is_public: bool,
    pub manifest: BuildManifest,
    pub name: String,
    pub owner_id: String,
    #[serde(default)]
    pub share_token: Option<String>,
    pub summary: String,
    pub updated_at: String,
    pub updated_revision: i64,
}

/// BuildCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildCreateRequest {
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub manifest: Option<BuildManifest>,
    pub name: String,
    #[serde(default)]
    pub summary: Option<String>,
}

/// BuildList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildList {
    pub items: Vec<BuildSummary>,
    pub total: i64,
}

/// BuildManifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifest {
    #[serde(default)]
    pub game_args: Option<Vec<String>>,
    #[serde(default)]
    pub game_version: Option<String>,
    #[serde(default)]
    pub jvm_args: Option<Vec<String>>,
    #[serde(default)]
    pub loader_kind: Option<String>,
    #[serde(default)]
    pub loader_version: Option<String>,
    #[serde(default)]
    pub memory_max_mb: Option<i64>,
    #[serde(default)]
    pub memory_min_mb: Option<i64>,
    #[serde(default)]
    pub mods: Option<Vec<BuildManifestMod>>,
    #[serde(default)]
    pub schema_version: Option<i64>,
}

/// BuildManifestMod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifestMod {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub file_path: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

/// BuildSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSummary {
    pub access: String,
    pub created_at: String,
    pub has_config_blob: bool,
    #[serde(default)]
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
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub manifest: Option<BuildManifest>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

/// CheckoutRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutRequest {
    #[serde(default)]
    pub cancel_url: Option<String>,
    #[serde(default)]
    pub context: Option<serde_json::Value>,
    #[serde(default)]
    pub create_subscription: Option<bool>,
    #[serde(default)]
    pub enable_auto_pull: Option<bool>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub product_code: Option<String>,
    #[serde(default)]
    pub product_id: Option<String>,
    #[serde(default)]
    pub quantity: Option<i64>,
    #[serde(default)]
    pub success_url: Option<String>,
}

/// CheckoutResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResult {
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
    pub edited_at: Option<String>,
    pub id: String,
    #[serde(default)]
    pub likes: Option<i64>,
    #[serde(default)]
    pub replies: Option<Vec<CommentReply>>,
    #[serde(default)]
    pub translation: Option<TextTranslation>,
}

/// CommentAuthor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAuthor {
    pub id: String,
}

/// CommentCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCreateRequest {
    pub content: String,
}

/// CommentLikeResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentLikeResult {
    pub comment_id: String,
    pub liked: bool,
    pub likes: i64,
}

/// CommentList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentList {
    pub items: Vec<Comment>,
    pub limit: i64,
    pub page: i64,
    pub total: i64,
}

/// CommentReply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    pub author: CommentAuthor,
    pub content: String,
    pub created_at: String,
    #[serde(default)]
    pub edited_at: Option<String>,
    pub id: String,
    #[serde(default)]
    pub translation: Option<TextTranslation>,
}

/// CompleteLinkRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteLinkRequest {
    pub project_id: i64,
    pub state: String,
}

/// CompleteLinkResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteLinkResult {
    pub guild_id: String,
    pub project_id: i64,
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
    #[serde(default)]
    pub size_bytes: Option<i64>,
}

/// CreateLinkTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLinkTokenRequest {
    pub project_id: i64,
}

/// CreatePlannedServerRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlannedServerRequest {
    pub address: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub owner_id: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
}

/// CreateStatusOverrideRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStatusOverrideRequest {
    pub ends_at: String,
    pub mode: String,
    #[serde(default)]
    pub note: Option<String>,
    pub starts_at: String,
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
    pub comment_id: String,
}

/// DeleteStatusOverrideResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStatusOverrideResponse {
    #[serde(default)]
    pub ok: Option<bool>,
}

/// DiscordLink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordLink {
    pub enabled: bool,
    #[serde(default)]
    pub guild_id: Option<String>,
    #[serde(default)]
    pub invite_url: Option<String>,
    #[serde(default)]
    pub member_count: Option<i64>,
    pub server_id: i64,
    pub verified: bool,
    #[serde(default)]
    pub verified_at: Option<String>,
}

/// DiscordLinkUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordLinkUpdateRequest {
    #[serde(default)]
    pub enabled: Option<serde_json::Value>,
    #[serde(default)]
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

/// DiscordRoleTarget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRoleTarget {
    #[serde(default)]
    pub desired_discord_user_ids: Option<Vec<String>>,
    pub discord_role_id: String,
    pub role_id: i64,
    pub role_key: String,
    pub role_name: String,
}

/// DiscordRoleTargets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRoleTargets {
    #[serde(default)]
    pub items: Option<Vec<DiscordRoleTarget>>,
    pub project_id: i64,
}

/// DiscordVerifyRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordVerifyRequest {
    #[serde(default)]
    pub invite_url: Option<String>,
}

/// DiscoveryCandidateEditRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryCandidateEditRequest {
    pub edits: serde_json::Value,
}

/// DnsVerification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsVerification {
    #[serde(default)]
    pub checked_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    pub record_name: String,
    #[serde(default)]
    pub record_type: Option<String>,
    pub record_value: String,
    pub server: VerificationServerSummary,
    pub status: VerificationStatus,
    pub token: String,
}

/// EmailChangeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailChangeRequest {
    #[serde(default)]
    pub current_password: Option<String>,
    pub email: String,
    #[serde(default)]
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

/// FilterCount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCount {
    pub key: String,
    pub value: i64,
}

/// FilterStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterStats {
    #[serde(default)]
    pub access: Option<serde_json::Value>,
    #[serde(default)]
    pub editions: Option<serde_json::Value>,
    #[serde(default)]
    pub features: Option<serde_json::Value>,
    #[serde(default)]
    pub hosting: Option<serde_json::Value>,
    #[serde(default)]
    pub regions: Option<serde_json::Value>,
    #[serde(default)]
    pub roles: Option<serde_json::Value>,
    pub total: i64,
    #[serde(default)]
    pub verified: Option<serde_json::Value>,
}

/// ForcePingResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForcePingResult {
    #[serde(default)]
    pub cooldown_seconds: Option<i64>,
    pub enqueued: bool,
    #[serde(default)]
    pub hourly_limit: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub remaining_hourly: Option<i64>,
    #[serde(default)]
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

/// GatewayToken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayToken {
    pub audience: String,
    pub expires_in: i64,
    #[serde(default)]
    pub link_code: Option<String>,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    pub server_id: i64,
    pub session_id: String,
    pub token: String,
    pub token_type: String,
}

/// GatewayTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayTokenRequest {
    #[serde(default)]
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
    #[serde(default)]
    pub avg_online: Option<f64>,
    pub collected_at: String,
    #[serde(default)]
    pub exclude_from_score: Option<bool>,
    #[serde(default)]
    pub last_online: Option<i64>,
    #[serde(default)]
    pub max_players: Option<i64>,
    #[serde(default)]
    pub peak_online: Option<i64>,
    #[serde(default)]
    pub status: Option<OnlineState>,
    #[serde(default)]
    pub status_source: Option<TrustState>,
}

/// HistoryResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub period: String,
    #[serde(default)]
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
    #[serde(default)]
    pub items: Option<Vec<IconEntry>>,
}

/// IconSelectRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSelectRequest {
    pub key: String,
    #[serde(default)]
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

/// ImportPull
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPull {
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub fetched_at: Option<String>,
    pub guild_id: String,
    #[serde(default)]
    pub guild_name: Option<String>,
    pub scanned_messages: i64,
    pub source: String,
}

/// ImportPullRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPullRequest {
    #[serde(default)]
    pub annotations: Option<serde_json::Value>,
    #[serde(default)]
    pub before_message_id: Option<String>,
    pub channel_id: String,
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
    pub discord_subject: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    pub user_id: i64,
}

/// InternalDiscordEnsureUserResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalDiscordEnsureUserResponse {
    #[serde(default)]
    pub created: Option<bool>,
    pub found: bool,
    #[serde(default)]
    pub is_shadow: Option<bool>,
    #[serde(default)]
    pub user_id: Option<i64>,
}

/// InternalDiscordIdentityLookupResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalDiscordIdentityLookupResponse {
    pub found: bool,
    #[serde(default)]
    pub user_id: Option<i64>,
}

/// InternalMinecraftIdentityLookupResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMinecraftIdentityLookupResponse {
    pub found: bool,
    #[serde(default)]
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
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
    pub email: Option<String>,
    pub found: bool,
    #[serde(default)]
    pub username: Option<String>,
}

/// InternalUserDiscordSubjectResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalUserDiscordSubjectResponse {
    #[serde(default)]
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
    #[serde(default)]
    pub aud: Option<String>,
    #[serde(default)]
    pub exp: Option<i64>,
    #[serde(default)]
    pub iat: Option<i64>,
    #[serde(default)]
    pub jti: Option<String>,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleState {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "unknown")]
    Unknown,
}

/// LikedCommentIds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikedCommentIds {
    pub comment_ids: Vec<String>,
}

/// LinkCompletionRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCompletionRequest {
    pub code: String,
    pub minecraft_nick: String,
    pub minecraft_uuid: String,
    pub project_id: i64,
    pub server_id: i64,
}

/// LinkCompletionResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCompletionResponse {
    #[serde(default)]
    pub account: Option<LinkedMinecraftAccount>,
    pub status: String,
    #[serde(default)]
    pub user_id: Option<i64>,
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

/// LinkSession
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSession {
    #[serde(default)]
    pub already_linked_project_id: Option<i64>,
    pub expires_at: String,
    pub guild_id: String,
    #[serde(default)]
    pub guild_name: Option<String>,
    pub status: LinkSessionStatus,
    #[serde(default)]
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
    pub id: i64,
    #[serde(default)]
    pub identity_scope_id: Option<i64>,
    pub identity_scope_type: MinecraftIdentityScopeType,
    pub link_source: MinecraftLinkSource,
    pub minecraft_nick: String,
    pub minecraft_uuid: String,
    #[serde(default)]
    pub proof_server_id: Option<i64>,
    pub verification_status: MinecraftVerificationStatus,
}

/// LiveDashboardStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDashboardStats {
    #[serde(default)]
    pub collected_at: Option<String>,
    pub players_online: i64,
    #[serde(default)]
    pub players_with_profile: Option<i64>,
    #[serde(default)]
    pub registered_profiles: Option<i64>,
    pub servers_online: i64,
    pub servers_with_agent: i64,
    #[serde(default)]
    pub total_playtime_hours: Option<i64>,
}

/// LiveStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStatus {
    pub collected_at: String,
    #[serde(default)]
    pub connection_state: Option<OnlineState>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    pub freshness_state: FreshnessState,
    #[serde(default)]
    pub max_players: Option<i64>,
    #[serde(default)]
    pub motd: Option<String>,
    #[serde(default)]
    pub online: Option<i64>,
    #[serde(default)]
    pub online_reason: Option<String>,
    pub online_state: OnlineState,
    #[serde(default)]
    pub players: Option<Vec<String>>,
    pub server_id: i64,
    #[serde(default)]
    pub source: Option<TrustState>,
    #[serde(default)]
    pub version: Option<String>,
}

/// LoginResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_jwt: String,
    pub audience: String,
    pub expires_in: i64,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
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

/// MeResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeResponse {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub minecraft_accounts: Option<Vec<MinecraftAccount>>,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    pub user_id: i64,
    #[serde(default)]
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
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

/// MinecraftAccountResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAccountResponse {
    pub account_type: String,
    #[serde(default)]
    pub created_at: Option<String>,
    pub id: i64,
    #[serde(default)]
    pub identity_scope_id: Option<i64>,
    pub identity_scope_type: String,
    pub link_source: String,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub proof_server_id: Option<i64>,
    #[serde(default)]
    pub uuid_type: Option<String>,
    pub verification_status: String,
    #[serde(default)]
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

/// MinecraftCandidateAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftCandidateAccount {
    pub account_type: MinecraftAccountType,
    pub found: bool,
    pub minecraft_nick: String,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

/// MinecraftGroupTarget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftGroupTarget {
    #[serde(default)]
    pub desired_minecraft_uuids: Option<Vec<String>>,
    pub luckperms_group: String,
    pub role_id: i64,
    pub role_key: String,
    pub role_name: String,
    pub server_id: i64,
}

/// MinecraftGroupTargets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftGroupTargets {
    #[serde(default)]
    pub items: Option<Vec<MinecraftGroupTarget>>,
    pub project_id: i64,
    pub server_id: i64,
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

/// MinecraftLinkCodeIssueRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftLinkCodeIssueRequest {
    pub minecraft_nick: String,
    #[serde(default)]
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
    pub challenge_id: i64,
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
    pub challenge_id: i64,
    pub code: String,
    pub expires_at: String,
    pub expires_in: i64,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
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
    #[serde(rename = "unknown")]
    Unknown,
}

/// MinecraftVerificationAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVerificationAccount {
    pub account_type: MinecraftAccountType,
    pub id: i64,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    pub verification_status: MinecraftVerificationStatus,
}

/// MinecraftVerificationState
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVerificationState {
    #[serde(default)]
    pub items: Option<Vec<MinecraftVerificationAccount>>,
    #[serde(default)]
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

/// MotdVerification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotdVerification {
    #[serde(default)]
    pub checked_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    pub server: VerificationServerSummary,
    pub status: VerificationStatus,
    pub token: String,
}

/// MyComment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyComment {
    #[serde(default)]
    pub can_moderate_comments: Option<bool>,
    #[serde(default)]
    pub can_reply_to_comments: Option<bool>,
    #[serde(default)]
    pub comment: Option<Comment>,
}

/// MyPlayerStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyPlayerStats {
    #[serde(default)]
    pub accounts: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub daily_activity: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub estimated: Option<bool>,
    #[serde(default)]
    pub events_breakdown: Option<serde_json::Value>,
    #[serde(default)]
    pub servers: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub source: Option<String>,
    pub total_playtime_seconds: i64,
}

/// MyServerIssuesPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyServerIssuesPage {
    #[serde(default)]
    pub items: Option<Vec<ServerIssuesItem>>,
    pub total_issues: i64,
    pub total_servers: i64,
}

/// MyServersPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyServersPage {
    #[serde(default)]
    pub items: Option<Vec<ServerCard>>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// NotificationPreferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(default)]
    pub whitelist_applicant: Option<NotificationTopicPreferences>,
    #[serde(default)]
    pub whitelist_staff: Option<NotificationTopicPreferences>,
}

/// NotificationPreferencesUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferencesUpdate {
    #[serde(default)]
    pub whitelist_applicant: Option<NotificationTopicPreferences>,
    #[serde(default)]
    pub whitelist_staff: Option<NotificationTopicPreferences>,
}

/// NotificationTopicPreferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTopicPreferences {
    #[serde(default)]
    pub enabled_channels: Option<Vec<String>>,
    #[serde(default)]
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
    #[serde(default)]
    pub audience: Option<String>,
    pub code: String,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    pub state: String,
    #[serde(default)]
    pub tenant_id: Option<i64>,
    #[serde(default)]
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
    #[serde(default)]
    pub avatar_url: Option<String>,
    pub connected: bool,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub provider: String,
    #[serde(default)]
    pub subject: Option<String>,
}

/// OAuthProvidersResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvidersResponse {
    #[serde(default)]
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
    #[serde(default)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnlineState {
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "unknown")]
    Unknown,
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

/// Order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub amount_minor: i64,
    pub ccy: i64,
    pub created_at: String,
    pub description: String,
    #[serde(default)]
    pub expires_at: Option<String>,
    pub external_ref: String,
    #[serde(default)]
    pub extra: Option<serde_json::Value>,
    pub id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub paid_at: Option<String>,
    #[serde(default)]
    pub payments: Option<Vec<Payment>>,
    pub product_id: String,
    #[serde(default)]
    pub refunded_at: Option<String>,
    pub status: String,
    #[serde(default)]
    pub status_reason: Option<String>,
    #[serde(default)]
    pub subscription_id: Option<String>,
    pub updated_at: String,
    pub user_id: String,
}

/// OrderList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderList {
    pub items: Vec<Order>,
    pub limit: i64,
    pub page: i64,
    pub total: i64,
}

/// Page[ServerSummary]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagePlatformApiModelsServersServerSummary {
    pub items: Vec<ServerSummary>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub per_page: Option<i64>,
    pub total: i64,
}

/// PasswordChangeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
    #[serde(default)]
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
    #[serde(default)]
    pub current_password: Option<String>,
    pub new_password: String,
    #[serde(default)]
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
    #[serde(default)]
    pub page_url: Option<String>,
    pub provider: String,
    #[serde(default)]
    pub provider_payment_id: Option<String>,
    pub status: String,
    pub updated_at: String,
}

/// PersonalAccessTokenCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalAccessTokenCreateRequest {
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub expires_in_days: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    #[serde(default)]
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
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub revoked_at: Option<String>,
    pub scope: Vec<String>,
    #[serde(default)]
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
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    pub user_id: i64,
    pub version: i64,
}

/// PlatformRoleListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRoleListResponse {
    #[serde(default)]
    pub items: Option<Vec<PlatformRoleResponse>>,
}

/// PlatformRoleRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRoleRequest {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub permissions: Option<Vec<String>>,
}

/// PlatformRoleResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRoleResponse {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(default)]
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
    #[serde(default)]
    pub last_server_id: Option<i64>,
    pub total_playtime_seconds: i64,
    pub username: String,
    pub uuid: String,
}

/// PlayerStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub found: bool,
    #[serde(default)]
    pub per_server_seconds: Option<serde_json::Value>,
    pub server_playtime_seconds: i64,
    #[serde(default)]
    pub source: Option<String>,
    pub total_playtime_seconds: i64,
}

/// PluginVerification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVerification {
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub link_code: Option<String>,
    pub server: VerificationServerSummary,
    pub status: VerificationStatus,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub token_type: Option<String>,
    #[serde(default)]
    pub verified_at: Option<String>,
}

/// PluginVerificationStartRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVerificationStartRequest {
    pub address: String,
    #[serde(default)]
    pub expires_in_hours: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub proxy_type: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub extra: Option<serde_json::Value>,
    pub geo_source: String,
    pub id: String,
    pub interval: String,
    pub name: String,
    #[serde(default)]
    pub price_country_code: Option<String>,
    pub product_type: String,
    pub provider_supported: bool,
    pub used_fallback: bool,
}

/// ProfileActivityHeatmap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileActivityHeatmap {
    #[serde(default)]
    pub days: Option<Vec<ProfileActivityHeatmapDay>>,
    #[serde(default)]
    pub total_events: Option<i64>,
    #[serde(default)]
    pub window_days: Option<i64>,
}

/// ProfileActivityHeatmapDay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileActivityHeatmapDay {
    pub date: String,
    #[serde(default)]
    pub events: Option<i64>,
    #[serde(default)]
    pub sessions: Option<i64>,
}

/// ProfileGameplaySummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileGameplaySummary {
    #[serde(default)]
    pub active_days: Option<i64>,
    #[serde(default)]
    pub linked_accounts: Option<Vec<ProfileLinkedMinecraftAccount>>,
    #[serde(default)]
    pub most_played_server: Option<ProfileMostPlayedServer>,
    #[serde(default)]
    pub streak_current: Option<i64>,
    #[serde(default)]
    pub streak_longest: Option<i64>,
    #[serde(default)]
    pub total_playtime_seconds: Option<i64>,
}

/// ProfileLinkedMinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileLinkedMinecraftAccount {
    #[serde(default)]
    pub nick: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
}

/// ProfileMostPlayedServer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMostPlayedServer {
    #[serde(default)]
    pub activity: Option<i64>,
    #[serde(default)]
    pub last_seen: Option<String>,
    #[serde(default)]
    pub playtime_seconds: Option<i64>,
    pub server_id: String,
}

/// ProfileOwnershipSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOwnershipSummary {
    #[serde(default)]
    pub project_count: Option<i64>,
    #[serde(default)]
    pub projects: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub server_count: Option<i64>,
    #[serde(default)]
    pub servers: Option<Vec<serde_json::Value>>,
}

/// ProfilePrivacyUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePrivacyUpdate {
    #[serde(default)]
    pub show_activity_stats: Option<bool>,
    #[serde(default)]
    pub show_bio: Option<bool>,
    #[serde(default)]
    pub show_join_date: Option<bool>,
    #[serde(default)]
    pub show_linked_accounts: Option<bool>,
    #[serde(default)]
    pub show_ownership: Option<bool>,
    #[serde(default)]
    pub show_status: Option<bool>,
    #[serde(default)]
    pub show_streak: Option<bool>,
    #[serde(default)]
    pub show_top_server: Option<bool>,
    #[serde(default)]
    pub show_user_id: Option<bool>,
}

/// ProfileUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdateRequest {
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub privacy: Option<ProfilePrivacyUpdate>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub time_format_preference: Option<String>,
    pub username: String,
}

/// Project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub catalog_mode: String,
    pub display_server: ServerSummary,
    pub display_server_id: i64,
    #[serde(default)]
    pub freshness_state: Option<FreshnessState>,
    #[serde(default)]
    pub game_editions: Option<Vec<String>>,
    #[serde(default)]
    pub hearts: Option<i64>,
    pub id: i64,
    #[serde(default)]
    pub last_status_at: Option<String>,
    #[serde(default)]
    pub max_players: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub online_players: Option<i64>,
    #[serde(default)]
    pub online_reason: Option<String>,
    #[serde(default)]
    pub online_server_id: Option<i64>,
    #[serde(default)]
    pub online_source: Option<TrustState>,
    #[serde(default)]
    pub online_state: Option<OnlineState>,
    pub online_strategy: OnlineStrategy,
    #[serde(default)]
    pub owner_id: Option<i64>,
    #[serde(default)]
    pub public_entrypoint: Option<String>,
    #[serde(default)]
    pub public_entrypoint_count: Option<i64>,
    #[serde(default)]
    pub public_entrypoint_state: Option<PublicEntrypointState>,
    #[serde(default)]
    pub public_profile_kind: Option<PublicProfileKind>,
    #[serde(default)]
    pub public_server_count: Option<i64>,
    #[serde(default)]
    pub regions: Option<Vec<String>>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub score_breakdown: Option<ScoreBreakdown>,
    #[serde(default)]
    pub server_count: Option<i64>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub trust_state: Option<TrustState>,
    #[serde(default)]
    pub uptime_24h_percentage: Option<f64>,
    #[serde(default)]
    pub uptime_7d_percentage: Option<f64>,
    #[serde(default)]
    pub verified_server_count: Option<i64>,
    #[serde(default)]
    pub votes_monthly: Option<i64>,
}

/// ProjectCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreateRequest {
    pub name: String,
    #[serde(default)]
    pub public_entrypoint: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
}

/// ProjectCreateResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreateResponse {
    pub id: i64,
    #[serde(default)]
    pub lifecycle_state: Option<LifecycleState>,
    pub name: String,
    #[serde(default)]
    pub public_entrypoint: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
}

/// ProjectDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDetail {
    pub project: Project,
    #[serde(default)]
    pub servers: Option<Vec<ServerSummary>>,
}

/// ProjectEngagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEngagement {
    pub comments: i64,
    pub hearts: i64,
    pub project_id: String,
    pub thumbs: i64,
}

/// ProjectEngagementStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEngagementStatus {
    pub hearted: bool,
    pub project_id: String,
    pub thumbed: bool,
}

/// ProjectFilterStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFilterStats {
    #[serde(default)]
    pub access: Option<Vec<FilterCount>>,
    #[serde(default)]
    pub editions: Option<Vec<FilterCount>>,
    #[serde(default)]
    pub features: Option<Vec<FilterCount>>,
    #[serde(default)]
    pub hosting: Option<Vec<FilterCount>>,
    #[serde(default)]
    pub regions: Option<Vec<FilterCount>>,
    #[serde(default)]
    pub roles: Option<Vec<FilterCount>>,
    pub total: i64,
    #[serde(default)]
    pub verified: Option<Vec<FilterCount>>,
}

/// ProjectHeartResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectHeartResult {
    pub hearted: bool,
    pub hearts: i64,
    pub project_id: String,
}

/// ProjectListResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub items: Vec<Project>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// ProjectResolveResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResolveResponse {
    pub project_id: i64,
    #[serde(default)]
    pub slug: Option<String>,
}

/// ProjectStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub active_servers: i64,
    #[serde(default)]
    pub avg_online: Option<f64>,
    #[serde(default)]
    pub chats: Option<i64>,
    #[serde(default)]
    pub commands: Option<i64>,
    #[serde(default)]
    pub first_event_at: Option<String>,
    #[serde(default)]
    pub joins: Option<i64>,
    #[serde(default)]
    pub last_event_at: Option<String>,
    #[serde(default)]
    pub peak_online: Option<i64>,
    pub period: String,
    pub server_count: i64,
    #[serde(default)]
    pub total_events: Option<i64>,
    #[serde(default)]
    pub unique_players: Option<i64>,
}

/// ProjectThumbResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectThumbResult {
    pub project_id: String,
    pub thumbed: bool,
    pub thumbs: i64,
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

/// PublicJWK
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicJWK {
    pub alg: String,
    pub e: String,
    pub kid: String,
    pub kty: String,
    pub n: String,
    #[serde(rename = "use")]
    #[serde(default)]
    pub r#use: Option<String>,
}

/// PublicProfile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfile {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub privacy: Option<UsersProfilePrivacy>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    pub user_id: i64,
    #[serde(default)]
    pub username: Option<String>,
}

/// PublicProfileCard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfileCard {
    #[serde(default)]
    pub avatar_url: Option<String>,
    pub found: bool,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    pub user_id: i64,
    #[serde(default)]
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

/// PublicProfileResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfileResponse {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    pub found: bool,
    #[serde(default)]
    pub privacy: Option<UserProfilePrivacy>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub username: Option<String>,
}

/// PublicProfilesBatchItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfilesBatchItem {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
    pub user_id: i64,
    #[serde(default)]
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
    #[serde(default)]
    pub is_owner: Option<bool>,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    pub user_id: i64,
}

/// RecentActivityItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivityItem {
    pub created_at: String,
    #[serde(default)]
    pub preview: Option<String>,
    pub project_id: String,
    #[serde(rename = "type")]
    pub r#type: ActivityType,
}

/// RecentVotes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentVotes {
    pub project_id: String,
    pub votes: Vec<VoteItem>,
}

/// RefreshTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    #[serde(default)]
    pub tenant_id: Option<i64>,
}

/// ReportUserRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportUserRequest {
    #[serde(default)]
    pub details: Option<String>,
    pub reason: String,
}

/// ReportUserResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportUserResult {
    pub id: String,
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
    #[serde(default)]
    pub managed: Option<bool>,
    pub name: String,
    #[serde(default)]
    pub position: Option<i64>,
}

/// RoleCatalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCatalog {
    pub available: bool,
    #[serde(default)]
    pub guild_id: Option<String>,
    #[serde(default)]
    pub guild_name: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
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

/// ScoreBreakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    #[serde(default)]
    pub avg_online: Option<f64>,
    #[serde(default)]
    pub comments: Option<i64>,
    #[serde(default)]
    pub hearts: Option<i64>,
    #[serde(default)]
    pub thumbs: Option<i64>,
    pub total: f64,
    #[serde(default)]
    pub verified_bonus: Option<f64>,
}

/// ServerBot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBot {
    #[serde(default)]
    pub bot_id: Option<String>,
    #[serde(default)]
    pub linked_guilds: Option<Vec<ServerBotLinkedGuild>>,
}

/// ServerBotLinkedGuild
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBotLinkedGuild {
    pub guild_id: String,
    #[serde(default)]
    pub guild_name: Option<String>,
    #[serde(default)]
    pub linked_at: Option<String>,
}

/// ServerBotUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBotUpdateRequest {
    #[serde(default)]
    pub bot_link: Option<String>,
}

/// ServerCard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCard {
    #[serde(default)]
    pub favicon_url: Option<String>,
    #[serde(default)]
    pub game_edition: Option<GameEdition>,
    #[serde(default)]
    pub icon_url: Option<String>,
    pub id: i64,
    pub ip_or_domain: String,
    #[serde(default)]
    pub is_verified: Option<bool>,
    #[serde(default)]
    pub last_max_players: Option<i64>,
    #[serde(default)]
    pub last_online_players: Option<i64>,
    #[serde(default)]
    pub maintenance_enabled: Option<bool>,
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
    pub role: ServerRole,
    #[serde(default)]
    pub show_in_public: Option<bool>,
    #[serde(default)]
    pub slug: Option<String>,
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
    #[serde(default)]
    pub bedrock_port: Option<i64>,
    #[serde(default)]
    pub game_edition: Option<GameEdition>,
    pub id: i64,
    pub ip_or_domain: String,
    #[serde(default)]
    pub is_verified: Option<bool>,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub ping_ip_or_domain: Option<String>,
    #[serde(default)]
    pub ping_port: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
    #[serde(default)]
    pub project_online_strategy: Option<OnlineStrategy>,
    #[serde(default)]
    pub proxy_type: Option<String>,
    pub role: ServerRole,
    #[serde(default)]
    pub verification_source: Option<TrustState>,
}

/// ServerEventPoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEventPoint {
    pub collected_at: String,
    pub event_type: String,
    #[serde(default)]
    pub extra: Option<serde_json::Value>,
    #[serde(default)]
    pub online: Option<i64>,
    #[serde(default)]
    pub online_delta: Option<i64>,
    #[serde(default)]
    pub player_id: Option<String>,
    #[serde(default)]
    pub player_name: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

/// ServerEvents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEvents {
    pub items: Vec<ServerEventPoint>,
    pub limit: i64,
    pub page: i64,
    pub period: String,
    pub total: i64,
}

/// ServerIssuesItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerIssuesItem {
    pub ip_or_domain: String,
    #[serde(default)]
    pub issues: Option<Vec<ServerServiceIssue>>,
    pub server_id: i64,
    pub server_name: String,
    #[serde(default)]
    pub server_slug: Option<String>,
}

/// ServerMaintenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMaintenance {
    pub enabled: bool,
    #[serde(default)]
    pub message: Option<String>,
}

/// ServerMaintenanceUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMaintenanceUpdateRequest {
    pub enabled: bool,
    #[serde(default)]
    pub message: Option<String>,
}

/// ServerMediaSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMediaSummary {
    #[serde(default)]
    pub icon_key: Option<String>,
    #[serde(default)]
    pub icon_url: Option<String>,
    pub icon_version: i64,
    pub server_id: i64,
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
    #[serde(default)]
    pub sync: Option<bool>,
}

/// ServerOwnership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerOwnership {
    pub can_edit: bool,
    pub is_owner: bool,
    pub server_id: i64,
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

/// ServerRoot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRoot {
    pub project_id: i64,
    pub root_server_id: i64,
    pub server_id: i64,
}

/// ServerServiceIssue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerServiceIssue {
    #[serde(default)]
    pub action: Option<String>,
    pub code: String,
    #[serde(default)]
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
    pub parent_id: i64,
}

/// ServerSetPingPortRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetPingPortRequest {
    pub port: i64,
}

/// ServerSetRegionsRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSetRegionsRequest {
    #[serde(default)]
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
    #[serde(default)]
    pub first_event_at: Option<String>,
    pub joins: i64,
    pub kicks: i64,
    #[serde(default)]
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
    #[serde(default)]
    pub subserver_ids: Option<Vec<i64>>,
}

/// ServerSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSummary {
    #[serde(default)]
    pub favicon_url: Option<String>,
    #[serde(default)]
    pub game_edition: Option<GameEdition>,
    #[serde(default)]
    pub icon_url: Option<String>,
    pub id: i64,
    pub ip_or_domain: String,
    #[serde(default)]
    pub is_verified: Option<bool>,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
    pub role: ServerRole,
}

/// ServerTeamManage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTeamManage {
    pub enabled: bool,
    #[serde(default)]
    pub members: Option<Vec<TeamMemberItem>>,
    pub project_id: i64,
    #[serde(default)]
    pub project_servers: Option<Vec<TeamScopeServer>>,
    #[serde(default)]
    pub roles: Option<Vec<TeamRoleItem>>,
    pub root_server_id: i64,
    pub server_id: i64,
}

/// ServerTeamPublic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTeamPublic {
    pub enabled: bool,
    #[serde(default)]
    pub inherited_from_server_id: Option<i64>,
    #[serde(default)]
    pub members: Option<Vec<PublicTeamMember>>,
    pub project_id: i64,
    pub server_id: i64,
}

/// ServerTelemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTelemetry {
    #[serde(default)]
    pub collected_at: Option<String>,
    #[serde(default)]
    pub latest: Option<serde_json::Value>,
    #[serde(default)]
    pub metrics: Option<Vec<ServerTelemetryMetric>>,
    pub period: String,
    #[serde(default)]
    pub source: Option<String>,
}

/// ServerTelemetryMetric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTelemetryMetric {
    pub avg: f64,
    pub cadence_hint_seconds: i64,
    pub key: String,
    pub kind: String,
    pub last: f64,
    pub max: f64,
    pub min: f64,
    pub samples: i64,
}

/// ServerTranslation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTranslation {
    #[serde(default)]
    pub engine: Option<String>,
    pub field: String,
    pub locale: String,
    #[serde(default)]
    pub original_text: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub source_locale: Option<String>,
    #[serde(default)]
    pub status: Option<TranslationStatus>,
    #[serde(default)]
    pub target_locale: Option<String>,
    #[serde(default)]
    pub translated_text: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
    pub items: Option<Vec<ServerTranslation>>,
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

/// ServiceHealthEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthEntry {
    pub checked_at: String,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub latency_ms: Option<i64>,
    pub name: String,
    pub status: ServiceHealth,
}

/// ServicesHealthResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesHealthResponse {
    #[serde(default)]
    pub items: Option<Vec<ServiceHealthEntry>>,
}

/// SessionInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(default)]
    pub current: Option<bool>,
    #[serde(default)]
    pub expires_at: Option<String>,
    pub id: String,
    #[serde(default)]
    pub ip_address: Option<String>,
    pub issued_at: String,
    #[serde(default)]
    pub revoked_at: Option<String>,
    #[serde(default)]
    pub user_agent: Option<String>,
}

/// SessionList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionList {
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    pub platform: String,
    #[serde(default)]
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
    #[serde(default)]
    pub instagram_url: Option<String>,
    pub server_id: i64,
    #[serde(default)]
    pub telegram_url: Option<String>,
    #[serde(default)]
    pub tiktok_url: Option<String>,
    #[serde(default)]
    pub twitch_url: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub website_url: Option<String>,
    #[serde(default)]
    pub youtube_url: Option<String>,
}

/// SocialLinksUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLinksUpdateRequest {
    #[serde(default)]
    pub instagram_url: Option<String>,
    #[serde(default)]
    pub telegram_url: Option<String>,
    #[serde(default)]
    pub tiktok_url: Option<String>,
    #[serde(default)]
    pub twitch_url: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub website_url: Option<String>,
    #[serde(default)]
    pub youtube_url: Option<String>,
}

/// StatusOverrideItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOverrideItem {
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<i64>,
    pub ends_at: String,
    pub id: String,
    pub mode: String,
    #[serde(default)]
    pub note: Option<String>,
    pub server_id: i64,
    pub starts_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
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
    #[serde(default)]
    pub canceled_at: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub extra: Option<serde_json::Value>,
    pub id: String,
    #[serde(default)]
    pub last_charge_at: Option<String>,
    #[serde(default)]
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
    pub limit: i64,
    pub page: i64,
    pub total: i64,
}

/// TeamMemberCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberCreateRequest {
    #[serde(default)]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub member_state: Option<MemberState>,
    #[serde(default)]
    pub role_assignments: Option<Vec<TeamMemberRoleAssignmentInput>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
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
    pub id: i64,
    pub is_owner: bool,
    pub is_public: bool,
    #[serde(default)]
    pub member_state: Option<MemberState>,
    #[serde(default)]
    pub role_assignments: Option<Vec<TeamMemberRoleAssignment>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    pub sort_order: i64,
    pub user_id: i64,
}

/// TeamMemberRoleAssignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberRoleAssignment {
    pub role_id: String,
    #[serde(default)]
    pub scope_id: Option<String>,
    #[serde(default)]
    pub scope_type: Option<TeamScopeType>,
}

/// TeamMemberRoleAssignmentInput
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberRoleAssignmentInput {
    pub role_id: String,
    #[serde(default)]
    pub scope_id: Option<String>,
    #[serde(default)]
    pub scope_type: Option<TeamScopeType>,
}

/// TeamMemberUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberUpdateRequest {
    #[serde(default)]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub member_state: Option<MemberState>,
    #[serde(default)]
    pub role_assignments: Option<Vec<TeamMemberRoleAssignmentInput>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub sort_order: Option<i64>,
}

/// TeamRoleCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRoleCreateRequest {
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub discord_role_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub luckperms_group: Option<String>,
    pub name: String,
    #[serde(default)]
    pub permissions: Option<Vec<String>>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
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
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub discord_role_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    pub id: i64,
    #[serde(default)]
    pub is_public: Option<bool>,
    pub key: String,
    #[serde(default)]
    pub luckperms_group: Option<String>,
    #[serde(default)]
    pub managed: Option<bool>,
    pub name: String,
    #[serde(default)]
    pub permissions: Option<Vec<String>>,
    #[serde(default)]
    pub permissions_bits: Option<i64>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
    pub sort_order: Option<i64>,
}

/// TeamRoleUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRoleUpdateRequest {
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub discord_role_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub luckperms_group: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub permissions: Option<Vec<String>>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
    pub sort_order: Option<i64>,
}

/// TeamScopeServer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamScopeServer {
    pub id: i64,
    pub name: String,
    pub server_role: ServerRole,
    #[serde(default)]
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamScopeType {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "whitelist_policy")]
    WhitelistPolicy,
    #[serde(rename = "unknown")]
    Unknown,
}

/// TextTranslation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTranslation {
    pub engine: String,
    pub original_text: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
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

/// TicketCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCreateRequest {
    pub content: String,
    #[serde(default)]
    pub minecraft_username: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub priority: Option<TicketPriority>,
    pub server_id: String,
    #[serde(default)]
    pub source: Option<TicketSource>,
    pub subject: String,
}

/// TicketDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketDetail {
    pub creator_id: String,
    #[serde(default)]
    pub messages: Option<Vec<TicketMessage>>,
    pub summary: TicketSummary,
}

/// TicketList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketList {
    pub items: Vec<TicketSummary>,
    pub limit: i64,
    pub page: i64,
    pub total: i64,
}

/// TicketMessage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessage {
    pub author_id: String,
    pub author_type: TicketAuthorType,
    pub content: String,
    pub created_at: String,
    pub id: String,
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

/// TicketStatusUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketStatusUpdateRequest {
    pub status: TicketStatus,
}

/// TicketSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSummary {
    pub created_at: String,
    pub id: String,
    pub priority: TicketPriority,
    pub server_id: String,
    pub status: TicketStatus,
    pub subject: String,
    pub updated_at: String,
}

/// TokenExchangeRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenExchangeRequest {
    pub audience: String,
    #[serde(default)]
    pub expires_in_minutes: Option<i64>,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    #[serde(default)]
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
    #[serde(default)]
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

/// UnlinkedMinecraftAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedMinecraftAccount {
    pub account_type: String,
    pub id: i64,
    #[serde(default)]
    pub identity_scope_id: Option<i64>,
    pub identity_scope_type: String,
    pub link_source: String,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub proof_server_id: Option<i64>,
    #[serde(default)]
    pub uuid_type: Option<String>,
    pub verification_status: String,
}

/// UpdateManifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub artifact_id: String,
    pub channel: String,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub download_urls: Option<Vec<String>>,
    pub file_name: String,
    #[serde(default)]
    pub modrinth_url: Option<String>,
    pub platform: String,
    pub product: String,
    pub sha256: String,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub signature_version: Option<String>,
    pub version: String,
}

/// UpdateManifestUpsert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifestUpsert {
    #[serde(default)]
    pub artifact_id: Option<String>,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub download_urls: Option<Vec<String>>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub modrinth_url: Option<String>,
    #[serde(default)]
    pub rollout_allow_servers: Option<Vec<i64>>,
    #[serde(default)]
    pub rollout_deny_servers: Option<Vec<i64>>,
    #[serde(default)]
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
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub current_version: Option<String>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub event: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub server_id: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub target_version: Option<String>,
}

/// UserDetailed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetailed {
    #[serde(default)]
    pub avatar_url: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub email: Option<String>,
    pub id: String,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
    pub status: String,
    pub username: String,
}

/// UserEngagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEngagement {
    #[serde(default)]
    pub top_project: Option<UserEngagementTopProject>,
    pub total_comments: i64,
    pub total_favorites: i64,
    pub total_votes: i64,
}

/// UserEngagementTopProject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEngagementTopProject {
    pub project_id: String,
    pub votes: i64,
}

/// UserLogin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub captcha_token: Option<String>,
    #[serde(default)]
    pub ip_address: Option<String>,
    pub password: String,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    #[serde(default)]
    pub tenant_id: Option<i64>,
    #[serde(default)]
    pub totp_code: Option<String>,
    pub username: String,
}

/// UserProfile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub email_verified: Option<bool>,
    pub id: i64,
    #[serde(default)]
    pub privacy: Option<MeProfilePrivacy>,
    #[serde(default)]
    pub slug: Option<String>,
    pub status: String,
    #[serde(default)]
    pub time_format_preference: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    pub username: String,
}

/// UserProfilePrivacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePrivacy {
    #[serde(default)]
    pub show_activity_stats: Option<bool>,
    #[serde(default)]
    pub show_bio: Option<bool>,
    #[serde(default)]
    pub show_join_date: Option<bool>,
    #[serde(default)]
    pub show_linked_accounts: Option<bool>,
    #[serde(default)]
    pub show_ownership: Option<bool>,
    #[serde(default)]
    pub show_status: Option<bool>,
    #[serde(default)]
    pub show_streak: Option<bool>,
    #[serde(default)]
    pub show_top_server: Option<bool>,
    #[serde(default)]
    pub show_user_id: Option<bool>,
}

/// UserPublic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub email_verified: Option<bool>,
    pub id: i64,
    #[serde(default)]
    pub privacy: Option<UserProfilePrivacy>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub time_format_preference: Option<UserTimeFormatPreference>,
    #[serde(default)]
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
    #[serde(default)]
    pub captcha_token: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub password: String,
    pub username: String,
}

/// UserRolesResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRolesResponse {
    #[serde(default)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserTimeFormatPreference {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "12h")]
    _12h,
    #[serde(rename = "24h")]
    _24h,
}

/// VerificationCheckRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCheckRequest {
    pub token: String,
}

/// VerificationServerSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationServerSummary {
    #[serde(default)]
    pub game_edition: Option<GameEdition>,
    pub id: i64,
    pub ip_or_domain: String,
    pub is_verified: bool,
    #[serde(default)]
    pub owner_id: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
    pub role: ServerRole,
    #[serde(default)]
    pub verification_level: Option<i64>,
    #[serde(default)]
    pub verification_source: Option<TrustState>,
    #[serde(default)]
    pub verified_at: Option<String>,
}

/// VerificationStartRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStartRequest {
    pub address: String,
    #[serde(default)]
    pub project_id: Option<i64>,
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

/// VoteItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteItem {
    pub user_id: String,
    pub voted_at: String,
}

/// VotingLinks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingLinks {
    #[serde(default)]
    pub allmc_url: Option<String>,
    #[serde(default)]
    pub disflip_url: Option<String>,
    #[serde(default)]
    pub leavepulse_url: Option<String>,
    #[serde(default)]
    pub monicore_url: Option<String>,
    pub server_id: i64,
}

/// VotingLinksUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingLinksUpdateRequest {
    #[serde(default)]
    pub allmc_url: Option<String>,
    #[serde(default)]
    pub disflip_url: Option<String>,
    #[serde(default)]
    pub leavepulse_url: Option<String>,
    #[serde(default)]
    pub monicore_url: Option<String>,
}

/// WebsocketTokenRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketTokenRequest {
    pub audience: String,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
}

/// WhitelistApplicantNotificationSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplicantNotificationSettings {
    #[serde(default)]
    pub discord_dm: Option<WhitelistNotificationChannelSettings>,
    #[serde(default)]
    pub email: Option<WhitelistNotificationChannelSettings>,
    #[serde(default)]
    pub minecraft_direct: Option<WhitelistNotificationChannelSettings>,
    #[serde(default)]
    pub telegram: Option<WhitelistNotificationChannelSettings>,
}

/// WhitelistApplication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplication {
    #[serde(default)]
    pub application_url: Option<String>,
    #[serde(default)]
    pub auto_approved: Option<bool>,
    pub created_at: String,
    #[serde(default)]
    pub discord_name: Option<String>,
    #[serde(default)]
    pub form_id: Option<i64>,
    pub id: i64,
    #[serde(default)]
    pub minecraft_account_type: Option<MinecraftAccountType>,
    #[serde(default)]
    pub minecraft_identity_state: Option<MinecraftIdentityState>,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
    #[serde(default)]
    pub review_reason: Option<String>,
    #[serde(default)]
    pub reviewed_at: Option<String>,
    pub server_id: i64,
    pub status: WhitelistApplicationStatus,
    #[serde(default)]
    pub status_alias: Option<String>,
    pub user_id: i64,
}

/// WhitelistApplicationList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplicationList {
    #[serde(default)]
    pub items: Option<Vec<WhitelistApplication>>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub per_page: Option<i64>,
    #[serde(default)]
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

/// WhitelistApplyRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistApplyRequest {
    #[serde(default)]
    pub answers: Option<serde_json::Value>,
    #[serde(default)]
    pub binding_id: Option<i64>,
    pub minecraft_account_type: String,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
}

/// WhitelistBindingDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistBindingDetail {
    #[serde(default)]
    pub discord_membership_mode: Option<DiscordMembershipMode>,
    #[serde(default)]
    pub enabled: Option<bool>,
    pub enforcement_mode: EnforcementMode,
    #[serde(default)]
    pub form_id: Option<String>,
    #[serde(default)]
    pub granted_role_ids: Option<Vec<String>>,
    pub id: String,
    #[serde(default)]
    pub mode: Option<WhitelistBindingMode>,
    #[serde(default)]
    pub notification_settings: Option<WhitelistNotificationSettings>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub restrict_chat: Option<bool>,
    pub scope_type: TeamScopeType,
    pub server_id: String,
    #[serde(default)]
    pub target_server_ids: Option<Vec<String>>,
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

/// WhitelistBindingTestResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistBindingTestResult {
    #[serde(default)]
    pub detail: Option<String>,
    pub sent: bool,
}

/// WhitelistBindingWriteRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistBindingWriteRequest {
    #[serde(default)]
    pub discord_membership_mode: Option<DiscordMembershipMode>,
    #[serde(default)]
    pub enabled: Option<bool>,
    pub enforcement_mode: EnforcementMode,
    #[serde(default)]
    pub form_id: Option<String>,
    #[serde(default)]
    pub granted_role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mode: Option<WhitelistBindingMode>,
    #[serde(default)]
    pub notification_settings: Option<WhitelistNotificationSettings>,
    #[serde(default)]
    pub restrict_chat: Option<bool>,
    pub scope_type: TeamScopeType,
    pub server_id: String,
    #[serde(default)]
    pub target_server_ids: Option<Vec<String>>,
}

/// WhitelistConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistConfig {
    #[serde(default)]
    pub binding_server_id: Option<i64>,
    pub enabled: bool,
    pub enforcement_mode: EnforcementMode,
    #[serde(default)]
    pub entries: Option<Vec<WhitelistEntry>>,
    #[serde(default)]
    pub form_fields: Option<Vec<WhitelistFormField>>,
    #[serde(default)]
    pub form_id: Option<i64>,
    #[serde(default)]
    pub form_name: Option<String>,
    pub restrict_chat: bool,
    pub scope_type: TeamScopeType,
    pub server_id: i64,
}

/// WhitelistDecisionRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDecisionRequest {
    #[serde(default)]
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
    #[serde(default)]
    pub added_by_user_id: Option<i64>,
    pub created_at: String,
    pub id: i64,
    #[serde(default)]
    pub minecraft_account_type: Option<MinecraftAccountType>,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    pub server_id: i64,
}

/// WhitelistDirectEntryPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDirectEntryPage {
    #[serde(default)]
    pub items: Option<Vec<WhitelistDirectEntry>>,
    pub total: i64,
}

/// WhitelistDirectRemoval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistDirectRemoval {
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    pub removed: bool,
}

/// WhitelistEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistEntry {
    #[serde(default)]
    pub discord_name: Option<String>,
    #[serde(default)]
    pub minecraft_account_type: Option<MinecraftAccountType>,
    #[serde(default)]
    pub minecraft_nick: Option<String>,
    #[serde(default)]
    pub minecraft_uuid: Option<String>,
    #[serde(default)]
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhitelistFieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "unknown")]
    Unknown,
}

/// WhitelistFormCard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormCard {
    pub auto_approve_enabled: bool,
    #[serde(default)]
    pub description: Option<String>,
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub project_id: Option<i64>,
    pub require_discord: bool,
    pub require_minecraft_nick: bool,
}

/// WhitelistFormCreateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormCreateRequest {
    #[serde(default)]
    pub auto_approve_enabled: Option<bool>,
    #[serde(default)]
    pub auto_approve_rules: Option<serde_json::Value>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub fields: Option<serde_json::Value>,
    #[serde(default)]
    pub import_mapping: Option<serde_json::Value>,
    pub name: String,
    #[serde(default)]
    pub project_id: Option<i64>,
    #[serde(default)]
    pub require_discord: Option<bool>,
    #[serde(default)]
    pub require_minecraft_nick: Option<bool>,
}

/// WhitelistFormDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormDetail {
    pub auto_approve_rules: serde_json::Value,
    pub fields: serde_json::Value,
    pub import_mapping: serde_json::Value,
    pub summary: WhitelistFormCard,
}

/// WhitelistFormField
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormField {
    pub field_type: WhitelistFieldType,
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub order: Option<i64>,
    pub required: bool,
}

/// WhitelistFormImportMappingRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormImportMappingRequest {
    pub import_mapping: serde_json::Value,
}

/// WhitelistFormPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormPage {
    #[serde(default)]
    pub items: Option<Vec<WhitelistFormCard>>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

/// WhitelistFormUpdateRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistFormUpdateRequest {
    pub patch: serde_json::Value,
}

/// WhitelistImportJob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistImportJob {
    #[serde(default)]
    pub applications_added: Option<i64>,
    #[serde(default)]
    pub batches_expected: Option<i64>,
    #[serde(default)]
    pub batches_received: Option<i64>,
    pub binding_id: i64,
    #[serde(default)]
    pub completed_at: Option<String>,
    pub conflict_policy: String,
    #[serde(default)]
    pub conflicts: Option<i64>,
    pub created_at: String,
    #[serde(default)]
    pub entries_added: Option<i64>,
    #[serde(default)]
    pub entries_updated: Option<i64>,
    #[serde(default)]
    pub error_reason: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    pub id: i64,
    pub include_history: bool,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
    pub server_id: i64,
    pub source: String,
    #[serde(default)]
    pub started_at: Option<String>,
    pub status: String,
    pub token: String,
}

/// WhitelistImportJobPage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistImportJobPage {
    #[serde(default)]
    pub items: Option<Vec<WhitelistImportJob>>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub per_page: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
}

/// WhitelistImportRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistImportRequest {
    pub binding_id: i64,
    #[serde(default)]
    pub conflict_policy: Option<String>,
    #[serde(default)]
    pub dry_run: Option<bool>,
    #[serde(default)]
    pub expires_in_hours: Option<i64>,
    #[serde(default)]
    pub form_id: Option<String>,
    #[serde(default)]
    pub import_account_mode: Option<String>,
    #[serde(default)]
    pub include_history: Option<bool>,
    #[serde(default)]
    pub options: Option<serde_json::Value>,
    #[serde(default)]
    pub source: Option<String>,
}

/// WhitelistMinecraftPullRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistMinecraftPullRequest {
    #[serde(default)]
    pub batch_size: Option<i64>,
    pub binding_id: i64,
    #[serde(default)]
    pub conflict_policy: Option<String>,
    #[serde(default)]
    pub expires_in_hours: Option<i64>,
    #[serde(default)]
    pub form_id: Option<String>,
    #[serde(default)]
    pub import_account_mode: Option<String>,
    #[serde(default)]
    pub include_history: Option<bool>,
    #[serde(default)]
    pub source: Option<String>,
}

/// WhitelistNotificationChannelSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistNotificationChannelSettings {
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub content_template: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub statuses: Option<Vec<String>>,
    #[serde(default)]
    pub subject_template: Option<String>,
    #[serde(default)]
    pub target_scope: Option<String>,
}

/// WhitelistNotificationSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistNotificationSettings {
    #[serde(default)]
    pub applicant: Option<WhitelistApplicantNotificationSettings>,
    #[serde(default)]
    pub staff: Option<WhitelistStaffNotificationSettings>,
    #[serde(default)]
    pub version: Option<i64>,
}

/// WhitelistStaffNotificationSettings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistStaffNotificationSettings {
    #[serde(default)]
    pub discord_channel: Option<WhitelistNotificationChannelSettings>,
    #[serde(default)]
    pub discord_dm: Option<WhitelistNotificationChannelSettings>,
    #[serde(default)]
    pub email: Option<WhitelistNotificationChannelSettings>,
    #[serde(default)]
    pub minecraft_broadcast: Option<WhitelistNotificationChannelSettings>,
    #[serde(default)]
    pub telegram: Option<WhitelistNotificationChannelSettings>,
}

/// WhitelistStatusRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistStatusRequest {
    #[serde(default)]
    pub reason: Option<String>,
    pub status: String,
}

/// Workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(default)]
    pub can_edit_draft: Option<bool>,
    #[serde(default)]
    pub can_manage: Option<bool>,
    #[serde(default)]
    pub can_verify: Option<bool>,
    pub catalog_mode: String,
    #[serde(default)]
    pub creator_user_id: Option<i64>,
    #[serde(default)]
    pub display_server: Option<ServerSummary>,
    #[serde(default)]
    pub display_server_id: Option<i64>,
    #[serde(default)]
    pub freshness_state: Option<FreshnessState>,
    #[serde(default)]
    pub game_editions: Option<Vec<String>>,
    #[serde(default)]
    pub hearts: Option<i64>,
    pub id: i64,
    #[serde(default)]
    pub last_status_at: Option<String>,
    #[serde(default)]
    pub lifecycle_state: Option<LifecycleState>,
    #[serde(default)]
    pub max_players: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub online_players: Option<i64>,
    #[serde(default)]
    pub online_reason: Option<String>,
    #[serde(default)]
    pub online_server_id: Option<i64>,
    #[serde(default)]
    pub online_source: Option<TrustState>,
    #[serde(default)]
    pub online_state: Option<OnlineState>,
    pub online_strategy: OnlineStrategy,
    #[serde(default)]
    pub owner_id: Option<i64>,
    #[serde(default)]
    pub public_entrypoint: Option<String>,
    #[serde(default)]
    pub public_entrypoint_count: Option<i64>,
    #[serde(default)]
    pub public_entrypoint_state: Option<PublicEntrypointState>,
    #[serde(default)]
    pub public_server_count: Option<i64>,
    #[serde(default)]
    pub regions: Option<Vec<String>>,
    #[serde(default)]
    pub root_server_id: Option<i64>,
    #[serde(default)]
    pub server_count: Option<i64>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub trust_state: Option<TrustState>,
    #[serde(default)]
    pub user_permissions: Option<Vec<String>>,
    #[serde(default)]
    pub verified_plugin_rollout_mode: Option<RolloutMode>,
    #[serde(default)]
    pub verified_plugin_rollout_state: Option<RolloutState>,
    #[serde(default)]
    pub verified_server_count: Option<i64>,
    #[serde(default)]
    pub votes_monthly: Option<i64>,
}

/// WorkspaceChangeSlugRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceChangeSlugRequest {
    #[serde(default)]
    pub slug: Option<String>,
}

/// WorkspaceDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDetail {
    #[serde(default)]
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
    #[serde(default)]
    pub slug: Option<String>,
    pub workspace_id: i64,
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
    #[serde(default)]
    pub scope: Option<Vec<String>>,
}

/// WsTokenResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsTokenResponse {
    pub audience: String,
    pub expires_in: i64,
    #[serde(default)]
    pub scope: Option<Vec<String>>,
    pub session_id: String,
    #[serde(default)]
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
    #[serde(default)]
    pub sync: Option<serde_json::Value>,
}

/// ProfilePrivacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeProfilePrivacy {
    #[serde(default)]
    pub show_activity_stats: Option<bool>,
    #[serde(default)]
    pub show_bio: Option<bool>,
    #[serde(default)]
    pub show_join_date: Option<bool>,
    #[serde(default)]
    pub show_linked_accounts: Option<bool>,
    #[serde(default)]
    pub show_ownership: Option<bool>,
    #[serde(default)]
    pub show_status: Option<bool>,
    #[serde(default)]
    pub show_streak: Option<bool>,
    #[serde(default)]
    pub show_top_server: Option<bool>,
    #[serde(default)]
    pub show_user_id: Option<bool>,
}

/// ProfilePrivacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersProfilePrivacy {
    #[serde(default)]
    pub hide_discord: Option<bool>,
    #[serde(default)]
    pub hide_email: Option<bool>,
    #[serde(default)]
    pub hide_minecraft_accounts: Option<bool>,
    #[serde(default)]
    pub hide_ownership: Option<bool>,
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
    pub project_id: Option<String>,
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

/// Query parameters for `me.projects.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeProjectsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// Query parameters for `project.history.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectHistoryListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Query parameters for `project.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
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
    pub role_id: Option<String>,
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
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_server_id: Option<i64>,
}

/// Query parameters for `server.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
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
    pub role_id: Option<String>,
}

/// Query parameters for `server.telemetry`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerTelemetryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
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

/// Query parameters for `user.activity.list`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserActivityListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Query parameters for `user.heatmap`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserHeatmapParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}

/// Query parameters for `updates.manifest`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatesManifestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
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
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_sources: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_online: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discord_members: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

/// Query parameters for `admin.discovery.approve`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminDiscoveryApproveParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
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

/// Query parameters for `projects.stats`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectsStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
}

/// Query parameters for `stats.filter`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatsFilterParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
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
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}
