# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** |  | [default to true]
**age_gate** | Option<**bool**> |  | [optional]
**can_request_invite** | **bool** |  | [default to true]
**capacity** | **i32** |  | 
**client_number** | **String** | Always returns \"unknown\". | 
**closed_at** | Option<**String**> |  | [optional]
**content_settings** | Option<[**models::InstanceContentSettings**](InstanceContentSettings.md)> |  | [optional]
**display_name** | Option<**String**> |  | [optional]
**friends** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**full** | **bool** |  | [default to false]
**game_server_version** | Option<**i32**> |  | [optional]
**group_access_type** | Option<[**models::GroupAccessType**](GroupAccessType.md)> |  | [optional]
**hard_close** | Option<**bool**> |  | [optional]
**has_capacity_for_you** | Option<**bool**> |  | [optional]
**hidden** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**id** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**instance_id** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**instance_persistence_enabled** | Option<**String**> |  | [optional]
**location** | **String** | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | 
**n_users** | **i32** |  | 
**name** | **String** |  | 
**nonce** | Option<**String**> |  | [optional]
**owner_id** | Option<**String**> | A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise | [optional]
**permanent** | **bool** |  | [default to false]
**photon_region** | [**models::Region**](Region.md) |  | 
**platforms** | [**models::InstancePlatforms**](InstancePlatforms.md) |  | 
**player_persistence_enabled** | Option<**bool**> |  | [optional]
**private** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**queue_enabled** | **bool** |  | 
**queue_size** | **i32** |  | 
**recommended_capacity** | **i32** |  | 
**region** | [**models::InstanceRegion**](InstanceRegion.md) |  | 
**role_restricted** | Option<**bool**> |  | [optional]
**secure_name** | **String** |  | 
**short_name** | Option<**String**> |  | [optional]
**strict** | **bool** |  | 
**tags** | **Vec<String>** | The tags array on Instances usually contain the language tags of the people in the instance.  | 
**r#type** | [**models::InstanceType**](InstanceType.md) |  | 
**user_count** | **i32** |  | 
**users** | Option<[**Vec<models::LimitedUserInstance>**](LimitedUserInstance.md)> | The users field is present on instances created by the requesting user. | [optional]
**world** | [**models::World**](World.md) |  | 
**world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


