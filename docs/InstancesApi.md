# \InstancesApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_instance**](InstancesApi.md#close_instance) | **DELETE** /instances/{worldId}:{instanceId} | Close Instance
[**create_instance**](InstancesApi.md#create_instance) | **POST** /instances | Create Instance
[**get_instance**](InstancesApi.md#get_instance) | **GET** /instances/{worldId}:{instanceId} | Get Instance
[**get_instance_by_short_name**](InstancesApi.md#get_instance_by_short_name) | **GET** /instances/s/{shortName} | Get Instance By Short Name
[**get_short_name**](InstancesApi.md#get_short_name) | **GET** /instances/{worldId}:{instanceId}/shortName | Get Instance Short Name
[**send_self_invite**](InstancesApi.md#send_self_invite) | **POST** /instances/{worldId}:{instanceId}/invite | Send Self Invite



## close_instance

> crate::models::Instance close_instance(world_id, instance_id, hard_close, closed_at)
Close Instance

Close an instance or update the closedAt time when it will be closed.  You can only close an instance if the ownerId is yourself or if the instance owner is a group and you have the `group-instance-moderate` permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |
**hard_close** | Option<**bool**> | Whether to hard close the instance. Defaults to false. |  |
**closed_at** | Option<**String**> | The time after which users won't be allowed to join the instances. If omitted, the instance will be closed immediately. |  |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> crate::models::Instance create_instance(create_instance_request)
Create Instance

Create an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_instance_request** | [**CreateInstanceRequest**](CreateInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> crate::models::Instance get_instance(world_id, instance_id)
Get Instance

Returns an instance. Please read [Instances Tutorial](https://vrchatapi.github.io/tutorials/instances/) for more information on Instances.  If an invalid instanceId is provided, this endpoint will simply return \"null\"!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_by_short_name

> crate::models::Instance get_instance_by_short_name(short_name)
Get Instance By Short Name

Returns an instance. Please read [Instances Tutorial](https://vrchatapi.github.io/tutorials/instances/) for more information on Instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**short_name** | **String** | Must be a valid instance short name. | [required] |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_short_name

> crate::models::InstanceShortNameResponse get_short_name(world_id, instance_id)
Get Instance Short Name

Returns an instance short name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |

### Return type

[**crate::models::InstanceShortNameResponse**](InstanceShortNameResponse.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_self_invite

> crate::models::Success send_self_invite(world_id, instance_id)
Send Self Invite

Sends an invite to the instance to yourself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

