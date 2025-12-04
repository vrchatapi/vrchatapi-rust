# UserSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** |  | [default to true]
**amount** | **f64** |  | 
**created_at** | **String** |  | 
**description** | **String** |  | 
**expires** | **String** |  | 
**id** | **String** |  | 
**is_bulk_gift** | **bool** |  | [default to false]
**is_gift** | **bool** |  | [default to false]
**license_groups** | **Vec<String>** |  | 
**period** | [**models::SubscriptionPeriod**](SubscriptionPeriod.md) |  | 
**starts** | Option<**String**> |  | [optional]
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 
**steam_item_id** | Option<**String**> |  | [optional]
**store** | **String** | Which \"Store\" it came from. Right now only Stores are \"Steam\" and \"Admin\". | 
**tier** | **i32** |  | 
**transaction_id** | **String** |  | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


