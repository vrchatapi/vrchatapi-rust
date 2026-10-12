# SubmitModerationReportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **String** | Valid values are the keys of the object `$.reportOptions[type]` from `GET /config`. Descriptions of these are found at `$.reportCategories[type]`. | 
**content_id** | **String** | The id of the user, group, world, avatar, inventory item, print, etc. being reported. | 
**description** | Option<**String**> | The subjective reason for the report | [optional]
**details** | Option<[**models::SubmitModerationReportRequestDetails**](SubmitModerationReportRequestDetails.md)> | Relevant details specific to the type of the report. `fileId` is for the image file attached to an inventory item, such as an emoji. `holderId` is for the user who owns an inventory item, such as a emoji. | [optional]
**reason** | **String** | Valid values are the strings in the array `$.reportOptions[type][category]` from `GET /config`. Descriptions of these are found at `$.reportReasons[type]`. | 
**r#type** | **String** | Valid values are the keys of the object `$.reportOptions` from `GET /config`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


