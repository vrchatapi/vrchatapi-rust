# GroupAuditLogEntryDataGroupPostCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_id** | **String** | The ID of the post author. | 
**image_id** | Option<**String**> | The image file ID attached to the post. | 
**text** | **String** | The text content of the post. | 
**title** | **String** | The title of the post. | 
**visibility** | [**models::GroupPostVisibility**](GroupPostVisibility.md) |  | 
**role_ids** | Option<**Vec<String>**> | The role IDs that can see the post. | 
**send_notification** | **bool** | Whether a notification was sent for this post. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


