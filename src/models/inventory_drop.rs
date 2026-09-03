use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryDrop {
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "dropExpiryDate", deserialize_with = "Option::deserialize")]
    pub drop_expiry_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "dropStatus", skip_serializing_if = "Option::is_none")]
    pub drop_status: Option<String>,
    #[serde(rename = "endDropDate")]
    pub end_drop_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "notificationDetails")]
    pub notification_details: models::InventoryNotificationDetails,
    #[serde(rename = "startDropDate")]
    pub start_drop_date: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "targetGroup")]
    pub target_group: String,
    #[serde(rename = "templateIds")]
    pub template_ids: Vec<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl InventoryDrop {
    pub fn new(
        author_id: String,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        drop_expiry_date: Option<chrono::DateTime<chrono::FixedOffset>>,
        end_drop_date: chrono::DateTime<chrono::FixedOffset>,
        id: String,
        is_disabled: bool,
        name: String,
        notification_details: models::InventoryNotificationDetails,
        start_drop_date: chrono::DateTime<chrono::FixedOffset>,
        tags: Vec<String>,
        target_group: String,
        template_ids: Vec<String>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> InventoryDrop {
        InventoryDrop {
            author_id,
            created_at,
            drop_expiry_date,
            drop_status: None,
            end_drop_date,
            id,
            is_disabled,
            name,
            notification_details,
            start_drop_date,
            tags,
            target_group,
            template_ids,
            updated_at,
        }
    }
}
