// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::{DataCell, IdentityMap};
use crate::etag_store::{EtagStore, MemoryEtagStore};
use crate::procedures::AdminNs;
use crate::procedures::AuthNs;
use crate::procedures::BillingNs;
use crate::procedures::BuildsNs;
use crate::procedures::DiscordNs;
use crate::procedures::MonitoringNs;
use crate::procedures::PasswordNs;
use crate::procedures::ProjectsNs;
use crate::procedures::RbacNs;
use crate::procedures::ServersNs;
use crate::procedures::StatsNs;
use crate::procedures::TicketsNs;
use crate::procedures::UpdatesNs;
use crate::procedures::UsersNs;
use crate::procedures::VerificationNs;
use crate::procedures::WhitelistNs;
use crate::resource;
use crate::resources::Application;
use crate::resources::Binding;
use crate::resources::Build;
use crate::resources::Comment;
use crate::resources::Form;
use crate::resources::Me;
use crate::resources::Order;
use crate::resources::Product;
use crate::resources::Project;
use crate::resources::Server;
use crate::resources::Session;
use crate::resources::Subscription;
use crate::resources::Ticket;
use crate::resources::User;
use crate::transport::{Channel, Method, Transport, TransportError};

/// A resource that can be built from a shared data cell + client.
pub trait FromCell {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self;
    fn type_name() -> &'static str;
}

impl FromCell for Application {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Application::new(data, client)
    }
    fn type_name() -> &'static str {
        "Application"
    }
}

impl FromCell for Binding {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Binding::new(data, client)
    }
    fn type_name() -> &'static str {
        "Binding"
    }
}

impl FromCell for Build {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Build::new(data, client)
    }
    fn type_name() -> &'static str {
        "Build"
    }
}

impl FromCell for Comment {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Comment::new(data, client)
    }
    fn type_name() -> &'static str {
        "Comment"
    }
}

impl FromCell for Form {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Form::new(data, client)
    }
    fn type_name() -> &'static str {
        "Form"
    }
}

impl FromCell for Me {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Me::new(data, client)
    }
    fn type_name() -> &'static str {
        "Me"
    }
}

impl FromCell for Order {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Order::new(data, client)
    }
    fn type_name() -> &'static str {
        "Order"
    }
}

impl FromCell for Product {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Product::new(data, client)
    }
    fn type_name() -> &'static str {
        "Product"
    }
}

impl FromCell for Project {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Project::new(data, client)
    }
    fn type_name() -> &'static str {
        "Project"
    }
}

impl FromCell for Server {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Server::new(data, client)
    }
    fn type_name() -> &'static str {
        "Server"
    }
}

impl FromCell for Session {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Session::new(data, client)
    }
    fn type_name() -> &'static str {
        "Session"
    }
}

impl FromCell for Subscription {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Subscription::new(data, client)
    }
    fn type_name() -> &'static str {
        "Subscription"
    }
}

impl FromCell for Ticket {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        Ticket::new(data, client)
    }
    fn type_name() -> &'static str {
        "Ticket"
    }
}

impl FromCell for User {
    fn from_cell(data: DataCell, client: Arc<LeavePulse>) -> Self {
        User::new(data, client)
    }
    fn type_name() -> &'static str {
        "User"
    }
}

/// The LeavePulse SDK client.
pub struct LeavePulse {
    transport: Box<dyn Transport>,
    etag_store: Arc<dyn EtagStore>,
    cache: IdentityMap,
}

impl LeavePulse {
    /// Build a client with the default in-memory ETag store.
    pub fn new(transport: Box<dyn Transport>) -> Arc<Self> {
        Self::with_etag_store(transport, Arc::new(MemoryEtagStore::new()))
    }

    /// Build a client with a custom ETag store (e.g. a persistent
    /// `FileEtagStore` shared across clients so 304s survive restarts).
    pub fn with_etag_store(
        transport: Box<dyn Transport>,
        etag_store: Arc<dyn EtagStore>,
    ) -> Arc<Self> {
        Arc::new(Self {
            transport,
            etag_store,
            cache: IdentityMap::new(),
        })
    }

    pub(crate) fn transport(&self) -> &dyn Transport {
        self.transport.as_ref()
    }

    pub(crate) fn etag_store(&self) -> &dyn EtagStore {
        self.etag_store.as_ref()
    }

    pub fn admin(self: &Arc<Self>) -> AdminNs {
        AdminNs::new(Arc::clone(self))
    }

    pub fn auth(self: &Arc<Self>) -> AuthNs {
        AuthNs::new(Arc::clone(self))
    }

    pub fn billing(self: &Arc<Self>) -> BillingNs {
        BillingNs::new(Arc::clone(self))
    }

    pub fn builds(self: &Arc<Self>) -> BuildsNs {
        BuildsNs::new(Arc::clone(self))
    }

    pub fn discord(self: &Arc<Self>) -> DiscordNs {
        DiscordNs::new(Arc::clone(self))
    }

    pub fn monitoring(self: &Arc<Self>) -> MonitoringNs {
        MonitoringNs::new(Arc::clone(self))
    }

    pub fn password(self: &Arc<Self>) -> PasswordNs {
        PasswordNs::new(Arc::clone(self))
    }

    pub fn projects(self: &Arc<Self>) -> ProjectsNs {
        ProjectsNs::new(Arc::clone(self))
    }

    pub fn rbac(self: &Arc<Self>) -> RbacNs {
        RbacNs::new(Arc::clone(self))
    }

    pub fn servers(self: &Arc<Self>) -> ServersNs {
        ServersNs::new(Arc::clone(self))
    }

    pub fn stats(self: &Arc<Self>) -> StatsNs {
        StatsNs::new(Arc::clone(self))
    }

    pub fn tickets(self: &Arc<Self>) -> TicketsNs {
        TicketsNs::new(Arc::clone(self))
    }

    pub fn updates(self: &Arc<Self>) -> UpdatesNs {
        UpdatesNs::new(Arc::clone(self))
    }

    pub fn users(self: &Arc<Self>) -> UsersNs {
        UsersNs::new(Arc::clone(self))
    }

    pub fn verification(self: &Arc<Self>) -> VerificationNs {
        VerificationNs::new(Arc::clone(self))
    }

    pub fn whitelist(self: &Arc<Self>) -> WhitelistNs {
        WhitelistNs::new(Arc::clone(self))
    }

    pub async fn build(self: &Arc<Self>, id: i64) -> Result<Build, TransportError> {
        let data = self
            .transport
            .request(
                Method::Get,
                &format!("/v1/builds/{}", id),
                Channel::Platform,
                None,
            )
            .await?;
        Ok(self.hydrate::<Build>("Build", data, None))
    }

    pub async fn form(self: &Arc<Self>, id: i64) -> Result<Form, TransportError> {
        let data = self
            .transport
            .request(
                Method::Get,
                &format!("/v1/whitelist/forms/{}", id),
                Channel::Platform,
                None,
            )
            .await?;
        Ok(self.hydrate::<Form>("Form", data, None))
    }

    pub async fn me(self: &Arc<Self>) -> Result<Me, TransportError> {
        let data = self
            .transport
            .request(Method::Get, "/v1/me", Channel::Platform, None)
            .await?;
        Ok(self.hydrate::<Me>("Me", data, None))
    }

    pub async fn order(self: &Arc<Self>, id: i64) -> Result<Order, TransportError> {
        let data = self
            .transport
            .request(
                Method::Get,
                &format!("/v1/billing/orders/{}", id),
                Channel::Platform,
                None,
            )
            .await?;
        Ok(self.hydrate::<Order>("Order", data, None))
    }

    pub async fn project(self: &Arc<Self>, id: i64) -> Result<Project, TransportError> {
        let data = self
            .transport
            .request(
                Method::Get,
                &format!("/v1/projects/{}", id),
                Channel::Platform,
                None,
            )
            .await?;
        Ok(self.hydrate::<Project>("Project", data, Some("project")))
    }

    pub async fn server(self: &Arc<Self>, id: i64) -> Result<Server, TransportError> {
        let data = self
            .transport
            .request(
                Method::Get,
                &format!("/v1/servers/{}", id),
                Channel::Platform,
                None,
            )
            .await?;
        Ok(self.hydrate::<Server>("Server", data, None))
    }

    pub async fn user(self: &Arc<Self>, id: i64) -> Result<User, TransportError> {
        let data = self
            .transport
            .request(
                Method::Get,
                &format!("/v1/users/{}/public-profile", id),
                Channel::Platform,
                None,
            )
            .await?;
        Ok(self.hydrate::<User>("User", data, None))
    }

    pub(crate) fn hydrate<T: FromCell>(
        self: &Arc<Self>,
        type_: &str,
        payload: Value,
        data_root: Option<&str>,
    ) -> T {
        let data = resource::normalize(payload, data_root);
        let id = resource::extract_id(&data);
        let cell = self.cache.upsert(type_, &id, data);
        T::from_cell(cell, Arc::clone(self))
    }

    pub(crate) fn hydrate_many<T: FromCell>(self: &Arc<Self>, type_: &str, raw: Value) -> Vec<T> {
        match raw {
            Value::Array(items) => items
                .into_iter()
                .map(|item| self.hydrate::<T>(type_, item, None))
                .collect(),
            _ => Vec::new(),
        }
    }
}
