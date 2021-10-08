# \SystemApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_config**](SystemApi.md#get_config) | **GET** /config | Fetch API Config
[**get_css**](SystemApi.md#get_css) | **GET** /css/app.js | Download Frontend CSS
[**get_current_online_users**](SystemApi.md#get_current_online_users) | **GET** /visits | Current Online Users
[**get_health**](SystemApi.md#get_health) | **GET** /health | Check API Health
[**get_java_script**](SystemApi.md#get_java_script) | **GET** /js/app.js | Download Frontend JavaScript
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
Download Frontend CSS

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

Returns in plain format the number of currently online users.  **NOTE:** The response type is not of JSON, but is an integer in plain ASCII format.

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_health

> crate::models::InlineResponse2002 get_health()
Check API Health

~~Gets the overall health status, the server name, and the current build version tag of the API.~~  **DEPRECATED:** VRChat has suddenly restricted this endpoint for unknown reasons, and now always return 401 Unauthorized.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_java_script

> String get_java_script(variant, branch)
Download Frontend JavaScript

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

Returns in plain format the current time of the API server.  **NOTE:** The response type is not of JSON, but is a string in plain ASCII format.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

