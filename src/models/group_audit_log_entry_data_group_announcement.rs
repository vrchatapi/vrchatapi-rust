use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupAnnouncement {
    /// The ID of the announcement author.
    #[serde(rename = "authorId")]
    pub author_id: String,
    /// The image file ID attached to the announcement.
    #[serde(rename = "imageId", deserialize_with = "Option::deserialize")]
    pub image_id: Option<String>,
    /// Whether a notification was sent for this announcement.
    #[serde(rename = "sendNotification")]
    pub send_notification: bool,
    /// The text content of the announcement.
    #[serde(rename = "text")]
    pub text: String,
    /// The title of the announcement.
    #[serde(rename = "title")]
    pub title: String,
}

impl GroupAuditLogEntryDataGroupAnnouncement {
    pub fn new(
        author_id: String,
        image_id: Option<String>,
        send_notification: bool,
        text: String,
        title: String,
    ) -> GroupAuditLogEntryDataGroupAnnouncement {
        GroupAuditLogEntryDataGroupAnnouncement {
            author_id,
            image_id,
            send_notification,
            text,
            title,
        }
    }
}
