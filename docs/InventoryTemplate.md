# InventoryTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**collections** | **Vec<String>** |  | 
**created_at** | **String** |  | 
**default_attributes** | [**serde_json::Value**](.md) |  | 
**description** | **String** |  | 
**equip_slots** | **Vec<String>** |  | 
**flags** | **Vec<String>** |  | 
**id** | **String** |  | 
**image_url** | **String** |  | 
**item_type** | [**models::InventoryItemType**](InventoryItemType.md) |  | 
**item_type_label** | **String** |  | 
**metadata** | Option<[**models::InventoryMetadata**](InventoryMetadata.md)> |  | [optional]
**name** | **String** |  | 
**notification_details** | Option<[**models::InventoryNotificationDetails**](InventoryNotificationDetails.md)> |  | [optional]
**status** | **String** |  | 
**tags** | **Vec<String>** |  | 
**updated_at** | **String** |  | 
**validate_user_attributes** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


