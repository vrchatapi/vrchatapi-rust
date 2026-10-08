use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FollowCalendarEventRequest {
    #[serde(rename = "isFollowing")]
    pub is_following: bool,
}

impl FollowCalendarEventRequest {
    pub fn new(is_following: bool) -> FollowCalendarEventRequest {
        FollowCalendarEventRequest { is_following }
    }
}
