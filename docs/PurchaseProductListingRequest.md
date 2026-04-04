# PurchaseProductListingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_data** | Option<[**models::PurchaseContextData**](PurchaseContextData.md)> |  | [optional]
**listing_id** | **String** |  | 
**listing_variant_id** | Option<**String**> |  | [optional]
**quantity** | **i32** |  | [default to 1]
**receiver_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**stackable** | Option<**bool**> |  | [optional]
**total_price** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


