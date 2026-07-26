# PrivateProfileActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instance_id** | Option<**String**> | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | [optional]
**last_activity** | Option<**String**> | Either a date-time or an empty string. | [optional]
**last_login** | Option<**String**> | Either a date-time or an empty string. | [optional]
**location** | Option<**String**> | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | [optional]
**platform** | Option<**String**> | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**state** | Option<[**models::UserState**](UserState.md)> |  | [optional]
**traveling_to_instance** | Option<**String**> |  | [optional]
**traveling_to_location** | Option<**String**> |  | [optional]
**traveling_to_world** | Option<**String**> |  | [optional]
**world_id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


