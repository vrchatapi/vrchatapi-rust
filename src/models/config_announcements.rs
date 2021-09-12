/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */

/// ConfigAnnouncements : Public Announcement



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigAnnouncements {
    /// Announcement name
    #[serde(rename = "name")]
    pub name: String,
    /// Announcement text
    #[serde(rename = "text")]
    pub text: String,
}

impl ConfigAnnouncements {
    /// Public Announcement
    pub fn new(name: String, text: String) -> ConfigAnnouncements {
        ConfigAnnouncements {
            name,
            text,
        }
    }
}


