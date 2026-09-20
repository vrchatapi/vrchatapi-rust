use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataDeliveryBehavior {
    #[serde(rename = "bypass24HourWindow", skip_serializing_if = "Option::is_none")]
    pub bypass24_hour_window: Option<bool>,
    #[serde(
        rename = "maxRedeliveryAttempts",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_redelivery_attempts: Option<i32>,
    #[serde(
        rename = "redeliverIfNoEngagement",
        skip_serializing_if = "Option::is_none"
    )]
    pub redeliver_if_no_engagement: Option<bool>,
}

impl InfoPushDataDeliveryBehavior {
    pub fn new() -> InfoPushDataDeliveryBehavior {
        InfoPushDataDeliveryBehavior {
            bypass24_hour_window: None,
            max_redelivery_attempts: None,
            redeliver_if_no_engagement: None,
        }
    }
}
