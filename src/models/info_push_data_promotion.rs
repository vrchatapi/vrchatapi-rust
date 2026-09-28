use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataPromotion {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "impressions")]
    pub impressions: i32,
    #[serde(rename = "notification")]
    pub notification: models::InfoPushDataPromotionNotification,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl InfoPushDataPromotion {
    pub fn new(
        id: String,
        impressions: i32,
        notification: models::InfoPushDataPromotionNotification,
        r#type: String,
    ) -> InfoPushDataPromotion {
        InfoPushDataPromotion {
            id,
            impressions,
            notification,
            r#type,
        }
    }
}
