use serde::{Deserialize, Serialize};

/// Represents known info about a Destiny Artifact.
/// We cannot guarantee that artifact definitions will be immutable between seasons - in fact, we've been told that they will be replaced between seasons. But this definition is built both to minimize the amount of lookups for related data that have to occur, and is built in hope that, if this plan changes, we will be able to accommodate it more easily.
#[derive(Deserialize, Serialize)]
pub struct DestinyArtifactDefinition {
    /// Any basic display info we know about the Artifact. Currently sourced from a related inventory item, but the source of this data is subject to change.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// Any Geometry/3D info we know about the Artifact. Currently sourced from a related inventory item's gearset information, but the source of this data is subject to change.
    #[serde(rename = "translationBlock")]
    pub translation_block: Option<crate::destiny::definitions::DestinyItemTranslationBlockDefinition>,

    /// Any Tier/Rank data related to this artifact, listed in display order.  Currently sourced from a Vendor, but this source is subject to change.
    #[serde(rename = "tiers")]
    pub tiers: Option<Vec<crate::destiny::definitions::artifacts::DestinyArtifactTierDefinition>>,

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

#[derive(Deserialize, Serialize)]
pub struct DestinyArtifactTierDefinition {
    /// An identifier, unique within the Artifact, for this specific tier.
    #[serde(rename = "tierHash")]
    pub tier_hash: u32,

    /// The human readable title of this tier, if any.
    #[serde(rename = "displayTitle")]
    pub display_title: Option<String>,

    /// A string representing the localized minimum requirement text for this Tier, if any.
    #[serde(rename = "progressRequirementMessage")]
    pub progress_requirement_message: Option<String>,

    /// The items that can be earned within this tier.
    #[serde(rename = "items")]
    pub items: Option<Vec<crate::destiny::definitions::artifacts::DestinyArtifactTierItemDefinition>>,

    /// The minimum number of "unlock points" that you must have used before you can unlock items from this tier.
    #[serde(rename = "minimumUnlockPointsUsedRequirement")]
    pub minimum_unlock_points_used_requirement: i32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyArtifactTierItemDefinition {
    /// The identifier of the Plug Item unlocked by activating this item in the Artifact.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,
}
