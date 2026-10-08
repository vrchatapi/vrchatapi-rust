use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[serde(
        rename = "ageVerificationBetaCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub age_verification_beta_code: Option<String>,
    #[serde(
        rename = "ageVerificationBetaSlots",
        skip_serializing_if = "Option::is_none"
    )]
    pub age_verification_beta_slots: Option<f64>,
    #[serde(
        rename = "ageVerificationSlotsAvailable",
        skip_serializing_if = "Option::is_none"
    )]
    pub age_verification_slots_available: Option<bool>,
    #[serde(
        rename = "allowGroupJoinPrompt",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_group_join_prompt: Option<bool>,
    #[serde(rename = "badges", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<String>>,
    #[serde(
        rename = "bannerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banner_id: Option<Option<String>>,
    #[serde(
        rename = "bannerUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub banner_url: Option<Option<String>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discriminator", skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(rename = "galleries", skip_serializing_if = "Option::is_none")]
    pub galleries: Option<Vec<models::GroupGallery>>,
    #[serde(
        rename = "iconId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_id: Option<Option<String>>,
    #[serde(
        rename = "iconUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_url: Option<Option<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isVerified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(rename = "joinState", skip_serializing_if = "Option::is_none")]
    pub join_state: Option<models::GroupJoinState>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(
        rename = "lastPostCreatedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_post_created_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "memberCount", skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(
        rename = "memberCountSyncedAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_count_synced_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "membershipStatus", skip_serializing_if = "Option::is_none")]
    pub membership_status: Option<models::GroupMemberStatus>,
    #[serde(
        rename = "myMember",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub my_member: Option<Option<models::GroupMyMember>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "nameplateId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub nameplate_id: Option<Option<serde_json::Value>>,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "nameplateUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub nameplate_url: Option<Option<serde_json::Value>>,
    #[serde(rename = "onlineMemberCount", skip_serializing_if = "Option::is_none")]
    pub online_member_count: Option<i32>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<models::GroupPrivacy>,
    /// Only returned if ?includeRoles=true is specified.
    #[serde(
        rename = "roles",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub roles: Option<Option<Vec<models::GroupRole>>>,
    #[serde(
        rename = "rules",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rules: Option<Option<String>>,
    #[serde(rename = "shortCode", skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    #[serde(
        rename = "storeId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub store_id: Option<Option<String>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(
        rename = "transferTargetId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_target_id: Option<Option<String>>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            age_verification_beta_code: None,
            age_verification_beta_slots: None,
            age_verification_slots_available: None,
            allow_group_join_prompt: None,
            badges: None,
            banner_id: None,
            banner_url: None,
            created_at: None,
            description: None,
            discriminator: None,
            galleries: None,
            icon_id: None,
            icon_url: None,
            id: None,
            is_verified: None,
            join_state: None,
            languages: None,
            last_post_created_at: None,
            links: None,
            member_count: None,
            member_count_synced_at: None,
            membership_status: None,
            my_member: None,
            name: None,
            nameplate_id: None,
            nameplate_url: None,
            online_member_count: None,
            owner_id: None,
            privacy: None,
            roles: None,
            rules: None,
            short_code: None,
            store_id: None,
            tags: None,
            transfer_target_id: None,
            updated_at: None,
        }
    }
}
