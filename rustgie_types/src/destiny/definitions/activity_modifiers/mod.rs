use serde::{Deserialize, Serialize};

/// Modifiers - in Destiny 1, these were referred to as "Skulls" - are changes that can be applied to an Activity.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivityModifierDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "displayInNavMode")]
    pub display_in_nav_mode: bool,

    #[serde(rename = "displayInActivitySelection")]
    pub display_in_activity_selection: bool,

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
