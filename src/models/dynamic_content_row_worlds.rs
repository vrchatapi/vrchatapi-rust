use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicContentRowWorlds {
    #[serde(flatten)]
    pub dynamic_content_row_base: models::DynamicContentRowBase,
    #[serde(
        rename = "avatarSpecific",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avatar_specific: Option<Option<bool>>,
    #[serde(rename = "bannersTag", skip_serializing_if = "Option::is_none")]
    pub banners_tag: Option<String>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(
        rename = "shortName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<Option<models::LocalizableString>>,
    #[serde(rename = "sortHeading")]
    pub sort_heading: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: models::OrderOption,
    #[serde(rename = "sortOwnership")]
    pub sort_ownership: String,
    /// Tag to filter content for this row. Not a `Tag`: that type forbids the empty string, which this field uses for a row that is not tag-filtered.
    #[serde(
        rename = "tag",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tag: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DynamicContentRowWorlds {
    pub fn new(
        dynamic_content_row_base: models::DynamicContentRowBase,
        sort_heading: String,
        sort_order: models::OrderOption,
        sort_ownership: String,
    ) -> DynamicContentRowWorlds {
        DynamicContentRowWorlds {
            dynamic_content_row_base,
            avatar_specific: None,
            banners_tag: None,
            index: None,
            short_name: None,
            sort_heading,
            sort_order,
            sort_ownership,
            tag: None,
            r#type: None,
        }
    }
}
