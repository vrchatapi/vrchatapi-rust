# GroupAuditLogEntryGroupInstanceAnnouncement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_display_name** | **String** | The display name of the user who performed the action. | 
**actor_id** | **String** | The ID of the user who performed the action. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the action was performed. | 
**description** | **String** | A human-readable description of the event. | 
**group_id** | **String** | The ID of the group the entry belongs to. | 
**id** | **String** | The unique ID of this audit log entry. | 
**data** | [**models::GroupAuditLogEntryDataGroupInstanceAnnouncement**](GroupAuditLogEntryDataGroupInstanceAnnouncement.md) |  | 
**event_type** | **EventType** |  (enum: group.instance.announcement) | 
**target_id** | **String** | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


