use crate::models;
use serde::{Deserialize, Serialize};

/// Either inventoryItemId by itself, or emojiId with optional emojiVersion
#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationV2DetailsBoop {
    #[serde(rename = "emojiId")]
    pub emoji_id: models::EmojiId,
    #[serde(rename = "emojiVersion", deserialize_with = "Option::deserialize")]
    pub emoji_version: Option<i32>,
    #[serde(rename = "inventoryItemId")]
    pub inventory_item_id: String,
}

impl NotificationV2DetailsBoop {
    /// Either inventoryItemId by itself, or emojiId with optional emojiVersion
    pub fn new(
        emoji_id: models::EmojiId,
        emoji_version: Option<i32>,
        inventory_item_id: String,
    ) -> NotificationV2DetailsBoop {
        NotificationV2DetailsBoop {
            emoji_id,
            emoji_version,
            inventory_item_id,
        }
    }
}
