use serde::{Deserialize, Serialize};

/// Defines the tier type of an item. Mostly this provides human readable properties for types like Common, Rare, etc...
/// It also provides some base data for infusion that could be useful.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemTierTypeDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If this tier defines infusion properties, they will be contained here.
    #[serde(rename = "infusionProcess")]
    pub infusion_process: Option<crate::destiny::definitions::items::DestinyItemTierTypeInfusionBlock>,

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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemTierTypeInfusionBlock {
    /// The default portion of quality that will transfer from the infuser to the infusee item. (InfuserQuality - InfuseeQuality) * baseQualityTransferRatio = base quality transferred.
    #[serde(rename = "baseQualityTransferRatio")]
    pub base_quality_transfer_ratio: f32,

    /// As long as InfuserQuality > InfuseeQuality, the amount of quality bestowed is guaranteed to be at least this value, even if the transferRatio would dictate that it should be less. The total amount of quality that ends up in the Infusee cannot exceed the Infuser's quality however (for instance, if you infuse a 300 item with a 301 item and the minimum quality increment is 10, the infused item will not end up with 310 quality)
    #[serde(rename = "minimumQualityIncrement")]
    pub minimum_quality_increment: i32,
}

/// A shortcut for the fact that some items have a "Preview Vendor" - See DestinyInventoryItemDefinition.preview.previewVendorHash - that is intended to be used to show what items you can get as a result of acquiring or using this item.
/// A common example of this in Destiny 1 was Eververse "Boxes," which could have many possible items. This "Preview Vendor" is not a vendor you can actually see in the game, but it defines categories and sale items for all of the possible items you could get from the Box so that the game can show them to you. We summarize that info here so that you don't have to do that Vendor lookup and aggregation manually.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyDerivedItemCategoryDefinition {
    /// The localized string for the category title. This will be something describing the items you can get as a group, or your likelihood/the quantity you'll get.
    #[serde(rename = "categoryDescription")]
    pub category_description: Option<String>,

    /// This is the list of all of the items for this category and the basic properties we'll know about them.
    #[serde(rename = "items")]
    pub items: Option<Vec<crate::destiny::definitions::items::DestinyDerivedItemDefinition>>,
}

/// This is a reference to, and summary data for, a specific item that you can get as a result of Using or Acquiring some other Item (For example, this could be summary information for an Emote that you can get by opening an an Eververse Box) See DestinyDerivedItemCategoryDefinition for more information.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyDerivedItemDefinition {
    /// The hash for the DestinyInventoryItemDefinition of this derived item, if there is one. Sometimes we are given this information as a manual override, in which case there won't be an actual DestinyInventoryItemDefinition for what we display, but you can still show the strings from this object itself.
    #[serde(rename = "itemHash")]
    pub item_hash: Option<u32>,

    /// The name of the derived item.
    #[serde(rename = "itemName")]
    pub item_name: Option<String>,

    /// Additional details about the derived item, in addition to the description.
    #[serde(rename = "itemDetail")]
    pub item_detail: Option<String>,

    /// A brief description of the item.
    #[serde(rename = "itemDescription")]
    pub item_description: Option<String>,

    /// An icon for the item.
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,

    /// If the item was derived from a "Preview Vendor", this will be an index into the DestinyVendorDefinition's itemList property. Otherwise, -1.
    #[serde(rename = "vendorItemIndex")]
    pub vendor_item_index: i32,
}

/// If an item is a Plug, its DestinyInventoryItemDefinition.plug property will be populated with an instance of one of these bad boys.
/// This gives information about when it can be inserted, what the plug's category is (and thus whether it is compatible with a socket... see DestinySocketTypeDefinition for information about Plug Categories and socket compatibility), whether it is enabled and other Plug info.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemPlugDefinition {
    /// The rules around when this plug can be inserted into a socket, aside from the socket's individual restrictions.
    /// The live data DestinyItemPlugComponent.insertFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
    #[serde(rename = "insertionRules")]
    pub insertion_rules: Option<Vec<crate::destiny::definitions::items::DestinyPlugRuleDefinition>>,

    /// The string identifier for the plug's category. Use the socket's DestinySocketTypeDefinition.plugWhitelist to determine whether this plug can be inserted into the socket.
    #[serde(rename = "plugCategoryIdentifier")]
    pub plug_category_identifier: Option<String>,

    /// The hash for the plugCategoryIdentifier. You can use this instead if you wish: I put both in the definition for debugging purposes.
    #[serde(rename = "plugCategoryHash")]
    pub plug_category_hash: u32,

    /// If you successfully socket the item, this will determine whether or not you get "refunded" on the plug.
    #[serde(rename = "onActionRecreateSelf")]
    pub on_action_recreate_self: bool,

    /// If inserting this plug requires materials, this is the hash identifier for looking up the DestinyMaterialRequirementSetDefinition for those requirements.
    #[serde(rename = "insertionMaterialRequirementHash")]
    pub insertion_material_requirement_hash: u32,

    /// In the game, if you're inspecting a plug item directly, this will be the item shown with the plug attached. Look up the DestinyInventoryItemDefinition for this hash for the item.
    #[serde(rename = "previewItemOverrideHash")]
    pub preview_item_override_hash: u32,

    /// It's not enough for the plug to be inserted. It has to be enabled as well. For it to be enabled, it may require materials. This is the hash identifier for the DestinyMaterialRequirementSetDefinition for those requirements, if there is one.
    #[serde(rename = "enabledMaterialRequirementHash")]
    pub enabled_material_requirement_hash: u32,

    /// The rules around whether the plug, once inserted, is enabled and providing its benefits.
    /// The live data DestinyItemPlugComponent.enableFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
    #[serde(rename = "enabledRules")]
    pub enabled_rules: Option<Vec<crate::destiny::definitions::items::DestinyPlugRuleDefinition>>,

    /// Plugs can have arbitrary, UI-defined identifiers that the UI designers use to determine the style applied to plugs. Unfortunately, we have neither a definitive list of these labels nor advance warning of when new labels might be applied or how that relates to how they get rendered. If you want to, you can refer to known labels to change your own styles: but know that new ones can be created arbitrarily, and we have no way of associating the labels with any specific UI style guidance... you'll have to piece that together on your end. Or do what we do, and just show plugs more generically, without specialized styles.
    #[serde(rename = "uiPlugLabel")]
    pub ui_plug_label: Option<String>,

    #[serde(rename = "plugStyle")]
    pub plug_style: enumflags2::BitFlags<crate::destiny::PlugUiStyles>,

    /// Indicates the rules about when this plug can be used. See the PlugAvailabilityMode enumeration for more information!
    #[serde(rename = "plugAvailability")]
    pub plug_availability: crate::destiny::PlugAvailabilityMode,

    /// If the plug meets certain state requirements, it may have an alternative label applied to it. This is the alternative label that will be applied in such a situation.
    #[serde(rename = "alternateUiPlugLabel")]
    pub alternate_ui_plug_label: Option<String>,

    /// The alternate plug of the plug: only applies when the item is in states that only the server can know about and control, unfortunately. See AlternateUiPlugLabel for the related label info.
    #[serde(rename = "alternatePlugStyle")]
    pub alternate_plug_style: enumflags2::BitFlags<crate::destiny::PlugUiStyles>,

    /// If TRUE, this plug is used for UI display purposes only, and doesn't have any interesting effects of its own.
    #[serde(rename = "isDummyPlug")]
    pub is_dummy_plug: bool,

    /// Do you ever get the feeling that a system has become so overburdened by edge cases that it probably should have become some other system entirely? So do I!
    /// In totally unrelated news, Plugs can now override properties of their parent items. This is some of the relevant definition data for those overrides.
    /// If this is populated, it will have the override data to be applied when this plug is applied to an item.
    #[serde(rename = "parentItemOverride")]
    pub parent_item_override: Option<crate::destiny::definitions::items::DestinyParentItemOverride>,

    /// IF not null, this plug provides Energy capacity to the item in which it is socketed. In Armor 2.0 for example, is implemented in a similar way to Masterworks, where visually it's a single area of the UI being clicked on to "Upgrade" to higher energy levels, but it's actually socketing new plugs.
    #[serde(rename = "energyCapacity")]
    pub energy_capacity: Option<crate::destiny::definitions::items::DestinyEnergyCapacityEntry>,

    /// IF not null, this plug has an energy cost. This contains the details of that cost.
    #[serde(rename = "energyCost")]
    pub energy_cost: Option<crate::destiny::definitions::items::DestinyEnergyCostEntry>,
}

/// Dictates a rule around whether the plug is enabled or insertable.
/// In practice, the live Destiny data will refer to these entries by index. You can then look up that index in the appropriate property (enabledRules or insertionRules) to get the localized string for the failure message if it failed.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyPlugRuleDefinition {
    /// The localized string to show if this rule fails.
    #[serde(rename = "failureMessage")]
    pub failure_message: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyParentItemOverride {
    #[serde(rename = "additionalEquipRequirementsDisplayStrings")]
    pub additional_equip_requirements_display_strings: Option<Vec<String>>,

    #[serde(rename = "pipIcon")]
    pub pip_icon: Option<String>,
}

/// Items can have Energy Capacity, and plugs can provide that capacity such as on a piece of Armor in Armor 2.0. This is how much "Energy" can be spent on activating plugs for this item.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyEnergyCapacityEntry {
    /// How much energy capacity this plug provides.
    #[serde(rename = "capacityValue")]
    pub capacity_value: i32,

    /// Energy provided by a plug is always of a specific type - this is the hash identifier for the energy type for which it provides Capacity.
    #[serde(rename = "energyTypeHash")]
    pub energy_type_hash: u32,

    /// The Energy Type for this energy capacity, in enum form for easy use.
    #[serde(rename = "energyType")]
    pub energy_type: crate::destiny::DestinyEnergyType,
}

/// Some plugs cost Energy, which is a stat on the item that can be increased by other plugs (that, at least in Armor 2.0, have a "masterworks-like" mechanic for upgrading). If a plug has costs, the details of that cost are defined here.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyEnergyCostEntry {
    /// The Energy cost for inserting this plug.
    #[serde(rename = "energyCost")]
    pub energy_cost: i32,

    /// The type of energy that this plug costs, as a reference to the DestinyEnergyTypeDefinition of the energy type.
    #[serde(rename = "energyTypeHash")]
    pub energy_type_hash: u32,

    /// The type of energy that this plug costs, in enum form.
    #[serde(rename = "energyType")]
    pub energy_type: crate::destiny::DestinyEnergyType,
}
