# UpdateUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_tos_version** | Option<**i32**> |  | [optional]
**bio** | Option<**String**> |  | [optional]
**bio_links** | Option<**Vec<String>**> |  | [optional]
**birthday** | Option<[**String**](string.md)> |  | [optional]
**content_filters** | Option<[**Vec<models::ContentFilter>**](ContentFilter.md)> | These tags begin with `content_` and control content gating | [optional]
**current_password** | Option<**String**> |  | [optional]
**display_name** | Option<**String**> | MUST specify currentPassword as well to change display name | [optional]
**email** | Option<**String**> |  | [optional]
**has_shared_connections_opt_out** | Option<**bool**> | Opt out of the Mutuals feature | [optional]
**is_booping_enabled** | Option<**bool**> |  | [optional]
**password** | Option<**String**> | MUST specify currentPassword as well to change password | [optional]
**pronouns** | Option<**String**> |  | [optional]
**revert_display_name** | Option<**bool**> | MUST specify currentPassword as well to revert display name | [optional]
**status** | Option<[**models::UserStatus**](UserStatus.md)> |  | [optional]
**status_description** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |   | [optional]
**unsubscribe** | Option<**bool**> |  | [optional]
**user_icon** | Option<**String**> | MUST be a valid VRChat /file/ url. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


