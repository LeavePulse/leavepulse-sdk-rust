// Generated from the LeavePulse contract. Do not edit.
use std::sync::Arc;

use serde_json::Value;

use crate::cache::DataCell;
use crate::client::LeavePulse;
use crate::models;
use crate::resource;
use crate::transport::{Channel, Method, TransportError};

/// Comment resource.
#[derive(Clone)]
pub struct Comment {
    data: DataCell,
    client: Arc<LeavePulse>,
}

impl Comment {
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

    /// Whether the current user may delete (RFC §4).
    pub fn can_delete(&self) -> bool {
        resource::has_capability(&self.data, "comment.delete")
    }

    /// Whether the current user may like (RFC §4).
    pub fn can_like(&self) -> bool {
        resource::has_capability(&self.data, "comment.like")
    }

    /// comment.delete
    pub async fn delete(&self, project_id: String) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Delete,
                &format!(
                    "/v1/community/projects/{}/comments/{}",
                    project_id,
                    self.id()
                ),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Comment = self.client.hydrate("Comment", data, None);
        Ok(())
    }

    /// comment.like
    pub async fn like(&self, project_id: String) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &format!(
                    "/v1/community/projects/{}/comments/{}/like",
                    project_id,
                    self.id()
                ),
                Channel::Platform,
                None,
            )
            .await?;
        let _: Comment = self.client.hydrate("Comment", data, None);
        Ok(())
    }

    /// comment.reply
    pub async fn reply(
        &self,
        project_id: String,
        body: models::CommentCreateRequest,
        params: models::CommentReplyParams,
    ) -> Result<(), TransportError> {
        let data = self
            .client
            .transport()
            .request(
                Method::Post,
                &resource::with_query(
                    &format!(
                        "/v1/community/projects/{}/comments/{}/replies",
                        project_id,
                        self.id()
                    ),
                    &[("target_locale", params.target_locale.map(|v| v.to_string()))],
                ),
                Channel::Platform,
                Some(serde_json::to_value(body).map_err(|e| TransportError::Transport(e.into()))?),
            )
            .await?;
        let _: Comment = self.client.hydrate("Comment", data, None);
        Ok(())
    }
}
