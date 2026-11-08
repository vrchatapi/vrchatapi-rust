use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserModerationRequest {
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "isPermanent", skip_serializing_if = "Option::is_none")]
    pub is_permanent: Option<bool>,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "type")]
    pub r#type: models::CreateUserModerationRequestType,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId", skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}

impl CreateUserModerationRequest {
    pub fn new(
        reason: String,
        r#type: models::CreateUserModerationRequestType,
    ) -> CreateUserModerationRequest {
        CreateUserModerationRequest {
            expires: None,
            instance_id: None,
            is_permanent: None,
            reason,
            r#type,
            world_id: None,
        }
    }
}
