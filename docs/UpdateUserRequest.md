# UpdateUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> |  | [optional]
**unsubscribe** | Option<**bool**> |  | [optional]
**birthday** | Option<[**String**](string.md)> |  | [optional]
**accepted_tos_version** | Option<**i32**> |  | [optional]
**tags** | Option<**Vec<String>**> |   | [optional]
**status** | Option<[**models::UserStatus**](UserStatus.md)> |  | [optional]
**status_description** | Option<**String**> |  | [optional]
**bio** | Option<**String**> |  | [optional]
**bio_links** | Option<**Vec<String>**> |  | [optional]
**pronouns** | Option<**String**> |  | [optional]
**is_booping_enabled** | Option<**bool**> |  | [optional]
**user_icon** | Option<**String**> | MUST be a valid VRChat /file/ url. | [optional]
**content_filters** | Option<**Vec<String>**> | These tags begin with `content_` and control content gating | [optional]
**display_name** | Option<**String**> | MUST specify currentPassword as well to change display name | [optional]
**revert_display_name** | Option<**bool**> | MUST specify currentPassword as well to revert display name | [optional]
**password** | Option<**String**> | MUST specify currentPassword as well to change password | [optional]
**current_password** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


