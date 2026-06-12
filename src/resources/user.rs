// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// User resource.
#[derive(Clone)]
pub struct User {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl User {
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

    /// Re-fetch this User and hydrate in place.
    pub async fn refresh(&self) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Get,
                &format!("/v1/users/{}/public-profile", self.id()),
                Channel::PlatformPublic,
                None,
            )
            .await?;
        let _: User = self.client.hydrate("User", data, None);
        Ok(())
    }

    /// Whether the current user may report (RFC §4).
    pub fn can_report(&self) -> bool {
        resource::has_capability(&self.data, "user.report")
    }

    /// user.report
    pub async fn report(
        &self,
        user_id: String,
        body: models::ReportUserRequest,
    ) -> Result<models::ReportUserResult, TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!("/v1/community/users/{}/report", user_id),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        serde_json::from_value(data).map_err(|e| TransportError::Transport(e.into()))
    }

    /// user.heatmap
    pub async fn heatmap(
        &self,
        params: models::UserHeatmapParams,
    ) -> Result<models::ProfileActivityHeatmap, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &resource::with_query(
                &format!("/v1/users/{}/profile/activity", self.id()),
                &[("days", params.days.map(|v| v.to_string()))],
            ),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// user.gameplay
    pub async fn gameplay(&self) -> Result<models::ProfileGameplaySummary, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/users/{}/profile/gameplay", self.id()),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }

    /// user.ownership
    pub async fn ownership(&self) -> Result<models::ProfileOwnershipSummary, TransportError> {
        let value = crate::etag_store::fetch_cached_or_throw(
            self.client.transport(),
            self.client.etag_store(),
            Method::Get,
            &format!("/v1/users/{}/profile/ownership", self.id()),
            Channel::PlatformPublic,
        )
        .await?;
        serde_json::from_value(value).map_err(|e| TransportError::Transport(e.into()))
    }
}
