# PrivateProfileActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instance_id** | **String** | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | 
**last_activity** | **String** | Either a date-time or an empty string. | 
**last_login** | **String** | Either a date-time or an empty string. | 
**location** | **String** | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | 
**platform** | **String** | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**state** | [**models::UserState**](UserState.md) |  | 
**traveling_to_instance** | **String** |  | 
**traveling_to_location** | **String** |  | 
**traveling_to_world** | **String** |  | 
**world_id** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


