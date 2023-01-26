use serde::{Deserialize, Serialize};

/// Defines a
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyCollectibleDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// Indicates whether the state of this Collectible is determined on a per-character or on an account-wide basis.
    #[serde(rename = "scope")]
    pub scope: crate::destiny::DestinyScope,

    /// A human readable string for a hint about how to acquire the item.
    #[serde(rename = "sourceString")]
    pub source_string: Option<String>,

    /// This is a hash identifier we are building on the BNet side in an attempt to let people group collectibles by similar sources.
    /// I can't promise that it's going to be 100% accurate, but if the designers were consistent in assigning the same source strings to items with the same sources, it *ought to* be. No promises though.
    /// This hash also doesn't relate to an actual definition, just to note: we've got nothing useful other than the source string for this data.
    #[serde(rename = "sourceHash")]
    pub source_hash: Option<u32>,

    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    #[serde(rename = "acquisitionInfo")]
    pub acquisition_info: Option<crate::destiny::definitions::collectibles::DestinyCollectibleAcquisitionBlock>,

    #[serde(rename = "stateInfo")]
    pub state_info: Option<crate::destiny::definitions::collectibles::DestinyCollectibleStateBlock>,

    #[serde(rename = "presentationInfo")]
    pub presentation_info: Option<crate::destiny::definitions::presentation::DestinyPresentationChildBlock>,

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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyCollectibleAcquisitionBlock {
    #[serde(rename = "acquireMaterialRequirementHash")]
    pub acquire_material_requirement_hash: Option<u32>,

    #[serde(rename = "acquireTimestampUnlockValueHash")]
    pub acquire_timestamp_unlock_value_hash: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyCollectibleStateBlock {
    #[serde(rename = "obscuredOverrideItemHash")]
    pub obscured_override_item_hash: Option<u32>,

    #[serde(rename = "requirements")]
    pub requirements: Option<crate::destiny::definitions::presentation::DestinyPresentationNodeRequirementsBlock>,
}
