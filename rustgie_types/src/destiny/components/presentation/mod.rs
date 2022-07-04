use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DestinyPresentationNodesComponent {
    #[serde(rename = "nodes")]
    pub nodes: Option<HashMap<u32, crate::destiny::components::presentation::DestinyPresentationNodeComponent>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyPresentationNodeComponent {
    #[serde(rename = "state")]
    pub state: enumflags2::BitFlags<crate::destiny::DestinyPresentationNodeState>,

    /// An optional property: presentation nodes MAY have objectives, which can be used to infer more human readable data about the progress. However, progressValue and completionValue ought to be considered the canonical values for progress on Progression Nodes.
    #[serde(rename = "objective")]
    pub objective: Option<crate::destiny::quests::DestinyObjectiveProgress>,

    /// How much of the presentation node is considered to be completed so far by the given character/profile.
    #[serde(rename = "progressValue")]
    pub progress_value: i32,

    /// The value at which the presentation node is considered to be completed.
    #[serde(rename = "completionValue")]
    pub completion_value: i32,

    /// If available, this is the current score for the record category that this node represents.
    #[serde(rename = "recordCategoryScore")]
    pub record_category_score: Option<i32>,
}
