# \PlayermoderationApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_all_player_moderations**](PlayermoderationApi.md#clear_all_player_moderations) | **DELETE** /auth/user/playermoderations | Clear All Player Moderations
[**delete_player_moderation**](PlayermoderationApi.md#delete_player_moderation) | **DELETE** /auth/user/playermoderations/{playerModerationId} | Delete Player Moderation
[**get_player_moderation**](PlayermoderationApi.md#get_player_moderation) | **GET** /auth/user/playermoderations/{playerModerationId} | Get Player Moderation
[**get_player_moderations**](PlayermoderationApi.md#get_player_moderations) | **GET** /auth/user/playermoderations | Search Player Moderations
[**moderate_user**](PlayermoderationApi.md#moderate_user) | **POST** /auth/user/playermoderations | Moderate User
[**unmoderate_user**](PlayermoderationApi.md#unmoderate_user) | **PUT** /auth/user/unplayermoderate | Unmoderate User



## clear_all_player_moderations

> crate::models::Success clear_all_player_moderations()
Clear All Player Moderations

⚠️ **This will delete every single player moderation you've ever made.**

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


## delete_player_moderation

> crate::models::Success delete_player_moderation(player_moderation_id)
Delete Player Moderation

Deletes a specific player moderation based on it's `pmod_` ID. The website uses `unmoderateUser` instead. You can delete the same player moderation multiple times successfully.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_moderation_id** | **String** | Must be a valid `pmod_` ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player_moderation

> crate::models::PlayerModeration get_player_moderation(player_moderation_id)
Get Player Moderation

Returns a single Player Moderation. This returns the exact same amount of information as the more generalised `getPlayerModerations`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_moderation_id** | **String** | Must be a valid `pmod_` ID. | [required] |

### Return type

[**crate::models::PlayerModeration**](PlayerModeration.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player_moderations

> Vec<crate::models::PlayerModeration> get_player_moderations(r#type, target_user_id)
Search Player Moderations

Returns a list of all player moderations made by **you**.  This endpoint does not have pagination, and will return *all* results. Use query parameters to limit your query if needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Must be one of PlayerModerationType, except unblock. Unblocking simply removes a block. |  |
**target_user_id** | Option<**String**> | Must be valid UserID. |  |

### Return type

[**Vec<crate::models::PlayerModeration>**](PlayerModeration.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## moderate_user

> crate::models::PlayerModeration moderate_user(moderate_user_request)
Moderate User

Moderate a user, e.g. unmute them or show their avatar.  Please see the [Player Moderation docs](https://vrchatapi.github.io/docs/api/#tag--playermoderation) on what playerModerations are, and how they differ from staff moderations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moderate_user_request** | [**ModerateUserRequest**](ModerateUserRequest.md) |  | [required] |

### Return type

[**crate::models::PlayerModeration**](PlayerModeration.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmoderate_user

> crate::models::Success unmoderate_user(moderate_user_request)
Unmoderate User

Removes a player moderation previously added through `moderateUser`. E.g if you previously have shown their avatar, but now want to reset it to default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moderate_user_request** | [**ModerateUserRequest**](ModerateUserRequest.md) |  | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

