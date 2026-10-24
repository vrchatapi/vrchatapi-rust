# DynamicContentRowEvents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [**models::LocalizableString**](LocalizableString.md) |  | 
**platform** | **String** | Usually \"ThisPlatformSupported\", but can also be other values such as \"all\" or platform specific identifiers. | 
**categories** | **Vec<String>** |  | 
**featured_results** | **String** |  | 
**minimum_interest_count** | **i32** |  | 
**minimum_remaining_minutes** | **i32** |  | 
**non_featured_results** | **String** |  | 
**params** | **std::collections::HashMap<String, serde_json::Value>** |  | 
**personalized_results** | **String** |  | 
**scope** | **String** |  | 
**tags** | Option<**Vec<String>**> |  | [optional]
**r#type** | **Type** |  (enum: events) | 
**upcoming_offset_minutes** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


