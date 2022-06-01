use serde::{Deserialize, Serialize};

/// Represents a Map View in the director: be them overview views, destination views, or other.
/// They have nodes which map to activities, and other various visual elements that we (or others) may or may not be able to use.
/// Activity graphs, most importantly, have nodes which can have activities in various states of playability.
/// Unfortunately, activity graphs are combined at runtime with Game UI-only assets such as fragments of map images, various in-game special effects, decals etc... that we don't get in these definitions.
/// If we end up having time, we may end up trying to manually populate those here: but the last time we tried that, before the lead-up to D1, it proved to be unmaintainable as the game's content changed. So don't bet the farm on us providing that content in this definition.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphDefinition {
    /// These represent the visual "nodes" on the map's view. These are the activities you can click on in the map.
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphNodeDefinition>>,

    /// Represents one-off/special UI elements that appear on the map.
    #[serde(rename = "artElements")]
    pub art_elements: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphArtElementDefinition>>,

    /// Represents connections between graph nodes. However, it lacks context that we'd need to make good use of it.
    #[serde(rename = "connections")]
    pub connections: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphConnectionDefinition>>,

    /// Objectives can display on maps, and this is supposedly metadata for that. I have not had the time to analyze the details of what is useful within however: we could be missing important data to make this work. Expect this property to be expanded on later if possible.
    #[serde(rename = "displayObjectives")]
    pub display_objectives: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphDisplayObjectiveDefinition>>,

    /// Progressions can also display on maps, but similarly to displayObjectives we appear to lack some required information and context right now. We will have to look into it later and add more data if possible.
    #[serde(rename = "displayProgressions")]
    pub display_progressions: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphDisplayProgressionDefinition>>,

    /// Represents links between this Activity Graph and other ones.
    #[serde(rename = "linkedGraphs")]
    pub linked_graphs: Option<Vec<crate::destiny::definitions::director::DestinyLinkedGraphDefinition>>,

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

/// This is the position and other data related to nodes in the activity graph that you can click to launch activities. An Activity Graph node will only have one active Activity at a time, which will determine the activity to be launched (and, unless overrideDisplay information is provided, will also determine the tooltip and other UI related to the node)
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphNodeDefinition {
    /// An identifier for the Activity Graph Node, only guaranteed to be unique within its parent Activity Graph.
    #[serde(rename = "nodeId")]
    pub node_id: u32,

    /// The node *may* have display properties that override the active Activity's display properties.
    #[serde(rename = "overrideDisplay")]
    pub override_display: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The position on the map for this node.
    #[serde(rename = "position")]
    pub position: Option<crate::destiny::definitions::common::DestinyPositionDefinition>,

    /// The node may have various visual accents placed on it, or styles applied. These are the list of possible styles that the Node can have. The game iterates through each, looking for the first one that passes a check of the required game/character/account state in order to show that style, and then renders the node in that style.
    #[serde(rename = "featuringStates")]
    pub featuring_states: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphNodeFeaturingStateDefinition>>,

    /// The node may have various possible activities that could be active for it, however only one may be active at a time. See the DestinyActivityGraphNodeActivityDefinition for details.
    #[serde(rename = "activities")]
    pub activities: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphNodeActivityDefinition>>,

    /// Represents possible states that the graph node can be in. These are combined with some checking that happens in the game client and server to determine which state is actually active at any given time.
    #[serde(rename = "states")]
    pub states: Option<Vec<crate::destiny::definitions::director::DestinyActivityGraphNodeStateEntry>>,
}

/// Nodes can have different visual states. This object represents a single visual state ("highlight type") that a node can be in, and the unlock expression condition to determine whether it should be set.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphNodeFeaturingStateDefinition {
    /// The node can be highlighted in a variety of ways - the game iterates through these and finds the first FeaturingState that is valid at the present moment given the Game, Account, and Character state, and renders the node in that state. See the ActivityGraphNodeHighlightType enum for possible values.
    #[serde(rename = "highlightType")]
    pub highlight_type: crate::destiny::ActivityGraphNodeHighlightType,
}

/// The actual activity to be redirected to when you click on the node. Note that a node can have many Activities attached to it: but only one will be active at any given time. The list of Node Activities will be traversed, and the first one found to be active will be displayed. This way, a node can layer multiple variants of an activity on top of each other. For instance, one node can control the weekly Crucible Playlist. There are multiple possible playlists, but only one is active for the week.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphNodeActivityDefinition {
    /// An identifier for this node activity. It is only guaranteed to be unique within the Activity Graph.
    #[serde(rename = "nodeActivityId")]
    pub node_activity_id: u32,

    /// The activity that will be activated if the user clicks on this node. Controls all activity-related information displayed on the node if it is active (the text shown in the tooltip etc)
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,
}

/// Represents a single state that a graph node might end up in. Depending on what's going on in the game, graph nodes could be shown in different ways or even excluded from view entirely.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphNodeStateEntry {
    #[serde(rename = "state")]
    pub state: crate::destiny::DestinyGraphNodeState,
}

/// These Art Elements are meant to represent one-off visual effects overlaid on the map. Currently, we do not have a pipeline to import the assets for these overlays, so this info exists as a placeholder for when such a pipeline exists (if it ever will)
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphArtElementDefinition {
    /// The position on the map of the art element.
    #[serde(rename = "position")]
    pub position: Option<crate::destiny::definitions::common::DestinyPositionDefinition>,
}

/// Nodes on a graph can be visually connected: this appears to be the information about which nodes to link. It appears to lack more detailed information, such as the path for that linking.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphConnectionDefinition {
    #[serde(rename = "sourceNodeHash")]
    pub source_node_hash: u32,

    #[serde(rename = "destNodeHash")]
    pub dest_node_hash: u32,
}

/// When a Graph needs to show active Objectives, this defines those objectives as well as an identifier.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphDisplayObjectiveDefinition {
    /// $NOTE $amola 2017-01-19 This field is apparently something that CUI uses to manually wire up objectives to display info. I am unsure how it works.
    #[serde(rename = "id")]
    pub id: u32,

    /// The objective being shown on the map.
    #[serde(rename = "objectiveHash")]
    pub objective_hash: u32,
}

/// When a Graph needs to show active Progressions, this defines those objectives as well as an identifier.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityGraphDisplayProgressionDefinition {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "progressionHash")]
    pub progression_hash: u32,
}

/// This describes links between the current graph and others, as well as when that link is relevant.
#[derive(Deserialize, Serialize)]
pub struct DestinyLinkedGraphDefinition {
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "unlockExpression")]
    pub unlock_expression: Option<crate::destiny::definitions::DestinyUnlockExpressionDefinition>,

    #[serde(rename = "linkedGraphId")]
    pub linked_graph_id: u32,

    #[serde(rename = "linkedGraphs")]
    pub linked_graphs: Option<Vec<crate::destiny::definitions::director::DestinyLinkedGraphEntryDefinition>>,

    #[serde(rename = "overview")]
    pub overview: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyLinkedGraphEntryDefinition {
    #[serde(rename = "activityGraphHash")]
    pub activity_graph_hash: u32,
}
