/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Transaction : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "id")]
    pub id: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userDisplayName", skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "status")]
    pub status: models::TransactionStatus,
    #[serde(rename = "subscription")]
    pub subscription: Box<models::Subscription>,
    #[serde(rename = "sandbox")]
    pub sandbox: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "steam", skip_serializing_if = "Option::is_none")]
    pub steam: Option<Box<models::TransactionSteamInfo>>,
    #[serde(rename = "agreement", skip_serializing_if = "Option::is_none")]
    pub agreement: Option<Box<models::TransactionAgreement>>,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "isGift", skip_serializing_if = "Option::is_none")]
    pub is_gift: Option<bool>,
    #[serde(rename = "isTokens", skip_serializing_if = "Option::is_none")]
    pub is_tokens: Option<bool>,
}

impl Transaction {
    pub fn new(id: String, status: models::TransactionStatus, subscription: models::Subscription, sandbox: bool, created_at: String, updated_at: String, error: String) -> Transaction {
        Transaction {
            id,
            user_id: None,
            user_display_name: None,
            status,
            subscription: Box::new(subscription),
            sandbox,
            created_at,
            updated_at,
            steam: None,
            agreement: None,
            error,
            is_gift: None,
            is_tokens: None,
        }
    }
}

