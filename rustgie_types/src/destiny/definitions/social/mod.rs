use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinySocialCommendationNodeDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The color associated with this group of commendations.
    #[serde(rename = "color")]
    pub color: Option<crate::destiny::misc::DestinyColor>,

    #[serde(rename = "parentCommendationNodeHash")]
    pub parent_commendation_node_hash: u32,

    /// A list of hashes that map to child commendation nodes. Only the root commendations node is expected to have child nodes.
    #[serde(rename = "childCommendationNodeHashes")]
    pub child_commendation_node_hashes: Option<Vec<u32>>,

    /// A list of hashes that map to child commendations.
    #[serde(rename = "childCommendationHashes")]
    pub child_commendation_hashes: Option<Vec<u32>>,

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
pub struct DestinySocialCommendationDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "cardImagePath")]
    pub card_image_path: Option<String>,

    #[serde(rename = "color")]
    pub color: Option<crate::destiny::misc::DestinyColor>,

    #[serde(rename = "displayPriority")]
    pub display_priority: i32,

    #[serde(rename = "activityGivingLimit")]
    pub activity_giving_limit: i32,

    #[serde(rename = "parentCommendationNodeHash")]
    pub parent_commendation_node_hash: u32,

    /// The display properties for the the activities that this commendation is available in.
    #[serde(rename = "displayActivities")]
    pub display_activities: Option<Vec<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>>,

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
