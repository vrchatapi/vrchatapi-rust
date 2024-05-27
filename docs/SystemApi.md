# \SystemApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_config**](SystemApi.md#get_config) | **GET** /config | Fetch API Config
[**get_css**](SystemApi.md#get_css) | **GET** /css/app.css | Download CSS
[**get_current_online_users**](SystemApi.md#get_current_online_users) | **GET** /visits | Current Online Users
[**get_health**](SystemApi.md#get_health) | **GET** /health | Check API Health
[**get_info_push**](SystemApi.md#get_info_push) | **GET** /infoPush | Show Information Notices
[**get_java_script**](SystemApi.md#get_java_script) | **GET** /js/app.js | Download JavaScript
[**get_system_time**](SystemApi.md#get_system_time) | **GET** /time | Current System Time



## get_config

> crate::models::ApiConfig get_config()
Fetch API Config

API config contains configuration that the clients needs to work properly.  Currently the most important value here is `clientApiKey` which is used for all other API endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiConfig**](APIConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_css

> String get_css(variant, branch)
Download CSS

Fetches the CSS code to the frontend React website.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant** | Option<**String**> | Specifies which `variant` of the site. Public is the end-user site, while `internal` is the staff-only site with special pages for moderation and management. |  |[default to public]
**branch** | Option<**String**> | Specifies which git branch the site should load frontend source code from. |  |[default to main]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/css, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_online_users

> i32 get_current_online_users()
Current Online Users

Returns the current number of online users.  **NOTE:** The response type is not a JSON object, but a simple JSON integer.

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_health

> crate::models::ApiHealth get_health()
Check API Health

~~Gets the overall health status, the server name, and the current build version tag of the API.~~  **DEPRECATED:** VRChat has suddenly restricted this endpoint for unknown reasons, and now always return 401 Unauthorized.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiHealth**](APIHealth.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_info_push

> Vec<crate::models::InfoPush> get_info_push(require, include)
Show Information Notices

IPS (Info Push System) is a system for VRChat to push out dynamic information to the client. This is primarily used by the Quick-Menu info banners, but can also be used to e.g. alert you to update your game to the latest version.  `include` is used to query what Information Pushes should be included in the response. If include is missing or empty, then no notices will normally be returned. This is an \"any of\" search.  `require` is used to limit what Information Pushes should be included in the response. This is usually used in combination with `include`, and is an \"all of\" search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**require** | Option<**String**> | Tags to include (comma-separated). All of the tags needs to be present. |  |
**include** | Option<**String**> | Tags to include (comma-separated). Any of the tags needs to be present. |  |

### Return type

[**Vec<crate::models::InfoPush>**](InfoPush.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_java_script

> String get_java_script(variant, branch)
Download JavaScript

Fetches the JavaScript code to the frontend React website.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant** | Option<**String**> | Specifies which `variant` of the site. Public is the end-user site, while `internal` is the staff-only site with special pages for moderation and management. |  |[default to public]
**branch** | Option<**String**> | Specifies which git branch the site should load frontend source code from. |  |[default to main]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/javascript, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_time

> String get_system_time()
Current System Time

Returns the current time of the API server.  **NOTE:** The response type is not a JSON object, but a simple JSON string.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

