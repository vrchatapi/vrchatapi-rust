use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomyAccount {
    #[serde(
        rename = "accountActivatedOn",
        deserialize_with = "Option::deserialize"
    )]
    pub account_activated_on: Option<String>,
    #[serde(rename = "accountId", deserialize_with = "Option::deserialize")]
    pub account_id: Option<String>,
    #[serde(
        rename = "accountSellerRegisteredOn",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_seller_registered_on: Option<Option<String>>,
    #[serde(
        rename = "accountSellerStatus",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_seller_status: Option<Option<String>>,
    #[serde(rename = "blocked")]
    pub blocked: bool,
    #[serde(rename = "canEarn", skip_serializing_if = "Option::is_none")]
    pub can_earn: Option<bool>,
    #[serde(rename = "canPayout", skip_serializing_if = "Option::is_none")]
    pub can_payout: Option<bool>,
    #[serde(rename = "canSpend")]
    pub can_spend: bool,
    #[serde(
        rename = "skrillEmail",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub skrill_email: Option<Option<String>>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(
        rename = "tiliaId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tilia_id: Option<Option<String>>,
    #[serde(
        rename = "tiliaType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tilia_type: Option<Option<String>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl EconomyAccount {
    pub fn new(
        account_activated_on: Option<String>,
        account_id: Option<String>,
        blocked: bool,
        can_spend: bool,
        source: String,
        user_id: String,
    ) -> EconomyAccount {
        EconomyAccount {
            account_activated_on,
            account_id,
            account_seller_registered_on: None,
            account_seller_status: None,
            blocked,
            can_earn: None,
            can_payout: None,
            can_spend,
            skrill_email: None,
            source,
            tilia_id: None,
            tilia_type: None,
            user_id,
        }
    }
}
