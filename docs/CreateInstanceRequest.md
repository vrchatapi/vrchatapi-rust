# CreateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**r#type** | [**crate::models::InstanceType**](InstanceType.md) |  | 
**region** | [**crate::models::Region**](Region.md) |  | 
**owner_id** | Option<**String**> | A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise | [optional]
**role_ids** | Option<**Vec<String>**> | Group roleIds that are allowed to join if the type is \"group\" and groupAccessType is \"member\" | [optional]
**group_access_type** | Option<[**crate::models::GroupAccessType**](GroupAccessType.md)> |  | [optional]
**queue_enabled** | Option<**bool**> |  | [optional][default to false]
**closed_at** | Option<**String**> | The time after which users won't be allowed to join the instance | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


