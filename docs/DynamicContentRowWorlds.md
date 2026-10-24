# DynamicContentRowWorlds

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [**models::LocalizableString**](LocalizableString.md) |  | 
**platform** | **String** | Usually \"ThisPlatformSupported\", but can also be other values such as \"all\" or platform specific identifiers. | 
**avatar_specific** | Option<**bool**> |  | [optional]
**banners_tag** | Option<**String**> |  | [optional]
**index** | Option<**i32**> |  | [optional]
**short_name** | Option<[**models::LocalizableString**](LocalizableString.md)> |  | [optional]
**sort_heading** | **String** |  | 
**sort_order** | [**models::OrderOption**](OrderOption.md) |  | 
**sort_ownership** | **String** |  | 
**tag** | Option<**String**> | Tag to filter content for this row. Not a `Tag`: that type forbids the empty string, which this field uses for a row that is not tag-filtered. | [optional]
**r#type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


