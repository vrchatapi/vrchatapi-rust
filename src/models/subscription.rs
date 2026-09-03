use crate::models;
use serde::{Deserialize, Serialize};

/// Subscription :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "appleProductId", skip_serializing_if = "Option::is_none")]
    pub apple_product_id: Option<String>,
    /// How many subscriptions a gifted bundle grants.
    #[serde(rename = "bulkSize", skip_serializing_if = "Option::is_none")]
    pub bulk_size: Option<i32>,
    #[serde(rename = "description")]
    pub description: String,
    /// Discount applied to a gifted bundle.
    #[serde(rename = "discountPercentage", skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<i32>,
    #[serde(rename = "googlePlanId", skip_serializing_if = "Option::is_none")]
    pub google_plan_id: Option<String>,
    #[serde(rename = "googleProductId", skip_serializing_if = "Option::is_none")]
    pub google_product_id: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "oculusSku", skip_serializing_if = "Option::is_none")]
    pub oculus_sku: Option<String>,
    #[serde(rename = "period")]
    pub period: models::SubscriptionPeriod,
    #[serde(
        rename = "periodAmount",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub period_amount: Option<Option<serde_json::Value>>,
    #[serde(rename = "picoSku", skip_serializing_if = "Option::is_none")]
    pub pico_sku: Option<String>,
    #[serde(rename = "steamItemId")]
    pub steam_item_id: String,
    #[serde(rename = "tier")]
    pub tier: i32,
}

impl Subscription {
    pub fn new(
        amount: f64,
        description: String,
        id: String,
        period: models::SubscriptionPeriod,
        steam_item_id: String,
        tier: i32,
    ) -> Subscription {
        Subscription {
            amount,
            apple_product_id: None,
            bulk_size: None,
            description,
            discount_percentage: None,
            google_plan_id: None,
            google_product_id: None,
            id,
            oculus_sku: None,
            period,
            period_amount: None,
            pico_sku: None,
            steam_item_id,
            tier,
        }
    }
}
