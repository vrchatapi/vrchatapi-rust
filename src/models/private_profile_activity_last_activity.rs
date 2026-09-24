use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateProfileActivityLastActivity {}

impl PrivateProfileActivityLastActivity {
    pub fn new() -> PrivateProfileActivityLastActivity {
        PrivateProfileActivityLastActivity {}
    }
}
