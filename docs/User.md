# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_privacy_version** | Option<**i32**> |  | [optional]
**accepted_tos_version** | Option<**i32**> |  | [optional]
**account_deletion_date** | Option<**String**> |  | [optional]
**account_deletion_log** | Option<**Vec<serde_json::Value>**> |  | [optional]
**age_verification_status** | [**models::AgeVerificationStatus**](AgeVerificationStatus.md) |  | 
**age_verified** | **bool** | `true` if, user is age verified (not 18+). | 
**allow_avatar_copying** | **bool** |  | [default to true]
**apple_details** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**badges** | Option<[**Vec<models::Badge>**](Badge.md)> |   | [optional]
**banner_color** | Option<**String**> |  | [optional]
**banner_type** | Option<**String**> |  | [optional]
**banner_url** | Option<**String**> |  | [optional]
**bio** | **String** |  | 
**bio_links** | **Vec<String>** |  | 
**current_avatar_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_tags** | **Vec<String>** |  | 
**current_avatar_thumbnail_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**date_joined** | **chrono::NaiveDate** |  | 
**developer_type** | [**models::DeveloperType**](DeveloperType.md) |  | 
**display_name** | **String** | A users visual display name. This is what shows up in-game, and can different from their `username`. Changing display name is restricted to a cooldown period. | 
**friend_key** | **String** |  | 
**friend_request_status** | Option<**String**> | State of a friend request between the caller and this user. VRChat sends the string `\"null\"`, not JSON `null`. | [optional]
**icon_frame** | Option<**String**> |  | [optional]
**icon_url** | Option<**String**> |  | [optional]
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**instance_id** | Option<**String**> | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | [optional]
**is_economy_creator** | Option<**bool**> |  | [optional]
**is_friend** | **bool** | Either their `friendKey`, or empty string if you are not friends. Unknown usage. | 
**last_activity** | **String** | Either a date-time or empty string. | 
**last_login** | **String** | Either a date-time or empty string. | 
**last_mobile** | Option<**String**> |  | [optional]
**last_platform** | **String** | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**location** | Option<**String**> | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | [optional]
**nameplate_effect** | Option<**String**> |  | [optional]
**note** | Option<**String**> |  | [optional]
**platform** | Option<**String**> |  | [optional]
**profile_effect** | Option<**String**> |  | [optional]
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
**world_id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


