# \UsersApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user**](UsersApi.md#get_user) | **GET** /users/{userId} | Get User by ID
[**get_user_by_name**](UsersApi.md#get_user_by_name) | **GET** /users/{username}/name | Get User by Username
[**get_user_group_requests**](UsersApi.md#get_user_group_requests) | **GET** /users/{userId}/groups/requested | Get User Group Requests
[**get_user_groups**](UsersApi.md#get_user_groups) | **GET** /users/{userId}/groups | Get User Groups
[**get_user_represented_group**](UsersApi.md#get_user_represented_group) | **GET** /users/{userId}/groups/represented | Get user's current represented group
[**search_users**](UsersApi.md#search_users) | **GET** /users | Search All Users
[**update_user**](UsersApi.md#update_user) | **PUT** /users/{userId} | Update User Info



## get_user

> crate::models::User get_user(user_id)
Get User by ID

Get public user information about a specific user using their ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_name

> crate::models::User get_user_by_name(username)
Get User by Username

~~Get public user information about a specific user using their name.~~  **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429). This endpoint now require Admin Credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username of the user | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group_requests

> Vec<crate::models::Group> get_user_group_requests(user_id)
Get User Group Requests

Returns a list of Groups the user has requested to be invited into.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_groups

> Vec<crate::models::LimitedUserGroups> get_user_groups(user_id)
Get User Groups

Get user's public groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**Vec<crate::models::LimitedUserGroups>**](LimitedUserGroups.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_represented_group

> crate::models::RepresentedGroup get_user_represented_group(user_id)
Get user's current represented group

Returns the current group that the user is currently representing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::RepresentedGroup**](representedGroup.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> Vec<crate::models::LimitedUser> search_users(search, developer_type, n, offset)
Search All Users

Search and list any users by text query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Searches by `displayName`. Will return empty array if search query is empty or missing. |  |
**developer_type** | Option<**String**> | Active user by developer type, none for normal users and internal for moderators |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<crate::models::LimitedUser>**](LimitedUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::CurrentUser update_user(user_id, update_user_request)
Update User Info

Update a users information such as the email and birthday.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**update_user_request** | Option<[**UpdateUserRequest**](UpdateUserRequest.md)> |  |  |

### Return type

[**crate::models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

