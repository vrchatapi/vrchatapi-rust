# FileVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **f32** | Incremental version counter, can only be increased. | [default to 0]
**status** | [**crate::models::FileStatus**](FileStatus.md) |  | 
**created_at** | **String** |  | [readonly]
**file** | Option<[**crate::models::FileData**](FileData.md)> |  | [optional]
**delta** | Option<[**crate::models::FileData**](FileData.md)> |  | [optional]
**signature** | Option<[**crate::models::FileData**](FileData.md)> |  | [optional]
**deleted** | Option<**bool**> | Usually only present if `true` | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


