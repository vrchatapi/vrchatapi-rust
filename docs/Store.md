# Store

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** |  | 
**display_name** | **String** |  | 
**id** | **String** |  | 
**seller_display_name** | **String** |  | 
**seller_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**store_id** | **String** |  | 
**store_type** | [**models::StoreType**](StoreType.md) |  | 
**tags** | **Vec<String>** |  | 
**listing_ids** | Option<**Vec<String>**> | Only for store type world and group | [optional]
**listings** | Option<[**Vec<models::ProductListing>**](ProductListing.md)> | Only for store type world and group | [optional]
**world_id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]
**group_id** | Option<**String**> |  | [optional]
**shelf_ids** | Option<**Vec<String>**> | Only for store type house | [optional]
**shelves** | Option<[**Vec<models::StoreShelf>**](StoreShelf.md)> | Only for store type house | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


