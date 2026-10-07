use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntryDataGroupUpdate : Carries only the fields the update changed.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupUpdate {
    #[serde(
        rename = "allowGroupJoinPrompt",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_group_join_prompt: Option<models::GroupAuditLogEntryBooleanChange>,
    #[serde(rename = "bannerId", skip_serializing_if = "Option::is_none")]
    pub banner_id: Option<models::GroupAuditLogEntryFileIdChange>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<models::GroupAuditLogEntryStringChange>,
    #[serde(rename = "iconId", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<models::GroupAuditLogEntryFileIdChange>,
    #[serde(rename = "joinState", skip_serializing_if = "Option::is_none")]
    pub join_state: Option<models::GroupAuditLogEntryJoinStateChange>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<models::GroupAuditLogEntryStringListChange>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<models::GroupAuditLogEntryStringListChange>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::GroupAuditLogEntryStringChange>,
    #[serde(rename = "nameplateId", skip_serializing_if = "Option::is_none")]
    pub nameplate_id: Option<models::GroupAuditLogEntryFileIdChange>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<models::GroupAuditLogEntryStringChange>,
    #[serde(rename = "shortCode", skip_serializing_if = "Option::is_none")]
    pub short_code: Option<models::GroupAuditLogEntryStringChange>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<models::GroupAuditLogEntryStringListChange>,
}

impl GroupAuditLogEntryDataGroupUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogEntryDataGroupUpdate {
        GroupAuditLogEntryDataGroupUpdate {
            allow_group_join_prompt: None,
            banner_id: None,
            description: None,
            icon_id: None,
            join_state: None,
            languages: None,
            links: None,
            name: None,
            nameplate_id: None,
            rules: None,
            short_code: None,
            tags: None,
        }
    }
}
