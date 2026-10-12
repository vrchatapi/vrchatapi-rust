use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupPostCreate {
    #[serde(flatten)]
    pub group_audit_log_data_group_post: models::GroupAuditLogDataGroupPost,
    /// The role IDs that can see the post.
    #[serde(rename = "roleIds", deserialize_with = "Option::deserialize")]
    pub role_ids: Option<Vec<String>>,
    /// Whether a notification was sent for this post.
    #[serde(rename = "sendNotification")]
    pub send_notification: bool,
}

impl GroupAuditLogDataGroupPostCreate {
    pub fn new(
        group_audit_log_data_group_post: models::GroupAuditLogDataGroupPost,
        role_ids: Option<Vec<String>>,
        send_notification: bool,
    ) -> GroupAuditLogDataGroupPostCreate {
        GroupAuditLogDataGroupPostCreate {
            group_audit_log_data_group_post,
            role_ids,
            send_notification,
        }
    }
}
