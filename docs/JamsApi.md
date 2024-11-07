# \JamsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_jam**](JamsApi.md#get_jam) | **GET** /jams/{jamId} | Show jam information
[**get_jam_submissions**](JamsApi.md#get_jam_submissions) | **GET** /jams/{jamId}/submissions | Show jam submissions
[**get_jams**](JamsApi.md#get_jams) | **GET** /jams | Show jams list



## get_jam

> models::Jam get_jam(jam_id)
Show jam information

Returns a jam.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jam_id** | **String** | Must be a valid query ID. | [required] |

### Return type

[**models::Jam**](Jam.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jam_submissions

> Vec<models::Submission> get_jam_submissions(jam_id)
Show jam submissions

Returns all submissions of a jam.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jam_id** | **String** | Must be a valid query ID. | [required] |

### Return type

[**Vec<models::Submission>**](Submission.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jams

> Vec<models::Jam> get_jams(r#type)
Show jams list

Lists World Jams or Avatar Jams, both currently running and ones that have ended.  `isActive` is used to select only active or already ended jams.  `type` is used to select only world or avatar jams, and can only take `world` or `avatar`. ``

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Only show jams of this type (`avatar` or `world`). |  |

### Return type

[**Vec<models::Jam>**](Jam.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

