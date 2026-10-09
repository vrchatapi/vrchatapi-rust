use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateFileRequestMimeType {
    #[serde(rename = "application/gzip")]
    ApplicationSlashGzip,
    #[serde(rename = "application/x-adminassetbundle")]
    ApplicationSlashXAdminassetbundle,
    #[serde(rename = "application/x-avatar")]
    ApplicationSlashXAvatar,
    #[serde(rename = "application/x-avatarbuilderresource")]
    ApplicationSlashXAvatarbuilderresource,
    #[serde(rename = "application/x-avatarpart")]
    ApplicationSlashXAvatarpart,
    #[serde(rename = "application/x-prop")]
    ApplicationSlashXProp,
    #[serde(rename = "application/x-world")]
    ApplicationSlashXWorld,
    #[serde(rename = "image/bmp")]
    ImageSlashBmp,
    #[serde(rename = "image/gif")]
    ImageSlashGif,
    #[serde(rename = "image/jpeg")]
    ImageSlashJpeg,
    #[serde(rename = "image/jpg")]
    ImageSlashJpg,
    #[serde(rename = "image/png")]
    ImageSlashPng,
    #[serde(rename = "image/svg+xml")]
    ImageSlashSvgPlusXml,
    #[serde(rename = "image/tiff")]
    ImageSlashTiff,
    #[serde(rename = "image/webp")]
    ImageSlashWebp,
    #[serde(rename = "video/mp4")]
    VideoSlashMp4,
}

impl std::fmt::Display for CreateFileRequestMimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ApplicationSlashGzip => write!(f, "application/gzip"),
            Self::ApplicationSlashXAdminassetbundle => write!(f, "application/x-adminassetbundle"),
            Self::ApplicationSlashXAvatar => write!(f, "application/x-avatar"),
            Self::ApplicationSlashXAvatarbuilderresource => {
                write!(f, "application/x-avatarbuilderresource")
            }
            Self::ApplicationSlashXAvatarpart => write!(f, "application/x-avatarpart"),
            Self::ApplicationSlashXProp => write!(f, "application/x-prop"),
            Self::ApplicationSlashXWorld => write!(f, "application/x-world"),
            Self::ImageSlashBmp => write!(f, "image/bmp"),
            Self::ImageSlashGif => write!(f, "image/gif"),
            Self::ImageSlashJpeg => write!(f, "image/jpeg"),
            Self::ImageSlashJpg => write!(f, "image/jpg"),
            Self::ImageSlashPng => write!(f, "image/png"),
            Self::ImageSlashSvgPlusXml => write!(f, "image/svg+xml"),
            Self::ImageSlashTiff => write!(f, "image/tiff"),
            Self::ImageSlashWebp => write!(f, "image/webp"),
            Self::VideoSlashMp4 => write!(f, "video/mp4"),
        }
    }
}

impl Default for CreateFileRequestMimeType {
    fn default() -> CreateFileRequestMimeType {
        Self::ApplicationSlashGzip
    }
}
