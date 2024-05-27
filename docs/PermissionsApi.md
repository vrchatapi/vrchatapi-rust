# \PermissionsApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_assigned_permissions**](PermissionsApi.md#get_assigned_permissions) | **GET** /auth/permissions | Get Assigned Permissions
[**get_permission**](PermissionsApi.md#get_permission) | **GET** /permissions/{permissionId} | Get Permission



## get_assigned_permissions

> Vec<crate::models::Permission> get_assigned_permissions()
Get Assigned Permissions

Returns a list of all permissions currently granted by the user. Permissions are assigned e.g. by subscribing to VRC+.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Permission>**](Permission.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permission

> crate::models::Permission get_permission(permission_id)
Get Permission

Returns a single permission. This endpoint is pretty useless, as it returns the exact same information as `/auth/permissions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission_id** | **String** | Must be a valid permission ID. | [required] |

### Return type

[**crate::models::Permission**](Permission.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

