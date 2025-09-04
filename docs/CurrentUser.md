# CurrentUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_tos_version** | **i32** |  | 
**accepted_privacy_version** | Option<**i32**> |  | [optional]
**account_deletion_date** | Option<[**String**](string.md)> |  | [optional]
**account_deletion_log** | Option<[**Vec<models::AccountDeletionLog>**](AccountDeletionLog.md)> |   | [optional]
**active_friends** | Option<**Vec<String>**> |   | [optional]
**age_verification_status** | [**models::AgeVerificationStatus**](AgeVerificationStatus.md) |  | 
**age_verified** | **bool** | `true` if, user is age verified (not 18+). | 
**allow_avatar_copying** | **bool** |  | 
**auth_token** | Option<**String**> | The auth token for NEWLY REGISTERED ACCOUNTS ONLY (/auth/register) | [optional]
**badges** | Option<[**Vec<models::Badge>**](Badge.md)> |   | [optional]
**bio** | **String** |  | 
**bio_links** | **Vec<String>** |   | 
**content_filters** | Option<**Vec<String>**> | These tags begin with `content_` and control content gating | [optional]
**current_avatar** | **String** |  | 
**current_avatar_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_thumbnail_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_tags** | **Vec<String>** |  | 
**date_joined** | [**String**](string.md) |  | 
**developer_type** | [**models::DeveloperType**](DeveloperType.md) |  | 
**display_name** | **String** |  | 
**email_verified** | **bool** |  | 
**fallback_avatar** | Option<**String**> |  | [optional]
**friend_group_names** | **Vec<String>** | Always empty array. | 
**friend_key** | **String** |  | 
**friends** | **Vec<String>** |  | 
**has_birthday** | **bool** |  | 
**hide_content_filter_settings** | Option<**bool**> |  | [optional]
**user_language** | Option<**String**> |  | [optional]
**user_language_code** | Option<**String**> |  | [optional]
**has_email** | **bool** |  | 
**has_logged_in_from_client** | **bool** |  | 
**has_pending_email** | **bool** |  | 
**home_location** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**is_adult** | **bool** |  | 
**is_booping_enabled** | Option<**bool**> |  | [optional][default to true]
**is_friend** | **bool** |  | [default to false]
**last_activity** | Option<**String**> |  | [optional]
**last_login** | **String** |  | 
**last_mobile** | Option<**String**> |  | 
**last_platform** | **String** | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**obfuscated_email** | **String** |  | 
**obfuscated_pending_email** | **String** |  | 
**oculus_id** | **String** |  | 
**google_id** | Option<**String**> |  | [optional]
**google_details** | Option<[**serde_json::Value**](.md)> |  | [optional]
**pico_id** | Option<**String**> |  | [optional]
**vive_id** | Option<**String**> |  | [optional]
**offline_friends** | Option<**Vec<String>**> |  | [optional]
**online_friends** | Option<**Vec<String>**> |  | [optional]
**past_display_names** | [**Vec<models::PastDisplayName>**](PastDisplayName.md) |   | 
**presence** | Option<[**models::CurrentUserPresence**](CurrentUserPresence.md)> |  | [optional]
**platform_history** | Option<[**Vec<models::CurrentUserPlatformHistoryInner>**](CurrentUser_platform_history_inner.md)> |  | [optional]
**profile_pic_override** | **String** |  | 
**profile_pic_override_thumbnail** | **String** |  | 
**pronouns** | **String** |  | 
**pronouns_history** | **Vec<String>** |  | 
**queued_instance** | Option<**String**> |  | [optional]
**receive_mobile_invitations** | Option<**bool**> |  | [optional]
**state** | [**models::UserState**](UserState.md) |  | 
**status** | [**models::UserStatus**](UserStatus.md) |  | 
**status_description** | **String** |  | 
**status_first_time** | **bool** |  | 
**status_history** | **Vec<String>** |  | 
**steam_details** | [**serde_json::Value**](.md) |  | 
**steam_id** | **String** |  | 
**tags** | **Vec<String>** |  | 
**two_factor_auth_enabled** | **bool** |  | 
**two_factor_auth_enabled_date** | Option<**String**> |  | [optional]
**unsubscribe** | **bool** |  | 
**updated_at** | Option<**String**> |  | [optional]
**user_icon** | **String** |  | 
**username** | Option<**String**> | -| **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


