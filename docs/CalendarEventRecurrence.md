# CalendarEventRecurrence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**days_of_week** | Option<[**Vec<models::CalendarDayOfWeek>**](CalendarDayOfWeek.md)> | Which days of the week the event will be scheduled, only valid/present for \"weekly\" recurring events | [optional]
**end** | Option<[**models::CalendarEventRecurrenceEnd**](CalendarEventRecurrenceEnd.md)> |  | [optional]
**frequency** | [**models::CalendarEventFrequency**](CalendarEventFrequency.md) |  | 
**interval** | **i32** | How often the event will be scheduled, in units of \"frequency\" | 
**timezone** | **String** | The timezone the event will be scheduled in, in Area/Location format | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


