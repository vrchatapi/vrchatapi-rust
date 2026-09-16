# PublicProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_verification_status** | Option<[**models::AgeVerificationStatus**](AgeVerificationStatus.md)> |  | [optional]
**age_verified** | Option<**bool**> | `true` if, user is age verified (not 18+). | [optional]
**background_texture_id** | Option<**String**> |  | [optional]
**background_type** | Option<**String**> |  | [optional]
**badges** | Option<[**Vec<models::Badge>**](Badge.md)> |  | [optional]
**banner_color** | Option<**String**> |  | [optional]
**banner_type** | Option<**String**> |  | [optional]
**banner_url** | Option<**String**> |  | [optional]
**bio** | Option<**String**> |  | [optional]
**bio_links** | Option<**Vec<String>**> |  | [optional]
**display_name** | Option<**String**> |  | [optional]
**groups** | Option<[**models::ProfileGroups**](ProfileGroups.md)> |  | [optional]
**has_vrc_plus** | Option<**bool**> |  | [optional]
**icon_frame** | Option<**String**> |  | [optional]
**icon_url** | Option<**String**> |  | [optional]
**id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**is_economy_creator** | Option<**bool**> |  | [optional]
**languages** | Option<**Vec<String>**> |  | [optional]
**nameplate_effect** | Option<**String**> |  | [optional]
**profile_effect** | Option<**String**> |  | [optional]
**pronouns** | Option<**String**> |  | [optional]
**public_worlds** | Option<[**Vec<models::LimitedWorld>**](LimitedWorld.md)> |  | [optional]
**represented_group** | Option<[**models::ProfileRepresentedGroup**](ProfileRepresentedGroup.md)> |  | [optional]
**theme_button_color** | Option<**String**> | Hex colour without a leading `#`. | [optional]
**theme_icon_color** | Option<**String**> | Hex colour without a leading `#`. | [optional]
**theme_id** | Option<**String**> |  | [optional]
**theme_subtext_color** | Option<**String**> | Hex colour without a leading `#`. | [optional]
**total_public_worlds_count** | Option<**i32**> |  | [optional]
**trust_tags** | Option<**Vec<String>**> |  | [optional]
**world_favorite_lists** | Option<**Vec<serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


