# SentNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**details** | [**serde_json::Value**](.md) |  | 
**id** | **String** |  | 
**message** | **String** |  | 
**receiver_user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**sender_user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**sender_username** | Option<**String**> | -| **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429). | [optional]
**r#type** | [**crate::models::NotificationType**](NotificationType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


