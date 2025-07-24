# \InventoryApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_inventory**](InventoryApi.md#get_inventory) | **GET** /inventory | Get Inventory
[**get_inventory_drops**](InventoryApi.md#get_inventory_drops) | **GET** /inventory/drops | List Inventory Drops
[**get_inventory_template**](InventoryApi.md#get_inventory_template) | **GET** /inventory/template/{inventoryTemplateId} | Get Inventory Template
[**get_own_inventory_item**](InventoryApi.md#get_own_inventory_item) | **GET** /inventory/{inventoryItemId} | Get Own Inventory Item
[**spawn_inventory_item**](InventoryApi.md#spawn_inventory_item) | **GET** /inventory/spawn | Spawn Inventory Item



## get_inventory

> models::Inventory get_inventory(n, offset, inventory_sort_order, inventory_item_type)
Get Inventory

Returns an Inventory object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**inventory_sort_order** | Option<**String**> | Sort order for inventory retrieval. |  |
**inventory_item_type** | Option<[**InventoryItemType**](.md)> | Filter for inventory retrieval. |  |

### Return type

[**models::Inventory**](Inventory.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_drops

> Vec<models::InventoryDrop> get_inventory_drops(active)
List Inventory Drops

Returns a list of InventoryDrop objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active** | Option<**bool**> | Filter for users' listings and inventory bundles. |  |

### Return type

[**Vec<models::InventoryDrop>**](InventoryDrop.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_template

> models::InventoryTemplate get_inventory_template(inventory_template_id)
Get Inventory Template

Returns an InventoryTemplate object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_template_id** | **String** | Must be a valid inventory template ID. | [required] |

### Return type

[**models::InventoryTemplate**](InventoryTemplate.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_own_inventory_item

> models::InventoryItem get_own_inventory_item(inventory_item_id)
Get Own Inventory Item

Returns an InventoryItem object held by the currently logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** | Must be a valid inventory item ID. | [required] |

### Return type

[**models::InventoryItem**](InventoryItem.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spawn_inventory_item

> models::InventorySpawn spawn_inventory_item(id)
Spawn Inventory Item

Returns an InventorySpawn object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id for inventory item spawning. | [required] |

### Return type

[**models::InventorySpawn**](InventorySpawn.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

