use serde::{Deserialize, Serialize};

/// This is the base class for all presentation system children. Presentation Nodes, Records, Collectibles, and Metrics.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeBaseDefinition {
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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyScoredPresentationNodeBaseDefinition {
    #[serde(rename = "maxCategoryRecordScore")]
    pub max_category_record_score: i32,

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

/// A PresentationNode is an entity that represents a logical grouping of other entities visually/organizationally.
/// For now, Presentation Nodes may contain the following... but it may be used for more in the future:
/// - Collectibles - Records (Or, as the public will call them, "Triumphs." Don't ask me why we're overloading the term "Triumph", it still hurts me to think about it) - Metrics (aka Stat Trackers) - Other Presentation Nodes, allowing a tree of Presentation Nodes to be created
/// Part of me wants to break these into conceptual definitions per entity being collected, but the possibility of these different types being mixed in the same UI and the possibility that it could actually be more useful to return the "bare metal" presentation node concept has resulted in me deciding against that for the time being.
/// We'll see if I come to regret this as well.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The original icon for this presentation node, before we futzed with it.
    #[serde(rename = "originalIcon")]
    pub original_icon: Option<String>,

    /// Some presentation nodes are meant to be explicitly shown on the "root" or "entry" screens for the feature to which they are related. You should use this icon when showing them on such a view, if you have a similar "entry point" view in your UI. If you don't have a UI, then I guess it doesn't matter either way does it?
    #[serde(rename = "rootViewIcon")]
    pub root_view_icon: Option<String>,

    #[serde(rename = "nodeType")]
    pub node_type: crate::destiny::DestinyPresentationNodeType,

    /// Indicates whether this presentation node's state is determined on a per-character or on an account-wide basis.
    #[serde(rename = "scope")]
    pub scope: crate::destiny::DestinyScope,

    /// If this presentation node shows a related objective (for instance, if it tracks the progress of its children), the objective being tracked is indicated here.
    #[serde(rename = "objectiveHash")]
    pub objective_hash: Option<u32>,

    /// If this presentation node has an associated "Record" that you can accomplish for completing its children, this is the identifier of that Record.
    #[serde(rename = "completionRecordHash")]
    pub completion_record_hash: Option<u32>,

    /// The child entities contained by this presentation node.
    #[serde(rename = "children")]
    pub children: Option<crate::destiny::definitions::presentation::DestinyPresentationNodeChildrenBlock>,

    /// A hint for how to display this presentation node when it's shown in a list.
    #[serde(rename = "displayStyle")]
    pub display_style: crate::destiny::DestinyPresentationDisplayStyle,

    /// A hint for how to display this presentation node when it's shown in its own detail screen.
    #[serde(rename = "screenStyle")]
    pub screen_style: crate::destiny::DestinyPresentationScreenStyle,

    /// The requirements for being able to interact with this presentation node and its children.
    #[serde(rename = "requirements")]
    pub requirements: Option<crate::destiny::definitions::presentation::DestinyPresentationNodeRequirementsBlock>,

    /// If this presentation node has children, but the game doesn't let you inspect the details of those children, that is indicated here.
    #[serde(rename = "disableChildSubscreenNavigation")]
    pub disable_child_subscreen_navigation: bool,

    #[serde(rename = "maxCategoryRecordScore")]
    pub max_category_record_score: i32,

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

/// As/if presentation nodes begin to host more entities as children, these lists will be added to. One list property exists per type of entity that can be treated as a child of this presentation node, and each holds the identifier of the entity and any associated information needed to display the UI for that entity (if anything)
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeChildrenBlock {
    #[serde(rename = "presentationNodes")]
    pub presentation_nodes: Option<Vec<crate::destiny::definitions::presentation::DestinyPresentationNodeChildEntry>>,

    #[serde(rename = "collectibles")]
    pub collectibles: Option<Vec<crate::destiny::definitions::presentation::DestinyPresentationNodeCollectibleChildEntry>>,

    #[serde(rename = "records")]
    pub records: Option<Vec<crate::destiny::definitions::presentation::DestinyPresentationNodeRecordChildEntry>>,

    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<crate::destiny::definitions::presentation::DestinyPresentationNodeMetricChildEntry>>,

    #[serde(rename = "craftables")]
    pub craftables: Option<Vec<crate::destiny::definitions::presentation::DestinyPresentationNodeCraftableChildEntry>>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeChildEntryBase {
    /// Use this value to sort the presentation node children in ascending order.
    #[serde(rename = "nodeDisplayPriority")]
    pub node_display_priority: u32,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeChildEntry {
    #[serde(rename = "presentationNodeHash")]
    pub presentation_node_hash: u32,

    /// Use this value to sort the presentation node children in ascending order.
    #[serde(rename = "nodeDisplayPriority")]
    pub node_display_priority: u32,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeCollectibleChildEntry {
    #[serde(rename = "collectibleHash")]
    pub collectible_hash: u32,

    /// Use this value to sort the presentation node children in ascending order.
    #[serde(rename = "nodeDisplayPriority")]
    pub node_display_priority: u32,
}

/// Presentation nodes can be restricted by various requirements. This defines the rules of those requirements, and the message(s) to be shown if these requirements aren't met.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeRequirementsBlock {
    /// If this node is not accessible due to Entitlements (for instance, you don't own the required game expansion), this is the message to show.
    #[serde(rename = "entitlementUnavailableMessage")]
    pub entitlement_unavailable_message: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationChildBlock {
    #[serde(rename = "presentationNodeType")]
    pub presentation_node_type: crate::destiny::DestinyPresentationNodeType,

    #[serde(rename = "parentPresentationNodeHashes")]
    pub parent_presentation_node_hashes: Option<Vec<u32>>,

    #[serde(rename = "displayStyle")]
    pub display_style: crate::destiny::DestinyPresentationDisplayStyle,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeRecordChildEntry {
    #[serde(rename = "recordHash")]
    pub record_hash: u32,

    /// Use this value to sort the presentation node children in ascending order.
    #[serde(rename = "nodeDisplayPriority")]
    pub node_display_priority: u32,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeMetricChildEntry {
    #[serde(rename = "metricHash")]
    pub metric_hash: u32,

    /// Use this value to sort the presentation node children in ascending order.
    #[serde(rename = "nodeDisplayPriority")]
    pub node_display_priority: u32,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPresentationNodeCraftableChildEntry {
    #[serde(rename = "craftableItemHash")]
    pub craftable_item_hash: u32,

    /// Use this value to sort the presentation node children in ascending order.
    #[serde(rename = "nodeDisplayPriority")]
    pub node_display_priority: u32,
}
