use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationV2DataEventAnnouncement {
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "ownerName")]
    pub owner_name: String,
    #[serde(rename = "title")]
    pub title: String,
}

impl NotificationV2DataEventAnnouncement {
    pub fn new(
        owner_id: String,
        owner_name: String,
        title: String,
    ) -> NotificationV2DataEventAnnouncement {
        NotificationV2DataEventAnnouncement {
            owner_id,
            owner_name,
            title,
        }
    }
}
