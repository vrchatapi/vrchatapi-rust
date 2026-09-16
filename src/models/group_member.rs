use crate::models;
use serde::{Deserialize, Serialize};

/// GroupMember : May be null when attempting to retrieve group membership for a user who is not part of the group
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMember {
    /// Only missing when explicitly fetching own user.
    #[serde(
        rename = "acceptedByDisplayName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepted_by_display_name: Option<Option<String>>,
    /// Only missing when explicitly fetching own user.
    #[serde(
        rename = "acceptedById",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepted_by_id: Option<Option<String>>,
    /// Only missing when explicitly fetching own user.
    #[serde(
        rename = "bannedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banned_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    /// Only missing when explicitly fetching own user.
    #[serde(
        rename = "createdAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// Missing when explicitly fetching own user, or when group isn't linked to a purchase.
    #[serde(
        rename = "hasJoinedFromPurchase",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_joined_from_purchase: Option<bool>,
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the user is representing the group. This makes the group show up above the name tag in-game.
    #[serde(rename = "isRepresenting")]
    pub is_representing: bool,
    #[serde(
        rename = "isSubscribedToAnnouncements",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_subscribed_to_announcements: Option<bool>,
    /// Only missing when explicitly fetching own user.
    #[serde(
        rename = "isSubscribedToEventAnnouncements",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_subscribed_to_event_announcements: Option<bool>,
    #[serde(
        rename = "joinedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub joined_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(
        rename = "lastPostReadAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_post_read_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "mRoleIds")]
    pub m_role_ids: Vec<String>,
    /// Only missing when explicitly fetching own user.
    #[serde(
        rename = "managerNotes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub manager_notes: Option<Option<String>>,
    #[serde(rename = "membershipStatus", skip_serializing_if = "Option::is_none")]
    pub membership_status: Option<models::GroupMemberStatus>,
    #[serde(rename = "roleIds")]
    pub role_ids: Vec<String>,
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<models::GroupMemberLimitedUser>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl GroupMember {
    /// May be null when attempting to retrieve group membership for a user who is not part of the group
    pub fn new(
        group_id: String,
        id: String,
        is_representing: bool,
        m_role_ids: Vec<String>,
        role_ids: Vec<String>,
        user_id: String,
    ) -> GroupMember {
        GroupMember {
            accepted_by_display_name: None,
            accepted_by_id: None,
            banned_at: None,
            created_at: None,
            group_id,
            has_joined_from_purchase: None,
            id,
            is_representing,
            is_subscribed_to_announcements: None,
            is_subscribed_to_event_announcements: None,
            joined_at: None,
            last_post_read_at: None,
            m_role_ids,
            manager_notes: None,
            membership_status: None,
            role_ids,
            user: None,
            user_id,
            visibility: None,
        }
    }
}
