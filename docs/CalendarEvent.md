# CalendarEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_type** | [**models::CalendarEventAccess**](CalendarEventAccess.md) |  | 
**category** | [**models::CalendarEventCategory**](CalendarEventCategory.md) |  | 
**close_instance_after_end_minutes** | Option<**i32**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**deleted_at** | Option<**String**> |  | [optional]
**description** | **String** |  | 
**duration_in_ms** | Option<**i32**> |  | [optional]
**ends_at** | **String** |  | 
**featured** | Option<**bool**> |  | [optional]
**guest_early_join_minutes** | Option<**i32**> |  | [optional]
**host_early_join_minutes** | Option<**i32**> |  | [optional]
**id** | **String** |  | 
**image_id** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**interested_user_count** | Option<**i32**> |  | [optional]
**is_draft** | Option<**bool**> |  | [optional]
**languages** | Option<**Vec<String>**> | Languages that might be spoken at this event | [optional]
**owner_id** | Option<**String**> |  | [optional]
**platforms** | Option<[**Vec<models::CalendarEventPlatform>**](CalendarEventPlatform.md)> |  | [optional]
**role_ids** | Option<**Vec<String>**> | Group roles that may join this event | [optional]
**starts_at** | **String** |  | 
**tags** | Option<**Vec<String>**> | Custom tags for this event | [optional]
**title** | **String** |  | 
**r#type** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**user_interest** | Option<[**models::CalendarEventUserInterest**](CalendarEvent_userInterest.md)> |  | [optional]
**uses_instance_overflow** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


