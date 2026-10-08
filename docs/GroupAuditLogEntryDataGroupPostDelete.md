# GroupAuditLogEntryDataGroupPostDelete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_id** | **String** | The ID of the post author. | 
**image_id** | Option<**String**> | The image file ID attached to the post. | 
**text** | **String** | The text content of the post. | 
**title** | **String** | The title of the post. | 
**visibility** | [**models::GroupPostVisibility**](GroupPostVisibility.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | The creation timestamp of the post. | 
**editor_id** | Option<**String**> | The ID of the user who last edited the post. | 
**image_url** | Option<**String**> | The URL of the post image. | 
**role_ids** | **Vec<String>** | The role IDs that could see the post. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | The last update timestamp of the post. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


