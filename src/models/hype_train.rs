use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypeTrain {
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    #[serde(rename = "cooldownSeconds")]
    pub cooldown_seconds: i32,
    #[serde(rename = "currentGiftCount")]
    pub current_gift_count: i32,
    #[serde(rename = "currentLevel")]
    pub current_level: i32,
    #[serde(rename = "endDate")]
    pub end_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "hypeTrainId")]
    pub hype_train_id: String,
    #[serde(rename = "levels")]
    pub levels: Vec<models::HypeTrainLevel>,
    #[serde(rename = "startDate")]
    pub start_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "totalGiftContributors")]
    pub total_gift_contributors: i32,
    #[serde(rename = "totalGiftGoal")]
    pub total_gift_goal: i32,
    #[serde(rename = "totalGiftsFromYouCount")]
    pub total_gifts_from_you_count: i32,
}

impl HypeTrain {
    pub fn new(
        campaign_id: String,
        cooldown_seconds: i32,
        current_gift_count: i32,
        current_level: i32,
        end_date: chrono::DateTime<chrono::FixedOffset>,
        hype_train_id: String,
        levels: Vec<models::HypeTrainLevel>,
        start_date: chrono::DateTime<chrono::FixedOffset>,
        total_gift_contributors: i32,
        total_gift_goal: i32,
        total_gifts_from_you_count: i32,
    ) -> HypeTrain {
        HypeTrain {
            campaign_id,
            cooldown_seconds,
            current_gift_count,
            current_level,
            end_date,
            hype_train_id,
            levels,
            start_date,
            total_gift_contributors,
            total_gift_goal,
            total_gifts_from_you_count,
        }
    }
}
