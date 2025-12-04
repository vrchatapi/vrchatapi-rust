# GroupLimitedMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**banned_at** | Option<**String**> | Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user. | [optional]
**created_at** | Option<**String**> | Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user. | [optional]
**group_id** | Option<**String**> |  | [optional]
**has_joined_from_purchase** | Option<**bool**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**is_representing** | Option<**bool**> | Whether the user is representing the group. This makes the group show up above the name tag in-game. | [optional][default to false]
**is_subscribed_to_announcements** | Option<**bool**> |  | [optional][default to false]
**is_subscribed_to_event_announcements** | Option<**bool**> |  | [optional]
**joined_at** | Option<**String**> |  | [optional]
**last_post_read_at** | Option<**String**> |  | [optional]
**m_role_ids** | Option<**Vec<String>**> |  | [optional]
**manager_notes** | Option<**String**> | Only visible via the /groups/:groupId/members endpoint, **not** when fetching a specific user. | [optional]
**membership_status** | Option<[**models::GroupMemberStatus**](GroupMemberStatus.md)> |  | [optional]
**role_ids** | Option<**Vec<String>**> |  | [optional]
**user_id** | Option<**String**> | A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed. | [optional]
**visibility** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


