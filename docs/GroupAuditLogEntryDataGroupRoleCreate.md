# GroupAuditLogEntryDataGroupRoleCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | The role description. | 
**is_added_on_join** | **bool** | Whether the role is automatically assigned on join. | 
**is_self_assignable** | **bool** | Whether users can self-assign this role. | 
**name** | **String** | The role name. | 
**order** | Option<**i32**> | The display order of the role. | [optional]
**permissions** | [**Vec<models::GroupPermissions>**](GroupPermissions.md) | The permissions assigned to this role. | 
**requires_purchase** | **bool** | Whether the role requires a purchase. | 
**requires_two_factor** | **bool** | Whether the role requires two-factor authentication. | 
**group_id** | **String** | The group ID. | 
**last_updated_by_user_id** | **String** | The ID of the user who last updated the role. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


