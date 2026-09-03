# ProductListing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** |  | 
**archived** | Option<**bool**> |  | [optional]
**attribution** | Option<[**models::ProductListingAttribution**](ProductListingAttribution.md)> |  | [optional]
**buyer_refundable** | **bool** |  | 
**collab_user_display_name** | Option<**String**> |  | [optional]
**collab_user_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**created** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**description** | **String** |  | 
**display_name** | **String** |  | 
**duration** | Option<**i32**> |  | [optional]
**duration_type** | Option<**String**> |  | [optional]
**group_icon** | Option<**String**> |  | [optional]
**group_id** | Option<**String**> |  | [optional]
**group_name** | Option<**String**> |  | [optional]
**has_avatar** | **bool** |  | 
**has_companion** | Option<**bool**> |  | [optional]
**has_inventory** | Option<**bool**> |  | [optional]
**has_udon** | **bool** |  | 
**hydrated_products** | Option<[**Vec<models::Product>**](Product.md)> |  | [optional]
**id** | **String** |  | 
**image_id** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**listing_type** | [**models::ProductListingType**](ProductListingType.md) |  | 
**listing_variants** | Option<[**Vec<models::ProductListingVariant>**](ProductListingVariant.md)> |  | [optional]
**permanent** | Option<**bool**> |  | [optional]
**price_tokens** | **i32** |  | 
**product_ids** | **Vec<String>** |  | 
**product_type** | [**models::ProductType**](ProductType.md) |  | 
**product_types** | Option<**Vec<String>**> |  | [optional]
**products** | **Vec<String>** | Product ids. The products themselves arrive in `hydratedProducts`. | 
**purchase_count** | Option<**i32**> |  | [optional]
**purchase_count_quantity** | Option<**i32**> |  | [optional]
**quantifiable** | Option<**bool**> |  | [optional]
**recurrable** | **bool** |  | 
**refundable** | **bool** |  | 
**seller_display_name** | **String** |  | 
**seller_id** | **String** |  | 
**sold_by_vrc** | Option<**bool**> |  | [optional]
**stackable** | **bool** |  | 
**store_ids** | **Vec<String>** |  | 
**subtitle** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**vrc_plus_discount_price** | Option<**i32**> |  | [optional]
**when_to_expire** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


