use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupGalleryCreate {
    /// The gallery description.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the gallery is members only.
    #[serde(rename = "membersOnly")]
    pub members_only: bool,
    /// The gallery name.
    #[serde(rename = "name")]
    pub name: String,
    /// The role IDs whose submissions are approved automatically.
    #[serde(
        rename = "roleIdsToAutoApprove",
        deserialize_with = "Option::deserialize"
    )]
    pub role_ids_to_auto_approve: Option<Vec<String>>,
    /// The role IDs that can manage the gallery.
    #[serde(rename = "roleIdsToManage", deserialize_with = "Option::deserialize")]
    pub role_ids_to_manage: Option<Vec<String>>,
    /// The role IDs that can submit to the gallery.
    #[serde(rename = "roleIdsToSubmit", deserialize_with = "Option::deserialize")]
    pub role_ids_to_submit: Option<Vec<String>>,
    /// The role IDs that can view the gallery.
    #[serde(rename = "roleIdsToView", deserialize_with = "Option::deserialize")]
    pub role_ids_to_view: Option<Vec<String>>,
}

impl GroupAuditLogEntryDataGroupGalleryCreate {
    pub fn new(
        description: String,
        members_only: bool,
        name: String,
        role_ids_to_auto_approve: Option<Vec<String>>,
        role_ids_to_manage: Option<Vec<String>>,
        role_ids_to_submit: Option<Vec<String>>,
        role_ids_to_view: Option<Vec<String>>,
    ) -> GroupAuditLogEntryDataGroupGalleryCreate {
        GroupAuditLogEntryDataGroupGalleryCreate {
            description,
            members_only,
            name,
            role_ids_to_auto_approve,
            role_ids_to_manage,
            role_ids_to_submit,
            role_ids_to_view,
        }
    }
}
