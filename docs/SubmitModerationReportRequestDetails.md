# SubmitModerationReportRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_id** | Option<**String**> |  | [optional]
**holder_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**image_type** | Option<**String**> | Relevant detail for reports about image content, such as emoji. | [optional]
**instance_age_gated** | Option<**bool**> | Relevant detail for reports taking place from within an instance. | [optional]
**instance_type** | Option<**String**> | Relevant detail for reports taking place from within an instance. | [optional]
**suggested_warnings** | Option<[**Vec<models::ContentFilter>**](ContentFilter.md)> | Relevant detail for reports about content that might not be tagged properly. | [optional]
**user_in_same_instance** | Option<**bool**> | Relevant detail for reports involving another user in the same instance world. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


