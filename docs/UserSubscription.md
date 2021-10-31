# UserSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**transaction_id** | **String** |  | 
**store** | **String** | Which \"Store\" it came from. Right now only Stores are \"Steam\" and \"Admin\". | 
**steam_item_id** | Option<**String**> |  | [optional]
**amount** | **f32** |  | 
**description** | **String** |  | 
**period** | [**crate::models::SubscriptionPeriod**](SubscriptionPeriod.md) |  | 
**tier** | **f32** |  | 
**active** | **bool** |  | [default to true]
**status** | [**crate::models::TransactionStatus**](TransactionStatus.md) |  | 
**expires** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**license_groups** | **Vec<String>** |  | 
**is_gift** | **bool** |  | [default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


