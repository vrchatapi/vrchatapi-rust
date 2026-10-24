use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushOnboardingAvatarData {
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "avatarId")]
    pub avatar_id: String,
    #[serde(rename = "bannerImageUrl")]
    pub banner_image_url: String,
    #[serde(rename = "finalName")]
    pub final_name: String,
    #[serde(rename = "iconImageUrl")]
    pub icon_image_url: String,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(rename = "overrideName", deserialize_with = "Option::deserialize")]
    pub override_name: Option<serde_json::Value>,
}

impl InfoPushOnboardingAvatarData {
    pub fn new(
        author_name: String,
        avatar_id: String,
        banner_image_url: String,
        final_name: String,
        icon_image_url: String,
        override_name: Option<serde_json::Value>,
    ) -> InfoPushOnboardingAvatarData {
        InfoPushOnboardingAvatarData {
            author_name,
            avatar_id,
            banner_image_url,
            final_name,
            icon_image_url,
            override_name,
        }
    }
}
