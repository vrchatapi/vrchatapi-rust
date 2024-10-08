/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TransactionSteamInfo :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionSteamInfo {
    #[serde(rename = "walletInfo")]
    pub wallet_info: models::TransactionSteamWalletInfo,
    /// Steam User ID
    #[serde(rename = "steamId")]
    pub steam_id: String,
    /// Steam Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Empty
    #[serde(rename = "steamUrl")]
    pub steam_url: String,
    /// Steam Transaction ID, NOT the same as VRChat TransactionID
    #[serde(rename = "transId")]
    pub trans_id: String,
}

impl TransactionSteamInfo {
    pub fn new(
        wallet_info: models::TransactionSteamWalletInfo,
        steam_id: String,
        order_id: String,
        steam_url: String,
        trans_id: String,
    ) -> TransactionSteamInfo {
        TransactionSteamInfo {
            wallet_info,
            steam_id,
            order_id,
            steam_url,
            trans_id,
        }
    }
}
