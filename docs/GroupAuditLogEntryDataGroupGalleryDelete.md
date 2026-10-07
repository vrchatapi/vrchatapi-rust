# GroupAuditLogEntryDataGroupGalleryDelete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | The gallery description. | 
**members_only** | **bool** | Whether the gallery is members only. | 
**name** | **String** | The gallery name. | 
**role_ids_to_auto_approve** | **Vec<String>** | The role IDs whose submissions are approved automatically. | 
**role_ids_to_manage** | **Vec<String>** | The role IDs that can manage the gallery. | 
**role_ids_to_submit** | **Vec<String>** | The role IDs that can submit to the gallery. | 
**role_ids_to_view** | **Vec<String>** | The role IDs that can view the gallery. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | The creation timestamp of the gallery. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | The last update timestamp of the gallery. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


