# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agreement** | Option<[**models::TransactionAgreement**](TransactionAgreement.md)> |  | [optional]
**created_at** | **String** |  | 
**error** | Option<**String**> |  | 
**id** | **String** |  | 
**is_gift** | Option<**bool**> |  | [optional][default to false]
**is_tokens** | Option<**bool**> |  | [optional][default to false]
**sandbox** | **bool** |  | [default to false]
**status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 
**steam** | Option<[**models::TransactionSteamInfo**](TransactionSteamInfo.md)> |  | [optional]
**subscription** | [**models::Subscription**](Subscription.md) |  | 
**updated_at** | **String** |  | 
**user_display_name** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


