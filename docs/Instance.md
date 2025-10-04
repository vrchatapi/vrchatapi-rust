# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** |  | [default to true]
**age_gate** | Option<**bool**> |  | [optional]
**can_request_invite** | **bool** |  | [default to true]
**capacity** | **i32** |  | 
**client_number** | **String** | Always returns \"unknown\". | 
**content_settings** | [**models::InstanceContentSettings**](InstanceContentSettings.md) |  | 
**display_name** | Option<**String**> |  | 
**full** | **bool** |  | [default to false]
**game_server_version** | Option<**i32**> |  | [optional]
**id** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**instance_id** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**instance_persistence_enabled** | Option<**String**> |  | 
**location** | **String** | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | 
**n_users** | **i32** |  | 
**name** | **String** |  | 
**owner_id** | Option<**String**> | A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise | [optional]
**permanent** | **bool** |  | [default to false]
**photon_region** | [**models::Region**](Region.md) |  | 
**platforms** | [**models::InstancePlatforms**](InstancePlatforms.md) |  | 
**player_persistence_enabled** | Option<**bool**> |  | 
**region** | [**models::InstanceRegion**](InstanceRegion.md) |  | 
**secure_name** | **String** |  | 
**short_name** | Option<**String**> |  | [optional]
**tags** | **Vec<String>** | The tags array on Instances usually contain the language tags of the people in the instance.  | 
**r#type** | [**models::InstanceType**](InstanceType.md) |  | 
**world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**hidden** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**friends** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**private** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**queue_enabled** | **bool** |  | 
**queue_size** | **i32** |  | 
**recommended_capacity** | **i32** |  | 
**role_restricted** | Option<**bool**> |  | [optional]
**strict** | **bool** |  | 
**user_count** | **i32** |  | 
**world** | [**models::World**](World.md) |  | 
**users** | Option<[**Vec<models::LimitedUserInstance>**](LimitedUserInstance.md)> | The users field is present on instances created by the requesting user. | [optional]
**group_access_type** | Option<[**models::GroupAccessType**](GroupAccessType.md)> |  | [optional]
**has_capacity_for_you** | Option<**bool**> |  | [optional]
**nonce** | Option<**String**> |  | [optional]
**closed_at** | Option<**String**> |  | [optional]
**hard_close** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


