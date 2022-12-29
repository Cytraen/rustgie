use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct DestinyRecordDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// Indicates whether this Record's state is determined on a per-character or on an account-wide basis.
    #[serde(rename = "scope")]
    pub scope: crate::destiny::DestinyScope,

    #[serde(rename = "presentationInfo")]
    pub presentation_info: Option<crate::destiny::definitions::presentation::DestinyPresentationChildBlock>,

    #[serde(rename = "loreHash")]
    pub lore_hash: Option<u32>,

    #[serde(rename = "objectiveHashes")]
    pub objective_hashes: Option<Vec<u32>>,

    #[serde(rename = "recordValueStyle")]
    pub record_value_style: crate::destiny::DestinyRecordValueStyle,

    #[serde(rename = "forTitleGilding")]
    pub for_title_gilding: bool,

    /// A hint to show a large icon for a reward
    #[serde(rename = "shouldShowLargeIcons")]
    pub should_show_large_icons: bool,

    #[serde(rename = "titleInfo")]
    pub title_info: Option<crate::destiny::definitions::records::DestinyRecordTitleBlock>,

    #[serde(rename = "completionInfo")]
    pub completion_info: Option<crate::destiny::definitions::records::DestinyRecordCompletionBlock>,

    #[serde(rename = "stateInfo")]
    pub state_info: Option<crate::destiny::definitions::records::SchemaRecordStateBlock>,

    #[serde(rename = "requirements")]
    pub requirements: Option<crate::destiny::definitions::presentation::DestinyPresentationNodeRequirementsBlock>,

    #[serde(rename = "expirationInfo")]
    pub expiration_info: Option<crate::destiny::definitions::records::DestinyRecordExpirationBlock>,

    /// Some records have multiple 'interval' objectives, and the record may be claimed at each completed interval
    #[serde(rename = "intervalInfo")]
    pub interval_info: Option<crate::destiny::definitions::records::DestinyRecordIntervalBlock>,

    /// If there is any publicly available information about rewards earned for achieving this record, this is the list of those items.
    /// However, note that some records intentionally have "hidden" rewards. These will not be returned in this list.
    #[serde(rename = "rewardItems")]
    pub reward_items: Option<Vec<crate::destiny::DestinyItemQuantity>>,

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

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyRecordTitleBlock {
    #[serde(rename = "hasTitle")]
    pub has_title: bool,

    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "titlesByGender")]
    pub titles_by_gender: Option<HashMap<crate::destiny::DestinyGender, String>>,

    /// For those who prefer to use the definitions.
    #[serde(rename = "titlesByGenderHash")]
    pub titles_by_gender_hash: Option<HashMap<u32, String>>,

    #[serde(rename = "gildingTrackingRecordHash")]
    pub gilding_tracking_record_hash: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyRecordCompletionBlock {
    /// The number of objectives that must be completed before the objective is considered "complete"
    #[serde(rename = "partialCompletionObjectiveCountThreshold")]
    pub partial_completion_objective_count_threshold: i32,

    #[serde(rename = "ScoreValue")]
    pub score_value: i32,

    #[serde(rename = "shouldFireToast")]
    pub should_fire_toast: bool,

    #[serde(rename = "toastStyle")]
    pub toast_style: crate::destiny::DestinyRecordToastStyle,
}

#[derive(Deserialize, Serialize)]
pub struct SchemaRecordStateBlock {
    #[serde(rename = "featuredPriority")]
    pub featured_priority: i32,

    #[serde(rename = "obscuredString")]
    pub obscured_string: Option<String>,
}

/// If this record has an expiration after which it cannot be earned, this is some information about that expiration.
#[derive(Deserialize, Serialize)]
pub struct DestinyRecordExpirationBlock {
    #[serde(rename = "hasExpiration")]
    pub has_expiration: bool,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "icon")]
    pub icon: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyRecordIntervalBlock {
    #[serde(rename = "intervalObjectives")]
    pub interval_objectives: Option<Vec<crate::destiny::definitions::records::DestinyRecordIntervalObjective>>,

    #[serde(rename = "intervalRewards")]
    pub interval_rewards: Option<Vec<crate::destiny::definitions::records::DestinyRecordIntervalRewards>>,

    #[serde(rename = "originalObjectiveArrayInsertionIndex")]
    pub original_objective_array_insertion_index: i32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyRecordIntervalObjective {
    #[serde(rename = "intervalObjectiveHash")]
    pub interval_objective_hash: u32,

    #[serde(rename = "intervalScoreValue")]
    pub interval_score_value: i32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyRecordIntervalRewards {
    #[serde(rename = "intervalRewardItems")]
    pub interval_reward_items: Option<Vec<crate::destiny::DestinyItemQuantity>>,
}
