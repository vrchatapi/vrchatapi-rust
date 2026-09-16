use crate::models;
use serde::{Deserialize, Serialize};

/// UnavailableWorld : Stands in for a world the API will not describe. `name` and `authorName` are `???`, `imageUrl` is empty, and the counts are `0`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnavailableWorld {
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "capacity")]
    pub capacity: i32,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "isSecure")]
    pub is_secure: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "occupants")]
    pub occupants: i32,
    #[serde(rename = "thumbnailImageUrl")]
    pub thumbnail_image_url: String,
}

impl UnavailableWorld {
    /// Stands in for a world the API will not describe. `name` and `authorName` are `???`, `imageUrl` is empty, and the counts are `0`.
    pub fn new(
        author_name: String,
        capacity: i32,
        id: String,
        image_url: String,
        is_secure: bool,
        name: String,
        occupants: i32,
        thumbnail_image_url: String,
    ) -> UnavailableWorld {
        UnavailableWorld {
            author_name,
            capacity,
            id,
            image_url,
            is_secure,
            name,
            occupants,
            thumbnail_image_url,
        }
    }
}
