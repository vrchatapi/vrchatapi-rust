/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFileRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "mimeType")]
    pub mime_type: crate::models::MimeType,
    #[serde(rename = "extension")]
    pub extension: String,
    ///  
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreateFileRequest {
    pub fn new(name: String, mime_type: crate::models::MimeType, extension: String) -> CreateFileRequest {
        CreateFileRequest {
            name,
            mime_type,
            extension,
            tags: None,
        }
    }
}


