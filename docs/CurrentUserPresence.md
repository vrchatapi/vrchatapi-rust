# CurrentUserPresence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatar_thumbnail** | Option<**String**> |  | [optional]
**current_avatar_tags** | Option<**Vec<String>**> |  | [optional]
**debugflag** | Option<**String**> |  | [optional]
**display_name** | Option<**String**> |  | [optional]
**groups** | Option<**Vec<String>**> |  | [optional]
**id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**instance** | Option<**String**> |  | [optional]
**instance_type** | Option<**String**> | either an InstanceType or an empty string | [optional]
**is_rejoining** | Option<**String**> |  | [optional]
**platform** | Option<**String**> | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**profile_pic_override** | Option<**String**> |  | [optional]
**status** | Option<**String**> | either a UserStatus or empty string | [optional]
**traveling_to_instance** | Option<**String**> |  | [optional]
**traveling_to_world** | Option<**String**> | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | [optional]
**user_icon** | Option<**String**> |  | [optional]
**world** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


