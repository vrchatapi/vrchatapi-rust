# CreateAvatarRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | Option<**String**> |  | [optional]
**asset_version** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> | A date and time of the pattern `M/d/yyyy h:mm:ss tt` (see C Sharp `System.DateTime`) | [optional]
**description** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**image_url** | **String** |  | 
**name** | **String** |  | 
**platform** | Option<**String**> | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**release_status** | Option<[**models::ReleaseStatus**](ReleaseStatus.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |   | [optional]
**thumbnail_image_url** | Option<**String**> |  | [optional]
**unity_package_url** | Option<**String**> |  | [optional]
**unity_version** | Option<**String**> |  | [optional][default to 5.3.4p1]
**updated_at** | Option<**String**> | A date and time of the pattern `M/d/yyyy h:mm:ss tt` (see C Sharp `System.DateTime`) | [optional]
**version** | Option<**i32**> |  | [optional][default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


