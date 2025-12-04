# LimitedUserInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_verification_status** | [**models::AgeVerificationStatus**](AgeVerificationStatus.md) |  | 
**age_verified** | **bool** | `true` if, user is age verified (not 18+). | 
**allow_avatar_copying** | **bool** |  | 
**bio** | Option<**String**> |  | [optional]
**bio_links** | Option<**Vec<String>**> |   | [optional]
**current_avatar_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**current_avatar_tags** | **Vec<String>** |  | 
**current_avatar_thumbnail_image_url** | **String** | When profilePicOverride is not empty, use it instead. | 
**date_joined** | Option<**String**> |  | 
**developer_type** | [**models::DeveloperType**](DeveloperType.md) |  | 
**display_name** | **String** |  | 
**friend_key** | **String** |  | 
**id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**image_url** | Option<**String**> |  | [optional]
**is_friend** | **bool** |  | 
**last_activity** | Option<**String**> |  | 
**last_mobile** | Option<**String**> |  | 
**last_platform** | **String** | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | 
**platform** | Option<**String**> | This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`. | [optional]
**profile_pic_override** | Option<**String**> |  | [optional]
**profile_pic_override_thumbnail** | Option<**String**> |  | [optional]
**pronouns** | **String** |  | 
**state** | [**models::UserState**](UserState.md) |  | 
**status** | [**models::UserStatus**](UserStatus.md) |  | 
**status_description** | **String** |  | 
**tags** | **Vec<String>** |  | 
**user_icon** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


