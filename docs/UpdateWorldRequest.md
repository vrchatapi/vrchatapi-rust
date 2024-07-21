# UpdateWorldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_url** | Option<**String**> |  | [optional]
**asset_version** | Option<**String**> |  | [optional]
**author_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**author_name** | Option<**String**> |  | [optional]
**capacity** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**platform** | Option<**String**> | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**release_status** | Option<[**models::ReleaseStatus**](ReleaseStatus.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |   | [optional]
**unity_package_url** | Option<**String**> |  | [optional]
**unity_version** | Option<**String**> |  | [optional][default to 5.3.4p1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


