# CreateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**r#type** | [**crate::models::InstanceType**](InstanceType.md) |  | 
**region** | [**crate::models::InstanceRegion**](InstanceRegion.md) |  | 
**owner_id** | Option<**String**> | A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise | [optional]
**role_ids** | Option<**Vec<String>**> | Group roleIds that are allowed to join if the type is \"group\" and groupAccessType is \"member\" | [optional]
**group_access_type** | Option<[**crate::models::GroupAccessType**](GroupAccessType.md)> |  | [optional]
**queue_enabled** | Option<**bool**> |  | [optional][default to false]
**closed_at** | Option<**String**> | The time after which users won't be allowed to join the instance. This doesn't work for public instances. | [optional]
**can_request_invite** | Option<**bool**> | Only applies to invite type instances to make them invite+ | [optional][default to false]
**hard_close** | Option<**bool**> | Currently unused, but will eventually be a flag to set if the closing of the instance should kick people. | [optional][default to false]
**invite_only** | Option<**bool**> |  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


