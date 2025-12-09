# \CalendarApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group_calendar_event**](CalendarApi.md#create_group_calendar_event) | **POST** /calendar/{groupId}/event | Create a calendar event
[**delete_group_calendar_event**](CalendarApi.md#delete_group_calendar_event) | **DELETE** /calendar/{groupId}/{calendarId} | Delete a calendar event
[**discover_calendar_events**](CalendarApi.md#discover_calendar_events) | **GET** /calendar/discover | Discover calendar events
[**follow_group_calendar_event**](CalendarApi.md#follow_group_calendar_event) | **POST** /calendar/{groupId}/{calendarId}/follow | Follow a calendar event
[**get_calendar_events**](CalendarApi.md#get_calendar_events) | **GET** /calendar | List calendar events
[**get_featured_calendar_events**](CalendarApi.md#get_featured_calendar_events) | **GET** /calendar/featured | List featured calendar events
[**get_followed_calendar_events**](CalendarApi.md#get_followed_calendar_events) | **GET** /calendar/following | List followed calendar events
[**get_group_calendar_event**](CalendarApi.md#get_group_calendar_event) | **GET** /calendar/{groupId}/{calendarId} | Get a calendar event
[**get_group_calendar_event_ics**](CalendarApi.md#get_group_calendar_event_ics) | **GET** /calendar/{groupId}/{calendarId}.ics | Download calendar event as ICS
[**get_group_calendar_events**](CalendarApi.md#get_group_calendar_events) | **GET** /calendar/{groupId} | List a group's calendar events
[**get_group_next_calendar_event**](CalendarApi.md#get_group_next_calendar_event) | **GET** /calendar/{groupId}/next | Get next calendar event
[**search_calendar_events**](CalendarApi.md#search_calendar_events) | **GET** /calendar/search | Search for calendar events
[**update_group_calendar_event**](CalendarApi.md#update_group_calendar_event) | **PUT** /calendar/{groupId}/{calendarId}/event | Update a calendar event



## create_group_calendar_event

> models::CalendarEvent create_group_calendar_event(group_id, create_calendar_event_request)
Create a calendar event

Creates an event for a group on the calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_calendar_event_request** | [**CreateCalendarEventRequest**](CreateCalendarEventRequest.md) |  | [required] |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_calendar_event

> models::Success delete_group_calendar_event(group_id, calendar_id)
Delete a calendar event

Delete a group calendar event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**calendar_id** | **String** | Must be a valid calendar ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discover_calendar_events

> models::CalendarEventDiscovery discover_calendar_events(scope, categories, tags, featured_results, non_featured_results, personalized_results, minimum_interest_count, minimum_remaining_minutes, upcoming_offset_minutes, n, next_cursor)
Discover calendar events

Get a list of calendar events Initially, call without a `nextCursor` parameter For every successive call, use the `nextCursor` property returned in the previous call & the `number` of entries desired for this call The `nextCursor` internally keeps track of the `offset` of the results, the initial request parameters, and accounts for discrepancies that might arise from time elapsed between calls

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<[**CalendarEventDiscoveryScope**](.md)> | Scope for calendar event discovery. |  |
**categories** | Option<**String**> | Filter for calendar event discovery. |  |
**tags** | Option<**String**> | Filter for calendar event discovery. |  |
**featured_results** | Option<[**CalendarEventDiscoveryInclusion**](.md)> | Filter for calendar event discovery. |  |
**non_featured_results** | Option<[**CalendarEventDiscoveryInclusion**](.md)> | Filter for calendar event discovery. |  |
**personalized_results** | Option<[**CalendarEventDiscoveryInclusion**](.md)> | Filter for calendar event discovery. |  |
**minimum_interest_count** | Option<**i32**> | Filter for calendar event discovery. |  |
**minimum_remaining_minutes** | Option<**i32**> | Filter for calendar event discovery. |  |
**upcoming_offset_minutes** | Option<**i32**> | Filter for calendar event discovery. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**next_cursor** | Option<**String**> | Cursor returned from previous calendar discovery queries (see nextCursor property of the schema CalendarEventDiscovery). |  |

### Return type

[**models::CalendarEventDiscovery**](CalendarEventDiscovery.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_group_calendar_event

> models::CalendarEvent follow_group_calendar_event(group_id, calendar_id, follow_calendar_event_request)
Follow a calendar event

Follow or unfollow an event on a group's calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**calendar_id** | **String** | Must be a valid calendar ID. | [required] |
**follow_calendar_event_request** | [**FollowCalendarEventRequest**](FollowCalendarEventRequest.md) |  | [required] |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_calendar_events

> models::PaginatedCalendarEventList get_calendar_events(date, n, offset)
List calendar events

Get a list of a user's calendar events for the month in ?date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> | The month to search in. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**models::PaginatedCalendarEventList**](PaginatedCalendarEventList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_featured_calendar_events

> models::PaginatedCalendarEventList get_featured_calendar_events(date, n, offset)
List featured calendar events

Get a list of a featured calendar events for the month in ?date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> | The month to search in. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**models::PaginatedCalendarEventList**](PaginatedCalendarEventList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_followed_calendar_events

> models::PaginatedCalendarEventList get_followed_calendar_events(date, n, offset)
List followed calendar events

Get a list of a followed calendar events for the month in ?date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> | The month to search in. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**models::PaginatedCalendarEventList**](PaginatedCalendarEventList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_calendar_event

> models::CalendarEvent get_group_calendar_event(group_id, calendar_id)
Get a calendar event

Get a group calendar event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**calendar_id** | **String** | Must be a valid calendar ID. | [required] |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_calendar_event_ics

> std::path::PathBuf get_group_calendar_event_ics(group_id, calendar_id)
Download calendar event as ICS

Returns the specified calendar in iCalendar (ICS) format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**calendar_id** | **String** | Must be a valid calendar ID. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/calendar, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_calendar_events

> models::PaginatedCalendarEventList get_group_calendar_events(group_id, date, n, offset)
List a group's calendar events

Get a list of a group's calendar events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**date** | Option<**String**> | The month to search in. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**models::PaginatedCalendarEventList**](PaginatedCalendarEventList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_next_calendar_event

> models::CalendarEvent get_group_next_calendar_event(group_id)
Get next calendar event

Get the closest future calendar event scheduled for a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_calendar_events

> models::PaginatedCalendarEventList search_calendar_events(search_term, utc_offset, n, offset)
Search for calendar events

Get a list of calendar events by search terms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_term** | **String** | Search term for calendar events. | [required] |
**utc_offset** | Option<**i32**> | The offset from UTC in hours of the client or authenticated user. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**models::PaginatedCalendarEventList**](PaginatedCalendarEventList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_calendar_event

> models::CalendarEvent update_group_calendar_event(group_id, calendar_id, update_calendar_event_request)
Update a calendar event

Updates an event for a group on the calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**calendar_id** | **String** | Must be a valid calendar ID. | [required] |
**update_calendar_event_request** | [**UpdateCalendarEventRequest**](UpdateCalendarEventRequest.md) |  | [required] |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

