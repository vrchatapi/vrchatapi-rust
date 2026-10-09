# \InstancesApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_instance**](InstancesApi.md#close_instance) | **DELETE** /instances/{worldId}:{instanceId} | Close Instance
[**create_instance**](InstancesApi.md#create_instance) | **POST** /instances | Create Instance
[**discover_instances**](InstancesApi.md#discover_instances) | **GET** /instances/discover | Discover Instances
[**get_active_instances**](InstancesApi.md#get_active_instances) | **GET** /instances/active | List Active Instances
[**get_instance**](InstancesApi.md#get_instance) | **GET** /instances/{worldId}:{instanceId} | Get Instance
[**get_instance_by_short_name**](InstancesApi.md#get_instance_by_short_name) | **GET** /instances/s/{shortName} | Get Instance By Short Name
[**get_instance_categories**](InstancesApi.md#get_instance_categories) | **GET** /instanceCategories | List Instance Categories
[**get_instance_vibes**](InstancesApi.md#get_instance_vibes) | **GET** /instanceVibes | List Instance Vibes
[**get_recent_locations**](InstancesApi.md#get_recent_locations) | **GET** /instances/recent | List Recent Locations
[**get_short_name**](InstancesApi.md#get_short_name) | **GET** /instances/{worldId}:{instanceId}/shortName | Get Instance Short Name
[**update_instance**](InstancesApi.md#update_instance) | **PUT** /instances/{worldId}:{instanceId} | Update Instance



## close_instance

> models::Instance close_instance(world_id, instance_id, hard_close, closed_at)
Close Instance

Close an instance or update the closedAt time when it will be closed.  You can only close an instance if the ownerId is yourself or if the instance owner is a group and you have the `group-instance-manage` permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |
**hard_close** | Option<**bool**> | Whether to hard close the instance. Defaults to false. |  |
**closed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The time after which users won't be allowed to join the instances. If omitted, the instance will be closed immediately. |  |

### Return type

[**models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> models::Instance create_instance(create_instance_request)
Create Instance

Create an instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_instance_request** | [**CreateInstanceRequest**](CreateInstanceRequest.md) |  | [required] |

### Return type

[**models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discover_instances

> models::InstanceDiscovery discover_instances(n, region, platform, world_id, group_id, vibe, category)
Discover Instances

Returns instances to discover, in an order that changes between requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of instances to return. |  |
**region** | Option<[**CreateInstanceRequestRegion**](CreateInstanceRequestRegion.md)> | Return only instances in this region. |  |
**platform** | Option<[**InstanceDiscoveryPlatform**](InstanceDiscoveryPlatform.md)> | The platform to discover instances for. |  |
**world_id** | Option<[**Vec<String>**](String.md)> | Return only instances of these worlds. |  |
**group_id** | Option<[**Vec<String>**](String.md)> | Return only instances of these groups. |  |
**vibe** | Option<[**Vec<String>**](String.md)> |  |  |
**category** | Option<**String**> |  |  |

### Return type

[**models::InstanceDiscovery**](InstanceDiscovery.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_instances

> Vec<models::Instance> get_active_instances(n, offset)
List Active Instances

Returns active instances, the most populated first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of instances to return. |  |
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::Instance>**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> models::Instance get_instance(world_id, instance_id)
Get Instance

Returns an instance. Please read [Instances Tutorial](https://vrchatapi.github.io/tutorials/instances/) for more information on Instances.  If an invalid instanceId is provided, this endpoint will simply return \"null\"!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |

### Return type

[**models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_by_short_name

> models::Instance get_instance_by_short_name(short_name)
Get Instance By Short Name

Returns an instance. Please read [Instances Tutorial](https://vrchatapi.github.io/tutorials/instances/) for more information on Instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**short_name** | **String** | Must be a valid instance short name. | [required] |

### Return type

[**models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_categories

> Vec<models::InstanceCategory> get_instance_categories()
List Instance Categories

Returns the categories an instance can be listed under.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::InstanceCategory>**](InstanceCategory.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_vibes

> Vec<models::InstanceVibe> get_instance_vibes()
List Instance Vibes

Returns the vibes an instance can be tagged with.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::InstanceVibe>**](InstanceVibe.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recent_locations

> Vec<String> get_recent_locations(n, offset)
List Recent Locations

Returns a list of recently visited locations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

**Vec<String>**

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_short_name

> models::InstanceShortNameResponse get_short_name(world_id, instance_id)
Get Instance Short Name

Returns an instance short name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |

### Return type

[**models::InstanceShortNameResponse**](InstanceShortNameResponse.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance

> models::Instance update_instance(world_id, instance_id, update_instance_request)
Update Instance

Set or remove the calendar event linked to a group instance.  Updating a group instance requires both the `group-instance-manage` and `group-instance-calendar-link` permissions.  The event must begin within the next six hours or have ended within the previous six hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**instance_id** | **String** | Must be a valid instance ID. | [required] |
**update_instance_request** | [**UpdateInstanceRequest**](UpdateInstanceRequest.md) |  | [required] |

### Return type

[**models::Instance**](Instance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

