# \PropsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_prop**](PropsApi.md#create_prop) | **POST** /props | Create Prop
[**delete_prop**](PropsApi.md#delete_prop) | **DELETE** /props/{propId} | Delete Prop
[**get_prop**](PropsApi.md#get_prop) | **GET** /props/{propId} | Get Prop
[**get_prop_publish_status**](PropsApi.md#get_prop_publish_status) | **GET** /props/{propId}/publish | Get Prop Publish Status
[**list_props**](PropsApi.md#list_props) | **GET** /props | List Props
[**publish_prop**](PropsApi.md#publish_prop) | **PUT** /props/{propId}/publish | Publish Prop
[**unpublish_prop**](PropsApi.md#unpublish_prop) | **DELETE** /props/{propId}/publish | Unpublish Prop
[**update_prop**](PropsApi.md#update_prop) | **PUT** /props/{propId} | Update Prop



## create_prop

> models::Prop create_prop(create_prop_request)
Create Prop

Create a Prop and return the new Prop object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_prop_request** | [**CreatePropRequest**](CreatePropRequest.md) |  | [required] |

### Return type

[**models::Prop**](Prop.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_prop

> delete_prop(prop_id)
Delete Prop

Delete a Prop.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_prop_publish_status

> models::PropPublishStatus get_prop_publish_status(prop_id)
Get Prop Publish Status

Returns a PropPublishStatus object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

[**models::PropPublishStatus**](PropPublishStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_props

> Vec<models::Prop> list_props(author_id, n, offset)
List Props

Returns a list Prop objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author_id** | **String** | Must be a valid user ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::Prop>**](Prop.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_prop

> models::PropPublishStatus publish_prop(prop_id)
Publish Prop

Publish a Prop and return the updated PropPublishStatus object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

[**models::PropPublishStatus**](PropPublishStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_prop

> models::PropPublishStatus unpublish_prop(prop_id)
Unpublish Prop

Unpublish a Prop and return the updated PropPublishStatus object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |

### Return type

[**models::PropPublishStatus**](PropPublishStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_prop

> models::Prop update_prop(prop_id, update_prop_request)
Update Prop

Updates a Prop and returns the updated Prop object. When updating the asset bundle, all of `name`, `assetUrl`, `platform`, `unityVersion`, `assetVersion`, `spawnType`, and `worldPlacementMask` must be present, as well as `propSignature` if this value is not blank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prop_id** | **String** | Prop ID. | [required] |
**update_prop_request** | [**UpdatePropRequest**](UpdatePropRequest.md) |  | [required] |

### Return type

[**models::Prop**](Prop.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

