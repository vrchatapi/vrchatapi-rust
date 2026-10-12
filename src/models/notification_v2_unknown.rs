use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationV2Unknown {
    #[serde(flatten)]
    pub notification_v2_base: models::NotificationV2Base,
    #[serde(rename = "data")]
    pub data: models::NotificationV2Data,
    #[serde(rename = "type")]
    pub r#type: models::NotificationV2Type,
}

impl NotificationV2Unknown {
    pub fn new(
        notification_v2_base: models::NotificationV2Base,
        data: models::NotificationV2Data,
        r#type: models::NotificationV2Type,
    ) -> NotificationV2Unknown {
        NotificationV2Unknown {
            notification_v2_base,
            data,
            r#type,
        }
    }
}
