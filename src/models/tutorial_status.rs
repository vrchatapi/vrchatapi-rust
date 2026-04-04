use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TutorialStatus {
    #[serde(rename = "completed")]
    pub completed: bool,
    #[serde(rename = "completedAnyTutorial")]
    pub completed_any_tutorial: bool,
    #[serde(rename = "completedTutorials")]
    pub completed_tutorials: Vec<String>,
    /// The ID of a tutorial, in the format `{platform}:{tutorial}:{version}`. `undefined:undefined:v1` is used as a null-ish or sentinel value.
    #[serde(rename = "tutorialKey")]
    pub tutorial_key: String,
}

impl TutorialStatus {
    pub fn new(
        completed: bool,
        completed_any_tutorial: bool,
        completed_tutorials: Vec<String>,
        tutorial_key: String,
    ) -> TutorialStatus {
        TutorialStatus {
            completed,
            completed_any_tutorial,
            completed_tutorials,
            tutorial_key,
        }
    }
}
