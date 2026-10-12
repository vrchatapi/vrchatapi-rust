use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationV2GroupAnnouncement {
    #[serde(flatten)]
    pub notification_v2_base: models::NotificationV2Base,
    #[serde(rename = "data")]
    pub data: models::NotificationV2DataGroupAnnouncement,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationV2GroupAnnouncement {
    pub fn new(
        notification_v2_base: models::NotificationV2Base,
        data: models::NotificationV2DataGroupAnnouncement,
        r#type: Type,
    ) -> NotificationV2GroupAnnouncement {
        NotificationV2GroupAnnouncement {
            notification_v2_base,
            data,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group.announcement")]
    GroupAnnouncement,
}

impl Default for Type {
    fn default() -> Type {
        Self::GroupAnnouncement
    }
}
