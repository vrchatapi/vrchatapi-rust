# LimitedUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bio** | Option<**String**> |  | [optional]
**bio_links** | Option<**Vec<String>**> |   | [optional]
**current_avatar_image_url** | Option<**String**> | When profilePicOverride is not empty, use it instead. | [optional]
**current_avatar_thumbnail_image_url** | Option<**String**> | When profilePicOverride is not empty, use it instead. | [optional]
**current_avatar_tags** | Option<**Vec<String>**> |  | [optional]
**developer_type** | [**crate::models::DeveloperType**](DeveloperType.md) |  | 
**display_name** | **String** |  | 
**fallback_avatar** | Option<**String**> |  | [optional]
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**is_friend** | **bool** |  | 
**last_platform** | **String** | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**profile_pic_override** | Option<**String**> |  | [optional]
**pronouns** | Option<**String**> |  | [optional]
**status** | [**crate::models::UserStatus**](UserStatus.md) |  | 
**status_description** | **String** |  | 
**tags** | **Vec<String>** | <- Always empty. | 
**user_icon** | Option<**String**> |  | [optional]
**username** | Option<**String**> | -| **DEPRECATED:** VRChat API no longer return usernames of other users. [See issue by Tupper for more information](https://github.com/pypy-vrc/VRCX/issues/429). | [optional]
**location** | Option<**String**> |  | [optional]
**friend_key** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


