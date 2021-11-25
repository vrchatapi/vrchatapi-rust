# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** |  | [default to true]
**can_request_invite** | **bool** |  | [default to true]
**capacity** | **i32** |  | 
**client_number** | **String** | Always returns \"unknown\". | 
**full** | **bool** |  | [default to false]
**id** | **String** | InstanceID be \"offline\" on User profiles if you are not friends with that user. | 
**instance_id** | **String** |  | 
**location** | **String** | InstanceID be \"offline\" on User profiles if you are not friends with that user. | 
**n_users** | **i32** |  | 
**name** | **String** |  | 
**owner_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**permanent** | **bool** |  | [default to false]
**photon_region** | [**crate::models::Region**](Region.md) |  | 
**platforms** | [**crate::models::InstancePlatforms**](InstancePlatforms.md) |  | 
**region** | [**crate::models::Region**](Region.md) |  | 
**short_name** | **String** |  | 
**tags** | **Vec<String>** | The tags array on Instances usually contain the language tags of the people in the instance.  | 
**_type** | [**crate::models::InstanceType**](InstanceType.md) |  | 
**world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**hidden** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**friends** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**private** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


