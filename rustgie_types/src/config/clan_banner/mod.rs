use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ClanBannerSource {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ClanBannerDecal {
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    #[serde(rename = "foregroundPath")]
    pub foreground_path: Option<String>,

    #[serde(rename = "backgroundPath")]
    pub background_path: Option<String>,
}
