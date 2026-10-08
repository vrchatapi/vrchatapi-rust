use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupCalendarEventDelete {
    #[serde(flatten)]
    pub group_audit_log_entry_data_group_calendar_event_create:
        models::GroupAuditLogEntryDataGroupCalendarEventCreate,
    /// The category of the event.
    #[serde(rename = "category")]
    pub category: String,
    /// Minutes after the event ends to close the instance.
    #[serde(rename = "closeInstanceAfterEndMinutes")]
    pub close_instance_after_end_minutes: i32,
    /// The creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    /// The deletion timestamp.
    #[serde(rename = "deletedAt", deserialize_with = "Option::deserialize")]
    pub deleted_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The duration of the event in milliseconds.
    #[serde(rename = "durationInMs")]
    pub duration_in_ms: i32,
    /// The end timestamp.
    #[serde(rename = "endsAt")]
    pub ends_at: chrono::DateTime<chrono::FixedOffset>,
    /// Whether the event is featured.
    #[serde(rename = "featured")]
    pub featured: bool,
    /// Minutes before the start that guests can join.
    #[serde(rename = "guestEarlyJoinMinutes")]
    pub guest_early_join_minutes: i32,
    /// Minutes before the start that hosts can join.
    #[serde(rename = "hostEarlyJoinMinutes")]
    pub host_early_join_minutes: i32,
    /// The number of interested users.
    #[serde(rename = "interestedUserCount")]
    pub interested_user_count: i32,
    /// Whether the event is a draft.
    #[serde(rename = "isDraft")]
    pub is_draft: bool,
    /// The languages for the event.
    #[serde(rename = "languages")]
    pub languages: Vec<String>,
    #[serde(rename = "occurrenceKind")]
    pub occurrence_kind: models::CalendarEventOccurrenceKind,
    #[serde(
        rename = "occurrenceModified",
        deserialize_with = "Option::deserialize"
    )]
    pub occurrence_modified: Option<String>,
    /// The ID of the group that owns the event.
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    /// The supported platforms.
    #[serde(rename = "platforms")]
    pub platforms: Vec<String>,
    /// The recurrence rule.
    #[serde(rename = "recurrence", deserialize_with = "Option::deserialize")]
    pub recurrence: Option<models::CalendarEventRecurrence>,
    /// Group roles that may join this event.
    #[serde(rename = "roleIds", deserialize_with = "Option::deserialize")]
    pub role_ids: Option<Vec<String>>,
    /// The ID of the recurring series the event belongs to.
    #[serde(rename = "seriesId", deserialize_with = "Option::deserialize")]
    pub series_id: Option<String>,
    /// The short code.
    #[serde(rename = "shortCode", deserialize_with = "Option::deserialize")]
    pub short_code: Option<String>,
    /// The start timestamp.
    #[serde(rename = "startsAt")]
    pub starts_at: chrono::DateTime<chrono::FixedOffset>,
    /// The event tags.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The last update timestamp.
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    /// Whether the event uses instance overflow.
    #[serde(rename = "usesInstanceOverflow")]
    pub uses_instance_overflow: bool,
}

impl GroupAuditLogEntryDataGroupCalendarEventDelete {
    pub fn new(
        group_audit_log_entry_data_group_calendar_event_create: models::GroupAuditLogEntryDataGroupCalendarEventCreate,
        category: String,
        close_instance_after_end_minutes: i32,
        created_at: chrono::DateTime<chrono::FixedOffset>,
        deleted_at: Option<chrono::DateTime<chrono::FixedOffset>>,
        duration_in_ms: i32,
        ends_at: chrono::DateTime<chrono::FixedOffset>,
        featured: bool,
        guest_early_join_minutes: i32,
        host_early_join_minutes: i32,
        interested_user_count: i32,
        is_draft: bool,
        languages: Vec<String>,
        occurrence_kind: models::CalendarEventOccurrenceKind,
        occurrence_modified: Option<String>,
        owner_id: String,
        platforms: Vec<String>,
        recurrence: Option<models::CalendarEventRecurrence>,
        role_ids: Option<Vec<String>>,
        series_id: Option<String>,
        short_code: Option<String>,
        starts_at: chrono::DateTime<chrono::FixedOffset>,
        tags: Vec<String>,
        updated_at: chrono::DateTime<chrono::FixedOffset>,
        uses_instance_overflow: bool,
    ) -> GroupAuditLogEntryDataGroupCalendarEventDelete {
        GroupAuditLogEntryDataGroupCalendarEventDelete {
            group_audit_log_entry_data_group_calendar_event_create,
            category,
            close_instance_after_end_minutes,
            created_at,
            deleted_at,
            duration_in_ms,
            ends_at,
            featured,
            guest_early_join_minutes,
            host_early_join_minutes,
            interested_user_count,
            is_draft,
            languages,
            occurrence_kind,
            occurrence_modified,
            owner_id,
            platforms,
            recurrence,
            role_ids,
            series_id,
            short_code,
            starts_at,
            tags,
            updated_at,
            uses_instance_overflow,
        }
    }
}
