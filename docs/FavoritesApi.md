# \FavoritesApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_favorite**](FavoritesApi.md#add_favorite) | **POST** /favorites | Add Favorite
[**clear_favorite_group**](FavoritesApi.md#clear_favorite_group) | **DELETE** /favorite/group/{favoriteGroupType}/{favoriteGroupName}/{userId} | Clear Favorite Group
[**get_favorite**](FavoritesApi.md#get_favorite) | **GET** /favorites/{favoriteId} | Show Favorite
[**get_favorite_group**](FavoritesApi.md#get_favorite_group) | **GET** /favorite/group/{favoriteGroupType}/{favoriteGroupName}/{userId} | Show Favorite Group
[**get_favorite_groups**](FavoritesApi.md#get_favorite_groups) | **GET** /favorite/groups | List Favorite Groups
[**get_favorites**](FavoritesApi.md#get_favorites) | **GET** /favorites | List Favorites
[**remove_favorite**](FavoritesApi.md#remove_favorite) | **DELETE** /favorites/{favoriteId} | Remove Favorite
[**update_favorite_group**](FavoritesApi.md#update_favorite_group) | **PUT** /favorite/group/{favoriteGroupType}/{favoriteGroupName}/{userId} | Update Favorite Group



## add_favorite

> crate::models::Favorite add_favorite(add_favorite_request)
Add Favorite

Add a new favorite.  Friend groups are named `group_0` through `group_3`. Avatar and World groups are named `avatars1` to `avatars4` and `worlds1` to `worlds4`.  You cannot add people whom you are not friends with to your friends list. Destroying a friendship removes the person as favorite on both sides.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_favorite_request** | Option<[**AddFavoriteRequest**](AddFavoriteRequest.md)> |  |  |

### Return type

[**crate::models::Favorite**](Favorite.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_favorite_group

> crate::models::Success clear_favorite_group(favorite_group_type, favorite_group_name, user_id)
Clear Favorite Group

Clear ALL contents of a specific favorite group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favorite_group_type** | **String** | The type of group to fetch, must be a valid FavoriteType. | [required] |
**favorite_group_name** | **String** | The name of the group to fetch, must be a name of a FavoriteGroup. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorite

> crate::models::Favorite get_favorite(favorite_id)
Show Favorite

Return information about a specific Favorite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favorite_id** | **String** | Must be a valid favorite ID. | [required] |

### Return type

[**crate::models::Favorite**](Favorite.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorite_group

> crate::models::FavoriteGroup get_favorite_group(favorite_group_type, favorite_group_name, user_id)
Show Favorite Group

Fetch information about a specific favorite group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favorite_group_type** | **String** | The type of group to fetch, must be a valid FavoriteType. | [required] |
**favorite_group_name** | **String** | The name of the group to fetch, must be a name of a FavoriteGroup. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::FavoriteGroup**](FavoriteGroup.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorite_groups

> Vec<crate::models::FavoriteGroup> get_favorite_groups(n, offset, owner_id)
List Favorite Groups

Return a list of favorite groups owned by a user. Returns the same information as `getFavoriteGroups`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**owner_id** | Option<**String**> | The owner of whoms favorite groups to return. Must be a UserID. |  |

### Return type

[**Vec<crate::models::FavoriteGroup>**](FavoriteGroup.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorites

> Vec<crate::models::Favorite> get_favorites(n, offset, r#type, tag)
List Favorites

Returns a list of favorites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**r#type** | Option<**String**> | The type of favorites to return, FavoriteType. |  |
**tag** | Option<**String**> | Tags to include (comma-separated). Any of the tags needs to be present. |  |

### Return type

[**Vec<crate::models::Favorite>**](Favorite.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_favorite

> crate::models::Success remove_favorite(favorite_id)
Remove Favorite

Remove a favorite from your favorites list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favorite_id** | **String** | Must be a valid favorite ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_favorite_group

> update_favorite_group(favorite_group_type, favorite_group_name, user_id, update_favorite_group_request)
Update Favorite Group

Update information about a specific favorite group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favorite_group_type** | **String** | The type of group to fetch, must be a valid FavoriteType. | [required] |
**favorite_group_name** | **String** | The name of the group to fetch, must be a name of a FavoriteGroup. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |
**update_favorite_group_request** | Option<[**UpdateFavoriteGroupRequest**](UpdateFavoriteGroupRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

