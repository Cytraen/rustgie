use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DestinyItemPlugBase {
    /// The hash identifier of the DestinyInventoryItemDefinition that represents this plug.
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,

    /// If true, this plug has met all of its insertion requirements. Big if true.
    #[serde(rename = "canInsert")]
    pub can_insert: bool,

    /// If true, this plug will provide its benefits while inserted.
    #[serde(rename = "enabled")]
    pub enabled: bool,

    /// If the plug cannot be inserted for some reason, this will have the indexes into the plug item definition's plug.insertionRules property, so you can show the reasons why it can't be inserted.
    /// This list will be empty if the plug can be inserted.
    #[serde(rename = "insertFailIndexes")]
    pub insert_fail_indexes: Option<Vec<i32>>,

    /// If a plug is not enabled, this will be populated with indexes into the plug item definition's plug.enabledRules property, so that you can show the reasons why it is not enabled.
    /// This list will be empty if the plug is enabled.
    #[serde(rename = "enableFailIndexes")]
    pub enable_fail_indexes: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyItemPlug {
    /// Sometimes, Plugs may have objectives: these are often used for flavor and display purposes, but they can be used for any arbitrary purpose (both fortunately and unfortunately). Recently (with Season 2) they were expanded in use to be used as the "gating" for whether the plug can be inserted at all. For instance, a Plug might be tracking the number of PVP kills you have made. It will use the parent item's data about that tracking status to determine what to show, and will generally show it using the DestinyObjectiveDefinition's progressDescription property. Refer to the plug's itemHash and objective property for more information if you would like to display even more data.
    #[serde(rename = "plugObjectives")]
    pub plug_objectives: Option<Vec<crate::destiny::quests::DestinyObjectiveProgress>>,

    /// The hash identifier of the DestinyInventoryItemDefinition that represents this plug.
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,

    /// If true, this plug has met all of its insertion requirements. Big if true.
    #[serde(rename = "canInsert")]
    pub can_insert: bool,

    /// If true, this plug will provide its benefits while inserted.
    #[serde(rename = "enabled")]
    pub enabled: bool,

    /// If the plug cannot be inserted for some reason, this will have the indexes into the plug item definition's plug.insertionRules property, so you can show the reasons why it can't be inserted.
    /// This list will be empty if the plug can be inserted.
    #[serde(rename = "insertFailIndexes")]
    pub insert_fail_indexes: Option<Vec<i32>>,

    /// If a plug is not enabled, this will be populated with indexes into the plug item definition's plug.enabledRules property, so that you can show the reasons why it is not enabled.
    /// This list will be empty if the plug is enabled.
    #[serde(rename = "enableFailIndexes")]
    pub enable_fail_indexes: Option<Vec<i32>>,
}
