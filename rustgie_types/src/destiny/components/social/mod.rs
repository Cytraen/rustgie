use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinySocialCommendationsComponent {
    #[serde(rename = "totalScore")]
    pub total_score: i32,

    #[serde(rename = "scoreDetailValues")]
    pub score_detail_values: Option<Vec<i32>>,

    #[serde(rename = "commendationNodeScoresByHash")]
    pub commendation_node_scores_by_hash: Option<HashMap<u32, i32>>,

    #[serde(rename = "commendationScoresByHash")]
    pub commendation_scores_by_hash: Option<HashMap<u32, i32>>,
}
