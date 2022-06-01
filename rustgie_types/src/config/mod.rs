pub mod clan_banner;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserTheme {
    #[serde(rename = "userThemeId")]
    pub user_theme_id: i32,

    #[serde(rename = "userThemeName")]
    pub user_theme_name: Option<String>,

    #[serde(rename = "userThemeDescription")]
    pub user_theme_description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupTheme {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "folder")]
    pub folder: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,
}
