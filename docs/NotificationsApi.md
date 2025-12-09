# \NotificationsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_friend_request**](NotificationsApi.md#accept_friend_request) | **PUT** /auth/user/notifications/{notificationId}/accept | Accept Friend Request
[**acknowledge_notification_v2**](NotificationsApi.md#acknowledge_notification_v2) | **POST** /notifications/{notificationId}/see | Acknowledge NotificationV2
[**clear_notifications**](NotificationsApi.md#clear_notifications) | **PUT** /auth/user/notifications/clear | Clear All Notifications
[**delete_all_notification_v2s**](NotificationsApi.md#delete_all_notification_v2s) | **DELETE** /notifications | Delete All NotificationV2s
[**delete_notification**](NotificationsApi.md#delete_notification) | **PUT** /auth/user/notifications/{notificationId}/hide | Delete Notification
[**delete_notification_v2**](NotificationsApi.md#delete_notification_v2) | **DELETE** /notifications/{notificationId} | Delete NotificationV2
[**get_notification**](NotificationsApi.md#get_notification) | **GET** /auth/user/notifications/{notificationId} | Show notification
[**get_notification_v2**](NotificationsApi.md#get_notification_v2) | **GET** /notifications/{notificationId} | Get NotificationV2
[**get_notification_v2s**](NotificationsApi.md#get_notification_v2s) | **GET** /notifications | List NotificationV2s
[**get_notifications**](NotificationsApi.md#get_notifications) | **GET** /auth/user/notifications | List Notifications
[**mark_notification_as_read**](NotificationsApi.md#mark_notification_as_read) | **PUT** /auth/user/notifications/{notificationId}/see | Mark Notification As Read
[**reply_notification_v2**](NotificationsApi.md#reply_notification_v2) | **POST** /notifications/{notificationId}/reply | Reply NotificationV2
[**respond_notification_v2**](NotificationsApi.md#respond_notification_v2) | **POST** /notifications/{notificationId}/respond | Respond NotificationV2



## accept_friend_request

> models::Success accept_friend_request(notification_id)
Accept Friend Request

Accept a friend request by notification `frq_` ID. Friend requests can be found using the NotificationsAPI `getNotifications` by filtering of type `friendRequest`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## acknowledge_notification_v2

> models::NotificationV2 acknowledge_notification_v2(notification_id)
Acknowledge NotificationV2

Acknowledge a specific notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::NotificationV2**](NotificationV2.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_notifications

> models::Success clear_notifications()
Clear All Notifications

Clear **all** notifications.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_notification_v2s

> models::Success delete_all_notification_v2s()
Delete All NotificationV2s

Delete all of the current user's notifications.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification

> models::Notification delete_notification(notification_id)
Delete Notification

Delete a notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::Notification**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_v2

> models::Success delete_notification_v2(notification_id)
Delete NotificationV2

Delete a specific notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification

> models::Notification get_notification(notification_id)
Show notification

Get a notification by notification `not_` ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::Notification**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_v2

> models::NotificationV2 get_notification_v2(notification_id)
Get NotificationV2

Get a specific notification. Appears to require admin credentials by default. Expect a 403 Forbidden error response for normal users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::NotificationV2**](NotificationV2.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_v2s

> Vec<models::NotificationV2> get_notification_v2s(limit)
List NotificationV2s

Retrieve all of the current user's notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum number of entries to get. |  |

### Return type

[**Vec<models::NotificationV2>**](NotificationV2.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> Vec<models::Notification> get_notifications(r#type, sent, hidden, after, n, offset)
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

[**Vec<models::Notification>**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_notification_as_read

> models::Notification mark_notification_as_read(notification_id)
Mark Notification As Read

Mark a notification as seen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::Notification**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reply_notification_v2

> models::NotificationV2 reply_notification_v2(notification_id, body)
Reply NotificationV2

Reply to a specific notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::NotificationV2**](NotificationV2.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## respond_notification_v2

> models::NotificationV2 respond_notification_v2(notification_id, respond_notification_v2_request)
Respond NotificationV2

Respond to a specific notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Must be a valid notification ID. | [required] |
**respond_notification_v2_request** | [**RespondNotificationV2Request**](RespondNotificationV2Request.md) |  | [required] |

### Return type

[**models::NotificationV2**](NotificationV2.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

