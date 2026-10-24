# DynamicContentRowInstances

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [**models::LocalizableString**](LocalizableString.md) |  | 
**platform** | **String** | Usually \"ThisPlatformSupported\", but can also be other values such as \"all\" or platform specific identifiers. | 
**group_pool** | Option<**String**> |  | [optional]
**min_occupants** | **i32** |  | 
**mode** | [**models::InstanceDiscoveryMode**](InstanceDiscoveryMode.md) |  | 
**n** | **i32** |  | 
**params** | **std::collections::HashMap<String, serde_json::Value>** |  | 
**region** | Option<[**models::InstanceRegion**](InstanceRegion.md)> |  | 
**r#type** | **Type** |  (enum: instances) | 
**world_pool** | Option<[**models::InstanceDiscoveryWorldPool**](InstanceDiscoveryWorldPool.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


