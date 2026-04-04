use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardBadge {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "hidden")]
    pub hidden: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "isLocalizationEnabled")]
    pub is_localization_enabled: bool,
    #[serde(rename = "machineName", skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl RewardBadge {
    pub fn new(
        created_at: String,
        created_by: String,
        description: String,
        file_name: String,
        hidden: bool,
        id: String,
        image_url: String,
        is_localization_enabled: bool,
        name: String,
        r#type: String,
        updated_at: String,
    ) -> RewardBadge {
        RewardBadge {
            created_at,
            created_by,
            description,
            file_name,
            hidden,
            id,
            image_url,
            is_localization_enabled,
            machine_name: None,
            name,
            r#type,
            updated_at,
        }
    }
}
