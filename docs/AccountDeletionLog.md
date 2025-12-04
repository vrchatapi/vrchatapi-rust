# AccountDeletionLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_time** | Option<**String**> | Date and time of the deletion request. | [optional]
**deletion_scheduled** | Option<**String**> | When the deletion is scheduled to happen, standard is 14 days after the request. | [optional]
**message** | Option<**String**> | Typically \"Deletion requested\" or \"Deletion canceled\". Other messages like \"Deletion completed\" may exist, but are these are not possible to see as a regular user. | [optional][default to Deletion requested]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


