# LimitedUserFriend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bio** | Option<**String**> |  | [optional]
**bio_links** | Option<**Vec<String>**> |   | [optional]
**current_avatar_image_url** | Option<**String**> | When profilePicOverride is not empty, use it instead. | [optional]
**current_avatar_tags** | Option<**Vec<String>**> |  | [optional]
**current_avatar_thumbnail_image_url** | Option<**String**> | When profilePicOverride is not empty, use it instead. | [optional]
**developer_type** | [**models::DeveloperType**](DeveloperType.md) |  | 
**display_name** | **String** |  | 
**friend_key** | **String** |  | 
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**image_url** | **String** |  | 
**is_friend** | **bool** |  | 
**last_activity** | Option<**String**> |  | 
**last_login** | Option<**String**> |  | 
**last_mobile** | Option<**String**> |  | 
**last_platform** | **String** | This is normally `android`, `ios`, `standalonewindows`, `web`, or the empty value ``, but also supposedly can be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**location** | **String** |  | 
**platform** | **String** |  | 
**profile_pic_override** | Option<**String**> |  | [optional]
**profile_pic_override_thumbnail** | Option<**String**> |  | [optional]
**status** | [**models::UserStatus**](UserStatus.md) |  | 
**status_description** | **String** |  | 
**tags** | **Vec<String>** | <- Always empty. | 
**user_icon** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


