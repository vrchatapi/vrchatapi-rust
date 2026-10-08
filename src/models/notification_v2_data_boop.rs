use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationV2DataBoop {
    #[serde(rename = "boopingUserDisplayName")]
    pub booping_user_display_name: String,
}

impl NotificationV2DataBoop {
    pub fn new(booping_user_display_name: String) -> NotificationV2DataBoop {
        NotificationV2DataBoop {
            booping_user_display_name,
        }
    }
}
