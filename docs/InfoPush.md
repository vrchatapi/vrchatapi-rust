# InfoPush

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_min_version** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**data** | [**models::InfoPushData**](InfoPushData.md) |  | 
**end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**experiment** | Option<[**models::InfoPushExperiment**](InfoPushExperiment.md)> |  | [optional]
**hash** | **String** | Unknown usage, MD5 | 
**id** | **String** |  | 
**is_enabled** | **bool** |  | [default to true]
**priority** | **i32** |  | 
**regions** | Option<**Vec<String>**> |  | [optional]
**release_status** | [**models::ReleaseStatus**](ReleaseStatus.md) |  | 
**require_client_tags** | Option<**Vec<String>**> |  | [optional]
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**tags** | **Vec<String>** |  | 
**r#type** | Option<**String**> |  | [optional]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


