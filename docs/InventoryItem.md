# InventoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collections** | **Vec<String>** |  | 
**created_at** | **String** |  | 
**default_attributes** | [**std::collections::HashMap<String, models::InventoryDefaultAttributesValue>**](InventoryDefaultAttributes_value.md) |  | 
**description** | **String** |  | 
**equip_slot** | Option<[**models::InventoryEquipSlot**](InventoryEquipSlot.md)> |  | [optional]
**equip_slots** | Option<[**Vec<models::InventoryEquipSlot>**](InventoryEquipSlot.md)> |  | [optional]
**expiry_date** | Option<**String**> |  | 
**flags** | **Vec<String>** |  | 
**holder_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**id** | **String** |  | 
**image_url** | **String** |  | 
**is_archived** | **bool** |  | 
**is_seen** | **bool** |  | 
**item_type** | [**models::InventoryItemType**](InventoryItemType.md) |  | 
**item_type_label** | **String** |  | 
**metadata** | [**models::InventoryMetadata**](InventoryMetadata.md) |  | 
**name** | **String** |  | 
**tags** | **Vec<String>** |  | 
**template_id** | **String** |  | 
**template_created_at** | **String** |  | 
**template_updated_at** | **String** |  | 
**updated_at** | **String** |  | 
**user_attributes** | [**models::InventoryUserAttributes**](InventoryUserAttributes.md) |  | 
**validate_user_attributes** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


