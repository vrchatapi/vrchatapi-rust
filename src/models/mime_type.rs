/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MimeType {
    #[serde(rename = "image/jpeg")]
    ImageSlashJpeg,
    #[serde(rename = "image/jpg")]
    ImageSlashJpg,
    #[serde(rename = "image/png")]
    ImageSlashPng,
    #[serde(rename = "image/webp")]
    ImageSlashWebp,
    #[serde(rename = "image/gif")]
    ImageSlashGif,
    #[serde(rename = "image/bmp")]
    ImageSlashBmp,
    #[serde(rename = "image/svg＋xml")]
    ImageSlashSvgxml,
    #[serde(rename = "image/tiff")]
    ImageSlashTiff,
    #[serde(rename = "application/x-avatar")]
    ApplicationSlashXAvatar,
    #[serde(rename = "application/x-world")]
    ApplicationSlashXWorld,
    #[serde(rename = "application/gzip")]
    ApplicationSlashGzip,
    #[serde(rename = "application/x-rsync-signature")]
    ApplicationSlashXRsyncSignature,
    #[serde(rename = "application/x-rsync-delta")]
    ApplicationSlashXRsyncDelta,
    #[serde(rename = "application/octet-stream")]
    ApplicationSlashOctetStream,

}

impl ToString for MimeType {
    fn to_string(&self) -> String {
        match self {
            Self::ImageSlashJpeg => String::from("image/jpeg"),
            Self::ImageSlashJpg => String::from("image/jpg"),
            Self::ImageSlashPng => String::from("image/png"),
            Self::ImageSlashWebp => String::from("image/webp"),
            Self::ImageSlashGif => String::from("image/gif"),
            Self::ImageSlashBmp => String::from("image/bmp"),
            Self::ImageSlashSvgxml => String::from("image/svg＋xml"),
            Self::ImageSlashTiff => String::from("image/tiff"),
            Self::ApplicationSlashXAvatar => String::from("application/x-avatar"),
            Self::ApplicationSlashXWorld => String::from("application/x-world"),
            Self::ApplicationSlashGzip => String::from("application/gzip"),
            Self::ApplicationSlashXRsyncSignature => String::from("application/x-rsync-signature"),
            Self::ApplicationSlashXRsyncDelta => String::from("application/x-rsync-delta"),
            Self::ApplicationSlashOctetStream => String::from("application/octet-stream"),
        }
    }
}

impl Default for MimeType {
    fn default() -> MimeType {
        Self::ImageSlashJpeg
    }
}




