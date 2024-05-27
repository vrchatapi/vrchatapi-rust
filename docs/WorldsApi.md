# \WorldsApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world**](WorldsApi.md#create_world) | **POST** /worlds | Create World
[**delete_world**](WorldsApi.md#delete_world) | **DELETE** /worlds/{worldId} | Delete World
[**get_active_worlds**](WorldsApi.md#get_active_worlds) | **GET** /worlds/active | List Active Worlds
[**get_favorited_worlds**](WorldsApi.md#get_favorited_worlds) | **GET** /worlds/favorites | List Favorited Worlds
[**get_recent_worlds**](WorldsApi.md#get_recent_worlds) | **GET** /worlds/recent | List Recent Worlds
[**get_world**](WorldsApi.md#get_world) | **GET** /worlds/{worldId} | Get World by ID
[**get_world_instance**](WorldsApi.md#get_world_instance) | **GET** /worlds/{worldId}/{instanceId} | Get World Instance
[**get_world_metadata**](WorldsApi.md#get_world_metadata) | **GET** /worlds/{worldId}/metadata | Get World Metadata
[**get_world_publish_status**](WorldsApi.md#get_world_publish_status) | **GET** /worlds/{worldId}/publish | Get World Publish Status
[**publish_world**](WorldsApi.md#publish_world) | **PUT** /worlds/{worldId}/publish | Publish World
[**search_worlds**](WorldsApi.md#search_worlds) | **GET** /worlds | Search All Worlds
[**unpublish_world**](WorldsApi.md#unpublish_world) | **DELETE** /worlds/{worldId}/publish | Unpublish World
[**update_world**](WorldsApi.md#update_world) | **PUT** /worlds/{worldId} | Update World



## create_world

> crate::models::World create_world(create_world_request)
Create World

Create a new world. This endpoint requires `assetUrl` to be a valid File object with `.vrcw` file extension, and `imageUrl` to be a valid File object with an image file extension.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_world_request** | Option<[**CreateWorldRequest**](CreateWorldRequest.md)> |  |  |

### Return type

[**crate::models::World**](World.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_world

> delete_world(world_id)
Delete World

Delete a world. Notice a world is never fully \"deleted\", only its ReleaseStatus is set to \"hidden\" and the linked Files are deleted. The WorldID is permanently reserved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_worlds

> Vec<crate::models::LimitedWorld> get_active_worlds(featured, sort, n, order, offset, search, tag, notag, release_status, max_unity_version, min_unity_version, platform)
List Active Worlds

Search and list currently Active worlds by query filters.

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

### Return type

[**Vec<crate::models::LimitedWorld>**](LimitedWorld.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorited_worlds

> Vec<crate::models::LimitedWorld> get_favorited_worlds(featured, sort, n, order, offset, search, tag, notag, release_status, max_unity_version, min_unity_version, platform, user_id)
List Favorited Worlds

Search and list favorited worlds by query filters.

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

[**Vec<crate::models::LimitedWorld>**](LimitedWorld.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recent_worlds

> Vec<crate::models::LimitedWorld> get_recent_worlds(featured, sort, n, order, offset, search, tag, notag, release_status, max_unity_version, min_unity_version, platform, user_id)
List Recent Worlds

Search and list recently visited worlds by query filters.

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

[**Vec<crate::models::LimitedWorld>**](LimitedWorld.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_world

> crate::models::World get_world(world_id)
Get World by ID

Get information about a specific World. Works unauthenticated but when so will always return `0` for certain fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

[**crate::models::World**](World.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_world_instance

> crate::models::Instance get_world_instance(world_id, instance_id)
Get World Instance

Returns a worlds instance.

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


## get_world_metadata

> crate::models::WorldMetadata get_world_metadata(world_id)
Get World Metadata

Return a worlds custom metadata. This is currently believed to be unused. Metadata can be set with `updateWorld` and can be any arbitrary object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

[**crate::models::WorldMetadata**](WorldMetadata.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_world_publish_status

> crate::models::WorldPublishStatus get_world_publish_status(world_id)
Get World Publish Status

Returns a worlds publish status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

[**crate::models::WorldPublishStatus**](WorldPublishStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_world

> publish_world(world_id)
Publish World

Publish a world. You can only publish one world per week.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_worlds

> Vec<crate::models::LimitedWorld> search_worlds(featured, sort, user, user_id, n, order, offset, search, tag, notag, release_status, max_unity_version, min_unity_version, platform)
Search All Worlds

Search and list any worlds by query filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**featured** | Option<**bool**> | Filters on featured results. |  |
**sort** | Option<[**SortOption**](.md)> | The sort order of the results. |  |
**user** | Option<**String**> | Set to `me` for searching own worlds. |  |
**user_id** | Option<**String**> | Filter by UserID. |  |
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

### Return type

[**Vec<crate::models::LimitedWorld>**](LimitedWorld.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_world

> unpublish_world(world_id)
Unpublish World

Unpublish a world.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_world

> crate::models::World update_world(world_id, update_world_request)
Update World

Update information about a specific World.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**world_id** | **String** | Must be a valid world ID. | [required] |
**update_world_request** | Option<[**UpdateWorldRequest**](UpdateWorldRequest.md)> |  |  |

### Return type

[**crate::models::World**](World.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

