# ProductPurchase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buyer_display_name** | **String** |  | 
**buyer_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**first_party** | Option<**bool**> |  | [optional]
**is_buyer** | **bool** |  | 
**is_gift** | **bool** |  | 
**is_receiver** | **bool** |  | 
**is_seller** | **bool** |  | 
**ledger_transaction_id** | Option<**i32**> |  | [optional]
**listing_currently_available** | **bool** |  | 
**listing_description** | Option<**String**> |  | [optional]
**listing_display_name** | **String** |  | 
**listing_id** | **String** |  | 
**listing_image_id** | **String** |  | 
**listing_subtitle** | **String** |  | 
**listing_type** | [**models::ProductListingType**](ProductListingType.md) |  | 
**products** | [**Vec<models::ProductPurchaseProduct>**](ProductPurchaseProduct.md) |  | 
**purchase_active** | **bool** |  | 
**purchase_context** | [**models::ProductPurchasePurchaseContext**](ProductPurchasePurchaseContext.md) |  | 
**purchase_current_status** | **String** |  | 
**purchase_date** | **String** |  | 
**purchase_duration** | Option<**i32**> |  | [optional]
**purchase_duration_type** | Option<**String**> |  | [optional]
**purchase_end_date** | Option<**String**> |  | 
**purchase_fee** | Option<**i32**> |  | [optional]
**purchase_id** | **String** |  | 
**purchase_latest** | **bool** |  | 
**purchase_price** | **i32** |  | 
**purchase_quantity** | **i32** |  | 
**purchase_start_date** | Option<**String**> |  | 
**purchase_token** | Option<**serde_json::Value**> |  | 
**purchase_type** | **String** |  | 
**purchase_unit_price** | **i32** |  | 
**purchase_value** | Option<**i32**> |  | [optional]
**receiver_display_name** | **String** |  | 
**receiver_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**recurrable** | **bool** |  | 
**refundable** | **bool** |  | 
**seller_display_name** | **String** |  | 
**seller_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**stackable** | **bool** |  | 
**will_recur** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


