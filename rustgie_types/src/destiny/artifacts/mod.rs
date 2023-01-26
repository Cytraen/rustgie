use serde::{Deserialize, Serialize};

/// Represents a Seasonal Artifact and all data related to it for the requested Account.
/// It can be combined with Character-scoped data for a full picture of what a character has available/has chosen, or just these settings can be used for overview information.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyArtifactProfileScoped {
    #[serde(rename = "artifactHash")]
    pub artifact_hash: u32,

    #[serde(rename = "pointProgression")]
    pub point_progression: Option<crate::destiny::DestinyProgression>,

    #[serde(rename = "pointsAcquired")]
    pub points_acquired: i32,

    #[serde(rename = "powerBonusProgression")]
    pub power_bonus_progression: Option<crate::destiny::DestinyProgression>,

    #[serde(rename = "powerBonus")]
    pub power_bonus: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyArtifactCharacterScoped {
    #[serde(rename = "artifactHash")]
    pub artifact_hash: u32,

    #[serde(rename = "pointsUsed")]
    pub points_used: i32,

    #[serde(rename = "resetCount")]
    pub reset_count: i32,

    #[serde(rename = "tiers")]
    pub tiers: Option<Vec<crate::destiny::artifacts::DestinyArtifactTier>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyArtifactTier {
    #[serde(rename = "tierHash")]
    pub tier_hash: u32,

    #[serde(rename = "isUnlocked")]
    pub is_unlocked: bool,

    #[serde(rename = "pointsToUnlock")]
    pub points_to_unlock: i32,

    #[serde(rename = "items")]
    pub items: Option<Vec<crate::destiny::artifacts::DestinyArtifactTierItem>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyArtifactTierItem {
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    #[serde(rename = "isActive")]
    pub is_active: bool,
}
