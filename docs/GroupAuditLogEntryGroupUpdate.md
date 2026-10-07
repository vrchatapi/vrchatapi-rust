# GroupAuditLogEntryGroupUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_display_name** | **String** | The display name of the user who performed the action. | 
**actor_id** | **String** | The ID of the user who performed the action. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the action was performed. | 
**description** | **String** | A human-readable description of the event. | 
**event_type** | **EventType** |  (enum: group.update) | 
**group_id** | **String** | The ID of the group the entry belongs to. | 
**id** | **String** | The unique ID of this audit log entry. | 
**data** | [**models::GroupAuditLogEntryDataGroupUpdate**](GroupAuditLogEntryDataGroupUpdate.md) |  | 
**target_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


