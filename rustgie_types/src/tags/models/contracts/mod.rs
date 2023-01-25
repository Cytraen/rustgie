use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct TagResponse {
    #[serde(rename = "tagText")]
    pub tag_text: Option<String>,

    #[serde(rename = "ignoreStatus")]
    pub ignore_status: Option<crate::ignores::IgnoreResponse>,
}
