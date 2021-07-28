# \WorldsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_worlds**](WorldsApi.md#search_worlds) | **GET** /worlds | Search all worlds



## search_worlds

> Vec<crate::models::LimitedWorld> search_worlds(featured, sort, user, user_id, n, order, offset, search, tag, notag, release_status, max_unity_version, min_unity_version, max_asset_version, min_asset_version, platform)
Search all worlds

Search and list any worlds by text query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**featured** | Option<**bool**> | is the world featured |  |
**sort** | Option<**String**> |  |  |[default to order]
**user** | Option<**String**> | Set to `me` for searching own worlds |  |
**user_id** | Option<**String**> | Filter by creator id, use `me` for only worlds owned by current user |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**order** | Option<**String**> |  |  |[default to descending]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**search** | Option<**String**> | Searches by `name`. Will return empty array if search query is empty or missing. |  |
**tag** | Option<[**Vec<String>**](String.md)> | Filter by Tag |  |
**notag** | Option<[**Vec<String>**](String.md)> | Tags to exclude |  |
**release_status** | Option<**String**> |  |  |[default to hidden]
**max_unity_version** | Option<**String**> | Current unity version the game is using |  |
**min_unity_version** | Option<**String**> | The min unity version the world support |  |
**max_asset_version** | Option<**String**> | Current asset version the game is using |  |
**min_asset_version** | Option<**String**> | The min asset version the world support |  |
**platform** | Option<**String**> | The platform the world supports (usually standalonewindows) |  |

### Return type

[**Vec<crate::models::LimitedWorld>**](LimitedWorld.md)

### Authorization

[apiKeyCookie](../README.md#apiKeyCookie), [authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

