use crate::models;
use serde::{Deserialize, Serialize};

/// Content whose `responseType` has no schema of its own.
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushContentUnknown {
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(rename = "nextCursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<serde_json::Value>,
    #[serde(rename = "responseType")]
    pub response_type: String,
    #[serde(rename = "results")]
    pub results: Vec<serde_json::Value>,
}

impl InfoPushContentUnknown {
    /// Content whose `responseType` has no schema of its own.
    pub fn new(
        next_cursor: Option<serde_json::Value>,
        response_type: String,
        results: Vec<serde_json::Value>,
    ) -> InfoPushContentUnknown {
        InfoPushContentUnknown {
            next_cursor,
            response_type,
            results,
        }
    }
}
