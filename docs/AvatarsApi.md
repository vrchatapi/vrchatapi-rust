# \AvatarsApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_avatar**](AvatarsApi.md#create_avatar) | **POST** /avatars | Create Avatar
[**delete_avatar**](AvatarsApi.md#delete_avatar) | **DELETE** /avatars/{avatarId} | Delete Avatar
[**get_avatar**](AvatarsApi.md#get_avatar) | **GET** /avatars/{avatarId} | Get Avatar
[**get_favorited_avatars**](AvatarsApi.md#get_favorited_avatars) | **GET** /avatars/favorites | List Favorited Avatars
[**get_own_avatar**](AvatarsApi.md#get_own_avatar) | **GET** /users/{userId}/avatar | Get Own Avatar
[**search_avatars**](AvatarsApi.md#search_avatars) | **GET** /avatars | Search Avatars
[**select_avatar**](AvatarsApi.md#select_avatar) | **PUT** /avatars/{avatarId}/select | Select Avatar
[**select_fallback_avatar**](AvatarsApi.md#select_fallback_avatar) | **PUT** /avatars/{avatarId}/selectFallback | Select Fallback Avatar
[**update_avatar**](AvatarsApi.md#update_avatar) | **PUT** /avatars/{avatarId} | Update Avatar



## create_avatar

> crate::models::Avatar create_avatar(create_avatar_request)
Create Avatar

Create an avatar. It's possible to optionally specify a ID if you want a custom one. Attempting to create an Avatar with an already claimed ID will result in a DB error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_avatar_request** | Option<[**CreateAvatarRequest**](CreateAvatarRequest.md)> |  |  |

### Return type

[**crate::models::Avatar**](Avatar.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_avatar

> crate::models::Avatar delete_avatar(avatar_id)
Delete Avatar

Delete an avatar. Notice an avatar is never fully \"deleted\", only its ReleaseStatus is set to \"hidden\" and the linked Files are deleted. The AvatarID is permanently reserved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**avatar_id** | **String** | Must be a valid avatar ID. | [required] |

### Return type

[**crate::models::Avatar**](Avatar.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatar

> crate::models::Avatar get_avatar(avatar_id)
Get Avatar

Get information about a specific Avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**avatar_id** | **String** | Must be a valid avatar ID. | [required] |

### Return type

[**crate::models::Avatar**](Avatar.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorited_avatars

> Vec<crate::models::Avatar> get_favorited_avatars(featured, sort, n, order, offset, search, tag, notag, release_status, max_unity_version, min_unity_version, platform, user_id)
List Favorited Avatars

Search and list favorited avatars by query filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**featured** | Option<**bool**> | Filters on featured results. |  |
**sort** | Option<[**SortOption**](.md)> | The sort order of the results. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**order** | Option<[**OrderOption**](.md)> | Result ordering |  |
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**search** | Option<**String**> | Filters by world name. |  |
**tag** | Option<**String**> | Tags to include (comma-separated). Any of the tags needs to be present. |  |
**notag** | Option<**String**> | Tags to exclude (comma-separated). |  |
**release_status** | Option<[**ReleaseStatus**](.md)> | Filter by ReleaseStatus. |  |
**max_unity_version** | Option<**String**> | The maximum Unity version supported by the asset. |  |
**min_unity_version** | Option<**String**> | The minimum Unity version supported by the asset. |  |
**platform** | Option<**String**> | The platform the asset supports. |  |
**user_id** | Option<**String**> | Target user to see information on, admin-only. |  |

### Return type

[**Vec<crate::models::Avatar>**](Avatar.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_own_avatar

> crate::models::Avatar get_own_avatar(user_id)
Get Own Avatar

Get the current avatar for the user. This will return an error for any other user than the one logged in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::Avatar**](Avatar.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_avatars

> Vec<crate::models::Avatar> search_avatars(featured, sort, user, user_id, n, order, offset, tag, notag, release_status, max_unity_version, min_unity_version, platform)
Search Avatars

Search and list avatars by query filters. You can only search your own or featured avatars. It is not possible as a normal user to search other peoples avatars.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**featured** | Option<**bool**> | Filters on featured results. |  |
**sort** | Option<[**SortOption**](.md)> | The sort order of the results. |  |
**user** | Option<**String**> | Set to `me` for searching own avatars. |  |
**user_id** | Option<**String**> | Filter by UserID. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**order** | Option<[**OrderOption**](.md)> | Result ordering |  |
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**tag** | Option<**String**> | Tags to include (comma-separated). Any of the tags needs to be present. |  |
**notag** | Option<**String**> | Tags to exclude (comma-separated). |  |
**release_status** | Option<[**ReleaseStatus**](.md)> | Filter by ReleaseStatus. |  |
**max_unity_version** | Option<**String**> | The maximum Unity version supported by the asset. |  |
**min_unity_version** | Option<**String**> | The minimum Unity version supported by the asset. |  |
**platform** | Option<**String**> | The platform the asset supports. |  |

### Return type

[**Vec<crate::models::Avatar>**](Avatar.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## select_avatar

> crate::models::CurrentUser select_avatar(avatar_id)
Select Avatar

Switches into that avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**avatar_id** | **String** | Must be a valid avatar ID. | [required] |

### Return type

[**crate::models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## select_fallback_avatar

> crate::models::CurrentUser select_fallback_avatar(avatar_id)
Select Fallback Avatar

Switches into that avatar as your fallback avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**avatar_id** | **String** | Must be a valid avatar ID. | [required] |

### Return type

[**crate::models::CurrentUser**](CurrentUser.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_avatar

> crate::models::Avatar update_avatar(avatar_id, update_avatar_request)
Update Avatar

Update information about a specific avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**avatar_id** | **String** | Must be a valid avatar ID. | [required] |
**update_avatar_request** | Option<[**UpdateAvatarRequest**](UpdateAvatarRequest.md)> |  |  |

### Return type

[**crate::models::Avatar**](Avatar.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

