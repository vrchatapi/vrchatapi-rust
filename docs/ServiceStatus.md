# ServiceStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**id** | **String** | The id of this service, NOT the id of the thing this service was requested for. | 
**progress** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**requester_user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**state** | **String** |  | 
**subject_id** | **String** | The id of the thing this service was requested for. | 
**subject_type** | **String** | The kind of the thing this service was requested for. | 
**r#type** | **String** | The kind of service that was requested. | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


