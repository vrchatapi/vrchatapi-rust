# CurrentUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_privacy_version** | Option<**i32**> |  | [optional]
**accepted_tos_version** | **i32** |  | 
**account_deletion_date** | Option<**chrono::NaiveDate**> |  | [optional]
**account_deletion_log** | Option<[**Vec<models::AccountDeletionLog>**](AccountDeletionLog.md)> |   | [optional]
**active_friends** | Option<**Vec<String>**> |   | [optional]
**age_verification_status** | [**models::AgeVerificationStatus**](AgeVerificationStatus.md) |  | 
**age_verified** | **bool** | `true` if, user is age verified (not 18+). | 
**allow_avatar_copying** | **bool** |  | 
**apple_details** | Option<**serde_json::Value**> |  | [optional]
**apple_id** | Option<**String**> |  | [optional]
**auth_token** | Option<**String**> | The auth token for NEWLY REGISTERED ACCOUNTS ONLY (/auth/register) | [optional]
**badges** | Option<[**Vec<models::Badge>**](Badge.md)> |   | [optional]
**banner_color** | Option<**String**> |  | [optional]
**banner_type** | Option<**String**> |  | [optional]
**bio** | **String** |  | 
**bio_links** | **Vec<String>** |   | 
**completed_tutorials** | Option<**Vec<String>**> |  | [optional]
**content_filters** | Option<**Vec<String>**> | These tags begin with `content_` and control content gating | [optional]
**current_avatar** | **String** |  | 
**current_avatar_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_tags** | **Vec<String>** |  | 
**current_avatar_thumbnail_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**date_joined** | **chrono::NaiveDate** |  | 
**developer_type** | [**models::DeveloperType**](DeveloperType.md) |  | 
**discord_details** | Option<[**models::DiscordDetails**](DiscordDetails.md)> |  | [optional]
**discord_id** | Option<**String**> | https://discord.com/developers/docs/reference#snowflakes | [optional]
**display_name** | **String** |  | 
**email_verified** | **bool** |  | 
**fallback_avatar** | Option<**String**> |  | [optional]
**friend_group_names** | **Vec<String>** | Always empty array. | 
**friend_key** | **String** |  | 
**friend_request_status** | Option<**String**> | State of a friend request between the caller and this user. VRChat sends the string `\"null\"`, not JSON `null`. | [optional]
**friends** | **Vec<String>** |  | 
**google_details** | Option<**serde_json::Value**> |  | [optional]
**google_id** | Option<**String**> |  | [optional]
**has_birthday** | **bool** |  | 
**has_discord_friends_opt_out** | Option<**bool**> |  | [optional]
**has_email** | **bool** |  | 
**has_logged_in_from_client** | **bool** |  | 
**has_pending_email** | **bool** |  | 
**has_shared_connections_opt_out** | Option<**bool**> |  | [optional]
**hide_content_filter_settings** | Option<**bool**> |  | [optional]
**home_location** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**icon_frame** | Option<**String**> |  | [optional]
**icon_url** | Option<**String**> |  | [optional]
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**instance_id** | Option<**String**> | InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance. | [optional]
**is_adult** | **bool** |  | 
**is_booping_enabled** | Option<**bool**> |  | [optional][default to true]
**is_economy_creator** | Option<**bool**> |  | [optional][default to false]
**is_friend** | **bool** |  | [default to false]
**is_temporary** | Option<**bool**> |  | [optional][default to false]
**last_activity** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**last_login** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**last_mobile** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | 
**last_platform** | **String** | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity version such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**location** | Option<**String**> | Represents a unique location, consisting of a world identifier and an instance identifier, or \"offline\" if the user is not on your friends list. | [optional]
**nameplate_effect** | Option<**String**> |  | [optional]
**note** | Option<**String**> |  | [optional]
**obfuscated_email** | **String** |  | 
**obfuscated_pending_email** | **String** |  | 
**oculus_id** | **String** |  | 
**offline_friends** | Option<**Vec<String>**> |  | [optional]
**online_friends** | Option<**Vec<String>**> |  | [optional]
**past_display_names** | [**Vec<models::PastDisplayName>**](PastDisplayName.md) |   | 
**personalization_opt_out** | Option<**bool**> |  | [optional]
**pico_id** | Option<**String**> |  | [optional]
**platform** | Option<**String**> |  | [optional]
**platform_history** | Option<[**Vec<models::CurrentUserPlatformHistoryInner>**](CurrentUserPlatformHistoryInner.md)> |  | [optional]
**presence** | Option<[**models::CurrentUserPresence**](CurrentUserPresence.md)> |  | [optional]
**profile_effect** | Option<**String**> |  | [optional]
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
**steam_details** | **serde_json::Value** |  | 
**steam_id** | **String** |  | 
**tags** | **Vec<String>** |  | 
**temporary_expiry_date** | Option<**serde_json::Value**> |  | [optional]
**traveling_to_instance** | Option<**String**> |  | [optional]
**traveling_to_location** | Option<**String**> |  | [optional]
**traveling_to_world** | Option<**String**> |  | [optional]
**twitch_details** | Option<**serde_json::Value**> |  | [optional]
**twitch_id** | Option<**String**> |  | [optional]
**two_factor_auth_enabled** | **bool** |  | 
**two_factor_auth_enabled_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**unsubscribe** | **bool** |  | 
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**user_icon** | **String** |  | 
**user_language** | Option<**String**> |  | [optional]
**user_language_code** | Option<**String**> |  | [optional]
**username** | Option<**String**> | Your own unique name, used during login. Distinct from `displayName`, and never returned for another user. | [optional]
**uses_generated_password** | **bool** |  | 
**vive_id** | Option<**String**> |  | [optional]
**world_id** | Option<**String**> | WorldID be \"offline\" on User profiles if you are not friends with that user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


