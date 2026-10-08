use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketNotificationUnknown {
    #[serde(flatten)]
    pub notification_base: models::NotificationBase,
    #[serde(
        rename = "details",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub details: Option<Option<serde_json::Value>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl WebsocketNotificationUnknown {
    pub fn new(
        notification_base: models::NotificationBase,
        r#type: String,
    ) -> WebsocketNotificationUnknown {
        WebsocketNotificationUnknown {
            notification_base,
            details: None,
            r#type,
        }
    }
}
