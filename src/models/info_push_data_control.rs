use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataControl {
    #[serde(rename = "control")]
    pub control: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "label")]
    pub label: models::LocalizedString,
    #[serde(rename = "options")]
    pub options: Vec<models::InfoPushDataControlOption>,
    #[serde(rename = "selection")]
    pub selection: String,
}

impl InfoPushDataControl {
    pub fn new(
        control: String,
        display: String,
        id: String,
        kind: String,
        label: models::LocalizedString,
        options: Vec<models::InfoPushDataControlOption>,
        selection: String,
    ) -> InfoPushDataControl {
        InfoPushDataControl {
            control,
            display,
            id,
            kind,
            label,
            options,
            selection,
        }
    }
}
