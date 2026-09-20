use crate::models;
use serde::{Deserialize, Serialize};

/// WorldFavoriteList : A world favorite group as a public profile lists it, with a sample of its worlds' thumbnails.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldFavoriteList {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<String>,
}

impl WorldFavoriteList {
    /// A world favorite group as a public profile lists it, with a sample of its worlds' thumbnails.
    pub fn new(count: i32, id: String, name: String, thumbnails: Vec<String>) -> WorldFavoriteList {
        WorldFavoriteList {
            count,
            id,
            name,
            thumbnails,
        }
    }
}
