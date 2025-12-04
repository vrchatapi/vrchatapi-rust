# Store

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** |  | 
**display_name** | **String** |  | 
**group_id** | Option<**String**> |  | [optional]
**id** | **String** |  | 
**listing_ids** | Option<**Vec<String>**> | Only for store type world and group | [optional]
**listings** | Option<[**Vec<models::ProductListing>**](ProductListing.md)> | Only for store type world and group | [optional]
**seller_display_name** | **String** |  | 
**seller_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**shelf_ids** | Option<**Vec<String>**> | Only for store type house | [optional]
**shelves** | Option<[**Vec<models::StoreShelf>**](StoreShelf.md)> | Only for store type house | [optional]
**store_id** | **String** |  | 
**store_type** | [**models::StoreType**](StoreType.md) |  | 
**tags** | **Vec<String>** |  | 
**world_id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


