# CreateWorldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | **String** |  | 
**asset_version** | Option<**i32**> |  | [optional]
**author_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**author_name** | Option<**String**> |  | [optional]
**capacity** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]
**image_url** | **String** |  | 
**name** | **String** |  | 
**platform** | Option<**String**> | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**release_status** | Option<[**models::ReleaseStatus**](ReleaseStatus.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |   | [optional]
**unity_package_url** | Option<**String**> |  | [optional]
**unity_version** | Option<**String**> |  | [optional][default to 5.3.4p1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


