# CreateUserModerationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expires** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**instance_id** | Option<**String**> | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | [optional]
**is_permanent** | Option<**bool**> |  | [optional]
**reason** | **String** |  | 
**r#type** | [**models::CreateUserModerationRequestType**](CreateUserModerationRequestType.md) |  | 
**world_id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


