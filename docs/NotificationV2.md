# NotificationV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_delete** | **bool** |  | 
**category** | **String** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**data** | [**models::NotificationV2Data**](NotificationV2Data.md) |  | 
**details** | Option<[**models::NotificationV2DetailsBoop**](NotificationV2DetailsBoop.md)> |  | [optional]
**display_data** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**expires_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**expiry_after_seen** | Option<**i32**> |  | 
**id** | **String** |  | 
**ignore_dnd** | **bool** |  | 
**image_url** | Option<**String**> |  | 
**is_system** | **bool** |  | 
**link** | Option<**String**> |  | 
**link_text** | Option<**String**> |  | 
**link_text_key** | Option<**String**> |  | 
**message** | **String** |  | 
**message_key** | Option<**String**> |  | [optional]
**receiver_user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**related_notifications_id** | Option<**String**> |  | 
**require_seen** | **bool** |  | 
**responses** | [**Vec<models::NotificationV2Response>**](NotificationV2Response.md) |  | 
**seen** | **bool** |  | 
**sender_user_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**sender_username** | Option<**String**> |  | 
**title** | **String** |  | 
**title_key** | Option<**String**> |  | 
**r#type** | [**models::NotificationV2Type**](NotificationV2Type.md) |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**version** | **i32** |  | [default to 2]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


