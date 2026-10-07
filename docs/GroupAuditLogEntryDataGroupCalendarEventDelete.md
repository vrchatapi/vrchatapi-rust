# GroupAuditLogEntryDataGroupCalendarEventDelete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_type** | [**models::CalendarEventAccess**](CalendarEventAccess.md) |  | 
**description** | **String** | The description of the calendar event. | 
**image_id** | **String** | The image file ID for the event. | 
**title** | **String** | The title of the calendar event. | 
**r#type** | **String** | The type of calendar entry. | 
**category** | **String** | The category of the event. | 
**close_instance_after_end_minutes** | **i32** | Minutes after the event ends to close the instance. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | The creation timestamp. | 
**deleted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The deletion timestamp. | 
**duration_in_ms** | **i32** | The duration of the event in milliseconds. | 
**ends_at** | **chrono::DateTime<chrono::FixedOffset>** | The end timestamp. | 
**featured** | **bool** | Whether the event is featured. | 
**guest_early_join_minutes** | **i32** | Minutes before the start that guests can join. | 
**host_early_join_minutes** | **i32** | Minutes before the start that hosts can join. | 
**interested_user_count** | **i32** | The number of interested users. | 
**is_draft** | **bool** | Whether the event is a draft. | 
**languages** | **Vec<String>** | The languages for the event. | 
**occurrence_kind** | [**models::CalendarEventOccurrenceKind**](CalendarEventOccurrenceKind.md) |  | 
**occurrence_modified** | Option<**String**> |  | 
**owner_id** | **String** | The ID of the group that owns the event. | 
**platforms** | **Vec<String>** | The supported platforms. | 
**recurrence** | Option<[**models::CalendarEventRecurrence**](CalendarEventRecurrence.md)> | The recurrence rule. | 
**role_ids** | Option<**Vec<String>**> | Group roles that may join this event. | 
**series_id** | Option<**String**> | The ID of the recurring series the event belongs to. | 
**short_code** | Option<**String**> | The short code. | 
**starts_at** | **chrono::DateTime<chrono::FixedOffset>** | The start timestamp. | 
**tags** | **Vec<String>** | The event tags. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | The last update timestamp. | 
**uses_instance_overflow** | **bool** | Whether the event uses instance overflow. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


