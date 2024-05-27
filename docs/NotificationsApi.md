# \NotificationsApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_friend_request**](NotificationsApi.md#accept_friend_request) | **PUT** /auth/user/notifications/{notificationId}/accept | Accept Friend Request
[**clear_notifications**](NotificationsApi.md#clear_notifications) | **PUT** /auth/user/notifications/clear | Clear All Notifications
[**delete_notification**](NotificationsApi.md#delete_notification) | **PUT** /auth/user/notifications/{notificationId}/hide | Delete Notification
[**get_notifications**](NotificationsApi.md#get_notifications) | **GET** /auth/user/notifications | List Notifications
[**mark_notification_as_read**](NotificationsApi.md#mark_notification_as_read) | **PUT** /auth/user/notifications/{notificationId}/see | Mark Notification As Read



## accept_friend_request

> crate::models::Success accept_friend_request(notification_id)
Accept Friend Request

Accept a friend request by notification `frq_` ID. Friend requests can be found using the NotificationsAPI `getNotifications` by filtering of type `friendRequest`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_notifications

> crate::models::Success clear_notifications()
Clear All Notifications

Clear **all** notifications.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification

> crate::models::Notification delete_notification(notification_id)
Delete Notification

Delete a notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**crate::models::Notification**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> Vec<crate::models::Notification> get_notifications(r#type, sent, hidden, after, n, offset)
List Notifications

Retrieve all of the current user's notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Only send notifications of this type (can use `all` for all). This parameter no longer does anything, and is deprecated. |  |
**sent** | Option<**bool**> | Return notifications sent by the user. Must be false or omitted. |  |
**hidden** | Option<**bool**> | Whether to return hidden or non-hidden notifications. True only allowed on type `friendRequest`. |  |
**after** | Option<**String**> | Only return notifications sent after this Date. Ignored if type is `friendRequest`. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<crate::models::Notification>**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_notification_as_read

> crate::models::Notification mark_notification_as_read(notification_id)
Mark Notification As Read

Mark a notification as seen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**crate::models::Notification**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

