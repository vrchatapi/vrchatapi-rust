use crate::models;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationV2DataGroupInvite {
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "managerUserDisplayName")]
    pub manager_user_display_name: String,
}

impl NotificationV2DataGroupInvite {
    pub fn new(
        group_name: String,
        manager_user_display_name: String,
    ) -> NotificationV2DataGroupInvite {
        NotificationV2DataGroupInvite {
            group_name,
            manager_user_display_name,
        }
    }
}
