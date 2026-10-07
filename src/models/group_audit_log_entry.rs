use crate::models;
use serde::{Deserialize, Serialize};

/// GroupAuditLogEntry : A group audit log entry. The shape of `data` depends on `eventType`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "eventType")]
pub enum GroupAuditLogEntry {
    #[serde(rename = "group.announcement")]
    GroupAnnouncement(models::GroupAuditLogEntryGroupAnnouncement),
    #[serde(rename = "group.calendarEvent.create")]
    GroupCalendarEventCreate(models::GroupAuditLogEntryGroupCalendarEventCreate),
    #[serde(rename = "group.calendarEvent.delete")]
    GroupCalendarEventDelete(models::GroupAuditLogEntryGroupCalendarEventDelete),
    #[serde(rename = "group.gallery.create")]
    GroupGalleryCreate(models::GroupAuditLogEntryGroupGalleryCreate),
    #[serde(rename = "group.gallery.delete")]
    GroupGalleryDelete(models::GroupAuditLogEntryGroupGalleryDelete),
    #[serde(rename = "group.gallery.update")]
    GroupGalleryUpdate(models::GroupAuditLogEntryGroupGalleryUpdate),
    #[serde(rename = "group.instance.announcement")]
    GroupInstanceAnnouncement(models::GroupAuditLogEntryGroupInstanceAnnouncement),
    #[serde(rename = "group.instance.close")]
    GroupInstanceClose(models::GroupAuditLogEntryGroupInstanceClose),
    #[serde(rename = "group.instance.create")]
    GroupInstanceCreate(models::GroupAuditLogEntryGroupInstanceCreate),
    #[serde(rename = "group.instance.kick")]
    GroupInstanceKick(models::GroupAuditLogEntryGroupInstanceKick),
    #[serde(rename = "group.instance.warn")]
    GroupInstanceWarn(models::GroupAuditLogEntryGroupInstanceWarn),
    #[serde(rename = "group.invite.cancel")]
    GroupInviteCancel(models::GroupAuditLogEntryGroupInviteCancel),
    #[serde(rename = "group.invite.create")]
    GroupInviteCreate(models::GroupAuditLogEntryGroupInviteCreate),
    #[serde(rename = "group.member.join")]
    GroupMemberJoin(models::GroupAuditLogEntryGroupMemberJoin),
    #[serde(rename = "group.member.leave")]
    GroupMemberLeave(models::GroupAuditLogEntryGroupMemberLeave),
    #[serde(rename = "group.member.remove")]
    GroupMemberRemove(models::GroupAuditLogEntryGroupMemberRemove),
    #[serde(rename = "group.member.role.assign")]
    GroupMemberRoleAssign(models::GroupAuditLogEntryGroupMemberRoleAssign),
    #[serde(rename = "group.member.role.unassign")]
    GroupMemberRoleUnassign(models::GroupAuditLogEntryGroupMemberRoleUnassign),
    #[serde(rename = "group.member.user.update")]
    GroupMemberUserUpdate(models::GroupAuditLogEntryGroupMemberUserUpdate),
    #[serde(rename = "group.post.create")]
    GroupPostCreate(models::GroupAuditLogEntryGroupPostCreate),
    #[serde(rename = "group.post.delete")]
    GroupPostDelete(models::GroupAuditLogEntryGroupPostDelete),
    #[serde(rename = "group.post.update")]
    GroupPostUpdate(models::GroupAuditLogEntryGroupPostUpdate),
    #[serde(rename = "group.request.block")]
    GroupRequestBlock(models::GroupAuditLogEntryGroupRequestBlock),
    #[serde(rename = "group.request.create")]
    GroupRequestCreate(models::GroupAuditLogEntryGroupRequestCreate),
    #[serde(rename = "group.request.reject")]
    GroupRequestReject(models::GroupAuditLogEntryGroupRequestReject),
    #[serde(rename = "group.request.withdraw")]
    GroupRequestWithdraw(models::GroupAuditLogEntryGroupRequestWithdraw),
    #[serde(rename = "group.role.create")]
    GroupRoleCreate(models::GroupAuditLogEntryGroupRoleCreate),
    #[serde(rename = "group.role.delete")]
    GroupRoleDelete(models::GroupAuditLogEntryGroupRoleDelete),
    #[serde(rename = "group.role.update")]
    GroupRoleUpdate(models::GroupAuditLogEntryGroupRoleUpdate),
    #[serde(rename = "group.update")]
    GroupUpdate(models::GroupAuditLogEntryGroupUpdate),
    #[serde(rename = "group.user.ban")]
    GroupUserBan(models::GroupAuditLogEntryGroupUserBan),
    #[serde(rename = "group.user.unban")]
    GroupUserUnban(models::GroupAuditLogEntryGroupUserUnban),
    #[serde(rename = "GroupAuditLogEntryUnknown")]
    GroupAuditLogEntryUnknown(models::GroupAuditLogEntryUnknown),
}

impl Default for GroupAuditLogEntry {
    fn default() -> Self {
        Self::GroupAnnouncement(Default::default())
    }
}
