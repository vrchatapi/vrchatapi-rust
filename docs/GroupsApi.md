# \GroupsApi

All URIs are relative to *https://api.vrchat.cloud/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_gallery_image**](GroupsApi.md#add_group_gallery_image) | **POST** /groups/{groupId}/galleries/{groupGalleryId}/images | Add Group Gallery Image
[**add_group_member_role**](GroupsApi.md#add_group_member_role) | **PUT** /groups/{groupId}/members/{userId}/roles/{groupRoleId} | Add Role to GroupMember
[**add_group_post**](GroupsApi.md#add_group_post) | **POST** /groups/{groupId}/posts | Create a post in a Group
[**ban_group_member**](GroupsApi.md#ban_group_member) | **POST** /groups/{groupId}/bans | Ban Group Member
[**block_group**](GroupsApi.md#block_group) | **POST** /groups/{groupId}/block | Block Group
[**cancel_group_request**](GroupsApi.md#cancel_group_request) | **DELETE** /groups/{groupId}/requests | Cancel Group Join Request
[**cancel_group_transfer**](GroupsApi.md#cancel_group_transfer) | **DELETE** /groups/{groupId}/transfer | Cancel Group Transfer
[**create_group**](GroupsApi.md#create_group) | **POST** /groups | Create Group
[**create_group_announcement**](GroupsApi.md#create_group_announcement) | **POST** /groups/{groupId}/announcement | Create Group Announcement
[**create_group_gallery**](GroupsApi.md#create_group_gallery) | **POST** /groups/{groupId}/galleries | Create Group Gallery
[**create_group_invite**](GroupsApi.md#create_group_invite) | **POST** /groups/{groupId}/invites | Invite User to Group
[**create_group_role**](GroupsApi.md#create_group_role) | **POST** /groups/{groupId}/roles | Create GroupRole
[**decline_group_invite**](GroupsApi.md#decline_group_invite) | **PUT** /groups/{groupId}/invites | Decline Invite from Group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /groups/{groupId} | Delete Group
[**delete_group_announcement**](GroupsApi.md#delete_group_announcement) | **DELETE** /groups/{groupId}/announcement | Delete Group Announcement
[**delete_group_gallery**](GroupsApi.md#delete_group_gallery) | **DELETE** /groups/{groupId}/galleries/{groupGalleryId} | Delete Group Gallery
[**delete_group_gallery_image**](GroupsApi.md#delete_group_gallery_image) | **DELETE** /groups/{groupId}/galleries/{groupGalleryId}/images/{groupGalleryImageId} | Delete Group Gallery Image
[**delete_group_invite**](GroupsApi.md#delete_group_invite) | **DELETE** /groups/{groupId}/invites/{userId} | Delete User Invite
[**delete_group_post**](GroupsApi.md#delete_group_post) | **DELETE** /groups/{groupId}/posts/{notificationId} | Delete a Group post
[**delete_group_role**](GroupsApi.md#delete_group_role) | **DELETE** /groups/{groupId}/roles/{groupRoleId} | Delete Group Role
[**get_group**](GroupsApi.md#get_group) | **GET** /groups/{groupId} | Get Group by ID
[**get_group_announcements**](GroupsApi.md#get_group_announcements) | **GET** /groups/{groupId}/announcement | Get Group Announcement
[**get_group_audit_log_entry_types**](GroupsApi.md#get_group_audit_log_entry_types) | **GET** /groups/{groupId}/auditLogTypes | Get Group Audit Log Entry Types
[**get_group_audit_logs**](GroupsApi.md#get_group_audit_logs) | **GET** /groups/{groupId}/auditLogs | Get Group Audit Logs
[**get_group_bans**](GroupsApi.md#get_group_bans) | **GET** /groups/{groupId}/bans | Get Group Bans
[**get_group_gallery_images**](GroupsApi.md#get_group_gallery_images) | **GET** /groups/{groupId}/galleries/{groupGalleryId} | Get Group Gallery Images
[**get_group_instances**](GroupsApi.md#get_group_instances) | **GET** /groups/{groupId}/instances | Get Group Instances
[**get_group_invites**](GroupsApi.md#get_group_invites) | **GET** /groups/{groupId}/invites | Get Group Invites Sent
[**get_group_member**](GroupsApi.md#get_group_member) | **GET** /groups/{groupId}/members/{userId} | Get Group Member
[**get_group_members**](GroupsApi.md#get_group_members) | **GET** /groups/{groupId}/members | List Group Members
[**get_group_permissions**](GroupsApi.md#get_group_permissions) | **GET** /groups/{groupId}/permissions | List Group Permissions
[**get_group_posts**](GroupsApi.md#get_group_posts) | **GET** /groups/{groupId}/posts | Get posts from a Group
[**get_group_requests**](GroupsApi.md#get_group_requests) | **GET** /groups/{groupId}/requests | Get Group Join Requests
[**get_group_role_templates**](GroupsApi.md#get_group_role_templates) | **GET** /groups/roleTemplates | Get Group Role Templates
[**get_group_roles**](GroupsApi.md#get_group_roles) | **GET** /groups/{groupId}/roles | Get Group Roles
[**get_group_transferability**](GroupsApi.md#get_group_transferability) | **GET** /groups/{groupId}/transfer | Get Group Transferability
[**initiate_or_accept_group_transfer**](GroupsApi.md#initiate_or_accept_group_transfer) | **POST** /groups/{groupId}/transfer | Initiate or Accept Group Transfer
[**join_group**](GroupsApi.md#join_group) | **POST** /groups/{groupId}/join | Join Group
[**kick_group_member**](GroupsApi.md#kick_group_member) | **DELETE** /groups/{groupId}/members/{userId} | Kick Group Member
[**leave_group**](GroupsApi.md#leave_group) | **POST** /groups/{groupId}/leave | Leave Group
[**remove_group_member_role**](GroupsApi.md#remove_group_member_role) | **DELETE** /groups/{groupId}/members/{userId}/roles/{groupRoleId} | Remove Role from GroupMember
[**respond_group_join_request**](GroupsApi.md#respond_group_join_request) | **PUT** /groups/{groupId}/requests/{userId} | Respond Group Join request
[**search_group_members**](GroupsApi.md#search_group_members) | **GET** /groups/{groupId}/members/search | Search Group Members
[**search_groups**](GroupsApi.md#search_groups) | **GET** /groups | Search Group
[**unban_group_member**](GroupsApi.md#unban_group_member) | **DELETE** /groups/{groupId}/bans/{userId} | Unban Group Member
[**update_group**](GroupsApi.md#update_group) | **PUT** /groups/{groupId} | Update Group
[**update_group_gallery**](GroupsApi.md#update_group_gallery) | **PUT** /groups/{groupId}/galleries/{groupGalleryId} | Update Group Gallery
[**update_group_member**](GroupsApi.md#update_group_member) | **PUT** /groups/{groupId}/members/{userId} | Update Group Member
[**update_group_post**](GroupsApi.md#update_group_post) | **PUT** /groups/{groupId}/posts/{notificationId} | Edits a Group post
[**update_group_representation**](GroupsApi.md#update_group_representation) | **PUT** /groups/{groupId}/representation | Update Group Representation
[**update_group_role**](GroupsApi.md#update_group_role) | **PUT** /groups/{groupId}/roles/{groupRoleId} | Update Group Role



## add_group_gallery_image

> models::GroupGalleryImage add_group_gallery_image(group_id, group_gallery_id, add_group_gallery_image_request)
Add Group Gallery Image

Adds an image to a Group gallery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |
**add_group_gallery_image_request** | [**AddGroupGalleryImageRequest**](AddGroupGalleryImageRequest.md) |  | [required] |

### Return type

[**models::GroupGalleryImage**](GroupGalleryImage.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_group_member_role

> Vec<String> add_group_member_role(group_id, user_id, group_role_id)
Add Role to GroupMember

Adds a Role to a Group Member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |
**group_role_id** | **String** | Must be a valid group role ID. | [required] |

### Return type

**Vec<String>**

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_group_post

> models::GroupPost add_group_post(group_id, create_group_post_request)
Create a post in a Group

Create a post in a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_post_request** | [**CreateGroupPostRequest**](CreateGroupPostRequest.md) |  | [required] |

### Return type

[**models::GroupPost**](GroupPost.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_group_member

> models::GroupMember ban_group_member(group_id, ban_group_member_request)
Ban Group Member

Bans a user from a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**ban_group_member_request** | [**BanGroupMemberRequest**](BanGroupMemberRequest.md) |  | [required] |

### Return type

[**models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_group

> models::Success block_group(group_id)
Block Group

Blocks a Group for the current user. To unblock a group, call kickGroupMember (DELETE /groups/{groupId}/members/{userId}).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_group_request

> cancel_group_request(group_id)
Cancel Group Join Request

Cancels a request sent to join the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_group_transfer

> models::Success cancel_group_transfer(group_id)
Cancel Group Transfer

Cancel a Group Transfer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> models::Group create_group(create_group_request)
Create Group

Creates a Group and returns a Group object. **Requires VRC+ Subscription.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_request** | [**CreateGroupRequest**](CreateGroupRequest.md) |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_announcement

> models::GroupAnnouncement create_group_announcement(group_id, create_group_announcement_request)
Create Group Announcement

Creates an Announcement for a Group. Warning: This will also remove all announcements. To make proper announcements, use the posts endpoint instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_announcement_request** | [**CreateGroupAnnouncementRequest**](CreateGroupAnnouncementRequest.md) |  | [required] |

### Return type

[**models::GroupAnnouncement**](GroupAnnouncement.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_gallery

> models::GroupGallery create_group_gallery(group_id, create_group_gallery_request)
Create Group Gallery

Creates a gallery for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_gallery_request** | [**CreateGroupGalleryRequest**](CreateGroupGalleryRequest.md) |  | [required] |

### Return type

[**models::GroupGallery**](GroupGallery.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_invite

> create_group_invite(group_id, create_group_invite_request)
Invite User to Group

Sends an invite to a user to join the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_invite_request** | [**CreateGroupInviteRequest**](CreateGroupInviteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_role

> models::GroupRole create_group_role(group_id, create_group_role_request)
Create GroupRole

Create a Group role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_role_request** | [**CreateGroupRoleRequest**](CreateGroupRoleRequest.md) |  | [required] |

### Return type

[**models::GroupRole**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decline_group_invite

> models::Success decline_group_invite(group_id, decline_group_invite_request)
Decline Invite from Group

Declines an invite to the user from a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**decline_group_invite_request** | Option<[**DeclineGroupInviteRequest**](DeclineGroupInviteRequest.md)> |  |  |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> models::Success delete_group(group_id, hard_delete)
Delete Group

Deletes a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**hard_delete** | Option<**bool**> |  |  |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_announcement

> models::Success delete_group_announcement(group_id)
Delete Group Announcement

Deletes the announcement for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_gallery

> models::Success delete_group_gallery(group_id, group_gallery_id)
Delete Group Gallery

Deletes a gallery for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_gallery_image

> models::Success delete_group_gallery_image(group_id, group_gallery_id, group_gallery_image_id)
Delete Group Gallery Image

Deletes an image from a Group gallery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |
**group_gallery_image_id** | **String** | Must be a valid group gallery image ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_invite

> delete_group_invite(group_id, user_id)
Delete User Invite

Deletes an Group invite sent to a User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_post

> models::Success delete_group_post(group_id, notification_id)
Delete a Group post

Delete a Group post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_role

> Vec<models::GroupRole> delete_group_role(group_id, group_role_id)
Delete Group Role

Deletes a Group Role by ID and returns the remaining roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_role_id** | **String** | Must be a valid group role ID. | [required] |

### Return type

[**Vec<models::GroupRole>**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> models::Group get_group(group_id, include_roles)
Get Group by ID

Returns a single Group by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**include_roles** | Option<**bool**> | Include roles for the Group object. Defaults to false. |  |

### Return type

[**models::Group**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_announcements

> models::GroupAnnouncement get_group_announcements(group_id)
Get Group Announcement

Returns the announcement for a Group. If no announcement has been made, then it returns **empty object**. If an announcement exists, then it will always return all fields except `imageId` and `imageUrl` which may be null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**models::GroupAnnouncement**](GroupAnnouncement.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_audit_log_entry_types

> Vec<String> get_group_audit_log_entry_types(group_id)
Get Group Audit Log Entry Types

Returns a list of audit log entry types for which the group has entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

**Vec<String>**

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_audit_logs

> models::PaginatedGroupAuditLogEntryList get_group_audit_logs(group_id, n, offset, start_date, end_date, actor_ids, event_types, target_ids)
Get Group Audit Logs

Returns a list of audit logs for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**start_date** | Option<**String**> | The start date of the search range. |  |
**end_date** | Option<**String**> | The end date of the search range. |  |
**actor_ids** | Option<**String**> | The comma-separated actor ids to search for. |  |
**event_types** | Option<**String**> | The comma-separated event types to search for. |  |
**target_ids** | Option<**String**> | The comma-separated target ids to search for. |  |

### Return type

[**models::PaginatedGroupAuditLogEntryList**](PaginatedGroupAuditLogEntryList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_bans

> Vec<models::GroupMember> get_group_bans(group_id, n, offset)
Get Group Bans

Returns a list of banned users for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_gallery_images

> Vec<models::GroupGalleryImage> get_group_gallery_images(group_id, group_gallery_id, n, offset, approved)
Get Group Gallery Images

Returns a list of images for a Group gallery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**approved** | Option<**bool**> | If specified, only returns images that have been approved or not approved. |  |

### Return type

[**Vec<models::GroupGalleryImage>**](GroupGalleryImage.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_instances

> Vec<models::GroupInstance> get_group_instances(group_id)
Get Group Instances

Returns a list of group instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**Vec<models::GroupInstance>**](GroupInstance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_invites

> Vec<models::GroupMember> get_group_invites(group_id, n, offset)
Get Group Invites Sent

Returns a list of members that have been invited to the Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_member

> models::GroupMember get_group_member(group_id, user_id)
Get Group Member

Returns a GroupMember.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_members

> Vec<models::GroupMember> get_group_members(group_id, n, offset, sort, role_id)
List Group Members

Returns a List of all **other** Group Members. This endpoint will never return the user calling the endpoint. Information about the user calling the endpoint must be found in the `myMember` field of the Group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**sort** | Option<[**GroupSearchSort**](.md)> | The sort order of Group Member results |  |
**role_id** | Option<**String**> | Only returns members with a specific groupRoleId |  |

### Return type

[**Vec<models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_permissions

> Vec<models::GroupPermission> get_group_permissions(group_id)
List Group Permissions

Returns a List of all possible/available permissions for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**Vec<models::GroupPermission>**](GroupPermission.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_posts

> models::GetGroupPosts200Response get_group_posts(group_id, n, offset, public_only)
Get posts from a Group

Get posts from a Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**public_only** | Option<**bool**> | See public posts only. |  |

### Return type

[**models::GetGroupPosts200Response**](getGroupPosts_200_response.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_requests

> Vec<models::GroupMember> get_group_requests(group_id, n, offset, blocked)
Get Group Join Requests

Returns a list of members that have requested to join the Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**blocked** | Option<**bool**> | See blocked join requests |  |

### Return type

[**Vec<models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_role_templates

> std::collections::HashMap<String, models::GroupRoleTemplateValues> get_group_role_templates()
Get Group Role Templates

Obtain predefined templates for group roles

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, models::GroupRoleTemplateValues>**](GroupRoleTemplateValues.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_roles

> Vec<models::GroupRole> get_group_roles(group_id)
Get Group Roles

Returns a Group Role by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**Vec<models::GroupRole>**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_transferability

> models::GroupTransferable get_group_transferability(group_id, transfer_target_id)
Get Group Transferability

Returns the transferability of the group to a given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**transfer_target_id** | Option<**String**> | The UserID of the prospective transferee. |  |

### Return type

[**models::GroupTransferable**](GroupTransferable.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_or_accept_group_transfer

> models::Success initiate_or_accept_group_transfer(group_id, transfer_group_request)
Initiate or Accept Group Transfer

To initiate, must be logged in as the current owner and specify the transferTargetId in the body. To accept, must be logged in as the user targetted by a pending transfer, no body is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**transfer_group_request** | Option<[**TransferGroupRequest**](TransferGroupRequest.md)> |  |  |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_group

> models::GroupMember join_group(group_id, confirm_override_block, join_group_request)
Join Group

Join a Group by ID and returns the member object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**confirm_override_block** | Option<**bool**> | Manually override the failure that would occur if the user has blocked the group. |  |
**join_group_request** | Option<[**JoinGroupRequest**](JoinGroupRequest.md)> |  |  |

### Return type

[**models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kick_group_member

> models::Success kick_group_member(group_id, user_id)
Kick Group Member

Kicks a Group Member from the Group. The current user must have the \"Remove Group Members\" permission. Also used for unblocking groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_group

> leave_group(group_id)
Leave Group

Leave a group by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_member_role

> Vec<String> remove_group_member_role(group_id, user_id, group_role_id)
Remove Role from GroupMember

Removes a Role from a Group Member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |
**group_role_id** | **String** | Must be a valid group role ID. | [required] |

### Return type

**Vec<String>**

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## respond_group_join_request

> respond_group_join_request(group_id, user_id, respond_group_join_request)
Respond Group Join request

Responds to a Group Join Request with Accept/Deny

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |
**respond_group_join_request** | [**RespondGroupJoinRequest**](RespondGroupJoinRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_group_members

> models::SearchGroupMembers200Response search_group_members(group_id, query, n, offset)
Search Group Members

Search for members in the group by displayName.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**query** | **String** | Filter for member displayName. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**models::SearchGroupMembers200Response**](searchGroupMembers_200_response.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_groups

> Vec<models::LimitedGroup> search_groups(query, offset, n)
Search Group

Searches Groups by name or shortCode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Query to search for, can be either Group Name or Group shortCode |  |
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]

### Return type

[**Vec<models::LimitedGroup>**](LimitedGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unban_group_member

> models::GroupMember unban_group_member(group_id, user_id)
Unban Group Member

Unbans a user from a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> models::Group update_group(group_id, update_group_request)
Update Group

Updates a Group and returns it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**update_group_request** | Option<[**UpdateGroupRequest**](UpdateGroupRequest.md)> |  |  |

### Return type

[**models::Group**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_gallery

> models::GroupGallery update_group_gallery(group_id, group_gallery_id, update_group_gallery_request)
Update Group Gallery

Updates a gallery for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |
**update_group_gallery_request** | Option<[**UpdateGroupGalleryRequest**](UpdateGroupGalleryRequest.md)> |  |  |

### Return type

[**models::GroupGallery**](GroupGallery.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_member

> models::GroupMember update_group_member(group_id, user_id, update_group_member_request)
Update Group Member

Updates a Group Member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |
**update_group_member_request** | Option<[**UpdateGroupMemberRequest**](UpdateGroupMemberRequest.md)> |  |  |

### Return type

[**models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_post

> models::GroupPost update_group_post(group_id, notification_id, create_group_post_request)
Edits a Group post

Edits a Group post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**notification_id** | **String** | Must be a valid notification ID. | [required] |
**create_group_post_request** | [**CreateGroupPostRequest**](CreateGroupPostRequest.md) |  | [required] |

### Return type

[**models::GroupPost**](GroupPost.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_representation

> models::Success update_group_representation(group_id, update_group_representation_request)
Update Group Representation

Updates whether the user is representing the group.  When `isRepresenting` is set to `true`, this flag will be set to `false` for all other groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**update_group_representation_request** | [**UpdateGroupRepresentationRequest**](UpdateGroupRepresentationRequest.md) |  | [required] |

### Return type

[**models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_role

> Vec<models::GroupRole> update_group_role(group_id, group_role_id, update_group_role_request)
Update Group Role

Updates a group role by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_role_id** | **String** | Must be a valid group role ID. | [required] |
**update_group_role_request** | Option<[**UpdateGroupRoleRequest**](UpdateGroupRoleRequest.md)> |  |  |

### Return type

[**Vec<models::GroupRole>**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

