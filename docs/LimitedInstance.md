# LimitedInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** |  | [default to true]
**capacity** | **i32** |  | 
**category_id** | Option<**String**> |  | 
**creation_languages** | **Vec<serde_json::Value>** |  | 
**description** | Option<**String**> |  | 
**disabled_prop_abilities** | **Vec<serde_json::Value>** |  | 
**display_name** | Option<**String**> |  | 
**display_vibe_id** | Option<**String**> |  | 
**dominant_language** | **String** |  | 
**full** | **bool** |  | [default to false]
**group_access_type** | Option<[**models::GroupAccessType**](GroupAccessType.md)> |  | [optional]
**id** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**instance_id** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**language_ratio** | **std::collections::HashMap<String, serde_json::Value>** |  | 
**languages** | **Vec<String>** | The keys of languageRatio, ordered by their share of the instance. | 
**languages_iso639** | **Vec<String>** |  | 
**location** | **String** | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | 
**minimum_avatar_performance** | Option<**String**> |  | 
**n_users** | **i32** |  | 
**owner_id** | Option<**String**> | A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise | 
**permanent** | **bool** |  | [default to false]
**photon_region** | [**models::Region**](Region.md) |  | 
**platforms** | [**models::InstancePlatforms**](InstancePlatforms.md) |  | 
**queue_enabled** | **bool** |  | 
**queue_size** | **i32** |  | 
**recommended_capacity** | **i32** |  | 
**region** | [**models::InstanceRegion**](InstanceRegion.md) |  | 
**role_restricted** | Option<**bool**> |  | [optional]
**short_name** | Option<**String**> |  | 
**tags** | **Vec<String>** | The tags array on Instances usually contain the language tags of the people in the instance.  | 
**r#type** | [**models::InstanceType**](InstanceType.md) |  | 
**user_count** | **i32** |  | 
**user_icons** | **Vec<String>** |  | 
**vibe_ids** | **Vec<String>** |  | 
**world** | [**models::World**](World.md) |  | 
**world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


