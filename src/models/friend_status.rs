/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FriendStatus {
    #[serde(rename = "incomingRequest")]
    pub incoming_request: bool,
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    #[serde(rename = "outgoingRequest")]
    pub outgoing_request: bool,
}

impl FriendStatus {
    pub fn new(incoming_request: bool, is_friend: bool, outgoing_request: bool) -> FriendStatus {
        FriendStatus {
            incoming_request,
            is_friend,
            outgoing_request,
        }
    }
}


