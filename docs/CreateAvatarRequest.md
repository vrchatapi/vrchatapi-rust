# CreateAvatarRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | Option<**String**> |  | [optional]
**asset_version** | Option<**String**> |  | [optional]
**platform** | Option<**String**> | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**created_at** | Option<**String**> | A date and time of the pattern `M/d/yyyy h:mm:ss tt` (see C Sharp `System.DateTime`) | [optional]
**updated_at** | Option<**String**> | A date and time of the pattern `M/d/yyyy h:mm:ss tt` (see C Sharp `System.DateTime`) | [optional]
**id** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |   | [optional]
**image_url** | **String** |  | 
**thumbnail_image_url** | Option<**String**> |  | [optional]
**release_status** | Option<[**models::ReleaseStatus**](ReleaseStatus.md)> |  | [optional]
**version** | Option<**i32**> |  | [optional][default to 1]
**featured** | Option<**bool**> | Enabling featured tag requires Admin Credentials. | [optional]
**unity_package_url** | Option<**String**> |  | [optional]
**unity_version** | Option<**String**> |  | [optional][default to 5.3.4p1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


