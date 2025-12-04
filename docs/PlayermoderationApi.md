# \PlayermoderationApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_all_player_moderations**](PlayermoderationApi.md#clear_all_player_moderations) | **DELETE** /auth/user/playermoderations | Clear All Player Moderations
[**get_player_moderations**](PlayermoderationApi.md#get_player_moderations) | **GET** /auth/user/playermoderations | Search Player Moderations
[**moderate_user**](PlayermoderationApi.md#moderate_user) | **POST** /auth/user/playermoderations | Moderate User
[**unmoderate_user**](PlayermoderationApi.md#unmoderate_user) | **PUT** /auth/user/unplayermoderate | Unmoderate User



## clear_all_player_moderations

> models::Success clear_all_player_moderations()
Clear All Player Moderations

⚠️ **This will delete every single player moderation you've ever made.**

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


## get_player_moderations

> Vec<models::PlayerModeration> get_player_moderations(r#type, target_user_id)
Search Player Moderations

Returns a list of all player moderations made by **you**.  This endpoint does not have pagination, and will return *all* results. Use query parameters to limit your query if needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<[**PlayerModerationType**](.md)> | Must be one of PlayerModerationType. |  |
**target_user_id** | Option<**String**> | Must be valid UserID. |  |

### Return type

[**Vec<models::PlayerModeration>**](PlayerModeration.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## moderate_user

> models::PlayerModeration moderate_user(moderate_user_request)
Moderate User

Moderate a user, e.g. unmute them or show their avatar.  Please see the [Player Moderation docs](https://vrchatapi.github.io/docs/api/#tag--playermoderation) on what playerModerations are, and how they differ from staff moderations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moderate_user_request** | [**ModerateUserRequest**](ModerateUserRequest.md) |  | [required] |

### Return type

[**models::PlayerModeration**](PlayerModeration.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmoderate_user

> models::Success unmoderate_user(moderate_user_request)
Unmoderate User

Removes a player moderation previously added through `moderateUser`. E.g if you previously have shown their avatar, but now want to reset it to default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moderate_user_request** | [**ModerateUserRequest**](ModerateUserRequest.md) |  | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

