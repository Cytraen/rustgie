use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct HyperlinkReference {
    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "url")]
    pub url: Option<String>,
}
