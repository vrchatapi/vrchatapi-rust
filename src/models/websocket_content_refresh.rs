use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketContentRefresh {
    #[serde(rename = "actionType")]
    pub action_type: models::WebsocketContentRefreshActionType,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "itemType")]
    pub item_type: models::InventoryItemType,
}

impl WebsocketContentRefresh {
    pub fn new(
        action_type: models::WebsocketContentRefreshActionType,
        content_type: String,
        file_id: String,
        item_id: String,
        item_type: models::InventoryItemType,
    ) -> WebsocketContentRefresh {
        WebsocketContentRefresh {
            action_type,
            content_type,
            file_id,
            item_id,
            item_type,
        }
    }
}
