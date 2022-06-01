use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DestinyCraftablesComponent {
    /// A map of craftable item hashes to craftable item state components.
    #[serde(rename = "craftables")]
    pub craftables: Option<HashMap<u32, crate::destiny::components::craftables::DestinyCraftableComponent>>,

    /// The hash for the root presentation node definition of craftable item categories.
    #[serde(rename = "craftingRootNodeHash")]
    pub crafting_root_node_hash: u32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyCraftableComponent {
    #[serde(rename = "visible")]
    pub visible: bool,

    /// If the requirements are not met for crafting this item, these will index into the list of failure strings.
    #[serde(rename = "failedRequirementIndexes")]
    pub failed_requirement_indexes: Option<Vec<i32>>,

    /// Plug item state for the crafting sockets.
    #[serde(rename = "sockets")]
    pub sockets: Option<Vec<crate::destiny::components::craftables::DestinyCraftableSocketComponent>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyCraftableSocketComponent {
    #[serde(rename = "plugSetHash")]
    pub plug_set_hash: u32,

    /// Unlock state for plugs in the socket plug set definition
    #[serde(rename = "plugs")]
    pub plugs: Option<Vec<crate::destiny::components::craftables::DestinyCraftableSocketPlugComponent>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyCraftableSocketPlugComponent {
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,

    /// Index into the unlock requirements to display failure descriptions
    #[serde(rename = "failedRequirementIndexes")]
    pub failed_requirement_indexes: Option<Vec<i32>>,
}
