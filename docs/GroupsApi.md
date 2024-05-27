# \GroupsApi

All URIs are relative to *https://vrchat.com/api/1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_gallery_image**](GroupsApi.md#add_group_gallery_image) | **POST** /groups/{groupId}/galleries/{groupGalleryId}/images | Add Group Gallery Image
[**add_group_member_role**](GroupsApi.md#add_group_member_role) | **PUT** /groups/{groupId}/members/{userId}/roles/{groupRoleId} | Add Role to GroupMember
[**add_group_post**](GroupsApi.md#add_group_post) | **POST** /groups/{groupId}/posts | Create a post in a Group
[**ban_group_member**](GroupsApi.md#ban_group_member) | **POST** /groups/{groupId}/bans | Ban Group Member
[**cancel_group_request**](GroupsApi.md#cancel_group_request) | **DELETE** /groups/{groupId}/requests | Cancel Group Join Request
[**create_group**](GroupsApi.md#create_group) | **POST** /groups | Create Group
[**create_group_announcement**](GroupsApi.md#create_group_announcement) | **POST** /groups/{groupId}/announcement | Create Group Announcement
[**create_group_gallery**](GroupsApi.md#create_group_gallery) | **POST** /groups/{groupId}/galleries | Create Group Gallery
[**create_group_invite**](GroupsApi.md#create_group_invite) | **POST** /groups/{groupId}/invites | Invite User to Group
[**create_group_role**](GroupsApi.md#create_group_role) | **POST** /groups/{groupId}/roles | Create GroupRole
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /groups/{groupId} | Delete Group
[**delete_group_announcement**](GroupsApi.md#delete_group_announcement) | **DELETE** /groups/{groupId}/announcement | Delete Group Announcement
[**delete_group_gallery**](GroupsApi.md#delete_group_gallery) | **DELETE** /groups/{groupId}/galleries/{groupGalleryId} | Delete Group Gallery
[**delete_group_gallery_image**](GroupsApi.md#delete_group_gallery_image) | **DELETE** /groups/{groupId}/galleries/{groupGalleryId}/images/{groupGalleryImageId} | Delete Group Gallery Image
[**delete_group_invite**](GroupsApi.md#delete_group_invite) | **DELETE** /groups/{groupId}/invites/{userId} | Delete User Invite
[**delete_group_post**](GroupsApi.md#delete_group_post) | **DELETE** /groups/{groupId}/posts/{notificationId} | Delete a Group post
[**delete_group_role**](GroupsApi.md#delete_group_role) | **DELETE** /groups/{groupId}/roles/{groupRoleId} | Delete Group Role
[**get_group**](GroupsApi.md#get_group) | **GET** /groups/{groupId} | Get Group by ID
[**get_group_announcements**](GroupsApi.md#get_group_announcements) | **GET** /groups/{groupId}/announcement | Get Group Announcement
[**get_group_audit_logs**](GroupsApi.md#get_group_audit_logs) | **GET** /groups/{groupId}/auditLogs | Get Group Audit Logs
[**get_group_bans**](GroupsApi.md#get_group_bans) | **GET** /groups/{groupId}/bans | Get Group Bans
[**get_group_gallery_images**](GroupsApi.md#get_group_gallery_images) | **GET** /groups/{groupId}/galleries/{groupGalleryId} | Get Group Gallery Images
[**get_group_instances**](GroupsApi.md#get_group_instances) | **GET** /groups/{groupId}/instances | Get Group Instances
[**get_group_invites**](GroupsApi.md#get_group_invites) | **GET** /groups/{groupId}/invites | Get Group Invites Sent
[**get_group_member**](GroupsApi.md#get_group_member) | **GET** /groups/{groupId}/members/{userId} | Get Group Member
[**get_group_members**](GroupsApi.md#get_group_members) | **GET** /groups/{groupId}/members | List Group Members
[**get_group_permissions**](GroupsApi.md#get_group_permissions) | **GET** /groups/{groupId}/permissions | List Group Permissions
[**get_group_post**](GroupsApi.md#get_group_post) | **GET** /groups/{groupId}/posts | Get posts from a Group
[**get_group_requests**](GroupsApi.md#get_group_requests) | **GET** /groups/{groupId}/requests | Get Group Join Requests
[**get_group_roles**](GroupsApi.md#get_group_roles) | **GET** /groups/{groupId}/roles | Get Group Roles
[**join_group**](GroupsApi.md#join_group) | **POST** /groups/{groupId}/join | Join Group
[**kick_group_member**](GroupsApi.md#kick_group_member) | **DELETE** /groups/{groupId}/members/{userId} | Kick Group Member
[**leave_group**](GroupsApi.md#leave_group) | **POST** /groups/{groupId}/leave | Leave Group
[**remove_group_member_role**](GroupsApi.md#remove_group_member_role) | **DELETE** /groups/{groupId}/members/{userId}/roles/{groupRoleId} | Remove Role from GroupMember
[**respond_group_join_request**](GroupsApi.md#respond_group_join_request) | **PUT** /groups/{groupId}/requests/{userId} | Respond Group Join request
[**search_groups**](GroupsApi.md#search_groups) | **GET** /groups | Search Group
[**unban_group_member**](GroupsApi.md#unban_group_member) | **DELETE** /groups/{groupId}/bans/{userId} | Unban Group Member
[**update_group**](GroupsApi.md#update_group) | **PUT** /groups/{groupId} | Update Group
[**update_group_gallery**](GroupsApi.md#update_group_gallery) | **PUT** /groups/{groupId}/galleries/{groupGalleryId} | Update Group Gallery
[**update_group_member**](GroupsApi.md#update_group_member) | **PUT** /groups/{groupId}/members/{userId} | Update Group Member
[**update_group_post**](GroupsApi.md#update_group_post) | **PUT** /groups/{groupId}/posts/{notificationId} | Edits a Group post
[**update_group_role**](GroupsApi.md#update_group_role) | **PUT** /groups/{groupId}/roles/{groupRoleId} | Update Group Role



## add_group_gallery_image

> crate::models::GroupGalleryImage add_group_gallery_image(group_id, group_gallery_id, add_group_gallery_image_request)
Add Group Gallery Image

Adds an image to a Group gallery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |
**add_group_gallery_image_request** | [**AddGroupGalleryImageRequest**](AddGroupGalleryImageRequest.md) |  | [required] |

### Return type

[**crate::models::GroupGalleryImage**](GroupGalleryImage.md)

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

> crate::models::GroupPost add_group_post(group_id, create_group_post_request)
Create a post in a Group

Create a post in a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_post_request** | [**CreateGroupPostRequest**](CreateGroupPostRequest.md) |  | [required] |

### Return type

[**crate::models::GroupPost**](GroupPost.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_group_member

> crate::models::GroupMember ban_group_member(group_id, ban_group_member_request)
Ban Group Member

Bans a user from a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**ban_group_member_request** | [**BanGroupMemberRequest**](BanGroupMemberRequest.md) |  | [required] |

### Return type

[**crate::models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
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


## create_group

> crate::models::Group create_group(create_group_request)
Create Group

Creates a Group and returns a Group object. **Requires VRC+ Subscription.**

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_request** | [**CreateGroupRequest**](CreateGroupRequest.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_announcement

> crate::models::GroupAnnouncement create_group_announcement(group_id, create_group_announcement_request)
Create Group Announcement

Creates an Announcement for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_announcement_request** | [**CreateGroupAnnouncementRequest**](CreateGroupAnnouncementRequest.md) |  | [required] |

### Return type

[**crate::models::GroupAnnouncement**](GroupAnnouncement.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_gallery

> crate::models::GroupGallery create_group_gallery(group_id, create_group_gallery_request)
Create Group Gallery

Creates a gallery for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_gallery_request** | [**CreateGroupGalleryRequest**](CreateGroupGalleryRequest.md) |  | [required] |

### Return type

[**crate::models::GroupGallery**](GroupGallery.md)

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

> crate::models::GroupRole create_group_role(group_id, create_group_role_request)
Create GroupRole

Create a Group role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**create_group_role_request** | [**CreateGroupRoleRequest**](CreateGroupRoleRequest.md) |  | [required] |

### Return type

[**crate::models::GroupRole**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> crate::models::Success delete_group(group_id)
Delete Group

Deletes a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_announcement

> crate::models::Success delete_group_announcement(group_id)
Delete Group Announcement

Deletes the announcement for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_gallery

> crate::models::Success delete_group_gallery(group_id, group_gallery_id)
Delete Group Gallery

Deletes a gallery for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_gallery_image

> crate::models::Success delete_group_gallery_image(group_id, group_gallery_id, group_gallery_image_id)
Delete Group Gallery Image

Deletes an image from a Group gallery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |
**group_gallery_image_id** | **String** | Must be a valid group gallery image ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

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

> crate::models::Success delete_group_post(group_id, notification_id)
Delete a Group post

Delete a Group post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**notification_id** | **String** | Must be a valid notification ID. | [required] |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_role

> Vec<crate::models::GroupRole> delete_group_role(group_id, group_role_id)
Delete Group Role

Deletes a Group Role by ID and returns the remaining roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_role_id** | **String** | Must be a valid group role ID. | [required] |

### Return type

[**Vec<crate::models::GroupRole>**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> crate::models::Group get_group(group_id, include_roles)
Get Group by ID

Returns a single Group by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**include_roles** | Option<**bool**> | Include roles for the Group object. Defaults to false. |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_announcements

> crate::models::GroupAnnouncement get_group_announcements(group_id)
Get Group Announcement

Returns the announcement for a Group. If no announcement has been made, then it returns **empty object**.  If an announcement exists, then it will always return all fields except `imageId` and `imageUrl` which may be null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**crate::models::GroupAnnouncement**](GroupAnnouncement.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_audit_logs

> crate::models::PaginatedGroupAuditLogEntryList get_group_audit_logs(group_id, n, offset, start_date, end_date)
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

### Return type

[**crate::models::PaginatedGroupAuditLogEntryList**](PaginatedGroupAuditLogEntryList.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_bans

> Vec<crate::models::GroupMember> get_group_bans(group_id, n, offset)
Get Group Bans

Returns a list of banned users for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<crate::models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_gallery_images

> Vec<crate::models::GroupGalleryImage> get_group_gallery_images(group_id, group_gallery_id, n, offset, approved)
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

[**Vec<crate::models::GroupGalleryImage>**](GroupGalleryImage.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_instances

> Vec<crate::models::GroupInstance> get_group_instances(group_id)
Get Group Instances

Returns a list of group instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**Vec<crate::models::GroupInstance>**](GroupInstance.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_invites

> Vec<crate::models::GroupMember> get_group_invites(group_id, n, offset)
Get Group Invites Sent

Returns a list of members that have been invited to the Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |

### Return type

[**Vec<crate::models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_member

> crate::models::GroupLimitedMember get_group_member(group_id, user_id)
Get Group Member

Returns a LimitedGroup Member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::GroupLimitedMember**](GroupLimitedMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_members

> Vec<crate::models::GroupMember> get_group_members(group_id, n, offset, sort)
List Group Members

Returns a List of all **other** Group Members. This endpoint will never return the user calling the endpoint. Information about the user calling the endpoint must be found in the `myMember` field of the Group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**sort** | Option<[**GroupSearchSort**](.md)> | The sort order of Group Member results |  |

### Return type

[**Vec<crate::models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_permissions

> Vec<crate::models::GroupPermission> get_group_permissions(group_id)
List Group Permissions

Returns a List of all possible/available permissions for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**Vec<crate::models::GroupPermission>**](GroupPermission.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_post

> crate::models::GroupPost get_group_post(group_id, n, offset, public_only)
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

[**crate::models::GroupPost**](GroupPost.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_requests

> Vec<crate::models::GroupMember> get_group_requests(group_id, n, offset, blocked)
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

[**Vec<crate::models::GroupMember>**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_roles

> Vec<crate::models::GroupRole> get_group_roles(group_id)
Get Group Roles

Returns a Group Role by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**Vec<crate::models::GroupRole>**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_group

> crate::models::GroupMember join_group(group_id)
Join Group

Join a Group by ID and returns the member object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |

### Return type

[**crate::models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kick_group_member

> kick_group_member(group_id, user_id)
Kick Group Member

Kicks a Group Member from the Group. The current user must have the \"Remove Group Members\" permission.

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


## search_groups

> Vec<crate::models::LimitedGroup> search_groups(query, offset, n)
Search Group

Searches Groups by name or shortCode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Query to search for, can be either Group Name or Group shortCode |  |
**offset** | Option<**i32**> | A zero-based offset from the default object sorting from where search results start. |  |
**n** | Option<**i32**> | The number of objects to return. |  |[default to 60]

### Return type

[**Vec<crate::models::LimitedGroup>**](LimitedGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unban_group_member

> crate::models::GroupMember unban_group_member(group_id, user_id)
Unban Group Member

Unbans a user from a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |

### Return type

[**crate::models::GroupMember**](GroupMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> crate::models::Group update_group(group_id, update_group_request)
Update Group

Updates a Group and returns it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**update_group_request** | Option<[**UpdateGroupRequest**](UpdateGroupRequest.md)> |  |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_gallery

> crate::models::GroupGallery update_group_gallery(group_id, group_gallery_id, update_group_gallery_request)
Update Group Gallery

Updates a gallery for a Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_gallery_id** | **String** | Must be a valid group gallery ID. | [required] |
**update_group_gallery_request** | Option<[**UpdateGroupGalleryRequest**](UpdateGroupGalleryRequest.md)> |  |  |

### Return type

[**crate::models::GroupGallery**](GroupGallery.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_member

> crate::models::GroupLimitedMember update_group_member(group_id, user_id, update_group_member_request)
Update Group Member

Updates a Group Member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**user_id** | **String** | Must be a valid user ID. | [required] |
**update_group_member_request** | Option<[**UpdateGroupMemberRequest**](UpdateGroupMemberRequest.md)> |  |  |

### Return type

[**crate::models::GroupLimitedMember**](GroupLimitedMember.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_post

> crate::models::GroupPost update_group_post(group_id, notification_id, create_group_post_request)
Edits a Group post

Edits a Group post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**notification_id** | **String** | Must be a valid notification ID. | [required] |
**create_group_post_request** | [**CreateGroupPostRequest**](CreateGroupPostRequest.md) |  | [required] |

### Return type

[**crate::models::GroupPost**](GroupPost.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_role

> Vec<crate::models::GroupRole> update_group_role(group_id, group_role_id, update_group_role_request)
Update Group Role

Updates a group role by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Must be a valid group ID. | [required] |
**group_role_id** | **String** | Must be a valid group role ID. | [required] |
**update_group_role_request** | Option<[**UpdateGroupRoleRequest**](UpdateGroupRoleRequest.md)> |  |  |

### Return type

[**Vec<crate::models::GroupRole>**](GroupRole.md)

### Authorization

[authCookie](../README.md#authCookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

