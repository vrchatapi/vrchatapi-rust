use crate::models;
use serde::{Deserialize, Serialize};

/// InterestsAndPreferences : Interests and preferences the current user has turned on. A key is present only while its value is `true`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterestsAndPreferences {
    #[serde(rename = "Anime", skip_serializing_if = "Option::is_none")]
    pub anime: Option<bool>,
    #[serde(rename = "Art", skip_serializing_if = "Option::is_none")]
    pub art: Option<bool>,
    #[serde(rename = "Avatars", skip_serializing_if = "Option::is_none")]
    pub avatars: Option<bool>,
    #[serde(rename = "BigGroup", skip_serializing_if = "Option::is_none")]
    pub big_group: Option<bool>,
    #[serde(rename = "Explore", skip_serializing_if = "Option::is_none")]
    pub explore: Option<bool>,
    #[serde(rename = "Fantasy", skip_serializing_if = "Option::is_none")]
    pub fantasy: Option<bool>,
    #[serde(rename = "Fashion", skip_serializing_if = "Option::is_none")]
    pub fashion: Option<bool>,
    #[serde(rename = "FindAvatars", skip_serializing_if = "Option::is_none")]
    pub find_avatars: Option<bool>,
    #[serde(rename = "Furries", skip_serializing_if = "Option::is_none")]
    pub furries: Option<bool>,
    #[serde(rename = "Horror", skip_serializing_if = "Option::is_none")]
    pub horror: Option<bool>,
    #[serde(rename = "LanguageLearning", skip_serializing_if = "Option::is_none")]
    pub language_learning: Option<bool>,
    #[serde(rename = "MeetPeople", skip_serializing_if = "Option::is_none")]
    pub meet_people: Option<bool>,
    #[serde(rename = "Music", skip_serializing_if = "Option::is_none")]
    pub music: Option<bool>,
    #[serde(rename = "Mystery", skip_serializing_if = "Option::is_none")]
    pub mystery: Option<bool>,
    #[serde(rename = "SciFi", skip_serializing_if = "Option::is_none")]
    pub sci_fi: Option<bool>,
    #[serde(rename = "SmallGroup", skip_serializing_if = "Option::is_none")]
    pub small_group: Option<bool>,
    #[serde(rename = "Surprise", skip_serializing_if = "Option::is_none")]
    pub surprise: Option<bool>,
}

impl InterestsAndPreferences {
    /// Interests and preferences the current user has turned on. A key is present only while its value is `true`.
    pub fn new() -> InterestsAndPreferences {
        InterestsAndPreferences {
            anime: None,
            art: None,
            avatars: None,
            big_group: None,
            explore: None,
            fantasy: None,
            fashion: None,
            find_avatars: None,
            furries: None,
            horror: None,
            language_learning: None,
            meet_people: None,
            music: None,
            mystery: None,
            sci_fi: None,
            small_group: None,
            surprise: None,
        }
    }
}
