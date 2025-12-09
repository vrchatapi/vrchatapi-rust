# NotificationV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_delete** | **bool** |  | 
**category** | **String** |  | 
**created_at** | **String** |  | 
**data** | [**serde_json::Value**](.md) |  | 
**expires_at** | **String** |  | 
**expiry_after_seen** | Option<**i32**> |  | 
**id** | **String** |  | 
**ignore_dnd** | **bool** |  | 
**image_url** | Option<**String**> |  | 
**is_system** | **bool** |  | 
**link** | **String** |  | 
**link_text** | **String** |  | 
**link_text_key** | Option<**String**> |  | 
**message** | **String** |  | 
**message_key** | Option<**String**> |  | [optional]
**receiver_user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**related_notifications_id** | Option<**String**> |  | 
**require_seen** | **bool** |  | 
**responses** | [**Vec<models::NotificationV2Response>**](NotificationV2Response.md) |  | 
**seen** | **bool** |  | 
**sender_user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**sender_username** | Option<**String**> |  | 
**title** | **String** |  | 
**title_key** | Option<**String**> |  | 
**r#type** | [**models::NotificationV2Type**](NotificationV2Type.md) |  | 
**updated_at** | **String** |  | 
**version** | **i32** |  | [default to 2]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


