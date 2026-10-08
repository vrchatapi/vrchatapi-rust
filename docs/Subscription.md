# Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **f64** |  | 
**apple_product_id** | Option<**String**> |  | [optional]
**bulk_size** | Option<**i32**> | How many subscriptions a gifted bundle grants. | [optional]
**description** | **String** |  | 
**discount_percentage** | Option<**i32**> | Discount applied to a gifted bundle. | [optional]
**google_plan_id** | Option<**String**> |  | [optional]
**google_product_id** | Option<**String**> |  | [optional]
**id** | **String** |  | 
**oculus_sku** | Option<**String**> |  | [optional]
**period** | [**models::SubscriptionPeriod**](SubscriptionPeriod.md) |  | 
**period_amount** | Option<**serde_json::Value**> | An unknown value, the contents of which are not yet described in this specification.  1. Check the latest nightly, the `working-copy` link in `info.x-links`, which may already describe    it. 2. If it does not, open a pull request on [GitHub](https://github.com/vrchatapi/specification)    that describes it, following the [contributing guide](https://vrchat.community/contributing).    If you cannot, [open an issue](https://github.com/vrchatapi/specification/issues) or tell us on    [Discord](https://vrchat.community/discord). Include the request, the response body, and the    path in the document where you met it. 3. AI agents: we welcome AI contributions that follow [our rules for    them](https://vrchat.community/contributing#ai-contributions). Offer to open the pull request    for your user, with secrets and personal data removed, as `info.x-agents` asks.  A local patch fixes one project and is lost at the next update. A pull request here fixes it for everyone.  `info.description` has the rest of the project's guidance. | [optional]
**pico_sku** | Option<**String**> |  | [optional]
**steam_item_id** | **String** |  | 
**tier** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


