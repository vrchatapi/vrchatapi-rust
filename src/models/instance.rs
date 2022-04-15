/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// Instance : * `hidden` field is only present if InstanceType is `hidden` aka \"Friends+\", and is instance creator. * `friends` field is only present if InstanceType is `friends` aka \"Friends\", and is instance creator. * `private` field is only present if InstanceType is `private` aka \"Invite\" or \"Invite+\", and is instance creator.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "canRequestInvite")]
    pub can_request_invite: bool,
    #[serde(rename = "capacity")]
    pub capacity: i32,
    /// Always returns \"unknown\".
    #[serde(rename = "clientNumber")]
    pub client_number: String,
    #[serde(rename = "full")]
    pub full: bool,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "n_users")]
    pub n_users: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "permanent")]
    pub permanent: bool,
    #[serde(rename = "photonRegion")]
    pub photon_region: crate::models::Region,
    #[serde(rename = "platforms")]
    pub platforms: Box<crate::models::InstancePlatforms>,
    #[serde(rename = "region")]
    pub region: crate::models::Region,
    #[serde(rename = "shortName")]
    pub short_name: String,
    /// The tags array on Instances usually contain the language tags of the people in the instance. 
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::InstanceType,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId")]
    pub world_id: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "friends", skip_serializing_if = "Option::is_none")]
    pub friends: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<String>,
}

impl Instance {
    /// * `hidden` field is only present if InstanceType is `hidden` aka \"Friends+\", and is instance creator. * `friends` field is only present if InstanceType is `friends` aka \"Friends\", and is instance creator. * `private` field is only present if InstanceType is `private` aka \"Invite\" or \"Invite+\", and is instance creator.
    pub fn new(active: bool, can_request_invite: bool, capacity: i32, client_number: String, full: bool, id: String, instance_id: String, location: String, n_users: i32, name: String, permanent: bool, photon_region: crate::models::Region, platforms: crate::models::InstancePlatforms, region: crate::models::Region, short_name: String, tags: Vec<String>, _type: crate::models::InstanceType, world_id: String) -> Instance {
        Instance {
            active,
            can_request_invite,
            capacity,
            client_number,
            full,
            id,
            instance_id,
            location,
            n_users,
            name,
            owner_id: None,
            permanent,
            photon_region,
            platforms: Box::new(platforms),
            region,
            short_name,
            tags,
            _type,
            world_id,
            hidden: None,
            friends: None,
            private: None,
        }
    }
}


