use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebsocketFriendActive {
    /// This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "user")]
    pub user: models::User,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl WebsocketFriendActive {
    pub fn new(platform: String, user: models::User, user_id: String) -> WebsocketFriendActive {
        WebsocketFriendActive {
            platform,
            user,
            user_id,
        }
    }
}
