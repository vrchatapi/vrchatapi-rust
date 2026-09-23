# ServiceStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**id** | **String** | The id of this service, NOT the id of the thing this service was requested for. | 
**progress** | **Vec<serde_json::Value>** |  | 
**requester_user_id** | **String** | The id of the user who requested this service. | 
**state** | **String** |  | 
**subject_id** | **String** | The id of the thing this service was requested for. | 
**subject_type** | **String** | The kind of the thing this service was requested for. | 
**r#type** | **String** | The kind of service that was requested. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


