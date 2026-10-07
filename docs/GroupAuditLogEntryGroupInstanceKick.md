# GroupAuditLogEntryGroupInstanceKick

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_display_name** | **String** | The display name of the user who performed the action. | 
**actor_id** | **String** | The ID of the user who performed the action. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the action was performed. | 
**description** | **String** | A human-readable description of the event. | 
**event_type** | **EventType** |  (enum: group.instance.kick) | 
**group_id** | **String** | The ID of the group the entry belongs to. | 
**id** | **String** | The unique ID of this audit log entry. | 
**data** | [**models::GroupAuditLogEntryDataGroupInstanceModeration**](GroupAuditLogEntryDataGroupInstanceModeration.md) |  | 
**target_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


