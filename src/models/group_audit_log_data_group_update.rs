use crate::models;
use serde::{Deserialize, Serialize};

/// Carries only the fields the update changed.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogDataGroupUpdate {
    #[serde(
        rename = "allowGroupJoinPrompt",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_group_join_prompt: Option<models::GroupAuditLogBooleanChange>,
    #[serde(rename = "bannerId", skip_serializing_if = "Option::is_none")]
    pub banner_id: Option<models::GroupAuditLogFileIdChange>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<models::GroupAuditLogStringChange>,
    #[serde(rename = "iconId", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<models::GroupAuditLogFileIdChange>,
    #[serde(rename = "joinState", skip_serializing_if = "Option::is_none")]
    pub join_state: Option<models::GroupAuditLogJoinStateChange>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<models::GroupAuditLogStringListChange>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<models::GroupAuditLogStringListChange>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::GroupAuditLogStringChange>,
    #[serde(rename = "nameplateId", skip_serializing_if = "Option::is_none")]
    pub nameplate_id: Option<models::GroupAuditLogFileIdChange>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<models::GroupAuditLogStringChange>,
    #[serde(rename = "shortCode", skip_serializing_if = "Option::is_none")]
    pub short_code: Option<models::GroupAuditLogStringChange>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<models::GroupAuditLogStringListChange>,
}

impl GroupAuditLogDataGroupUpdate {
    /// Carries only the fields the update changed.
    pub fn new() -> GroupAuditLogDataGroupUpdate {
        GroupAuditLogDataGroupUpdate {
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
