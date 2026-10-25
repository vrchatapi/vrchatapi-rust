# InfoPushData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_name** | **String** |  | 
**avatar_id** | **String** |  | 
**banner_image_url** | **String** |  | 
**final_name** | **String** |  | 
**icon_image_url** | **String** |  | 
**override_name** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | 
**name** | **String** |  | 
**tooltip_description** | [**models::LocalizableString**](LocalizableString.md) |  | 
**description** | [**models::LocalizableString**](LocalizableString.md) |  | 
**image_url** | **String** |  | 
**ips_query** | [**models::InfoPushIpsQuery**](InfoPushIpsQuery.md) |  | 
**is_new** | **bool** |  | 
**thumbnail_image_url** | Option<**String**> |  | 
**content_list** | [**models::DynamicContentRow**](DynamicContentRow.md) |  | 
**rows** | Option<**i32**> | Number of rows to render. | 
**article** | [**models::InfoPushDataArticle**](InfoPushDataArticle.md) |  | 
**category** | **String** |  | 
**weight** | **i32** |  | 
**categories** | [**Vec<models::InfoPushDataCategory>**](InfoPushDataCategory.md) |  | 
**featured_avatar_category_id** | Option<**String**> |  | [optional]
**hover_to_join** | Option<**bool**> |  | [optional]
**short_name** | Option<[**models::LocalizableString**](LocalizableString.md)> |  | [optional]
**disclaimer_text** | **String** |  | 
**domain_list** | [**Vec<models::InfoPushDataDomain>**](InfoPushDataDomain.md) |  | 
**world_tag** | **String** |  | 
**search** | [**models::InfoPushDataSearch**](InfoPushDataSearch.md) |  | 
**listing_ids** | **Vec<String>** |  | 
**on_pressed** | Option<[**models::InfoPushDataClickable**](InfoPushDataClickable.md)> |  | [optional]
**version** | **String** |  | 
**body** | Option<**String**> |  | [optional]
**cta** | Option<[**models::InfoPushDataCallToAction**](InfoPushDataCallToAction.md)> |  | [optional]
**delivery_behavior** | Option<[**models::InfoPushDataDeliveryBehavior**](InfoPushDataDeliveryBehavior.md)> |  | [optional]
**image_file_id** | Option<**String**> |  | [optional]
**media_type** | Option<**String**> |  | [optional]
**promotion** | Option<[**models::InfoPushDataPromotion**](InfoPushDataPromotion.md)> |  | [optional]
**show_in_world_ids** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**template** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**video_file_id** | Option<**String**> |  | [optional]
**video_url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


