# InviteMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_be_updated** | **bool** |  | [default to true]
**id** | **String** |  | 
**message** | **String** |  | 
**message_type** | [**models::InviteMessageType**](InviteMessageType.md) |  | 
**remaining_cooldown_minutes** | **i32** | Changes to 60 when updated, although probably server-side configurable. | [default to 0]
**slot** | **i32** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | A date and time in the form of an RFC 3339 `date-time`, except that the UTC offset can be written without its colon, as in `+0000`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


