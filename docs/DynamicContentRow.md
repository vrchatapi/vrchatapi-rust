# DynamicContentRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**any_style** | Option<**Vec<String>**> |  | [optional]
**any_tag** | Option<**Vec<String>**> |  | [optional]
**avatar_specific** | Option<**bool**> |  | [optional]
**banners_tag** | Option<**String**> |  | [optional]
**categories** | Option<**Vec<String>**> |  | [optional]
**featured_results** | Option<**String**> |  | [optional]
**index** | Option<**i32**> |  | [optional]
**marketplace** | Option<**String**> |  | [optional]
**max_price** | Option<**i32**> |  | [optional]
**min_occupants** | Option<**i32**> |  | [optional]
**min_price** | Option<**i32**> |  | [optional]
**minimum_interest_count** | Option<**i32**> |  | [optional]
**minimum_remaining_minutes** | Option<**i32**> |  | [optional]
**mode** | Option<**String**> |  | [optional]
**n** | Option<**i32**> |  | [optional]
**name** | [**models::DynamicContentRowName**](DynamicContentRowName.md) |  | 
**non_featured_results** | Option<**String**> |  | [optional]
**notag** | Option<**Vec<String>**> |  | [optional]
**params** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**personalized_results** | Option<**String**> |  | [optional]
**platform** | **String** | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**region** | Option<**String**> |  | [optional]
**scope** | Option<**String**> |  | [optional]
**short_name** | Option<[**models::DynamicContentRowShortName**](DynamicContentRowShortName.md)> |  | [optional]
**sort_heading** | Option<**String**> |  | [optional]
**sort_order** | Option<**String**> |  | [optional]
**sort_ownership** | Option<**String**> |  | [optional]
**style** | Option<**String**> |  | [optional]
**tag** | Option<**String**> | Tag to filter content for this row. Not a `Tag`: that type forbids the empty string, which this field uses for a row that is not tag-filtered. | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**r#type** | Option<**String**> | Type is not present if it is a world. | [optional]
**upcoming_offset_minutes** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


