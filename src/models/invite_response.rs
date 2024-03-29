/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteResponse {
    #[serde(rename = "responseSlot")]
    pub response_slot: i32,
}

impl InviteResponse {
    pub fn new(response_slot: i32) -> InviteResponse {
        InviteResponse {
            response_slot,
        }
    }
}


