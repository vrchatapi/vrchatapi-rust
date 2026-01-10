# GroupMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_by_display_name** | Option<**String**> | Only missing when explicitly fetching own user. | [optional]
**accepted_by_id** | Option<**String**> | Only missing when explicitly fetching own user. | [optional]
**banned_at** | Option<**String**> | Only missing when explicitly fetching own user. | [optional]
**created_at** | Option<**String**> | Only missing when explicitly fetching own user. | [optional]
**group_id** | **String** |  | 
**has_joined_from_purchase** | Option<**bool**> | Missing when explicitly fetching own user, or when group isn't linked to a purchase. | [optional]
**id** | **String** |  | 
**is_representing** | **bool** | Whether the user is representing the group. This makes the group show up above the name tag in-game. | [default to false]
**is_subscribed_to_announcements** | **bool** |  | [default to false]
**is_subscribed_to_event_announcements** | Option<**bool**> | Only missing when explicitly fetching own user. | [optional]
**joined_at** | Option<**String**> |  | 
**last_post_read_at** | Option<**String**> |  | 
**m_role_ids** | **Vec<String>** |  | 
**manager_notes** | Option<**String**> | Only missing when explicitly fetching own user. | [optional]
**membership_status** | [**models::GroupMemberStatus**](GroupMemberStatus.md) |  | 
**role_ids** | **Vec<String>** |  | 
**user** | Option<[**models::GroupMemberLimitedUser**](GroupMemberLimitedUser.md)> |  | [optional]
**user_id** | **String** | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | 
**visibility** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


