use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutsComponent {
    #[serde(rename = "loadouts")]
    pub loadouts: Option<Vec<crate::destiny::components::loadouts::DestinyLoadoutComponent>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutComponent {
    #[serde(rename = "colorHash")]
    pub color_hash: u32,

    #[serde(rename = "iconHash")]
    pub icon_hash: u32,

    #[serde(rename = "nameHash")]
    pub name_hash: u32,

    #[serde(rename = "items")]
    pub items: Option<Vec<crate::destiny::components::loadouts::DestinyLoadoutItemComponent>>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutItemComponent {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: i64,

    #[serde(rename = "plugItemHashes")]
    pub plug_item_hashes: Option<Vec<u32>>,
}
