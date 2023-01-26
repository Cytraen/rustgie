use serde::{Deserialize, Serialize};

/// Represents types of Energy that can be used for costs and payments related to Armor 2.0 mods.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyEnergyTypeDefinition {
    /// The description of the energy type, icon etc...
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// A variant of the icon that is transparent and colorless.
    #[serde(rename = "transparentIconPath")]
    pub transparent_icon_path: Option<String>,

    /// If TRUE, the game shows this Energy type's icon. Otherwise, it doesn't. Whether you show it or not is up to you.
    #[serde(rename = "showIcon")]
    pub show_icon: bool,

    /// We have an enumeration for Energy types for quick reference. This is the current definition's Energy type enum value.
    #[serde(rename = "enumValue")]
    pub enum_value: crate::destiny::DestinyEnergyType,

    /// If this Energy Type can be used for determining the Type of Energy that an item can consume, this is the hash for the DestinyInvestmentStatDefinition that represents the stat which holds the Capacity for that energy type. (Note that this is optional because "Any" is a valid cost, but not valid for Capacity - an Armor must have a specific Energy Type for determining the energy type that the Armor is restricted to use)
    #[serde(rename = "capacityStatHash")]
    pub capacity_stat_hash: Option<u32>,

    /// If this Energy Type can be used as a cost to pay for socketing Armor 2.0 items, this is the hash for the DestinyInvestmentStatDefinition that stores the plug's raw cost.
    #[serde(rename = "costStatHash")]
    pub cost_stat_hash: u32,

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
