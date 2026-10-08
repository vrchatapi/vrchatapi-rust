# Avatar

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acknowledgements** | Option<**String**> |  | [optional]
**active_asset_review_id** | Option<**String**> | Only present for the avatar author on avatars under active review. | [optional]
**asset_url** | Option<**String**> | Not present from general search `/avatars`, only on specific requests `/avatars/{avatarId}`. | [optional]
**asset_url_object** | Option<**serde_json::Value**> | Not present from general search `/avatars`, only on specific requests `/avatars/{avatarId}`. **Deprecation:** `Object` has unknown usage/fields, and is always empty. Use normal `Url` field instead. | [optional]
**attribution** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**author_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**author_name** | **String** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**description** | **String** |  | 
**featured** | **bool** |  | [default to false]
**highest_price** | Option<**i32**> |  | [optional]
**id** | **String** |  | 
**image_url** | **String** |  | 
**listing_date** | Option<**String**> |  | 
**lock** | Option<**bool**> |  | [optional]
**lowest_price** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**pending_upload** | Option<**bool**> |  | [optional][default to false]
**performance** | [**models::AvatarPerformance**](AvatarPerformance.md) |  | 
**product_id** | Option<**String**> |  | [optional]
**published_listings** | Option<[**Vec<models::AvatarPublishedListingsInner>**](AvatarPublishedListingsInner.md)> |  | [optional]
**release_status** | [**models::ReleaseStatus**](ReleaseStatus.md) |  | 
**searchable** | Option<**bool**> |  | [optional][default to false]
**styles** | [**models::AvatarStyles**](AvatarStyles.md) |  | 
**tags** | **Vec<String>** |  | 
**thumbnail_image_url** | **String** |  | 
**unity_package_url** | **String** |  | 
**unity_package_url_object** | [**models::AvatarUnityPackageUrlObject**](AvatarUnityPackageUrlObject.md) |  | 
**unity_packages** | [**HashSet<models::UnityPackage>**](UnityPackage.md) |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**version** | **i32** |  | [default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


