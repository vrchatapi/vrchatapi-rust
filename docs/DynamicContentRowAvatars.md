# DynamicContentRowAvatars

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [**models::LocalizableString**](LocalizableString.md) |  | 
**platform** | **String** | Usually \"ThisPlatformSupported\", but can also be other values such as \"all\" or platform specific identifiers. | 
**any_style** | Option<**Vec<String>**> |  | [optional]
**any_tag** | Option<**Vec<String>**> |  | [optional]
**marketplace** | Option<**String**> |  | [optional]
**max_price** | Option<**i32**> |  | [optional]
**min_price** | Option<**i32**> |  | [optional]
**notag** | Option<**Vec<String>**> |  | [optional]
**params** | **std::collections::HashMap<String, serde_json::Value>** |  | 
**sort_heading** | **String** |  | 
**sort_order** | [**models::OrderOption**](OrderOption.md) |  | 
**sort_ownership** | **String** |  | 
**style** | Option<**String**> |  | [optional]
**tag** | Option<**String**> | Tag to filter content for this row. Not a `Tag`: that type forbids the empty string, which this field uses for a row that is not tag-filtered. | [optional]
**r#type** | **Type** |  (enum: avatar) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


