use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateUserModerationRequestType {
    #[serde(rename = "avatar")]
    Avatar,
    #[serde(rename = "ban")]
    Ban,
    #[serde(rename = "banpubliconly")]
    Banpubliconly,
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "emoji")]
    Emoji,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "gallery")]
    Gallery,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "hardban")]
    Hardban,
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "kick")]
    Kick,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "nuisance")]
    Nuisance,
    #[serde(rename = "print")]
    Print,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "publicinstance")]
    Publicinstance,
    #[serde(rename = "sticker")]
    Sticker,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "world")]
    World,
}

impl std::fmt::Display for CreateUserModerationRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Avatar => write!(f, "avatar"),
            Self::Ban => write!(f, "ban"),
            Self::Banpubliconly => write!(f, "banpubliconly"),
            Self::Chat => write!(f, "chat"),
            Self::Emoji => write!(f, "emoji"),
            Self::Event => write!(f, "event"),
            Self::Gallery => write!(f, "gallery"),
            Self::Group => write!(f, "group"),
            Self::Hardban => write!(f, "hardban"),
            Self::Icon => write!(f, "icon"),
            Self::Kick => write!(f, "kick"),
            Self::Note => write!(f, "note"),
            Self::Nuisance => write!(f, "nuisance"),
            Self::Print => write!(f, "print"),
            Self::Profile => write!(f, "profile"),
            Self::Publicinstance => write!(f, "publicinstance"),
            Self::Sticker => write!(f, "sticker"),
            Self::Voice => write!(f, "voice"),
            Self::Warn => write!(f, "warn"),
            Self::Warning => write!(f, "warning"),
            Self::World => write!(f, "world"),
        }
    }
}

impl Default for CreateUserModerationRequestType {
    fn default() -> CreateUserModerationRequestType {
        Self::Avatar
    }
}
