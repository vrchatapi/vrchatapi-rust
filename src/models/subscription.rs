/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Subscription :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "steamItemId")]
    pub steam_item_id: String,
    #[serde(rename = "oculusSku", skip_serializing_if = "Option::is_none")]
    pub oculus_sku: Option<String>,
    #[serde(rename = "googleProductId", skip_serializing_if = "Option::is_none")]
    pub google_product_id: Option<String>,
    #[serde(rename = "googlePlanId", skip_serializing_if = "Option::is_none")]
    pub google_plan_id: Option<String>,
    #[serde(rename = "picoSku", skip_serializing_if = "Option::is_none")]
    pub pico_sku: Option<String>,
    #[serde(rename = "appleProductId", skip_serializing_if = "Option::is_none")]
    pub apple_product_id: Option<String>,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "period")]
    pub period: models::SubscriptionPeriod,
    #[serde(rename = "tier")]
    pub tier: i32,
}

impl Subscription {
    pub fn new(
        id: String,
        steam_item_id: String,
        amount: f64,
        description: String,
        period: models::SubscriptionPeriod,
        tier: i32,
    ) -> Subscription {
        Subscription {
            id,
            steam_item_id,
            oculus_sku: None,
            google_product_id: None,
            google_plan_id: None,
            pico_sku: None,
            apple_product_id: None,
            amount,
            description,
            period,
            tier,
        }
    }
}
