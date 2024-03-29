/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// Favorite : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Favorite {
    /// MUST be either AvatarID, UserID or WorldID.
    #[serde(rename = "favoriteId")]
    pub favorite_id: String,
    #[serde(rename = "id")]
    pub id: String,
    ///  
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::FavoriteType,
}

impl Favorite {
    /// 
    pub fn new(favorite_id: String, id: String, tags: Vec<String>, r#type: crate::models::FavoriteType) -> Favorite {
        Favorite {
            favorite_id,
            id,
            tags,
            r#type,
        }
    }
}


