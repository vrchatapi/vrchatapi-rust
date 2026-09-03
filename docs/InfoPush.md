# InfoPush

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_min_version** | Option<**serde_json::Value**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**data** | [**models::InfoPushData**](InfoPushData.md) |  | 
**end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**experiment** | Option<[**models::InfoPushExperiment**](InfoPushExperiment.md)> |  | [optional]
**hash** | **String** | Unknown usage, MD5 | 
**id** | **String** |  | 
**is_enabled** | **bool** |  | [default to true]
**priority** | **i32** |  | 
**regions** | Option<**Vec<String>**> |  | [optional]
**release_status** | [**models::ReleaseStatus**](ReleaseStatus.md) |  | 
**require_client_tags** | Option<**Vec<String>**> |  | [optional]
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**tags** | **Vec<String>** |   | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


