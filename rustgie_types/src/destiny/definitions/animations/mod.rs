use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyAnimationReference {
    #[serde(rename = "animName")]
    pub anim_name: Option<String>,

    #[serde(rename = "animIdentifier")]
    pub anim_identifier: Option<String>,

    #[serde(rename = "path")]
    pub path: Option<String>,
}
