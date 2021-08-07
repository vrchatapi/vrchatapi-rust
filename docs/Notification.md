# Notification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**sender_user_id** | **String** |  | [readonly]
**sender_username** | **String** |  | 
**_type** | [**crate::models::NotificationType**](NotificationType.md) |  | 
**message** | **String** |  | 
**details** | **String** | **NOTICE:** This is not a JSON object, this is a json **encoded** object, meaning you have to json-de-encode to get the NotificationDetail object depending on the NotificationType. | [default to {}]
**seen** | **bool** |  | [default to false]
**created_at** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


