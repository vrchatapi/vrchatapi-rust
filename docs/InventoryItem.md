# InventoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acquisition** | Option<**String**> |  | [optional]
**attribution** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**collections** | **Vec<String>** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**default_attributes** | [**std::collections::HashMap<String, models::InventoryDefaultAttributesValue>**](InventoryDefaultAttributesValue.md) |  | 
**description** | **String** |  | 
**equip_slot** | Option<[**models::InventoryEquipSlot**](InventoryEquipSlot.md)> |  | [optional]
**equip_slots** | Option<[**Vec<models::InventoryEquipSlot>**](InventoryEquipSlot.md)> |  | [optional]
**expiry_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**flags** | **Vec<String>** |  | 
**holder_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**id** | **String** |  | 
**image_url** | **String** |  | 
**is_archived** | **bool** |  | 
**is_seen** | **bool** |  | 
**item_type** | [**models::InventoryItemType**](InventoryItemType.md) |  | 
**item_type_label** | **String** |  | 
**last_equipped** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**metadata** | [**models::InventoryMetadata**](InventoryMetadata.md) |  | 
**name** | **String** |  | 
**quantifiable** | **bool** |  | 
**tags** | **Vec<String>** |  | 
**template_id** | **String** |  | 
**template_created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**template_updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**user_attributes** | [**models::InventoryUserAttributes**](InventoryUserAttributes.md) |  | 
**validate_user_attributes** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


