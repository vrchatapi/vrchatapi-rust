use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupCreateDiscoverability {
    #[serde(rename = "isDiscoverableComputed")]
    pub is_discoverable_computed: bool,
    #[serde(rename = "isModerationDiscoverable")]
    pub is_moderation_discoverable: bool,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "isModerationDiscoverableReason",
        deserialize_with = "Option::deserialize"
    )]
    pub is_moderation_discoverable_reason: Option<serde_json::Value>,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "isOverrideDiscoverable",
        deserialize_with = "Option::deserialize"
    )]
    pub is_override_discoverable: Option<serde_json::Value>,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "isOverrideDiscoverableReason",
        deserialize_with = "Option::deserialize"
    )]
    pub is_override_discoverable_reason: Option<serde_json::Value>,
}

impl GroupAuditLogEntryDataGroupCreateDiscoverability {
    pub fn new(
        is_discoverable_computed: bool,
        is_moderation_discoverable: bool,
        is_moderation_discoverable_reason: Option<serde_json::Value>,
        is_override_discoverable: Option<serde_json::Value>,
        is_override_discoverable_reason: Option<serde_json::Value>,
    ) -> GroupAuditLogEntryDataGroupCreateDiscoverability {
        GroupAuditLogEntryDataGroupCreateDiscoverability {
            is_discoverable_computed,
            is_moderation_discoverable,
            is_moderation_discoverable_reason,
            is_override_discoverable,
            is_override_discoverable_reason,
        }
    }
}
