# CurrentUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_tos_version** | **i32** |  | 
**account_deletion_date** | Option<[**String**](string.md)> |  | [optional]
**active_friends** | Option<**Vec<String>**> |  | [optional]
**allow_avatar_copying** | **bool** |  | 
**bio** | **String** |  | 
**bio_links** | **Vec<String>** |  | 
**current_avatar** | **String** |  | 
**current_avatar_asset_url** | **String** |  | 
**current_avatar_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_thumbnail_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**date_joined** | [**String**](string.md) |  | 
**developer_type** | [**crate::models::DeveloperType**](DeveloperType.md) |  | 
**display_name** | **String** |  | 
**email_verified** | **bool** |  | 
**fallback_avatar** | Option<**String**> |  | [optional]
**friend_group_names** | **Vec<String>** | Always empty array. | 
**friend_key** | **String** |  | 
**friends** | **Vec<String>** |  | 
**has_birthday** | **bool** |  | 
**has_email** | **bool** |  | 
**has_logged_in_from_client** | **bool** |  | 
**has_pending_email** | **bool** |  | 
**home_location** | **String** | WorldID be \"offline\" on User profiles if you are not friends with that user. | 
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**is_friend** | **bool** |  | [default to false]
**last_login** | **String** |  | 
**last_platform** | **String** | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**obfuscated_email** | **String** |  | 
**obfuscated_pending_email** | **String** |  | 
**oculus_id** | **String** |  | 
**offline_friends** | Option<**Vec<String>**> |  | [optional]
**online_friends** | Option<**Vec<String>**> |  | [optional]
**past_display_names** | [**Vec<crate::models::PastDisplayName>**](PastDisplayName.md) |  | 
**profile_pic_override** | **String** |  | 
**state** | [**crate::models::UserState**](UserState.md) |  | 
**status** | [**crate::models::UserStatus**](UserStatus.md) |  | 
**status_description** | **String** |  | 
**status_first_time** | **bool** |  | 
**status_history** | **Vec<String>** |  | 
**steam_details** | [**serde_json::Value**](.md) |  | 
**steam_id** | **String** |  | 
**tags** | **Vec<String>** |  | 
**two_factor_auth_enabled** | **bool** |  | 
**unsubscribe** | **bool** |  | 
**user_icon** | **String** |  | 
**username** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


