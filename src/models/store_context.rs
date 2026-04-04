use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreContext {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
}

impl StoreContext {
    pub fn new(id: String, image_url: Option<String>, name: String) -> StoreContext {
        StoreContext {
            id,
            image_url,
            name,
        }
    }
}
