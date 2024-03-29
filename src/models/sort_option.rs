/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOption {
    #[serde(rename = "popularity")]
    Popularity,
    #[serde(rename = "heat")]
    Heat,
    #[serde(rename = "trust")]
    Trust,
    #[serde(rename = "shuffle")]
    Shuffle,
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "favorites")]
    Favorites,
    #[serde(rename = "reportScore")]
    ReportScore,
    #[serde(rename = "reportCount")]
    ReportCount,
    #[serde(rename = "publicationDate")]
    PublicationDate,
    #[serde(rename = "labsPublicationDate")]
    LabsPublicationDate,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "_created_at")]
    CreatedAt,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "_updated_at")]
    UpdatedAt,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "relevance")]
    Relevance,
    #[serde(rename = "magic")]
    Magic,
    #[serde(rename = "name")]
    Name,

}

impl ToString for SortOption {
    fn to_string(&self) -> String {
        match self {
            Self::Popularity => String::from("popularity"),
            Self::Heat => String::from("heat"),
            Self::Trust => String::from("trust"),
            Self::Shuffle => String::from("shuffle"),
            Self::Random => String::from("random"),
            Self::Favorites => String::from("favorites"),
            Self::ReportScore => String::from("reportScore"),
            Self::ReportCount => String::from("reportCount"),
            Self::PublicationDate => String::from("publicationDate"),
            Self::LabsPublicationDate => String::from("labsPublicationDate"),
            Self::Created => String::from("created"),
            Self::CreatedAt => String::from("_created_at"),
            Self::Updated => String::from("updated"),
            Self::UpdatedAt => String::from("_updated_at"),
            Self::Order => String::from("order"),
            Self::Relevance => String::from("relevance"),
            Self::Magic => String::from("magic"),
            Self::Name => String::from("name"),
        }
    }
}

impl Default for SortOption {
    fn default() -> SortOption {
        Self::Popularity
    }
}




