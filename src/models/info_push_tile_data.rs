use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushTileData {
    #[serde(rename = "iconImageUrl")]
    pub icon_image_url: String,
    #[serde(rename = "name")]
    pub name: models::LocalizableString,
    #[serde(rename = "tooltipDescription")]
    pub tooltip_description: models::LocalizableString,
}

impl InfoPushTileData {
    pub fn new(
        icon_image_url: String,
        name: models::LocalizableString,
        tooltip_description: models::LocalizableString,
    ) -> InfoPushTileData {
        InfoPushTileData {
            icon_image_url,
            name,
            tooltip_description,
        }
    }
}
