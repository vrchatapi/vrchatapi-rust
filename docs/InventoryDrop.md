# InventoryDrop

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**drop_expiry_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | 
**drop_status** | Option<**String**> |  | [optional]
**end_drop_date** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**id** | **String** |  | 
**is_disabled** | **bool** |  | 
**name** | **String** |  | 
**notification_details** | [**models::InventoryNotificationDetails**](InventoryNotificationDetails.md) |  | 
**start_drop_date** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**tags** | **Vec<String>** |  | 
**target_group** | **String** |  | 
**template_ids** | **Vec<String>** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


