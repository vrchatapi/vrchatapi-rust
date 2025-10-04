# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_verification_status** | [**models::AgeVerificationStatus**](AgeVerificationStatus.md) |  | 
**age_verified** | **bool** | `true` if, user is age verified (not 18+). | 
**allow_avatar_copying** | **bool** |  | [default to true]
**badges** | Option<[**Vec<models::Badge>**](Badge.md)> |   | [optional]
**bio** | **String** |  | 
**bio_links** | **Vec<String>** |  | 
**current_avatar_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_thumbnail_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_tags** | **Vec<String>** |  | 
**date_joined** | [**String**](string.md) |  | 
**developer_type** | [**models::DeveloperType**](DeveloperType.md) |  | 
**display_name** | **String** | A users visual display name. This is what shows up in-game, and can different from their `username`. Changing display name is restricted to a cooldown period. | 
**friend_key** | **String** |  | 
**friend_request_status** | Option<**String**> |  | [optional]
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**instance_id** | Option<**String**> | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | [optional]
**is_friend** | **bool** | Either their `friendKey`, or empty string if you are not friends. Unknown usage. | 
**last_activity** | **String** | Either a date-time or empty string. | 
**last_login** | **String** | Either a date-time or empty string. | 
**last_mobile** | Option<**String**> |  | [optional]
**last_platform** | **String** | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**location** | Option<**String**> | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | [optional]
**note** | Option<**String**> |  | [optional]
**platform** | Option<**String**> |  | [optional]
**profile_pic_override** | **String** |  | 
**profile_pic_override_thumbnail** | **String** |  | 
**pronouns** | **String** |  | 
**state** | [**models::UserState**](UserState.md) |  | 
**status** | [**models::UserStatus**](UserStatus.md) |  | 
**status_description** | **String** |  | 
**tags** | **Vec<String>** |   | 
**traveling_to_instance** | Option<**String**> |  | [optional]
**traveling_to_location** | Option<**String**> |  | [optional]
**traveling_to_world** | Option<**String**> |  | [optional]
**user_icon** | **String** |  | 
**username** | Option<**String**> | -| A users unique name, used during login. This is different from `displayName` which is what shows up in-game. A users `username` can never be changed.' **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429). | [optional]
**world_id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


