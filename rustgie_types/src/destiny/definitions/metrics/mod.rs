use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DestinyMetricDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "trackingObjectiveHash")]
    pub tracking_objective_hash: u32,

    #[serde(rename = "lowerValueIsBetter")]
    pub lower_value_is_better: bool,

    #[serde(rename = "presentationNodeType")]
    pub presentation_node_type: crate::destiny::DestinyPresentationNodeType,

    #[serde(rename = "traitIds")]
    pub trait_ids: Option<Vec<String>>,

    #[serde(rename = "traitHashes")]
    pub trait_hashes: Option<Vec<u32>>,

    /// A quick reference to presentation nodes that have this node as a child. Presentation nodes can be parented under multiple parents.
    #[serde(rename = "parentNodeHashes")]
    pub parent_node_hashes: Option<Vec<u32>>,

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
    /// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    #[serde(rename = "hash")]
    pub hash: u32,

    /// The index of the entity as it was found in the investment tables.
    #[serde(rename = "index")]
    pub index: i32,

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    #[serde(rename = "redacted")]
    pub redacted: bool,
}
