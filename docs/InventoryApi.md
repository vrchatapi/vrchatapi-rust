# \InventoryApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**consume_own_inventory_item**](InventoryApi.md#consume_own_inventory_item) | **PUT** /inventory/{inventoryItemId}/consume | Consume Own Inventory Item
[**delete_own_inventory_item**](InventoryApi.md#delete_own_inventory_item) | **DELETE** /inventory/{inventoryItemId} | Delete Own Inventory Item
[**equip_own_inventory_item**](InventoryApi.md#equip_own_inventory_item) | **PUT** /inventory/{inventoryItemId}/equip | Equip Own Inventory Item
[**get_inventory**](InventoryApi.md#get_inventory) | **GET** /inventory | Get Inventory
[**get_inventory_collections**](InventoryApi.md#get_inventory_collections) | **GET** /inventory/collections | List Inventory Collections
[**get_inventory_drops**](InventoryApi.md#get_inventory_drops) | **GET** /inventory/drops | List Inventory Drops
[**get_inventory_template**](InventoryApi.md#get_inventory_template) | **GET** /inventory/template/{inventoryTemplateId} | Get Inventory Template
[**get_own_inventory_item**](InventoryApi.md#get_own_inventory_item) | **GET** /inventory/{inventoryItemId} | Get Own Inventory Item
[**get_user_inventory_item**](InventoryApi.md#get_user_inventory_item) | **GET** /user/{userId}/inventory/{inventoryItemId} | Get User Inventory Item
[**share_inventory_item_direct**](InventoryApi.md#share_inventory_item_direct) | **POST** /inventory/cloning/direct | Share Inventory Item Direct
[**share_inventory_item_pedestal**](InventoryApi.md#share_inventory_item_pedestal) | **GET** /inventory/cloning/pedestal | Share Inventory Item by Pedestal
[**spawn_inventory_item**](InventoryApi.md#spawn_inventory_item) | **GET** /inventory/spawn | Spawn Inventory Item
[**unequip_own_inventory_slot**](InventoryApi.md#unequip_own_inventory_slot) | **DELETE** /inventory/{inventoryItemId}/equip | Unequip Own Inventory Slot
[**update_own_inventory_item**](InventoryApi.md#update_own_inventory_item) | **PUT** /inventory/{inventoryItemId} | Update Own Inventory Item



## consume_own_inventory_item

> models::InventoryConsumptionResults consume_own_inventory_item(inventory_item_id)
Consume Own Inventory Item

Returns the modified InventoryItem object as held by the currently logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** | Must be a valid inventory item ID. | [required] |

### Return type

[**models::InventoryConsumptionResults**](InventoryConsumptionResults.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_own_inventory_item

> models::SuccessFlag delete_own_inventory_item(inventory_item_id)
Delete Own Inventory Item

Deletes an InventoryItem from the inventory of the currently logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** | Must be a valid inventory item ID. | [required] |

### Return type

[**models::SuccessFlag**](SuccessFlag.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## equip_own_inventory_item

> models::InventoryItem equip_own_inventory_item(inventory_item_id, equip_inventory_item_request)
Equip Own Inventory Item

Returns the modified InventoryItem object as held by the currently logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** | Must be a valid inventory item ID. | [required] |
**equip_inventory_item_request** | Option<[**EquipInventoryItemRequest**](EquipInventoryItemRequest.md)> |  |  |

### Return type

[**models::InventoryItem**](InventoryItem.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory

> models::Inventory get_inventory(n, offset, holder_id, equip_slot, order, tags, types, flags, not_types, not_flags, archived)
Get Inventory

Returns an Inventory object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**holder_id** | Option<**String**> | The UserID of the owner of the inventory; defaults to the currently authenticated user. |  |
**equip_slot** | Option<[**InventoryEquipSlot**](.md)> | Filter for inventory retrieval. |  |
**order** | Option<**String**> | Sort order for inventory retrieval. |  |
**tags** | Option<**String**> | Filter tags for inventory retrieval (comma-separated). |  |
**types** | Option<[**InventoryItemType**](.md)> | Filter for inventory retrieval. |  |
**flags** | Option<[**InventoryFlag**](.md)> | Filter flags for inventory retrieval (comma-separated). |  |
**not_types** | Option<[**InventoryItemType**](.md)> | Filter out types for inventory retrieval (comma-separated). |  |
**not_flags** | Option<[**InventoryFlag**](.md)> | Filter out flags for inventory retrieval (comma-separated). |  |
**archived** | Option<**bool**> | Filter archived status for inventory retrieval. |  |

### Return type

[**models::Inventory**](Inventory.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_collections

> Vec<String> get_inventory_collections()
List Inventory Collections

Returns a list of collection names.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

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


## get_user_inventory_item

> models::InventoryItem get_user_inventory_item(user_id, inventory_item_id)
Get User Inventory Item

Returns an InventoryItem object held by the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be a valid user ID. | [required] |
**inventory_item_id** | **String** | Must be a valid inventory item ID. | [required] |

### Return type

[**models::InventoryItem**](InventoryItem.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_inventory_item_direct

> models::OkStatus share_inventory_item_direct(item_id, duration, share_inventory_item_direct_request)
Share Inventory Item Direct

Share content directly with other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Id for inventory item sharing. | [required] |
**duration** | **i32** | The duration before the sharing pedestal despawns. | [required] |[default to 90]
**share_inventory_item_direct_request** | [**ShareInventoryItemDirectRequest**](ShareInventoryItemDirectRequest.md) |  | [required] |

### Return type

[**models::OkStatus**](OkStatus.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_inventory_item_pedestal

> models::InventorySpawn share_inventory_item_pedestal(item_id, duration)
Share Inventory Item by Pedestal

Returns an InventorySpawn object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Id for inventory item sharing. | [required] |
**duration** | **i32** | The duration before the sharing pedestal despawns. | [required] |[default to 90]

### Return type

[**models::InventorySpawn**](InventorySpawn.md)

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


## unequip_own_inventory_slot

> String unequip_own_inventory_slot(inventory_item_id)
Unequip Own Inventory Slot

Unequips the InventoryItem in the given slot of the inventory of the currently logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | [**InventoryEquipSlot**](.md) | Selector for inventory slot management. | [required] |

### Return type

**String**

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_own_inventory_item

> models::InventoryItem update_own_inventory_item(inventory_item_id, update_inventory_item_request)
Update Own Inventory Item

Returns the modified InventoryItem object as held by the currently logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** | Must be a valid inventory item ID. | [required] |
**update_inventory_item_request** | Option<[**UpdateInventoryItemRequest**](UpdateInventoryItemRequest.md)> |  |  |

### Return type

[**models::InventoryItem**](InventoryItem.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

