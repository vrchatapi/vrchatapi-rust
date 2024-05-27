# \FriendsApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_friend_request**](FriendsApi.md#delete_friend_request) | **DELETE** /user/{userId}/friendRequest | Delete Friend Request
[**friend**](FriendsApi.md#friend) | **POST** /user/{userId}/friendRequest | Send Friend Request
[**get_friend_status**](FriendsApi.md#get_friend_status) | **GET** /user/{userId}/friendStatus | Check Friend Status
[**get_friends**](FriendsApi.md#get_friends) | **GET** /auth/user/friends | List Friends
[**unfriend**](FriendsApi.md#unfriend) | **DELETE** /auth/user/friends/{userId} | Unfriend



## delete_friend_request

> crate::models::Success delete_friend_request(user_id)
Delete Friend Request

Deletes an outgoing pending friend request to another user. To delete an incoming friend request, use the `deleteNotification` endpoint instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## friend

> crate::models::Notification friend(user_id)
Send Friend Request

Send a friend request to another user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::Notification**](Notification.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_friend_status

> crate::models::FriendStatus get_friend_status(user_id)
Check Friend Status

Retrieve if the user is currently a friend with a given user, if they have an outgoing friend request, and if they have an incoming friend request. The proper way to receive and accept friend request is by checking if the user has an incoming `Notification` of type `friendRequest`, and then accepting that notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::FriendStatus**](FriendStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_friends

> Vec<crate::models::LimitedUser> get_friends(offset, n, offline)
List Friends

List information about friends.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offline** | Option<**bool**> | Returns *only* offline users if true, returns only online and active users if false |  |

### Return type

[**Vec<crate::models::LimitedUser>**](LimitedUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfriend

> crate::models::Success unfriend(user_id)
Unfriend

Unfriend a user by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

