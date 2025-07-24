# \PropsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_prop**](PropsApi.md#get_prop) | **GET** /props/{propId} | Get Prop



## get_prop

> models::Prop get_prop(prop_id)
Get Prop

Returns a Prop object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

[**models::Prop**](Prop.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

