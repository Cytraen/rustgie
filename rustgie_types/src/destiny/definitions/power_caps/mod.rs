use serde::{Deserialize, Serialize};

/// Defines a 'power cap' (limit) for gear items, based on the rarity tier and season of release.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyPowerCapDefinition {
    /// The raw value for a power cap.
    #[serde(rename = "powerCap")]
    pub power_cap: i32,

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
