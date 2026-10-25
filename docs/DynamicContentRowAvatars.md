# DynamicContentRowAvatars

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | [**models::LocalizableString**](LocalizableString.md) |  | 
**platform** | **String** | Usually \"ThisPlatformSupported\", but can also be other values such as \"all\" or platform specific identifiers. | 
**any_style** | Option<**Vec<String>**> |  | [optional]
**any_tag** | Option<**Vec<String>**> |  | [optional]
**marketplace** | Option<**String**> |  | [optional]
**max_price** | Option<**i32**> |  | [optional]
**min_price** | Option<**i32**> |  | [optional]
**notag** | Option<**Vec<String>**> |  | [optional]
**params** | **std::collections::HashMap<String, serde_json::Value>** |  | 
**primary_style** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**sort_heading** | **String** |  | 
**sort_order** | [**models::OrderOption**](OrderOption.md) |  | 
**sort_ownership** | **String** |  | 
**style** | Option<**String**> |  | [optional]
**tag** | Option<**String**> | Tag to filter content for this row. Not a `Tag`: that type forbids the empty string, which this field uses for a row that is not tag-filtered. | [optional]
**r#type** | **Type** |  (enum: avatar) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


