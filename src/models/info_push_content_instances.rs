use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushContentInstances {
    #[serde(rename = "attributionId")]
    pub attribution_id: String,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(rename = "nextCursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<serde_json::Value>,
    #[serde(rename = "responseType", default)]
    pub response_type: ResponseType,
    #[serde(rename = "results")]
    pub results: Vec<models::LimitedInstance>,
}

impl InfoPushContentInstances {
    pub fn new(
        attribution_id: String,
        next_cursor: Option<serde_json::Value>,
        response_type: ResponseType,
        results: Vec<models::LimitedInstance>,
    ) -> InfoPushContentInstances {
        InfoPushContentInstances {
            attribution_id,
            next_cursor,
            response_type,
            results,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "instance")]
    Instance,
}

impl Default for ResponseType {
    fn default() -> ResponseType {
        Self::Instance
    }
}
