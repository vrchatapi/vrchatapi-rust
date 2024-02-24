# GroupAuditLogEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**group_id** | Option<**String**> |  | [optional]
**actor_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**actor_display_name** | Option<**String**> |  | [optional]
**target_id** | Option<**String**> | Typically GroupID or GroupRoleID, but could be other types of IDs. | [optional]
**event_type** | Option<**String**> | The type of event that occurred. This is a string that is prefixed with the type of object that the event occurred on. For example, a group role update event would be prefixed with `group.role`. | [optional][default to group.update]
**description** | Option<**String**> | A human-readable description of the event. | [optional]
**data** | Option<[**serde_json::Value**](.md)> | The data associated with the event. The format of this data is dependent on the event type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


