use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct ClanBannerSource {}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct ClanBannerDecal {
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    #[serde(rename = "foregroundPath")]
    pub foreground_path: Option<String>,

    #[serde(rename = "backgroundPath")]
    pub background_path: Option<String>,
}
