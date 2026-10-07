use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAuditLogEntryDataGroupCalendarEventCreate {
    #[serde(rename = "accessType")]
    pub access_type: models::CalendarEventAccess,
    /// The description of the calendar event.
    #[serde(rename = "description")]
    pub description: String,
    /// The image file ID for the event.
    #[serde(rename = "imageId", deserialize_with = "Option::deserialize")]
    pub image_id: Option<String>,
    /// The title of the calendar event.
    #[serde(rename = "title")]
    pub title: String,
    /// The type of calendar entry.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl GroupAuditLogEntryDataGroupCalendarEventCreate {
    pub fn new(
        access_type: models::CalendarEventAccess,
        description: String,
        image_id: Option<String>,
        title: String,
        r#type: String,
    ) -> GroupAuditLogEntryDataGroupCalendarEventCreate {
        GroupAuditLogEntryDataGroupCalendarEventCreate {
            access_type,
            description,
            image_id,
            title,
            r#type,
        }
    }
}
