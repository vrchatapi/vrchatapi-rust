# \PrintsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_print**](PrintsApi.md#delete_print) | **DELETE** /prints/{printId} | Delete Print
[**get_print**](PrintsApi.md#get_print) | **GET** /prints/{printId} | Get Print
[**get_user_prints**](PrintsApi.md#get_user_prints) | **GET** /prints/user/{userId} | Get Own Prints



## delete_print

> delete_print(print_id)
Delete Print

Returns a print.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**print_id** | **String** | Print ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_print

> models::Print get_print(print_id)
Get Print

Returns a print.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**print_id** | **String** | Print ID. | [required] |

### Return type

[**models::Print**](Print.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_prints

> Vec<models::Print> get_user_prints(user_id)
Get Own Prints

Returns a list of all prints of the user. User id has to be your own userId, as you can't request other user's prints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**Vec<models::Print>**](Print.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

