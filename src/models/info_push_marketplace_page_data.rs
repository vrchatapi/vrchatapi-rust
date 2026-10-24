use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushMarketplacePageData {
    #[serde(flatten)]
    pub info_push_tile_data: models::InfoPushTileData,
    #[serde(rename = "categories")]
    pub categories: Vec<models::InfoPushDataCategory>,
    #[serde(
        rename = "featuredAvatarCategoryId",
        skip_serializing_if = "Option::is_none"
    )]
    pub featured_avatar_category_id: Option<String>,
}

impl InfoPushMarketplacePageData {
    pub fn new(
        info_push_tile_data: models::InfoPushTileData,
        categories: Vec<models::InfoPushDataCategory>,
    ) -> InfoPushMarketplacePageData {
        InfoPushMarketplacePageData {
            info_push_tile_data,
            categories,
            featured_avatar_category_id: None,
        }
    }
}
