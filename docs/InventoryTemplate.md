# InventoryTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attribution** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**collections** | **Vec<String>** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**default_attributes** | **serde_json::Value** |  | 
**description** | **String** |  | 
**drop_status** | Option<**String**> |  | [optional]
**equip_slots** | **Vec<String>** |  | 
**flags** | **Vec<String>** |  | 
**id** | **String** |  | 
**image_url** | **String** |  | 
**initial_toggle_state** | Option<**bool**> |  | [optional]
**item_type** | [**models::InventoryItemType**](InventoryItemType.md) |  | 
**item_type_label** | **String** |  | 
**metadata** | Option<[**models::InventoryMetadata**](InventoryMetadata.md)> |  | [optional]
**name** | **String** |  | 
**notification_details** | Option<[**models::InventoryNotificationDetails**](InventoryNotificationDetails.md)> |  | [optional]
**product_id** | Option<**String**> |  | [optional]
**published_listings** | Option<**Vec<String>**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**tags** | **Vec<String>** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**validate_user_attributes** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


