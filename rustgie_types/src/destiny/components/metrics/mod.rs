use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyMetricsComponent {
    #[serde(rename = "metrics")]
    pub metrics: Option<HashMap<u32, crate::destiny::components::metrics::DestinyMetricComponent>>,

    #[serde(rename = "metricsRootNodeHash")]
    pub metrics_root_node_hash: u32,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyMetricComponent {
    #[serde(rename = "invisible")]
    pub invisible: bool,

    #[serde(rename = "objectiveProgress")]
    pub objective_progress: Option<crate::destiny::quests::DestinyObjectiveProgress>,
}
