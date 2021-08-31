# CurrentUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [readonly]
**username** | **String** |  | 
**display_name** | **String** |  | 
**user_icon** | **String** |  | 
**bio** | **String** |  | 
**bio_links** | **Vec<String>** |  | 
**profile_pic_override** | **String** |  | 
**status_description** | **String** |  | 
**past_display_names** | [**Vec<crate::models::PastDisplayName>**](PastDisplayName.md) |  | 
**has_email** | **bool** |  | 
**has_pending_email** | **bool** |  | 
**obfuscated_email** | **String** |  | 
**obfuscated_pending_email** | **String** |  | 
**email_verified** | **bool** |  | 
**has_birthday** | **bool** |  | 
**unsubscribe** | **bool** |  | 
**status_history** | **Vec<String>** |  | 
**status_first_time** | **bool** |  | 
**friends** | **Vec<String>** |  | 
**friend_group_names** | **Vec<String>** | Always empty array. | 
**current_avatar_image_url** | **String** |  | 
**current_avatar_thumbnail_image_url** | **String** |  | 
**fallback_avatar** | **String** |  | 
**current_avatar** | **String** |  | 
**current_avatar_asset_url** | **String** |  | 
**account_deletion_date** | Option<[**String**](string.md)> |  | [optional]
**accepted_tos_version** | **f32** |  | 
**steam_id** | **String** |  | 
**steam_details** | [**serde_json::Value**](.md) |  | 
**oculus_id** | **String** |  | 
**has_logged_in_from_client** | **bool** |  | 
**home_location** | **String** |  | 
**two_factor_auth_enabled** | **bool** |  | 
**state** | [**crate::models::UserState**](UserState.md) |  | 
**tags** | **Vec<String>** |  | 
**developer_type** | [**crate::models::DeveloperType**](DeveloperType.md) |  | 
**last_login** | **String** |  | 
**last_platform** | [**crate::models::Platform**](Platform.md) |  | 
**allow_avatar_copying** | **bool** |  | 
**status** | [**crate::models::UserStatus**](UserStatus.md) |  | 
**date_joined** | [**String**](string.md) |  | [readonly]
**is_friend** | **bool** |  | [default to false]
**friend_key** | **String** |  | 
**online_friends** | Option<**Vec<String>**> |  | [optional]
**active_friends** | Option<**Vec<String>**> |  | [optional]
**offline_friends** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


