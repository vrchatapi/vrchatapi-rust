/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "friendRequest")]
    FriendRequest,
    #[serde(rename = "invite")]
    Invite,
    #[serde(rename = "inviteResponse")]
    InviteResponse,
    #[serde(rename = "requestInvite")]
    RequestInvite,
    #[serde(rename = "requestInviteResponse")]
    RequestInviteResponse,
    #[serde(rename = "votetokick")]
    Votetokick,

}

impl ToString for NotificationType {
    fn to_string(&self) -> String {
        match self {
            Self::FriendRequest => String::from("friendRequest"),
            Self::Invite => String::from("invite"),
            Self::InviteResponse => String::from("inviteResponse"),
            Self::RequestInvite => String::from("requestInvite"),
            Self::RequestInviteResponse => String::from("requestInviteResponse"),
            Self::Votetokick => String::from("votetokick"),
        }
    }
}




