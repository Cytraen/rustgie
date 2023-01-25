use serde::{Deserialize, Serialize};

/// Represents the public-facing status of an activity: any data about what is currently active in the Activity, regardless of an individual character's progress in it.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPublicActivityStatus {
    /// Active Challenges for the activity, if any - represented as hashes for DestinyObjectiveDefinitions.
    #[serde(rename = "challengeObjectiveHashes")]
    pub challenge_objective_hashes: Option<Vec<u32>>,

    /// The active modifiers on this activity, if any - represented as hashes for DestinyActivityModifierDefinitions.
    #[serde(rename = "modifierHashes")]
    pub modifier_hashes: Option<Vec<u32>>,

    /// If the activity itself provides any specific "mock" rewards, this will be the items and their quantity.
    /// Why "mock", you ask? Because these are the rewards as they are represented in the tooltip of the Activity.
    /// These are often pointers to fake items that look good in a tooltip, but represent an abstract concept of what you will get for a reward rather than the specific items you may obtain.
    #[serde(rename = "rewardTooltipItems")]
    pub reward_tooltip_items: Option<Vec<crate::destiny::DestinyItemQuantity>>,
}
