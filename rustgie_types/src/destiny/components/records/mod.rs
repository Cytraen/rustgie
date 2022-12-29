use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct DestinyRecordsComponent {
    #[serde(rename = "records")]
    pub records: Option<HashMap<u32, crate::destiny::components::records::DestinyRecordComponent>>,

    /// The hash for the root presentation node definition of Triumph categories.
    #[serde(rename = "recordCategoriesRootNodeHash")]
    pub record_categories_root_node_hash: u32,

    /// The hash for the root presentation node definition of Triumph Seals.
    #[serde(rename = "recordSealsRootNodeHash")]
    pub record_seals_root_node_hash: u32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyRecordComponent {
    #[serde(rename = "state")]
    pub state: enumflags2::BitFlags<crate::destiny::DestinyRecordState>,

    #[serde(rename = "objectives")]
    pub objectives: Option<Vec<crate::destiny::quests::DestinyObjectiveProgress>>,

    #[serde(rename = "intervalObjectives")]
    pub interval_objectives: Option<Vec<crate::destiny::quests::DestinyObjectiveProgress>>,

    #[serde(rename = "intervalsRedeemedCount")]
    pub intervals_redeemed_count: i32,

    /// If available, this is the number of times this record has been completed. For example, the number of times a seal title has been gilded.
    #[serde(rename = "completedCount")]
    pub completed_count: Option<i32>,

    /// If available, a list that describes which reward rewards should be shown (true) or hidden (false). This property is for regular record rewards, and not for interval objective rewards.
    #[serde(rename = "rewardVisibilty")]
    pub reward_visibilty: Option<Vec<bool>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyProfileRecordsComponent {
    /// Your 'active' Triumphs score, maintained for backwards compatibility.
    #[serde(rename = "score")]
    pub score: i32,

    /// Your 'active' Triumphs score.
    #[serde(rename = "activeScore")]
    pub active_score: i32,

    /// Your 'legacy' Triumphs score.
    #[serde(rename = "legacyScore")]
    pub legacy_score: i32,

    /// Your 'lifetime' Triumphs score.
    #[serde(rename = "lifetimeScore")]
    pub lifetime_score: i32,

    /// If this profile is tracking a record, this is the hash identifier of the record it is tracking.
    #[serde(rename = "trackedRecordHash")]
    pub tracked_record_hash: Option<u32>,

    #[serde(rename = "records")]
    pub records: Option<HashMap<u32, crate::destiny::components::records::DestinyRecordComponent>>,

    /// The hash for the root presentation node definition of Triumph categories.
    #[serde(rename = "recordCategoriesRootNodeHash")]
    pub record_categories_root_node_hash: u32,

    /// The hash for the root presentation node definition of Triumph Seals.
    #[serde(rename = "recordSealsRootNodeHash")]
    pub record_seals_root_node_hash: u32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyCharacterRecordsComponent {
    #[serde(rename = "featuredRecordHashes")]
    pub featured_record_hashes: Option<Vec<u32>>,

    #[serde(rename = "records")]
    pub records: Option<HashMap<u32, crate::destiny::components::records::DestinyRecordComponent>>,

    /// The hash for the root presentation node definition of Triumph categories.
    #[serde(rename = "recordCategoriesRootNodeHash")]
    pub record_categories_root_node_hash: u32,

    /// The hash for the root presentation node definition of Triumph Seals.
    #[serde(rename = "recordSealsRootNodeHash")]
    pub record_seals_root_node_hash: u32,
}
