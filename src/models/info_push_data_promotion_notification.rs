use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataPromotionNotification {
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "command")]
    pub command: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
}

impl InfoPushDataPromotionNotification {
    pub fn new(
        body: String,
        command: String,
        image_url: String,
        title: String,
    ) -> InfoPushDataPromotionNotification {
        InfoPushDataPromotionNotification {
            body,
            command,
            image_url,
            parameter: None,
            title,
        }
    }
}
