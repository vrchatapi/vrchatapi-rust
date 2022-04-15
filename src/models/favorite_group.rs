/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FavoriteGroup {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ownerDisplayName")]
    pub owner_display_name: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::FavoriteType,
    #[serde(rename = "visibility")]
    pub visibility: crate::models::FavoriteGroupVisibility,
}

impl FavoriteGroup {
    pub fn new(display_name: String, id: String, name: String, owner_display_name: String, owner_id: String, tags: Vec<String>, _type: crate::models::FavoriteType, visibility: crate::models::FavoriteGroupVisibility) -> FavoriteGroup {
        FavoriteGroup {
            display_name,
            id,
            name,
            owner_display_name,
            owner_id,
            tags,
            _type,
            visibility,
        }
    }
}


