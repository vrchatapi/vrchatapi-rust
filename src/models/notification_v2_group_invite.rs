use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationV2GroupInvite {
    #[serde(flatten)]
    pub notification_v2_base: models::NotificationV2Base,
    #[serde(rename = "data")]
    pub data: models::NotificationV2DataGroupInvite,
    #[serde(rename = "type", default)]
    pub r#type: Type,
}

impl NotificationV2GroupInvite {
    pub fn new(
        notification_v2_base: models::NotificationV2Base,
        data: models::NotificationV2DataGroupInvite,
        r#type: Type,
    ) -> NotificationV2GroupInvite {
        NotificationV2GroupInvite {
            notification_v2_base,
            data,
            r#type,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group.invite")]
    GroupInvite,
}

impl Default for Type {
    fn default() -> Type {
        Self::GroupInvite
    }
}
