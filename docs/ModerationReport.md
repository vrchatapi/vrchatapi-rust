# ModerationReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **String** | Valid values are the keys of the object `$.reportOptions[type]` from `GET /config`. Descriptions of these are found at `$.reportCategories[type]`. | 
**content_id** | **String** |  | 
**content_name** | **String** |  | 
**content_thumbnail_image_url** | Option<**String**> |  | 
**description** | **String** | The subjective reason for the report | 
**evidence_required** | **bool** |  | 
**id** | **String** |  | 
**reason** | **String** | Valid values are the strings in the array `$.reportOptions[type][category]` from `GET /config`. Descriptions of these are found at `$.reportReasons[type]`. | 
**support_required** | **bool** |  | 
**r#type** | **String** | Valid values are the keys of the object `$.reportOptions` from `GET /config`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


