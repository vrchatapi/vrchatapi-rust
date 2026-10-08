# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_verification_beta_code** | Option<**String**> |  | [optional]
**age_verification_beta_slots** | Option<**f64**> |  | [optional]
**age_verification_slots_available** | Option<**bool**> |  | [optional]
**allow_group_join_prompt** | Option<**bool**> |  | [optional]
**badges** | Option<**Vec<String>**> |  | [optional]
**banner_id** | Option<**String**> |  | [optional]
**banner_url** | Option<**String**> |  | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**discriminator** | Option<**String**> |  | [optional]
**galleries** | Option<[**Vec<models::GroupGallery>**](GroupGallery.md)> |  | [optional]
**icon_id** | Option<**String**> |  | [optional]
**icon_url** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**is_verified** | Option<**bool**> |  | [optional][default to false]
**join_state** | Option<[**models::GroupJoinState**](GroupJoinState.md)> |  | [optional]
**languages** | Option<**Vec<String>**> |  | [optional]
**last_post_created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**links** | Option<**Vec<String>**> |  | [optional]
**member_count** | Option<**i32**> |  | [optional]
**member_count_synced_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**membership_status** | Option<[**models::GroupMemberStatus**](GroupMemberStatus.md)> |  | [optional]
**my_member** | Option<[**models::GroupMyMember**](GroupMyMember.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**nameplate_id** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**nameplate_url** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**online_member_count** | Option<**i32**> |  | [optional]
**owner_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**privacy** | Option<[**models::GroupPrivacy**](GroupPrivacy.md)> |  | [optional]
**roles** | Option<[**Vec<models::GroupRole>**](GroupRole.md)> | Only returned if ?includeRoles=true is specified. | [optional]
**rules** | Option<**String**> |  | [optional]
**short_code** | Option<**String**> |  | [optional]
**store_id** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**transfer_target_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


