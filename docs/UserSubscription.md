# UserSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**transaction_id** | **String** |  | 
**store** | **String** | Which \"Store\" it came from. Right now only Stores are \"Steam\" and \"Admin\". | 
**steam_item_id** | Option<**String**> |  | [optional]
**amount** | **f64** |  | 
**description** | **String** |  | 
**period** | [**models::SubscriptionPeriod**](SubscriptionPeriod.md) |  | 
**tier** | **f64** |  | 
**active** | **bool** |  | [default to true]
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 
**starts** | Option<**String**> |  | [optional]
**expires** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**license_groups** | **Vec<String>** |  | 
**is_gift** | **bool** |  | [default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


