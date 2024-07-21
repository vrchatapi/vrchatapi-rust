# FileVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**deleted** | Option<**bool**> | Usually only present if `true` | [optional][default to true]
**delta** | Option<[**models::FileData**](FileData.md)> |  | [optional]
**file** | Option<[**models::FileData**](FileData.md)> |  | [optional]
**signature** | Option<[**models::FileData**](FileData.md)> |  | [optional]
**status** | [**models::FileStatus**](FileStatus.md) |  | 
**version** | **i32** | Incremental version counter, can only be increased. | [default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


