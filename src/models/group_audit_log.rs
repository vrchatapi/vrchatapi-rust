use crate::models;
use serde::{Deserialize, Serialize};

/// A group audit log entry. The shape of `data` depends on `eventType`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "eventType")]
pub enum GroupAuditLog {
    #[serde(rename = "group.announcement")]
    GroupAnnouncement(models::GroupAuditLogGroupAnnouncement),
    #[serde(rename = "group.calendarEvent.create")]
    GroupCalendarEventCreate(models::GroupAuditLogGroupCalendarEventCreate),
    #[serde(rename = "group.calendarEvent.delete")]
    GroupCalendarEventDelete(models::GroupAuditLogGroupCalendarEventDelete),
    #[serde(rename = "group.create")]
    GroupCreate(models::GroupAuditLogGroupCreate),
    #[serde(rename = "group.gallery.create")]
    GroupGalleryCreate(models::GroupAuditLogGroupGalleryCreate),
    #[serde(rename = "group.gallery.delete")]
    GroupGalleryDelete(models::GroupAuditLogGroupGalleryDelete),
    #[serde(rename = "group.gallery.update")]
    GroupGalleryUpdate(models::GroupAuditLogGroupGalleryUpdate),
    #[serde(rename = "group.instance.announcement")]
    GroupInstanceAnnouncement(models::GroupAuditLogGroupInstanceAnnouncement),
    #[serde(rename = "group.instance.close")]
    GroupInstanceClose(models::GroupAuditLogGroupInstanceClose),
    #[serde(rename = "group.instance.create")]
    GroupInstanceCreate(models::GroupAuditLogGroupInstanceCreate),
    #[serde(rename = "group.instance.kick")]
    GroupInstanceKick(models::GroupAuditLogGroupInstanceKick),
    #[serde(rename = "group.instance.warn")]
    GroupInstanceWarn(models::GroupAuditLogGroupInstanceWarn),
    #[serde(rename = "group.invite.cancel")]
    GroupInviteCancel(models::GroupAuditLogGroupInviteCancel),
    #[serde(rename = "group.invite.create")]
    GroupInviteCreate(models::GroupAuditLogGroupInviteCreate),
    #[serde(rename = "group.member.join")]
    GroupMemberJoin(models::GroupAuditLogGroupMemberJoin),
    #[serde(rename = "group.member.leave")]
    GroupMemberLeave(models::GroupAuditLogGroupMemberLeave),
    #[serde(rename = "group.member.remove")]
    GroupMemberRemove(models::GroupAuditLogGroupMemberRemove),
    #[serde(rename = "group.member.role.assign")]
    GroupMemberRoleAssign(models::GroupAuditLogGroupMemberRoleAssign),
    #[serde(rename = "group.member.role.unassign")]
    GroupMemberRoleUnassign(models::GroupAuditLogGroupMemberRoleUnassign),
    #[serde(rename = "group.member.user.update")]
    GroupMemberUserUpdate(models::GroupAuditLogGroupMemberUserUpdate),
    #[serde(rename = "group.post.create")]
    GroupPostCreate(models::GroupAuditLogGroupPostCreate),
    #[serde(rename = "group.post.delete")]
    GroupPostDelete(models::GroupAuditLogGroupPostDelete),
    #[serde(rename = "group.post.update")]
    GroupPostUpdate(models::GroupAuditLogGroupPostUpdate),
    #[serde(rename = "group.request.block")]
    GroupRequestBlock(models::GroupAuditLogGroupRequestBlock),
    #[serde(rename = "group.request.create")]
    GroupRequestCreate(models::GroupAuditLogGroupRequestCreate),
    #[serde(rename = "group.request.reject")]
    GroupRequestReject(models::GroupAuditLogGroupRequestReject),
    #[serde(rename = "group.request.withdraw")]
    GroupRequestWithdraw(models::GroupAuditLogGroupRequestWithdraw),
    #[serde(rename = "group.role.create")]
    GroupRoleCreate(models::GroupAuditLogGroupRoleCreate),
    #[serde(rename = "group.role.delete")]
    GroupRoleDelete(models::GroupAuditLogGroupRoleDelete),
    #[serde(rename = "group.role.update")]
    GroupRoleUpdate(models::GroupAuditLogGroupRoleUpdate),
    #[serde(rename = "group.update")]
    GroupUpdate(models::GroupAuditLogGroupUpdate),
    #[serde(rename = "group.user.ban")]
    GroupUserBan(models::GroupAuditLogGroupUserBan),
    #[serde(rename = "group.user.unban")]
    GroupUserUnban(models::GroupAuditLogGroupUserUnban),
    #[serde(untagged)]
    GroupAuditLogUnknown(models::GroupAuditLogUnknown),
}

impl Default for GroupAuditLog {
    fn default() -> Self {
        Self::GroupAnnouncement(Default::default())
    }
}

impl From<models::GroupAuditLogGroupAnnouncement> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupAnnouncement) -> Self {
        Self::GroupAnnouncement(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupAnnouncement {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupAnnouncement(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupCalendarEventCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupCalendarEventCreate) -> Self {
        Self::GroupCalendarEventCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupCalendarEventCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupCalendarEventCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupCalendarEventDelete> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupCalendarEventDelete) -> Self {
        Self::GroupCalendarEventDelete(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupCalendarEventDelete {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupCalendarEventDelete(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupCreate) -> Self {
        Self::GroupCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupGalleryCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupGalleryCreate) -> Self {
        Self::GroupGalleryCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupGalleryCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupGalleryCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupGalleryDelete> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupGalleryDelete) -> Self {
        Self::GroupGalleryDelete(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupGalleryDelete {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupGalleryDelete(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupGalleryUpdate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupGalleryUpdate) -> Self {
        Self::GroupGalleryUpdate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupGalleryUpdate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupGalleryUpdate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupInstanceAnnouncement> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupInstanceAnnouncement) -> Self {
        Self::GroupInstanceAnnouncement(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupInstanceAnnouncement {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupInstanceAnnouncement(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupInstanceClose> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupInstanceClose) -> Self {
        Self::GroupInstanceClose(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupInstanceClose {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupInstanceClose(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupInstanceCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupInstanceCreate) -> Self {
        Self::GroupInstanceCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupInstanceCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupInstanceCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupInstanceKick> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupInstanceKick) -> Self {
        Self::GroupInstanceKick(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupInstanceKick {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupInstanceKick(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupInstanceWarn> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupInstanceWarn) -> Self {
        Self::GroupInstanceWarn(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupInstanceWarn {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupInstanceWarn(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupInviteCancel> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupInviteCancel) -> Self {
        Self::GroupInviteCancel(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupInviteCancel {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupInviteCancel(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupInviteCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupInviteCreate) -> Self {
        Self::GroupInviteCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupInviteCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupInviteCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupMemberJoin> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupMemberJoin) -> Self {
        Self::GroupMemberJoin(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupMemberJoin {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupMemberJoin(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupMemberLeave> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupMemberLeave) -> Self {
        Self::GroupMemberLeave(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupMemberLeave {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupMemberLeave(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupMemberRemove> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupMemberRemove) -> Self {
        Self::GroupMemberRemove(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupMemberRemove {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupMemberRemove(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupMemberRoleAssign> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupMemberRoleAssign) -> Self {
        Self::GroupMemberRoleAssign(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupMemberRoleAssign {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupMemberRoleAssign(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupMemberRoleUnassign> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupMemberRoleUnassign) -> Self {
        Self::GroupMemberRoleUnassign(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupMemberRoleUnassign {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupMemberRoleUnassign(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupMemberUserUpdate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupMemberUserUpdate) -> Self {
        Self::GroupMemberUserUpdate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupMemberUserUpdate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupMemberUserUpdate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupPostCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupPostCreate) -> Self {
        Self::GroupPostCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupPostCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupPostCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupPostDelete> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupPostDelete) -> Self {
        Self::GroupPostDelete(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupPostDelete {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupPostDelete(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupPostUpdate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupPostUpdate) -> Self {
        Self::GroupPostUpdate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupPostUpdate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupPostUpdate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupRequestBlock> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupRequestBlock) -> Self {
        Self::GroupRequestBlock(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupRequestBlock {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupRequestBlock(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupRequestCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupRequestCreate) -> Self {
        Self::GroupRequestCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupRequestCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupRequestCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupRequestReject> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupRequestReject) -> Self {
        Self::GroupRequestReject(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupRequestReject {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupRequestReject(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupRequestWithdraw> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupRequestWithdraw) -> Self {
        Self::GroupRequestWithdraw(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupRequestWithdraw {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupRequestWithdraw(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupRoleCreate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupRoleCreate) -> Self {
        Self::GroupRoleCreate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupRoleCreate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupRoleCreate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupRoleDelete> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupRoleDelete) -> Self {
        Self::GroupRoleDelete(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupRoleDelete {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupRoleDelete(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupRoleUpdate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupRoleUpdate) -> Self {
        Self::GroupRoleUpdate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupRoleUpdate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupRoleUpdate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupUpdate> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupUpdate) -> Self {
        Self::GroupUpdate(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupUpdate {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupUpdate(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupUserBan> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupUserBan) -> Self {
        Self::GroupUserBan(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupUserBan {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupUserBan(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogGroupUserUnban> for GroupAuditLog {
    fn from(value: models::GroupAuditLogGroupUserUnban) -> Self {
        Self::GroupUserUnban(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogGroupUserUnban {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupUserUnban(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::GroupAuditLogUnknown> for GroupAuditLog {
    fn from(value: models::GroupAuditLogUnknown) -> Self {
        Self::GroupAuditLogUnknown(value)
    }
}

impl TryFrom<GroupAuditLog> for models::GroupAuditLogUnknown {
    type Error = GroupAuditLog;

    fn try_from(value: GroupAuditLog) -> Result<Self, Self::Error> {
        match value {
            GroupAuditLog::GroupAuditLogUnknown(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
