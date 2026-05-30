// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::client::LeavePulse;
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
    pub async fn candidates(&self, page: Option<i64>, limit: Option<i64>, status: Option<String>, search: Option<String>, source: Option<String>, edition: Option<String>, region: Option<String>, min_sources: Option<String>, min_mc_online: Option<String>, min_discord_members: Option<String>, sort: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/admin/discovery/candidates".to_string(), &[("page", page.map(|v| v.to_string())), ("limit", limit.map(|v| v.to_string())), ("status", status.map(|v| v.to_string())), ("search", search.map(|v| v.to_string())), ("source", source.map(|v| v.to_string())), ("edition", edition.map(|v| v.to_string())), ("region", region.map(|v| v.to_string())), ("min_sources", min_sources.map(|v| v.to_string())), ("min_mc_online", min_mc_online.map(|v| v.to_string())), ("min_discord_members", min_discord_members.map(|v| v.to_string())), ("sort", sort.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.discovery.edit
    pub async fn edit(&self, candidate_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Patch, &format!("/v1/admin/discovery/candidates/{}", candidate_id), Channel::Platform, Some(body)).await
    }

    /// admin.discovery.approve
    pub async fn approve(&self, candidate_id: i64, show_in_public: Option<bool>, server_id: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &resource::with_query(&format!("/v1/admin/discovery/candidates/{}/actions/approve", candidate_id), &[("show_in_public", show_in_public.map(|v| v.to_string())), ("server_id", server_id.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.discovery.ignore
    pub async fn ignore(&self, candidate_id: i64, reason: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &resource::with_query(&format!("/v1/admin/discovery/candidates/{}/actions/ignore", candidate_id), &[("reason", reason.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.discovery.observations
    pub async fn observations(&self, candidate_id: i64, limit: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&format!("/v1/admin/discovery/candidates/{}/observations", candidate_id), &[("limit", limit.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.discovery.preview
    pub async fn preview(&self, candidate_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/admin/discovery/candidates/{}/preview", candidate_id), Channel::Platform, None).await
    }

    /// admin.discovery.sources
    pub async fn sources(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/admin/discovery/sources".to_string(), Channel::Platform, None).await
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
    pub async fn list(&self, server_id: i64, start: Option<String>, end: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&format!("/v1/admin/servers/{}/status-overrides", server_id), &[("start", start.map(|v| v.to_string())), ("end", end.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.overrides.create
    pub async fn create(&self, server_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/servers/{}/status-overrides", server_id), Channel::Platform, Some(body)).await
    }

    /// admin.overrides.delete
    pub async fn delete(&self, server_id: i64, override_id: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Delete, &format!("/v1/admin/servers/{}/status-overrides/{}", server_id, override_id), Channel::Platform, None).await
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
    pub async fn search(&self, q: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/admin/players".to_string(), &[("q", q.map(|v| v.to_string())), ("page", page.map(|v| v.to_string())), ("per_page", per_page.map(|v| v.to_string()))]), Channel::Platform, None).await
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
    pub async fn list(&self, q: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/admin/projects".to_string(), &[("q", q.map(|v| v.to_string())), ("page", page.map(|v| v.to_string())), ("per_page", per_page.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.projects.delete
    pub async fn delete(&self, project_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Delete, &format!("/v1/admin/projects/{}", project_id), Channel::Platform, None).await
    }

    /// admin.projects.change_slug
    pub async fn change_slug(&self, project_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/projects/{}/actions/change-slug", project_id), Channel::Platform, Some(body)).await
    }

    /// admin.projects.rename
    pub async fn rename(&self, project_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/projects/{}/actions/rename", project_id), Channel::Platform, Some(body)).await
    }

    /// admin.projects.set_online_strategy
    pub async fn set_online_strategy(&self, project_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/projects/{}/actions/set-online-strategy", project_id), Channel::Platform, Some(body)).await
    }

    /// admin.projects.set_rollout_mode
    pub async fn set_rollout_mode(&self, project_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/projects/{}/actions/set-rollout-mode", project_id), Channel::Platform, Some(body)).await
    }

    /// admin.projects.transfer_ownership
    pub async fn transfer_ownership(&self, project_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/projects/{}/actions/transfer-ownership", project_id), Channel::Platform, Some(body)).await
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
    pub async fn list(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/admin/roles".to_string(), Channel::Platform, None).await
    }

    /// admin.roles.create
    pub async fn create(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/admin/roles".to_string(), Channel::Platform, Some(body)).await
    }

    /// admin.roles.delete
    pub async fn delete(&self, role_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Delete, &format!("/v1/admin/roles/{}", role_id), Channel::Platform, None).await
    }

    /// admin.roles.update
    pub async fn update(&self, role_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Patch, &format!("/v1/admin/roles/{}", role_id), Channel::Platform, Some(body)).await
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
    pub async fn list(&self, page: Option<i64>, per_page: Option<i64>, q: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/admin/servers".to_string(), &[("page", page.map(|v| v.to_string())), ("per_page", per_page.map(|v| v.to_string())), ("q", q.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.servers.create
    pub async fn create(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/admin/servers".to_string(), Channel::Platform, Some(body)).await
    }

    /// admin.servers.stats
    pub async fn stats(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/admin/servers/stats".to_string(), Channel::Platform, None).await
    }

    /// admin.servers.delete
    pub async fn delete(&self, server_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Delete, &format!("/v1/admin/servers/{}", server_id), Channel::Platform, None).await
    }

    /// admin.servers.update
    pub async fn update(&self, server_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Patch, &format!("/v1/admin/servers/{}", server_id), Channel::Platform, Some(body)).await
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
    pub async fn revoke(&self, session_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/sessions/{}/actions/revoke", session_id), Channel::Platform, None).await
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
    pub async fn health(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/admin/system/services-health".to_string(), Channel::Platform, None).await
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
    pub async fn list(&self, page: Option<i64>, per_page: Option<i64>, q: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/admin/users".to_string(), &[("page", page.map(|v| v.to_string())), ("per_page", per_page.map(|v| v.to_string())), ("q", q.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.users.by_minecraft
    pub async fn by_minecraft(&self, uuid: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/admin/users/by-minecraft-uuid/{}", uuid), Channel::Platform, None).await
    }

    /// admin.users.search
    pub async fn search(&self, q: Option<String>, limit: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/admin/users/search".to_string(), &[("q", q.map(|v| v.to_string())), ("limit", limit.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// admin.users.get
    pub async fn get(&self, user_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/admin/users/{}", user_id), Channel::Platform, None).await
    }

    /// admin.users.update
    pub async fn update(&self, user_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Patch, &format!("/v1/admin/users/{}", user_id), Channel::Platform, Some(body)).await
    }

    /// admin.users.set_discord
    pub async fn set_discord(&self, user_id: i64, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Patch, &format!("/v1/admin/users/{}/discord", user_id), Channel::Platform, Some(body)).await
    }

    /// admin.users.roles
    pub async fn roles(&self, user_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/admin/users/{}/roles", user_id), Channel::Platform, None).await
    }

    /// admin.users.remove_role
    pub async fn remove_role(&self, user_id: i64, role_slug: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Delete, &format!("/v1/admin/users/{}/roles/{}", user_id, role_slug), Channel::Platform, None).await
    }

    /// admin.users.assign_role
    pub async fn assign_role(&self, user_id: i64, role_slug: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/v1/admin/users/{}/roles/{}", user_id, role_slug), Channel::Platform, None).await
    }

    /// admin.users.sessions
    pub async fn sessions(&self, user_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/admin/users/{}/sessions", user_id), Channel::Platform, None).await
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

    pub fn discovery(&self) -> AdminDiscoveryNs { AdminDiscoveryNs::new(Arc::clone(&self.client)) }

    pub fn overrides(&self) -> AdminOverridesNs { AdminOverridesNs::new(Arc::clone(&self.client)) }

    pub fn players(&self) -> AdminPlayersNs { AdminPlayersNs::new(Arc::clone(&self.client)) }

    pub fn projects(&self) -> AdminProjectsNs { AdminProjectsNs::new(Arc::clone(&self.client)) }

    pub fn roles(&self) -> AdminRolesNs { AdminRolesNs::new(Arc::clone(&self.client)) }

    pub fn servers(&self) -> AdminServersNs { AdminServersNs::new(Arc::clone(&self.client)) }

    pub fn sessions(&self) -> AdminSessionsNs { AdminSessionsNs::new(Arc::clone(&self.client)) }

    pub fn system(&self) -> AdminSystemNs { AdminSystemNs::new(Arc::clone(&self.client)) }

    pub fn users(&self) -> AdminUsersNs { AdminUsersNs::new(Arc::clone(&self.client)) }
}

/// AuthOauthNs procedure namespace.
pub struct AuthOauthNs {
    client: Arc<LeavePulse>,
}

impl AuthOauthNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// auth.oauth.callback
    pub async fn callback(&self, provider: String, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/auth/oauth/{}/callback", provider), Channel::Auth, Some(body)).await
    }

    /// auth.oauth.start
    pub async fn start(&self, provider: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/auth/oauth/{}/start", provider), Channel::Auth, None).await
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

    pub fn oauth(&self) -> AuthOauthNs { AuthOauthNs::new(Arc::clone(&self.client)) }

    /// auth.login
    pub async fn login(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/auth/login".to_string(), Channel::Auth, Some(body)).await
    }

    /// auth.logout
    pub async fn logout(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/auth/logout".to_string(), Channel::Auth, None).await
    }

    /// auth.refresh
    pub async fn refresh(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/auth/refresh".to_string(), Channel::Auth, Some(body)).await
    }

    /// auth.register
    pub async fn register(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/auth/register".to_string(), Channel::Auth, Some(body)).await
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
    pub async fn list(&self, page: Option<i64>, limit: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/billing/orders".to_string(), &[("page", page.map(|v| v.to_string())), ("limit", limit.map(|v| v.to_string()))]), Channel::Platform, None).await
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
    pub async fn list(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/billing/products".to_string(), Channel::Platform, None).await
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
    pub async fn list(&self, page: Option<i64>, limit: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/billing/subscriptions".to_string(), &[("page", page.map(|v| v.to_string())), ("limit", limit.map(|v| v.to_string()))]), Channel::Platform, None).await
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

    pub fn orders(&self) -> BillingOrdersNs { BillingOrdersNs::new(Arc::clone(&self.client)) }

    pub fn products(&self) -> BillingProductsNs { BillingProductsNs::new(Arc::clone(&self.client)) }

    pub fn subscriptions(&self) -> BillingSubscriptionsNs { BillingSubscriptionsNs::new(Arc::clone(&self.client)) }

    /// billing.checkout
    pub async fn checkout(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/billing/checkout".to_string(), Channel::Platform, Some(body)).await
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
    pub async fn create(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/builds".to_string(), Channel::Platform, Some(body)).await
    }

    /// builds.import
    pub async fn import(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/builds/import".to_string(), Channel::Platform, Some(body)).await
    }

    /// builds.shared_with_me
    pub async fn shared_with_me(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/builds/shared-with-me".to_string(), Channel::Platform, None).await
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
    pub async fn complete(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/discord/link/complete".to_string(), Channel::Platform, Some(body)).await
    }

    /// discord.link.session
    pub async fn session(&self, state: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/discord/link/session".to_string(), &[("state", state.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// discord.link.token
    pub async fn token(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/discord/link/token".to_string(), Channel::Platform, Some(body)).await
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

    pub fn link(&self) -> DiscordLinkNs { DiscordLinkNs::new(Arc::clone(&self.client)) }
}

/// MonitoringNs procedure namespace.
pub struct MonitoringNs {
    client: Arc<LeavePulse>,
}

impl MonitoringNs {
    pub(crate) fn new(client: Arc<LeavePulse>) -> Self {
        Self { client }
    }

    /// monitoring.landing
    pub async fn landing(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/monitoring/landing".to_string(), Channel::Platform, None).await
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
    pub async fn reset_confirm(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/password/reset/confirm".to_string(), Channel::Platform, Some(body)).await
    }

    /// password.reset_request
    pub async fn reset_request(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/password/reset/request".to_string(), Channel::Platform, Some(body)).await
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
    pub async fn stats(&self, q: Option<String>, edition: Option<String>, access: Option<String>, features: Option<String>, region: Option<String>, hosting: Option<String>, verified: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/projects/stats".to_string(), &[("q", q.map(|v| v.to_string())), ("edition", edition.map(|v| v.to_string())), ("access", access.map(|v| v.to_string())), ("features", features.map(|v| v.to_string())), ("region", region.map(|v| v.to_string())), ("hosting", hosting.map(|v| v.to_string())), ("verified", verified.map(|v| v.to_string()))]), Channel::Platform, None).await
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
    pub async fn batch_resolve(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/rbac/batch-resolve".to_string(), Channel::Platform, Some(body)).await
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
    pub async fn resolve(&self, server_ref: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/servers/resolve/{}", server_ref), Channel::Platform, None).await
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
    pub async fn filter(&self, q: Option<String>, edition: Option<String>, access: Option<String>, features: Option<String>, region: Option<String>, hosting: Option<String>, verified: Option<String>, role: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/stats/filter".to_string(), &[("q", q.map(|v| v.to_string())), ("edition", edition.map(|v| v.to_string())), ("access", access.map(|v| v.to_string())), ("features", features.map(|v| v.to_string())), ("region", region.map(|v| v.to_string())), ("hosting", hosting.map(|v| v.to_string())), ("verified", verified.map(|v| v.to_string())), ("role", role.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// stats.live
    pub async fn live(&self) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &"/v1/stats/live".to_string(), Channel::Platform, None).await
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
    pub async fn create(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/community/tickets".to_string(), Channel::Platform, Some(body)).await
    }

    /// tickets.mine
    pub async fn mine(&self, page: Option<i64>, limit: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/community/tickets/my".to_string(), &[("page", page.map(|v| v.to_string())), ("limit", limit.map(|v| v.to_string()))]), Channel::Platform, None).await
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
    pub async fn manifest(&self, channel: Option<String>, platform: Option<String>, server_id: Option<String>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/updates/v1/launcher/manifest".to_string(), &[("channel", channel.map(|v| v.to_string())), ("platform", platform.map(|v| v.to_string())), ("server_id", server_id.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// updates.manifest_upsert
    pub async fn manifest_upsert(&self, channel: String, platform: String, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Put, &format!("/updates/v1/launcher/manifests/{}/{}", channel, platform), Channel::Platform, Some(body)).await
    }

    /// updates.manifest_delete
    pub async fn manifest_delete(&self, channel: String, platform: String) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &format!("/updates/v1/launcher/manifests/{}/{}/delete", channel, platform), Channel::Platform, None).await
    }

    /// updates.report
    pub async fn report(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/updates/v1/launcher/report".to_string(), Channel::Platform, Some(body)).await
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
    pub async fn batch(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/users/public-profiles".to_string(), Channel::Platform, Some(body)).await
    }

    /// users.search
    pub async fn search(&self, q: Option<String>, limit: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/users/search".to_string(), &[("q", q.map(|v| v.to_string())), ("limit", limit.map(|v| v.to_string()))]), Channel::Platform, None).await
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
    pub async fn start_dns(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/servers/verification/dns".to_string(), Channel::Platform, Some(body)).await
    }

    /// verification.check_dns
    pub async fn check_dns(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/servers/verification/dns/check".to_string(), Channel::Platform, Some(body)).await
    }

    /// verification.start_motd
    pub async fn start_motd(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/servers/verification/motd".to_string(), Channel::Platform, Some(body)).await
    }

    /// verification.check_motd
    pub async fn check_motd(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/servers/verification/motd/check".to_string(), Channel::Platform, Some(body)).await
    }

    /// verification.start_plugin
    pub async fn start_plugin(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/servers/verification/plugin".to_string(), Channel::Platform, Some(body)).await
    }

    /// verification.check_plugin
    pub async fn check_plugin(&self, server_id: i64) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &format!("/v1/servers/verification/plugin/{}", server_id), Channel::Platform, None).await
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
    pub async fn create(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/whitelist/bindings".to_string(), Channel::Platform, Some(body)).await
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
    pub async fn list(&self, project_id: Option<String>, search: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Get, &resource::with_query(&"/v1/whitelist/forms".to_string(), &[("project_id", project_id.map(|v| v.to_string())), ("search", search.map(|v| v.to_string())), ("page", page.map(|v| v.to_string())), ("per_page", per_page.map(|v| v.to_string()))]), Channel::Platform, None).await
    }

    /// whitelist.forms.create
    pub async fn create(&self, body: Value) -> Result<Value, TransportError> {
        self.client.transport().request(Method::Post, &"/v1/whitelist/forms".to_string(), Channel::Platform, Some(body)).await
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

    pub fn bindings(&self) -> WhitelistBindingsNs { WhitelistBindingsNs::new(Arc::clone(&self.client)) }

    pub fn forms(&self) -> WhitelistFormsNs { WhitelistFormsNs::new(Arc::clone(&self.client)) }
}
