# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**user_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**user_display_name** | Option<**String**> |  | [optional]
**status** | [**crate::models::TransactionStatus**](TransactionStatus.md) |  | 
**subscription** | [**crate::models::Subscription**](Subscription.md) |  | 
**sandbox** | **bool** |  | [default to false]
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**steam** | Option<[**crate::models::TransactionSteamInfo**](TransactionSteamInfo.md)> |  | [optional]
**agreement** | Option<[**crate::models::TransactionAgreement**](TransactionAgreement.md)> |  | [optional]
**error** | **String** |  | 
**is_gift** | Option<**bool**> |  | [optional][default to false]
**is_tokens** | Option<**bool**> |  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


