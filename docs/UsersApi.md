# \UsersApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_tags**](UsersApi.md#add_tags) | **POST** /users/{userId}/addTags | Add User Tags
[**check_user_persistence_exists**](UsersApi.md#check_user_persistence_exists) | **GET** /users/{userId}/{worldId}/persist/exists | Check User Persistence Exists
[**delete_user_persistence**](UsersApi.md#delete_user_persistence) | **DELETE** /users/{userId}/{worldId}/persist | Delete User Persistence
[**get_user**](UsersApi.md#get_user) | **GET** /users/{userId} | Get User by ID
[**get_user_by_name**](UsersApi.md#get_user_by_name) | **GET** /users/{username}/name | Get User by Username
[**get_user_feedback**](UsersApi.md#get_user_feedback) | **GET** /users/{userId}/feedback | Get User Feedback
[**get_user_group_instances**](UsersApi.md#get_user_group_instances) | **GET** /users/{userId}/instances/groups | Get User Group Instances
[**get_user_group_requests**](UsersApi.md#get_user_group_requests) | **GET** /users/{userId}/groups/requested | Get User Group Requests
[**get_user_groups**](UsersApi.md#get_user_groups) | **GET** /users/{userId}/groups | Get User Groups
[**get_user_note**](UsersApi.md#get_user_note) | **GET** /userNotes/{userNoteId} | Get User Note
[**get_user_notes**](UsersApi.md#get_user_notes) | **GET** /userNotes | Get User Notes
[**get_user_represented_group**](UsersApi.md#get_user_represented_group) | **GET** /users/{userId}/groups/represented | Get user's current represented group
[**remove_tags**](UsersApi.md#remove_tags) | **POST** /users/{userId}/removeTags | Remove User Tags
[**search_users**](UsersApi.md#search_users) | **GET** /users | Search All Users
[**update_badge**](UsersApi.md#update_badge) | **PUT** /users/{userId}/badges/{badgeId} | Update User Badge
[**update_user**](UsersApi.md#update_user) | **PUT** /users/{userId} | Update User Info
[**update_user_note**](UsersApi.md#update_user_note) | **POST** /userNotes | Update User Note



## add_tags

> models::CurrentUser add_tags(user_id, change_user_tags_request)
Add User Tags

Adds tags to the user's profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**change_user_tags_request** | [**ChangeUserTagsRequest**](ChangeUserTagsRequest.md) |  | [required] |

### Return type

[**models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_user_persistence_exists

> check_user_persistence_exists(user_id, world_id)
Check User Persistence Exists

Checks whether the user has persistence data for a given world

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_persistence

> delete_user_persistence(user_id, world_id)
Delete User Persistence

Deletes the user's persistence data for a given world

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::User get_user(user_id)
Get User by ID

Get public user information about a specific user using their ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_name

> models::User get_user_by_name(username)
Get User by Username

~~Get public user information about a specific user using their name.~~  **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429). This endpoint now require Admin Credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username of the user | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_feedback

> Vec<models::Feedback> get_user_feedback(user_id, content_id, n, offset)
Get User Feedback

Get user's submitted feedback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**content_id** | Option<**bool**> | Filter for users' previously submitted feedback, e.g., a groupId, useeId, avatarId, etc. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::Feedback>**](Feedback.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group_instances

> models::GetUserGroupInstances200Response get_user_group_instances(user_id)
Get User Group Instances

Returns a list of group instances for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::GetUserGroupInstances200Response**](getUserGroupInstances_200_response.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group_requests

> Vec<models::Group> get_user_group_requests(user_id)
Get User Group Requests

Returns a list of Groups the user has requested to be invited into.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**Vec<models::Group>**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_groups

> Vec<models::LimitedUserGroups> get_user_groups(user_id)
Get User Groups

Get user's public groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**Vec<models::LimitedUserGroups>**](LimitedUserGroups.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_note

> models::UserNote get_user_note(user_note_id)
Get User Note

Get a particular user note

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_note_id** | **String** | Must be a valid user note ID. | [required] |

### Return type

[**models::UserNote**](UserNote.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_notes

> Vec<models::UserNote> get_user_notes(n, offset)
Get User Notes

Get recently updated user notes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::UserNote>**](UserNote.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_represented_group

> models::RepresentedGroup get_user_represented_group(user_id)
Get user's current represented group

Returns the current group that the user is currently representing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::RepresentedGroup**](representedGroup.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_tags

> models::CurrentUser remove_tags(user_id, change_user_tags_request)
Remove User Tags

Removes tags from the user's profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**change_user_tags_request** | [**ChangeUserTagsRequest**](ChangeUserTagsRequest.md) |  | [required] |

### Return type

[**models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> Vec<models::LimitedUserSearch> search_users(search, developer_type, n, offset)
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

[**Vec<models::LimitedUserSearch>**](LimitedUserSearch.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_badge

> update_badge(user_id, badge_id, update_user_badge_request)
Update User Badge

Updates a user's badge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**badge_id** | **String** | Must be a valid badge ID. | [required] |
**update_user_badge_request** | [**UpdateUserBadgeRequest**](UpdateUserBadgeRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::CurrentUser update_user(user_id, update_user_request)
Update User Info

Update a users information such as the email and birthday.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**update_user_request** | Option<[**UpdateUserRequest**](UpdateUserRequest.md)> |  |  |

### Return type

[**models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_note

> models::UserNote update_user_note(update_user_note_request)
Update User Note

Updates the currently authenticated user's note on a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_note_request** | [**UpdateUserNoteRequest**](UpdateUserNoteRequest.md) |  | [required] |

### Return type

[**models::UserNote**](UserNote.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

