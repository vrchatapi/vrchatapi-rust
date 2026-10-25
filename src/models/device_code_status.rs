use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeviceCodeStatus {
    DeviceCodePending(models::DeviceCodePending),
    DeviceCodeApproved(models::DeviceCodeApproved),
}

impl Default for DeviceCodeStatus {
    fn default() -> Self {
        Self::DeviceCodePending(Default::default())
    }
}

impl From<models::DeviceCodePending> for DeviceCodeStatus {
    fn from(value: models::DeviceCodePending) -> Self {
        Self::DeviceCodePending(value)
    }
}

impl TryFrom<DeviceCodeStatus> for models::DeviceCodePending {
    type Error = DeviceCodeStatus;

    fn try_from(value: DeviceCodeStatus) -> Result<Self, Self::Error> {
        match value {
            DeviceCodeStatus::DeviceCodePending(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl From<models::DeviceCodeApproved> for DeviceCodeStatus {
    fn from(value: models::DeviceCodeApproved) -> Self {
        Self::DeviceCodeApproved(value)
    }
}

impl TryFrom<DeviceCodeStatus> for models::DeviceCodeApproved {
    type Error = DeviceCodeStatus;

    fn try_from(value: DeviceCodeStatus) -> Result<Self, Self::Error> {
        match value {
            DeviceCodeStatus::DeviceCodeApproved(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
