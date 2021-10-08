# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** |  | [default to true]
**can_request_invite** | **bool** |  | [default to true]
**capacity** | **i32** |  | 
**client_number** | **i32** |  | 
**full** | **bool** |  | [default to false]
**id** | **String** |  | 
**instance_id** | **String** |  | 
**location** | **String** |  | 
**n_users** | **i32** |  | 
**name** | **String** |  | 
**nonce** | Option<**String**> |  | [optional]
**owner_id** | **String** |  | 
**permanent** | **bool** |  | [default to false]
**photon_region** | **String** |  | 
**platforms** | [**crate::models::InstancePlatforms**](Instance_platforms.md) |  | 
**region** | **String** |  | 
**short_name** | **String** |  | 
**tags** | **Vec<String>** |  | 
**_type** | **String** |  | 
**users** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Always empty on non-existing instances, and non-present on existing instances. | [optional]
**world** | Option<[**serde_json::Value**](.md)> | Only present on non-existing instances, and only contains a very small subject of World object. Use World API instead. | [optional]
**world_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


