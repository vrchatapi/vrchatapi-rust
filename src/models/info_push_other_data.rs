use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushOtherData {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "cta", skip_serializing_if = "Option::is_none")]
    pub cta: Option<models::InfoPushDataCallToAction>,
    #[serde(rename = "deliveryBehavior", skip_serializing_if = "Option::is_none")]
    pub delivery_behavior: Option<models::InfoPushDataDeliveryBehavior>,
    #[serde(rename = "imageFileId", skip_serializing_if = "Option::is_none")]
    pub image_file_id: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "onPressed", skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<models::InfoPushDataClickable>,
    #[serde(rename = "promotion", skip_serializing_if = "Option::is_none")]
    pub promotion: Option<models::InfoPushDataPromotion>,
    /// An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance.
    #[serde(
        rename = "showInWorldIds",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_in_world_ids: Option<Option<serde_json::Value>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "videoFileId", skip_serializing_if = "Option::is_none")]
    pub video_file_id: Option<String>,
    #[serde(rename = "videoUrl", skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
}

impl InfoPushOtherData {
    pub fn new() -> InfoPushOtherData {
        InfoPushOtherData {
            body: None,
            category: None,
            cta: None,
            delivery_behavior: None,
            image_file_id: None,
            image_url: None,
            media_type: None,
            name: None,
            on_pressed: None,
            promotion: None,
            show_in_world_ids: None,
            template: None,
            title: None,
            video_file_id: None,
            video_url: None,
        }
    }
}
