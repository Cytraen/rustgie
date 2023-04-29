pub mod activity_modifiers;
pub mod animations;
pub mod artifacts;
pub mod breaker_types;
pub mod checklists;
pub mod collectibles;
pub mod common;
pub mod director;
pub mod energy_types;
pub mod guardian_ranks;
pub mod items;
pub mod loadouts;
pub mod lore;
pub mod metrics;
pub mod milestones;
pub mod power_caps;
pub mod presentation;
pub mod progression;
pub mod records;
pub mod reporting;
pub mod seasons;
pub mod social;
pub mod sockets;
pub mod sources;
pub mod traits;
pub mod vendors;

use anyhow::{anyhow, Result};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Provides common properties for destiny definitions.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyDefinition {
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

/// A "Progression" in Destiny is best explained by an example.
/// A Character's "Level" is a progression: it has Experience that can be earned, levels that can be gained, and is evaluated and displayed at various points in the game. A Character's "Faction Reputation" is also a progression for much the same reason.
/// Progression is used by a variety of systems, and the definition of a Progression will generally only be useful if combining with live data (such as a character's DestinyCharacterProgressionComponent.progressions property, which holds that character's live Progression states).
/// Fundamentally, a Progression measures your "Level" by evaluating the thresholds in its Steps (one step per level, except for the last step which can be repeated indefinitely for "Levels" that have no ceiling) against the total earned "progression points"/experience. (for simplicity purposes, we will henceforth refer to earned progression points as experience, though it need not be a mechanic that in any way resembles Experience in a traditional sense).
/// Earned experience is calculated in a variety of ways, determined by the Progression's scope. These go from looking up a stored value to performing exceedingly obtuse calculations. This is why we provide live data in DestinyCharacterProgressionComponent.progressions, so you don't have to worry about those.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProgressionDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::DestinyProgressionDisplayPropertiesDefinition>,

    /// The "Scope" of the progression indicates the source of the progression's live data.
    /// See the DestinyProgressionScope enum for more info: but essentially, a Progression can either be backed by a stored value, or it can be a calculated derivative of other values.
    #[serde(rename = "scope")]
    pub scope: crate::destiny::DestinyProgressionScope,

    /// If this is True, then the progression doesn't have a maximum level.
    #[serde(rename = "repeatLastStep")]
    pub repeat_last_step: bool,

    /// If there's a description of how to earn this progression in the local config, this will be that localized description.
    #[serde(rename = "source")]
    pub source: Option<String>,

    /// Progressions are divided into Steps, which roughly equate to "Levels" in the traditional sense of a Progression. Notably, the last step can be repeated indefinitely if repeatLastStep is true, meaning that the calculation for your level is not as simple as comparing your current progress to the max progress of the steps.
    /// These and more calculations are done for you if you grab live character progression data, such as in the DestinyCharacterProgressionComponent.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::destiny::definitions::DestinyProgressionStepDefinition>>,

    /// If true, the Progression is something worth showing to users.
    /// If false, BNet isn't going to show it. But that doesn't mean you can't. We're all friends here.
    #[serde(rename = "visible")]
    pub visible: bool,

    /// If the value exists, this is the hash identifier for the Faction that owns this Progression.
    /// This is purely for convenience, if you're looking at a progression and want to know if and who it's related to in terms of Faction Reputation.
    #[serde(rename = "factionHash")]
    pub faction_hash: Option<u32>,

    /// The #RGB string value for the color related to this progression, if there is one.
    #[serde(rename = "color")]
    pub color: Option<crate::destiny::misc::DestinyColor>,

    /// For progressions that have it, this is the rank icon we use in the Companion, displayed above the progressions' rank value.
    #[serde(rename = "rankIcon")]
    pub rank_icon: Option<String>,

    #[serde(rename = "rewardItems")]
    pub reward_items: Option<Vec<crate::destiny::definitions::DestinyProgressionRewardItemQuantity>>,

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
pub struct DestinyProgressionDisplayPropertiesDefinition {
    /// When progressions show your "experience" gained, that bar has units (i.e. "Experience", "Bad Dudes Snuffed Out", whatever). This is the localized string for that unit of measurement.
    #[serde(rename = "displayUnitsName")]
    pub display_units_name: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Note that "icon" is sometimes misleading, and should be interpreted in the context of the entity. For instance, in Destiny 1 the DestinyRecordBookDefinition's icon was a big picture of a book.
    /// But usually, it will be a small square image that you can use as... well, an icon.
    /// They are currently represented as 96px x 96px images.
    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "iconSequences")]
    pub icon_sequences: Option<Vec<crate::destiny::definitions::common::DestinyIconSequenceDefinition>>,

    /// If this item has a high-res icon (at least for now, many things won't), then the path to that icon will be here.
    #[serde(rename = "highResIcon")]
    pub high_res_icon: Option<String>,

    #[serde(rename = "hasIcon")]
    pub has_icon: bool,
}

/// This defines a single Step in a progression (which roughly equates to a level. See DestinyProgressionDefinition for caveats).
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProgressionStepDefinition {
    /// Very rarely, Progressions will have localized text describing the Level of the progression. This will be that localized text, if it exists. Otherwise, the standard appears to be to simply show the level numerically.
    #[serde(rename = "stepName")]
    pub step_name: Option<String>,

    /// This appears to be, when you "level up", whether a visual effect will display and on what entity. See DestinyProgressionStepDisplayEffect for slightly more info.
    #[serde(rename = "displayEffectType")]
    pub display_effect_type: crate::destiny::DestinyProgressionStepDisplayEffect,

    /// The total amount of progression points/"experience" you will need to initially reach this step. If this is the last step and the progression is repeating indefinitely (DestinyProgressionDefinition.repeatLastStep), this will also be the progress needed to level it up further by repeating this step again.
    #[serde(rename = "progressTotal")]
    pub progress_total: i32,

    /// A listing of items rewarded as a result of reaching this level.
    #[serde(rename = "rewardItems")]
    pub reward_items: Option<Vec<crate::destiny::DestinyItemQuantity>>,

    /// If this progression step has a specific icon related to it, this is the icon to show.
    #[serde(rename = "icon")]
    pub icon: Option<String>,
}

/// So much of what you see in Destiny is actually an Item used in a new and creative way. This is the definition for Items in Destiny, which started off as just entities that could exist in your Inventory but ended up being the backing data for so much more: quests, reward previews, slots, and subclasses.
/// In practice, you will want to associate this data with "live" item data from a Bungie.Net Platform call: these definitions describe the item in generic, non-instanced terms: but an actual instance of an item can vary widely from these generic definitions.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyInventoryItemDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// Tooltips that only come up conditionally for the item. Check the live data DestinyItemComponent.tooltipNotificationIndexes property for which of these should be shown at runtime.
    #[serde(rename = "tooltipNotifications")]
    pub tooltip_notifications: Option<Vec<crate::destiny::definitions::DestinyItemTooltipNotification>>,

    /// If this item has a collectible related to it, this is the hash identifier of that collectible entry.
    #[serde(rename = "collectibleHash")]
    pub collectible_hash: Option<u32>,

    /// If available, this is the original 'active' release watermark overlay for the icon. If the item has different versions, this can be overridden by the 'display version watermark icon' from the 'quality' block. Alternatively, if there is no watermark for the version, and the item version has a power cap below the current season power cap, this can be overridden by the iconWatermarkShelved property.
    #[serde(rename = "iconWatermark")]
    pub icon_watermark: Option<String>,

    /// If available, this is the 'shelved' release watermark overlay for the icon. If the item version has a power cap below the current season power cap, it can be treated as 'shelved', and should be shown with this 'shelved' watermark overlay.
    #[serde(rename = "iconWatermarkShelved")]
    pub icon_watermark_shelved: Option<String>,

    /// A secondary icon associated with the item. Currently this is used in very context specific applications, such as Emblem Nameplates.
    #[serde(rename = "secondaryIcon")]
    pub secondary_icon: Option<String>,

    /// Pulled from the secondary icon, this is the "secondary background" of the secondary icon. Confusing? Sure, that's why I call it "overlay" here: because as far as it's been used thus far, it has been for an optional overlay image. We'll see if that holds up, but at least for now it explains what this image is a bit better.
    #[serde(rename = "secondaryOverlay")]
    pub secondary_overlay: Option<String>,

    /// Pulled from the Secondary Icon, this is the "special" background for the item. For Emblems, this is the background image used on the Details view: but it need not be limited to that for other types of items.
    #[serde(rename = "secondarySpecial")]
    pub secondary_special: Option<String>,

    /// Sometimes, an item will have a background color. Most notably this occurs with Emblems, who use the Background Color for small character nameplates such as the "friends" view you see in-game. There are almost certainly other items that have background color as well, though I have not bothered to investigate what items have it nor what purposes they serve: use it as you will.
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<crate::destiny::misc::DestinyColor>,

    /// If we were able to acquire an in-game screenshot for the item, the path to that screenshot will be returned here. Note that not all items have screenshots: particularly not any non-equippable items.
    #[serde(rename = "screenshot")]
    pub screenshot: Option<String>,

    /// The localized title/name of the item's type. This can be whatever the designers want, and has no guarantee of consistency between items.
    #[serde(rename = "itemTypeDisplayName")]
    pub item_type_display_name: Option<String>,

    #[serde(rename = "flavorText")]
    pub flavor_text: Option<String>,

    /// A string identifier that the game's UI uses to determine how the item should be rendered in inventory screens and the like. This could really be anything - at the moment, we don't have the time to really breakdown and maintain all the possible strings this could be, partly because new ones could be added ad hoc. But if you want to use it to dictate your own UI, or look for items with a certain display style, go for it!
    #[serde(rename = "uiItemDisplayStyle")]
    pub ui_item_display_style: Option<String>,

    /// It became a common enough pattern in our UI to show Item Type and Tier combined into a single localized string that I'm just going to go ahead and start pre-creating these for items.
    #[serde(rename = "itemTypeAndTierDisplayName")]
    pub item_type_and_tier_display_name: Option<String>,

    /// In theory, it is a localized string telling you about how you can find the item. I really wish this was more consistent. Many times, it has nothing. Sometimes, it's instead a more narrative-forward description of the item. Which is cool, and I wish all properties had that data, but it should really be its own property.
    #[serde(rename = "displaySource")]
    pub display_source: Option<String>,

    /// An identifier that the game UI uses to determine what type of tooltip to show for the item. These have no corresponding definitions that BNet can link to: so it'll be up to you to interpret and display your UI differently according to these styles (or ignore it).
    #[serde(rename = "tooltipStyle")]
    pub tooltip_style: Option<String>,

    /// If the item can be "used", this block will be non-null, and will have data related to the action performed when using the item. (Guess what? 99% of the time, this action is "dismantle". Shocker)
    #[serde(rename = "action")]
    pub action: Option<crate::destiny::definitions::DestinyItemActionBlockDefinition>,

    /// Recipe items will have relevant crafting information available here.
    #[serde(rename = "crafting")]
    pub crafting: Option<crate::destiny::definitions::DestinyItemCraftingBlockDefinition>,

    /// If this item can exist in an inventory, this block will be non-null. In practice, every item that currently exists has one of these blocks. But note that it is not necessarily guaranteed.
    #[serde(rename = "inventory")]
    pub inventory: Option<crate::destiny::definitions::DestinyItemInventoryBlockDefinition>,

    /// If this item is a quest, this block will be non-null. In practice, I wish I had called this the Quest block, but at the time it wasn't clear to me whether it would end up being used for purposes other than quests. It will contain data about the steps in the quest, and mechanics we can use for displaying and tracking the quest.
    #[serde(rename = "setData")]
    pub set_data: Option<crate::destiny::definitions::DestinyItemSetBlockDefinition>,

    /// If this item can have stats (such as a weapon, armor, or vehicle), this block will be non-null and populated with the stats found on the item.
    #[serde(rename = "stats")]
    pub stats: Option<crate::destiny::definitions::DestinyItemStatBlockDefinition>,

    /// If the item is an emblem that has a special Objective attached to it - for instance, if the emblem tracks PVP Kills, or what-have-you. This is a bit different from, for example, the Vanguard Kill Tracker mod, which pipes data into the "art channel". When I get some time, I would like to standardize these so you can get at the values they expose without having to care about what they're being used for and how they are wired up, but for now here's the raw data.
    #[serde(rename = "emblemObjectiveHash")]
    pub emblem_objective_hash: Option<u32>,

    /// If this item can be equipped, this block will be non-null and will be populated with the conditions under which it can be equipped.
    #[serde(rename = "equippingBlock")]
    pub equipping_block: Option<crate::destiny::definitions::DestinyEquippingBlockDefinition>,

    /// If this item can be rendered, this block will be non-null and will be populated with rendering information.
    #[serde(rename = "translationBlock")]
    pub translation_block: Option<crate::destiny::definitions::DestinyItemTranslationBlockDefinition>,

    /// If this item can be Used or Acquired to gain other items (for instance, how Eververse Boxes can be consumed to get items from the box), this block will be non-null and will give summary information for the items that can be acquired.
    #[serde(rename = "preview")]
    pub preview: Option<crate::destiny::definitions::DestinyItemPreviewBlockDefinition>,

    /// If this item can have a level or stats, this block will be non-null and will be populated with default quality (item level, "quality", and infusion) data. See the block for more details, there's often less upfront information in D2 so you'll want to be aware of how you use quality and item level on the definition level now.
    #[serde(rename = "quality")]
    pub quality: Option<crate::destiny::definitions::DestinyItemQualityBlockDefinition>,

    /// The conceptual "Value" of an item, if any was defined. See the DestinyItemValueBlockDefinition for more details.
    #[serde(rename = "value")]
    pub value: Option<crate::destiny::definitions::DestinyItemValueBlockDefinition>,

    /// If this item has a known source, this block will be non-null and populated with source information. Unfortunately, at this time we are not generating sources: that is some aggressively manual work which we didn't have time for, and I'm hoping to get back to at some point in the future.
    #[serde(rename = "sourceData")]
    pub source_data: Option<crate::destiny::definitions::DestinyItemSourceBlockDefinition>,

    /// If this item has Objectives (extra tasks that can be accomplished related to the item... most frequently when the item is a Quest Step and the Objectives need to be completed to move on to the next Quest Step), this block will be non-null and the objectives defined herein.
    #[serde(rename = "objectives")]
    pub objectives: Option<crate::destiny::definitions::DestinyItemObjectiveBlockDefinition>,

    /// If this item has available metrics to be shown, this block will be non-null have the appropriate hashes defined.
    #[serde(rename = "metrics")]
    pub metrics: Option<crate::destiny::definitions::DestinyItemMetricBlockDefinition>,

    /// If this item *is* a Plug, this will be non-null and the info defined herein. See DestinyItemPlugDefinition for more information.
    #[serde(rename = "plug")]
    pub plug: Option<crate::destiny::definitions::items::DestinyItemPlugDefinition>,

    /// If this item has related items in a "Gear Set", this will be non-null and the relationships defined herein.
    #[serde(rename = "gearset")]
    pub gearset: Option<crate::destiny::definitions::DestinyItemGearsetBlockDefinition>,

    /// If this item is a "reward sack" that can be opened to provide other items, this will be non-null and the properties of the sack contained herein.
    #[serde(rename = "sack")]
    pub sack: Option<crate::destiny::definitions::DestinyItemSackBlockDefinition>,

    /// If this item has any Sockets, this will be non-null and the individual sockets on the item will be defined herein.
    #[serde(rename = "sockets")]
    pub sockets: Option<crate::destiny::definitions::DestinyItemSocketBlockDefinition>,

    /// Summary data about the item.
    #[serde(rename = "summary")]
    pub summary: Option<crate::destiny::definitions::DestinyItemSummaryBlockDefinition>,

    /// If the item has a Talent Grid, this will be non-null and the properties of the grid defined herein. Note that, while many items still have talent grids, the only ones with meaningful Nodes still on them will be Subclass/"Build" items.
    #[serde(rename = "talentGrid")]
    pub talent_grid: Option<crate::destiny::definitions::DestinyItemTalentGridBlockDefinition>,

    /// If the item has stats, this block will be defined. It has the "raw" investment stats for the item. These investment stats don't take into account the ways that the items can spawn, nor do they take into account any Stat Group transformations. I have retained them for debugging purposes, but I do not know how useful people will find them.
    #[serde(rename = "investmentStats")]
    pub investment_stats: Option<Vec<crate::destiny::definitions::DestinyItemInvestmentStatDefinition>>,

    /// If the item has any *intrinsic* Perks (Perks that it will provide regardless of Sockets, Talent Grid, and other transitory state), they will be defined here.
    #[serde(rename = "perks")]
    pub perks: Option<Vec<crate::destiny::definitions::DestinyItemPerkEntryDefinition>>,

    /// If the item has any related Lore (DestinyLoreDefinition), this will be the hash identifier you can use to look up the lore definition.
    #[serde(rename = "loreHash")]
    pub lore_hash: Option<u32>,

    /// There are times when the game will show you a "summary/vague" version of an item - such as a description of its type represented as a DestinyInventoryItemDefinition - rather than display the item itself.
    /// This happens sometimes when summarizing possible rewards in a tooltip. This is the item displayed instead, if it exists.
    #[serde(rename = "summaryItemHash")]
    pub summary_item_hash: Option<u32>,

    /// If any animations were extracted from game content for this item, these will be the definitions of those animations.
    #[serde(rename = "animations")]
    pub animations: Option<Vec<crate::destiny::definitions::animations::DestinyAnimationReference>>,

    /// BNet may forbid the execution of actions on this item via the API. If that is occurring, allowActions will be set to false.
    #[serde(rename = "allowActions")]
    pub allow_actions: bool,

    /// If we added any help or informational URLs about this item, these will be those links.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::links::HyperlinkReference>>,

    /// The boolean will indicate to us (and you!) whether something *could* happen when you transfer this item from the Postmaster that might be considered a "destructive" action.
    /// It is not feasible currently to tell you (or ourelves!) in a consistent way whether this *will* actually cause a destructive action, so we are playing it safe: if it has the potential to do so, we will not allow it to be transferred from the Postmaster by default. You will need to check for this flag before transferring an item from the Postmaster, or else you'll end up receiving an error.
    #[serde(rename = "doesPostmasterPullHaveSideEffects")]
    pub does_postmaster_pull_have_side_effects: bool,

    /// The intrinsic transferability of an item.
    /// I hate that this boolean is negative - but there's a reason.
    /// Just because an item is intrinsically transferrable doesn't mean that it can be transferred, and we don't want to imply that this is the only source of that transferability.
    #[serde(rename = "nonTransferrable")]
    pub non_transferrable: bool,

    /// BNet attempts to make a more formal definition of item "Categories", as defined by DestinyItemCategoryDefinition. This is a list of all Categories that we were able to algorithmically determine that this item is a member of. (for instance, that it's a "Weapon", that it's an "Auto Rifle", etc...)
    /// The algorithm for these is, unfortunately, volatile. If you believe you see a miscategorized item, please let us know on the Bungie API forums.
    #[serde(rename = "itemCategoryHashes")]
    pub item_category_hashes: Option<Vec<u32>>,

    /// In Destiny 1, we identified some items as having particular categories that we'd like to know about for various internal logic purposes. These are defined in SpecialItemType, and while these days the itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    #[serde(rename = "specialItemType")]
    pub special_item_type: crate::destiny::SpecialItemType,

    /// A value indicating the "base" the of the item. This enum is a useful but dramatic oversimplification of what it means for an item to have a "Type". Still, it's handy in many situations.
    /// itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    #[serde(rename = "itemType")]
    pub item_type: crate::destiny::DestinyItemType,

    /// A value indicating the "sub-type" of the item. For instance, where an item might have an itemType value "Weapon", this will be something more specific like "Auto Rifle".
    /// itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    #[serde(rename = "itemSubType")]
    pub item_sub_type: crate::destiny::DestinyItemSubType,

    /// We run a similarly weak-sauce algorithm to try and determine whether an item is restricted to a specific class. If we find it to be restricted in such a way, we set this classType property to match the class' enumeration value so that users can easily identify class restricted items.
    /// If you see a mis-classed item, please inform the developers in the Bungie API forum.
    #[serde(rename = "classType")]
    pub class_type: crate::destiny::DestinyClass,

    /// Some weapons and plugs can have a "Breaker Type": a special ability that works sort of like damage type vulnerabilities. This is (almost?) always set on items by plugs.
    #[serde(rename = "breakerType")]
    pub breaker_type: crate::destiny::DestinyBreakerType,

    /// Since we also have a breaker type definition, this is the hash for that breaker type for your convenience. Whether you use the enum or hash and look up the definition depends on what's cleanest for your code.
    #[serde(rename = "breakerTypeHash")]
    pub breaker_type_hash: Option<u32>,

    /// If true, then you will be allowed to equip the item if you pass its other requirements.
    /// This being false means that you cannot equip the item under any circumstances.
    #[serde(rename = "equippable")]
    pub equippable: bool,

    /// Theoretically, an item can have many possible damage types. In *practice*, this is not true, but just in case weapons start being made that have multiple (for instance, an item where a socket has reusable plugs for every possible damage type that you can choose from freely), this field will return all of the possible damage types that are available to the weapon by default.
    #[serde(rename = "damageTypeHashes")]
    pub damage_type_hashes: Option<Vec<u32>>,

    /// This is the list of all damage types that we know ahead of time the item can take on. Unfortunately, this does not preclude the possibility of something funky happening to give the item a damage type that cannot be predicted beforehand: for example, if some designer decides to create arbitrary non-reusable plugs that cause damage type to change.
    /// This damage type prediction will only use the following to determine potential damage types:
    /// - Intrinsic perks
    /// - Talent Node perks
    /// - Known, reusable plugs for sockets
    #[serde(rename = "damageTypes")]
    pub damage_types: Option<Vec<crate::destiny::DamageType>>,

    /// If the item has a damage type that could be considered to be default, it will be populated here.
    /// For various upsetting reasons, it's surprisingly cumbersome to figure this out. I hope you're happy.
    #[serde(rename = "defaultDamageType")]
    pub default_damage_type: crate::destiny::DamageType,

    /// Similar to defaultDamageType, but represented as the hash identifier for a DestinyDamageTypeDefinition.
    /// I will likely regret leaving in the enumeration versions of these properties, but for now they're very convenient.
    #[serde(rename = "defaultDamageTypeHash")]
    pub default_damage_type_hash: Option<u32>,

    /// If this item is related directly to a Season of Destiny, this is the hash identifier for that season.
    #[serde(rename = "seasonHash")]
    pub season_hash: Option<u32>,

    /// If true, this is a dummy vendor-wrapped item template. Items purchased from Eververse will be "wrapped" by one of these items so that we can safely provide refund capabilities before the item is "unwrapped".
    #[serde(rename = "isWrapper")]
    pub is_wrapper: bool,

    /// Traits are metadata tags applied to this item. For example: armor slot, weapon type, foundry, faction, etc. These IDs come from the game and don't map to any content, but should still be useful.
    #[serde(rename = "traitIds")]
    pub trait_ids: Option<Vec<String>>,

    /// These are the corresponding trait definition hashes for the entries in traitIds.
    #[serde(rename = "traitHashes")]
    pub trait_hashes: Option<Vec<u32>>,

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
pub struct DestinyItemTooltipNotification {
    #[serde(rename = "displayString")]
    pub display_string: Option<String>,

    #[serde(rename = "displayStyle")]
    pub display_style: Option<String>,
}

/// If an item can have an action performed on it (like "Dismantle"), it will be defined here if you care.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemActionBlockDefinition {
    /// Localized text for the verb of the action being performed.
    #[serde(rename = "verbName")]
    pub verb_name: Option<String>,

    /// Localized text describing the action being performed.
    #[serde(rename = "verbDescription")]
    pub verb_description: Option<String>,

    /// The content has this property, however it's not entirely clear how it is used.
    #[serde(rename = "isPositive")]
    pub is_positive: bool,

    /// If the action has an overlay screen associated with it, this is the name of that screen. Unfortunately, we cannot return the screen's data itself.
    #[serde(rename = "overlayScreenName")]
    pub overlay_screen_name: Option<String>,

    /// The icon associated with the overlay screen for the action, if any.
    #[serde(rename = "overlayIcon")]
    pub overlay_icon: Option<String>,

    /// The number of seconds to delay before allowing this action to be performed again.
    #[serde(rename = "requiredCooldownSeconds")]
    pub required_cooldown_seconds: i32,

    /// If the action requires other items to exist or be destroyed, this is the list of those items and requirements.
    #[serde(rename = "requiredItems")]
    pub required_items: Option<Vec<crate::destiny::definitions::DestinyItemActionRequiredItemDefinition>>,

    /// If performing this action earns you Progression, this is the list of progressions and values granted for those progressions by performing this action.
    #[serde(rename = "progressionRewards")]
    pub progression_rewards: Option<Vec<crate::destiny::definitions::DestinyProgressionRewardDefinition>>,

    /// The internal identifier for the action.
    #[serde(rename = "actionTypeLabel")]
    pub action_type_label: Option<String>,

    /// Theoretically, an item could have a localized string for a hint about the location in which the action should be performed. In practice, no items yet have this property.
    #[serde(rename = "requiredLocation")]
    pub required_location: Option<String>,

    /// The identifier hash for the Cooldown associated with this action. We have not pulled this data yet for you to have more data to use for cooldowns.
    #[serde(rename = "requiredCooldownHash")]
    pub required_cooldown_hash: u32,

    /// If true, the item is deleted when the action completes.
    #[serde(rename = "deleteOnAction")]
    pub delete_on_action: bool,

    /// If true, the entire stack is deleted when the action completes.
    #[serde(rename = "consumeEntireStack")]
    pub consume_entire_stack: bool,

    /// If true, this action will be performed as soon as you earn this item. Some rewards work this way, providing you a single item to pick up from a reward-granting vendor in-game and then immediately consuming itself to provide you multiple items.
    #[serde(rename = "useOnAcquire")]
    pub use_on_acquire: bool,
}

/// The definition of an item and quantity required in a character's inventory in order to perform an action.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemActionRequiredItemDefinition {
    /// The minimum quantity of the item you have to have.
    #[serde(rename = "count")]
    pub count: i32,

    /// The hash identifier of the item you need to have. Use it to look up the DestinyInventoryItemDefinition for more info.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// If true, the item/quantity will be deleted from your inventory when the action is performed. Otherwise, you'll retain these required items after the action is complete.
    #[serde(rename = "deleteOnAction")]
    pub delete_on_action: bool,
}

/// Inventory Items can reward progression when actions are performed on them. A common example of this in Destiny 1 was Bounties, which would reward Experience on your Character and the like when you completed the bounty.
/// Note that this maps to a DestinyProgressionMappingDefinition, and *not* a DestinyProgressionDefinition directly. This is apparently so that multiple progressions can be granted progression points/experience at the same time.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProgressionRewardDefinition {
    /// The hash identifier of the DestinyProgressionMappingDefinition that contains the progressions for which experience should be applied.
    #[serde(rename = "progressionMappingHash")]
    pub progression_mapping_hash: u32,

    /// The amount of experience to give to each of the mapped progressions.
    #[serde(rename = "amount")]
    pub amount: i32,

    /// If true, the game's internal mechanisms to throttle progression should be applied.
    #[serde(rename = "applyThrottles")]
    pub apply_throttles: bool,
}

/// Aggregations of multiple progressions.
/// These are used to apply rewards to multiple progressions at once. They can sometimes have human readable data as well, but only extremely sporadically.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProgressionMappingDefinition {
    /// Infrequently defined in practice. Defer to the individual progressions' display properties.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The localized unit of measurement for progression across the progressions defined in this mapping. Unfortunately, this is very infrequently defined. Defer to the individual progressions' display units.
    #[serde(rename = "displayUnits")]
    pub display_units: Option<String>,

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

/// If an item can have an action performed on it (like "Dismantle"), it will be defined here if you care.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemCraftingBlockDefinition {
    /// A reference to the item definition that is created when crafting with this 'recipe' item.
    #[serde(rename = "outputItemHash")]
    pub output_item_hash: u32,

    /// A list of socket type hashes that describes which sockets are required for crafting with this recipe.
    #[serde(rename = "requiredSocketTypeHashes")]
    pub required_socket_type_hashes: Option<Vec<u32>>,

    #[serde(rename = "failedRequirementStrings")]
    pub failed_requirement_strings: Option<Vec<String>>,

    /// A reference to the base material requirements for crafting with this recipe.
    #[serde(rename = "baseMaterialRequirements")]
    pub base_material_requirements: Option<u32>,

    /// A list of 'bonus' socket plugs that may be available if certain requirements are met.
    #[serde(rename = "bonusPlugs")]
    pub bonus_plugs: Option<Vec<crate::destiny::definitions::DestinyItemCraftingBlockBonusPlugDefinition>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemCraftingBlockBonusPlugDefinition {
    #[serde(rename = "socketTypeHash")]
    pub socket_type_hash: u32,

    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,
}

/// Represent a set of material requirements: Items that either need to be owned or need to be consumed in order to perform an action.
/// A variety of other entities refer to these as gatekeepers and payments for actions that can be performed in game.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyMaterialRequirementSetDefinition {
    /// The list of all materials that are required.
    #[serde(rename = "materials")]
    pub materials: Option<Vec<crate::destiny::definitions::DestinyMaterialRequirement>>,

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

/// Many actions relating to items require you to expend materials: - Activating a talent node - Inserting a plug into a socket The items will refer to material requirements by a materialRequirementsHash in these cases, and this is the definition for those requirements in terms of the item required, how much of it is required and other interesting info. This is one of the rare/strange times where a single contract class is used both in definitions *and* in live data response contracts. I'm not sure yet whether I regret that.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyMaterialRequirement {
    /// The hash identifier of the material required. Use it to look up the material's DestinyInventoryItemDefinition.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// If True, the material will be removed from the character's inventory when the action is performed.
    #[serde(rename = "deleteOnAction")]
    pub delete_on_action: bool,

    /// The amount of the material required.
    #[serde(rename = "count")]
    pub count: i32,

    /// If true, the material requirement count value is constant. Since The Witch Queen expansion, some material requirement counts can be dynamic and will need to be returned with an API call.
    #[serde(rename = "countIsConstant")]
    pub count_is_constant: bool,

    /// If True, this requirement is "silent": don't bother showing it in a material requirements display. I mean, I'm not your mom: I'm not going to tell you you *can't* show it. But we won't show it in our UI.
    #[serde(rename = "omitFromRequirements")]
    pub omit_from_requirements: bool,
}

/// If the item can exist in an inventory - the overwhelming majority of them can and do - then this is the basic properties regarding the item's relationship with the inventory.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemInventoryBlockDefinition {
    /// If this string is populated, you can't have more than one stack with this label in a given inventory. Note that this is different from the equipping block's unique label, which is used for equipping uniqueness.
    #[serde(rename = "stackUniqueLabel")]
    pub stack_unique_label: Option<String>,

    /// The maximum quantity of this item that can exist in a stack.
    #[serde(rename = "maxStackSize")]
    pub max_stack_size: i32,

    /// The hash identifier for the DestinyInventoryBucketDefinition to which this item belongs. I should have named this "bucketHash", but too many things refer to it now. Sigh.
    #[serde(rename = "bucketTypeHash")]
    pub bucket_type_hash: u32,

    /// If the item is picked up by the lost loot queue, this is the hash identifier for the DestinyInventoryBucketDefinition into which it will be placed. Again, I should have named this recoveryBucketHash instead.
    #[serde(rename = "recoveryBucketTypeHash")]
    pub recovery_bucket_type_hash: u32,

    /// The hash identifier for the Tier Type of the item, use to look up its DestinyItemTierTypeDefinition if you need to show localized data for the item's tier.
    #[serde(rename = "tierTypeHash")]
    pub tier_type_hash: u32,

    /// If TRUE, this item is instanced. Otherwise, it is a generic item that merely has a quantity in a stack (like Glimmer).
    #[serde(rename = "isInstanceItem")]
    pub is_instance_item: bool,

    /// The localized name of the tier type, which is a useful shortcut so you don't have to look up the definition every time. However, it's mostly a holdover from days before we had a DestinyItemTierTypeDefinition to refer to.
    #[serde(rename = "tierTypeName")]
    pub tier_type_name: Option<String>,

    /// The enumeration matching the tier type of the item to known values, again for convenience sake.
    #[serde(rename = "tierType")]
    pub tier_type: crate::destiny::TierType,

    /// The tooltip message to show, if any, when the item expires.
    #[serde(rename = "expirationTooltip")]
    pub expiration_tooltip: Option<String>,

    /// If the item expires while playing in an activity, we show a different message.
    #[serde(rename = "expiredInActivityMessage")]
    pub expired_in_activity_message: Option<String>,

    /// If the item expires in orbit, we show a... more different message. ("Consummate V's, consummate!")
    #[serde(rename = "expiredInOrbitMessage")]
    pub expired_in_orbit_message: Option<String>,

    #[serde(rename = "suppressExpirationWhenObjectivesComplete")]
    pub suppress_expiration_when_objectives_complete: bool,

    /// A reference to the associated crafting 'recipe' item definition, if this item can be crafted.
    #[serde(rename = "recipeItemHash")]
    pub recipe_item_hash: Option<u32>,
}

/// An Inventory (be it Character or Profile level) is comprised of many Buckets. An example of a bucket is "Primary Weapons", where all of the primary weapons on a character are gathered together into a single visual element in the UI: a subset of the inventory that has a limited number of slots, and in this case also has an associated Equipment Slot for equipping an item in the bucket.
/// Item definitions declare what their "default" bucket is (DestinyInventoryItemDefinition.inventory.bucketTypeHash), and Item instances will tell you which bucket they are currently residing in (DestinyItemComponent.bucketHash). You can use this information along with the DestinyInventoryBucketDefinition to show these items grouped by bucket.
/// You cannot transfer an item to a bucket that is not its Default without going through a Vendor's "accepted items" (DestinyVendorDefinition.acceptedItems). This is how transfer functionality like the Vault is implemented, as a feature of a Vendor. See the vendor's acceptedItems property for more details.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyInventoryBucketDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// Where the bucket is found. 0 = Character, 1 = Account
    #[serde(rename = "scope")]
    pub scope: crate::destiny::BucketScope,

    /// An enum value for what items can be found in the bucket. See the BucketCategory enum for more details.
    #[serde(rename = "category")]
    pub category: crate::destiny::BucketCategory,

    /// Use this property to provide a quick-and-dirty recommended ordering for buckets in the UI. Most UIs will likely want to forsake this for something more custom and manual.
    #[serde(rename = "bucketOrder")]
    pub bucket_order: i32,

    /// The maximum # of item "slots" in a bucket. A slot is a given combination of item + quantity.
    /// For instance, a Weapon will always take up a single slot, and always have a quantity of 1. But a material could take up only a single slot with hundreds of quantity.
    #[serde(rename = "itemCount")]
    pub item_count: i32,

    /// Sometimes, inventory buckets represent conceptual "locations" in the game that might not be expected. This value indicates the conceptual location of the bucket, regardless of where it is actually contained on the character/account.
    /// See ItemLocation for details.
    /// Note that location includes the Vault and the Postmaster (both of whom being just inventory buckets with additional actions that can be performed on them through a Vendor)
    #[serde(rename = "location")]
    pub location: crate::destiny::ItemLocation,

    /// If TRUE, there is at least one Vendor that can transfer items to/from this bucket. See the DestinyVendorDefinition's acceptedItems property for more information on how transferring works.
    #[serde(rename = "hasTransferDestination")]
    pub has_transfer_destination: bool,

    /// If True, this bucket is enabled. Disabled buckets may include buckets that were included for test purposes, or that were going to be used but then were abandoned but never removed from content *cough*.
    #[serde(rename = "enabled")]
    pub enabled: bool,

    /// if a FIFO bucket fills up, it will delete the oldest item from said bucket when a new item tries to be added to it. If this is FALSE, the bucket will not allow new items to be placed in it until room is made by the user manually deleting items from it. You can see an example of this with the Postmaster's bucket.
    #[serde(rename = "fifo")]
    pub fifo: bool,

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

/// Primarily for Quests, this is the definition of properties related to the item if it is a quest and its various quest steps.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSetBlockDefinition {
    /// A collection of hashes of set items, for items such as Quest Metadata items that possess this data.
    #[serde(rename = "itemList")]
    pub item_list: Option<Vec<crate::destiny::definitions::DestinyItemSetBlockEntryDefinition>>,

    /// If true, items in the set can only be added in increasing order, and adding an item will remove any previous item. For Quests, this is by necessity true. Only one quest step is present at a time, and previous steps are removed as you advance in the quest.
    #[serde(rename = "requireOrderedSetItemAdd")]
    pub require_ordered_set_item_add: bool,

    /// If true, the UI should treat this quest as "featured"
    #[serde(rename = "setIsFeatured")]
    pub set_is_featured: bool,

    /// A string identifier we can use to attempt to identify the category of the Quest.
    #[serde(rename = "setType")]
    pub set_type: Option<String>,

    /// The name of the quest line that this quest step is a part of.
    #[serde(rename = "questLineName")]
    pub quest_line_name: Option<String>,

    /// The description of the quest line that this quest step is a part of.
    #[serde(rename = "questLineDescription")]
    pub quest_line_description: Option<String>,

    /// An additional summary of this step in the quest line.
    #[serde(rename = "questStepSummary")]
    pub quest_step_summary: Option<String>,
}

/// Defines a particular entry in an ItemSet (AKA a particular Quest Step in a Quest)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSetBlockEntryDefinition {
    /// Used for tracking which step a user reached. These values will be populated in the user's internal state, which we expose externally as a more usable DestinyQuestStatus object. If this item has been obtained, this value will be set in trackingUnlockValueHash.
    #[serde(rename = "trackingValue")]
    pub tracking_value: i32,

    /// This is the hash identifier for a DestinyInventoryItemDefinition representing this quest step.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,
}

/// Information about the item's calculated stats, with as much data as we can find for the stats without having an actual instance of the item.
/// Note that this means the entire concept of providing these stats is fundamentally insufficient: we cannot predict with 100% accuracy the conditions under which an item can spawn, so we use various heuristics to attempt to simulate the conditions as accurately as possible. Actual stats for items in-game can and will vary, but these should at least be useful base points for comparison and display.
/// It is also worth noting that some stats, like Magazine size, have further calculations performed on them by scripts in-game and on the game servers that BNet does not have access to. We cannot know how those stats are further transformed, and thus some stats will be inaccurate even on instances of items in BNet vs. how they appear in-game. This is a known limitation of our item statistics, without any planned fix.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemStatBlockDefinition {
    /// If true, the game won't show the "primary" stat on this item when you inspect it.
    /// NOTE: This is being manually mapped, because I happen to want it in a block that isn't going to directly create this derivative block.
    #[serde(rename = "disablePrimaryStatDisplay")]
    pub disable_primary_stat_display: bool,

    /// If the item's stats are meant to be modified by a DestinyStatGroupDefinition, this will be the identifier for that definition.
    /// If you are using live data or precomputed stats data on the DestinyInventoryItemDefinition.stats.stats property, you don't have to worry about statGroupHash and how it alters stats: the already altered stats are provided to you. But if you want to see how the sausage gets made, or perform computations yourself, this is valuable information.
    #[serde(rename = "statGroupHash")]
    pub stat_group_hash: Option<u32>,

    /// If you are looking for precomputed values for the stats on a weapon, this is where they are stored. Technically these are the "Display" stat values. Please see DestinyStatsDefinition for what Display Stat Values means, it's a very long story... but essentially these are the closest values BNet can get to the item stats that you see in-game.
    /// These stats are keyed by the DestinyStatDefinition's hash identifier for the stat that's found on the item.
    #[serde(rename = "stats")]
    pub stats: Option<HashMap<u32, crate::destiny::definitions::DestinyInventoryItemStatDefinition>>,

    /// A quick and lazy way to determine whether any stat other than the "primary" stat is actually visible on the item. Items often have stats that we return in case people find them useful, but they're not part of the "Stat Group" and thus we wouldn't display them in our UI. If this is False, then we're not going to display any of these stats other than the primary one.
    #[serde(rename = "hasDisplayableStats")]
    pub has_displayable_stats: bool,

    /// This stat is determined to be the "primary" stat, and can be looked up in the stats or any other stat collection related to the item.
    /// Use this hash to look up the stat's value using DestinyInventoryItemDefinition.stats.stats, and the renderable data for the primary stat in the related DestinyStatDefinition.
    #[serde(rename = "primaryBaseStatHash")]
    pub primary_base_stat_hash: u32,
}

/// Defines a specific stat value on an item, and the minimum/maximum range that we could compute for the item based on our heuristics for how the item might be generated.
/// Not guaranteed to match real-world instances of the item, but should hopefully at least be close. If it's not close, let us know on the Bungie API forums.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyInventoryItemStatDefinition {
    /// The hash for the DestinyStatDefinition representing this stat.
    #[serde(rename = "statHash")]
    pub stat_hash: u32,

    /// This value represents the stat value assuming the minimum possible roll but accounting for any mandatory bonuses that should be applied to the stat on item creation.
    /// In Destiny 1, this was different from the "minimum" value because there were certain conditions where an item could be theoretically lower level/value than the initial roll.
    /// In Destiny 2, this is not possible unless Talent Grids begin to be used again for these purposes or some other system change occurs... thus in practice, value and minimum should be the same in Destiny 2. Good riddance.
    #[serde(rename = "value")]
    pub value: i32,

    /// The minimum possible value for this stat that we think the item can roll.
    #[serde(rename = "minimum")]
    pub minimum: i32,

    /// The maximum possible value for this stat that we think the item can roll.
    /// WARNING: In Destiny 1, this field was calculated using the potential stat rolls on the item's talent grid. In Destiny 2, items no longer have meaningful talent grids and instead have sockets: but the calculation of this field was never altered to adapt to this change. As such, this field should be considered deprecated until we can address this oversight.
    #[serde(rename = "maximum")]
    pub maximum: i32,

    /// The maximum possible value for the stat as shown in the UI, if it is being shown somewhere that reveals maximum in the UI (such as a bar chart-style view).
    /// This is pulled directly from the item's DestinyStatGroupDefinition, and placed here for convenience.
    /// If not returned, there is no maximum to use (and thus the stat should not be shown in a way that assumes there is a limit to the stat)
    #[serde(rename = "displayMaximum")]
    pub display_maximum: Option<i32>,
}

/// This represents a stat that's applied to a character or an item (such as a weapon, piece of armor, or a vehicle).
/// An example of a stat might be Attack Power on a weapon.
/// Stats go through a complex set of transformations before they end up being shown to the user as a number or a progress bar, and those transformations are fundamentally intertwined with the concept of a "Stat Group" (DestinyStatGroupDefinition). Items have both Stats and a reference to a Stat Group, and it is the Stat Group that takes the raw stat information and gives it both rendering metadata (such as whether to show it as a number or a progress bar) and the final transformation data (interpolation tables to turn the raw investment stat into a display stat). Please see DestinyStatGroupDefinition for more information on that transformational process.
/// Stats are segregated from Stat Groups because different items and types of items can refer to the same stat, but have different "scales" for the stat while still having the same underlying value. For example, both a Shotgun and an Auto Rifle may have a "raw" impact stat of 50, but the Auto Rifle's Stat Group will scale that 50 down so that, when it is displayed, it is a smaller value relative to the shotgun. (this is a totally made up example, don't assume shotguns have naturally higher impact than auto rifles because of this)
/// A final caveat is that some stats, even after this "final" transformation, go through yet another set of transformations directly in the game as a result of dynamic, stateful scripts that get run. BNet has no access to these scripts, nor any way to know which scripts get executed. As a result, the stats for an item that you see in-game - particularly for stats that are often impacted by Perks, like Magazine Size - can change dramatically from what we return on Bungie.Net. This is a known issue with no fix coming down the pipeline. Take these stats with a grain of salt.
/// Stats actually go through four transformations, for those interested:
/// 1) "Sandbox" stat, the "most raw" form. These are pretty much useless without transformations applied, and thus are not currently returned in the API. If you really want these, we can provide them. Maybe someone could do something cool with it?
/// 2) "Investment" stat (the stat's value after DestinyStatDefinition's interpolation tables and aggregation logic is applied to the "Sandbox" stat value)
/// 3) "Display" stat (the stat's base UI-visible value after DestinyStatGroupDefinition's interpolation tables are applied to the Investment Stat value. For most stats, this is what is displayed.)
/// 4) Underlying in-game stat (the stat's actual value according to the game, after the game runs dynamic scripts based on the game and character's state. This is the final transformation that BNet does not have access to. For most stats, this is not actually displayed to the user, with the exception of Magazine Size which is then piped back to the UI for display in-game, but not to BNet.)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyStatDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// Stats can exist on a character or an item, and they may potentially be aggregated in different ways. The DestinyStatAggregationType enum value indicates the way that this stat is being aggregated.
    #[serde(rename = "aggregationType")]
    pub aggregation_type: crate::destiny::DestinyStatAggregationType,

    /// True if the stat is computed rather than being delivered as a raw value on items.
    /// For instance, the Light stat in Destiny 1 was a computed stat.
    #[serde(rename = "hasComputedBlock")]
    pub has_computed_block: bool,

    /// The category of the stat, according to the game.
    #[serde(rename = "statCategory")]
    pub stat_category: crate::destiny::DestinyStatCategory,

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

/// When an inventory item (DestinyInventoryItemDefinition) has Stats (such as Attack Power), the item will refer to a Stat Group. This definition enumerates the properties used to transform the item's "Investment" stats into "Display" stats.
/// See DestinyStatDefinition's documentation for information about the transformation of Stats, and the meaning of an Investment vs. a Display stat.
/// If you don't want to do these calculations on your own, fear not: pulling live data from the BNet endpoints will return display stat values pre-computed and ready for you to use. I highly recommend this approach, saves a lot of time and also accounts for certain stat modifiers that can't easily be accounted for without live data (such as stat modifiers on Talent Grids and Socket Plugs)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyStatGroupDefinition {
    /// The maximum possible value that any stat in this group can be transformed into.
    /// This is used by stats that *don't* have scaledStats entries below, but that still need to be displayed as a progress bar, in which case this is used as the upper bound for said progress bar. (the lower bound is always 0)
    #[serde(rename = "maximumValue")]
    pub maximum_value: i32,

    /// This apparently indicates the position of the stats in the UI? I've returned it in case anyone can use it, but it's not of any use to us on BNet. Something's being lost in translation with this value.
    #[serde(rename = "uiPosition")]
    pub ui_position: i32,

    /// Any stat that requires scaling to be transformed from an "Investment" stat to a "Display" stat will have an entry in this list. For more information on what those types of stats mean and the transformation process, see DestinyStatDefinition.
    /// In retrospect, I wouldn't mind if this was a dictionary keyed by the stat hash instead. But I'm going to leave it be because [[After Apple Picking]].
    #[serde(rename = "scaledStats")]
    pub scaled_stats: Option<Vec<crate::destiny::definitions::DestinyStatDisplayDefinition>>,

    /// The game has the ability to override, based on the stat group, what the localized text is that is displayed for Stats being shown on the item.
    /// Mercifully, no Stat Groups use this feature currently. If they start using them, we'll all need to start using them (and those of you who are more prudent than I am can go ahead and start pre-checking for this.)
    #[serde(rename = "overrides")]
    pub overrides: Option<HashMap<u32, crate::destiny::definitions::DestinyStatOverrideDefinition>>,

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

/// Describes the way that an Item Stat (see DestinyStatDefinition) is transformed using the DestinyStatGroupDefinition related to that item. See both of the aforementioned definitions for more information about the stages of stat transformation.
/// This represents the transformation of a stat into a "Display" stat (the closest value that BNet can get to the in-game display value of the stat)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyStatDisplayDefinition {
    /// The hash identifier for the stat being transformed into a Display stat.
    /// Use it to look up the DestinyStatDefinition, or key into a DestinyInventoryItemDefinition's stats property.
    #[serde(rename = "statHash")]
    pub stat_hash: u32,

    /// Regardless of the output of interpolation, this is the maximum possible value that the stat can be. It should also be used as the upper bound for displaying the stat as a progress bar (the minimum always being 0)
    #[serde(rename = "maximumValue")]
    pub maximum_value: i32,

    /// If this is true, the stat should be displayed as a number. Otherwise, display it as a progress bar. Or, you know, do whatever you want. There's no displayAsNumeric police.
    #[serde(rename = "displayAsNumeric")]
    pub display_as_numeric: bool,

    /// The interpolation table representing how the Investment Stat is transformed into a Display Stat.
    /// See DestinyStatDefinition for a description of the stages of stat transformation.
    #[serde(rename = "displayInterpolation")]
    pub display_interpolation: Option<Vec<crate::interpolation::InterpolationPoint>>,
}

/// Stat Groups (DestinyStatGroupDefinition) has the ability to override the localized text associated with stats that are to be shown on the items with which they are associated.
/// This defines a specific overridden stat. You could theoretically check these before rendering your stat UI, and for each stat that has an override show these displayProperties instead of those on the DestinyStatDefinition.
/// Or you could be like us, and skip that for now because the game has yet to actually use this feature. But know that it's here, waiting for a resilliant young designer to take up the mantle and make us all look foolish by showing the wrong name for stats.
/// Note that, if this gets used, the override will apply only to items using the overriding Stat Group. Other items will still show the default stat's name/description.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyStatOverrideDefinition {
    /// The hash identifier of the stat whose display properties are being overridden.
    #[serde(rename = "statHash")]
    pub stat_hash: u32,

    /// The display properties to show instead of the base DestinyStatDefinition display properties.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,
}

/// Items that can be equipped define this block. It contains information we need to understand how and when the item can be equipped.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyEquippingBlockDefinition {
    /// If the item is part of a gearset, this is a reference to that gearset item.
    #[serde(rename = "gearsetItemHash")]
    pub gearset_item_hash: Option<u32>,

    /// If defined, this is the label used to check if the item has other items of matching types already equipped.
    /// For instance, when you aren't allowed to equip more than one Exotic Weapon, that's because all exotic weapons have identical uniqueLabels and the game checks the to-be-equipped item's uniqueLabel vs. all other already equipped items (other than the item in the slot that's about to be occupied).
    #[serde(rename = "uniqueLabel")]
    pub unique_label: Option<String>,

    /// The hash of that unique label. Does not point to a specific definition.
    #[serde(rename = "uniqueLabelHash")]
    pub unique_label_hash: u32,

    /// An equipped item *must* be equipped in an Equipment Slot. This is the hash identifier of the DestinyEquipmentSlotDefinition into which it must be equipped.
    #[serde(rename = "equipmentSlotTypeHash")]
    pub equipment_slot_type_hash: u32,

    /// These are custom attributes on the equippability of the item.
    /// For now, this can only be "equip on acquire", which would mean that the item will be automatically equipped as soon as you pick it up.
    #[serde(rename = "attributes")]
    pub attributes: enumflags2::BitFlags<crate::destiny::EquippingItemBlockAttributes>,

    /// Ammo type used by a weapon is no longer determined by the bucket in which it is contained. If the item has an ammo type - i.e. if it is a weapon - this will be the type of ammunition expected.
    #[serde(rename = "ammoType")]
    pub ammo_type: crate::destiny::DestinyAmmunitionType,

    /// These are strings that represent the possible Game/Account/Character state failure conditions that can occur when trying to equip the item. They match up one-to-one with requiredUnlockExpressions.
    #[serde(rename = "displayStrings")]
    pub display_strings: Option<Vec<String>>,
}

/// Characters can not only have Inventory buckets (containers of items that are generally matched by their type or functionality), they can also have Equipment Slots.
/// The Equipment Slot is an indicator that the related bucket can have instanced items equipped on the character. For instance, the Primary Weapon bucket has an Equipment Slot that determines whether you can equip primary weapons, and holds the association between its slot and the inventory bucket from which it can have items equipped.
/// An Equipment Slot must have a related Inventory Bucket, but not all inventory buckets must have Equipment Slots.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyEquipmentSlotDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// These technically point to "Equipment Category Definitions". But don't get excited. There's nothing of significant value in those definitions, so I didn't bother to expose them. You can use the hash here to group equipment slots by common functionality, which serves the same purpose as if we had the Equipment Category definitions exposed.
    #[serde(rename = "equipmentCategoryHash")]
    pub equipment_category_hash: u32,

    /// The inventory bucket that owns this equipment slot.
    #[serde(rename = "bucketTypeHash")]
    pub bucket_type_hash: u32,

    /// If True, equipped items should have their custom art dyes applied when rendering the item. Otherwise, custom art dyes on an item should be ignored if the item is equipped in this slot.
    #[serde(rename = "applyCustomArtDyes")]
    pub apply_custom_art_dyes: bool,

    /// The Art Dye Channels that apply to this equipment slot.
    #[serde(rename = "artDyeChannels")]
    pub art_dye_channels: Option<Vec<crate::destiny::definitions::DestinyArtDyeReference>>,

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
pub struct DestinyArtDyeReference {
    #[serde(rename = "artDyeChannelHash")]
    pub art_dye_channel_hash: u32,
}

/// This Block defines the rendering data associated with the item, if any.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemTranslationBlockDefinition {
    #[serde(rename = "weaponPatternIdentifier")]
    pub weapon_pattern_identifier: Option<String>,

    #[serde(rename = "weaponPatternHash")]
    pub weapon_pattern_hash: u32,

    #[serde(rename = "defaultDyes")]
    pub default_dyes: Option<Vec<crate::destiny::DyeReference>>,

    #[serde(rename = "lockedDyes")]
    pub locked_dyes: Option<Vec<crate::destiny::DyeReference>>,

    #[serde(rename = "customDyes")]
    pub custom_dyes: Option<Vec<crate::destiny::DyeReference>>,

    #[serde(rename = "arrangements")]
    pub arrangements: Option<Vec<crate::destiny::definitions::DestinyGearArtArrangementReference>>,

    #[serde(rename = "hasGeometry")]
    pub has_geometry: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyGearArtArrangementReference {
    #[serde(rename = "classHash")]
    pub class_hash: u32,

    #[serde(rename = "artArrangementHash")]
    pub art_arrangement_hash: u32,
}

/// Defines a Character Class in Destiny 2. These are types of characters you can play, like Titan, Warlock, and Hunter.
#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyClassDefinition {
    /// In Destiny 1, we added a convenience Enumeration for referring to classes. We've kept it, though mostly for posterity. This is the enum value for this definition's class.
    #[serde(rename = "classType")]
    pub class_type: crate::destiny::DestinyClass,

    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// A localized string referring to the singular form of the Class's name when referred to in gendered form. Keyed by the DestinyGender.
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "genderedClassNames")]
    pub gendered_class_names: Option<HashMap<crate::destiny::DestinyGender, String>>,

    #[serde(rename = "genderedClassNamesByGenderHash")]
    pub gendered_class_names_by_gender_hash: Option<HashMap<u32, String>>,

    /// Mentors don't really mean anything anymore. Don't expect this to be populated.
    #[serde(rename = "mentorVendorHash")]
    pub mentor_vendor_hash: Option<u32>,

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

/// Gender is a social construct, and as such we have definitions for Genders. Right now there happens to only be two, but we'll see what the future holds.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyGenderDefinition {
    /// This is a quick reference enumeration for all of the currently defined Genders. We use the enumeration for quicker lookups in related data, like DestinyClassDefinition.genderedClassNames.
    #[serde(rename = "genderType")]
    pub gender_type: crate::destiny::DestinyGender,

    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

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

/// These are the definitions for Vendors.
/// In Destiny, a Vendor can be a lot of things - some things that you wouldn't expect, and some things that you don't even see directly in the game. Vendors are the Dolly Levi of the Destiny universe.
/// - Traditional Vendors as you see in game: people who you come up to and who give you quests, rewards, or who you can buy things from.
/// - Kiosks/Collections, which are really just Vendors that don't charge currency (or charge some pittance of a currency) and whose gating for purchases revolves more around your character's state.
/// - Previews for rewards or the contents of sacks. These are implemented as Vendors, where you can't actually purchase from them but the items that they have for sale and the categories of sale items reflect the rewards or contents of the sack. This is so that the game could reuse the existing Vendor display UI for rewards and save a bunch of wheel reinvention.
/// - Item Transfer capabilities, like the Vault and Postmaster. Vendors can have "acceptedItem" buckets that determine the source and destination buckets for transfers. When you interact with such a vendor, these buckets are what gets shown in the UI instead of any items that the Vendor would have for sale. Yep, the Vault is a vendor.
/// It is pretty much guaranteed that they'll be used for even more features in the future. They have come to be seen more as generic categorized containers for items than "vendors" in a traditional sense, for better or worse.
/// Where possible and time allows, we'll attempt to split those out into their own more digestible derived "Definitions": but often time does not allow that, as you can see from the above ways that vendors are used which we never split off from Vendor Definitions externally.
/// Since Vendors are so many things to so many parts of the game, the definition is understandably complex. You will want to combine this data with live Vendor information from the API when it is available.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::DestinyVendorDisplayPropertiesDefinition>,

    /// The type of reward progression that this vendor has. Default - The original rank progression from token redemption. Ritual - Progression from ranks in ritual content. For example: Crucible (Shaxx), Gambit (Drifter), and Battlegrounds (War Table).
    #[serde(rename = "vendorProgressionType")]
    pub vendor_progression_type: crate::destiny::DestinyVendorProgressionType,

    /// If the vendor has a custom localized string describing the "buy" action, that is returned here.
    #[serde(rename = "buyString")]
    pub buy_string: Option<String>,

    /// Ditto for selling. Not that you can sell items to a vendor anymore. Will it come back? Who knows. The string's still there.
    #[serde(rename = "sellString")]
    pub sell_string: Option<String>,

    /// If the vendor has an item that should be displayed as the "featured" item, this is the hash identifier for that DestinyVendorItemDefinition.
    /// Apparently this is usually a related currency, like a reputation token. But it need not be restricted to that.
    #[serde(rename = "displayItemHash")]
    pub display_item_hash: u32,

    /// If this is true, you aren't allowed to buy whatever the vendor is selling.
    #[serde(rename = "inhibitBuying")]
    pub inhibit_buying: bool,

    /// If this is true, you're not allowed to sell whatever the vendor is buying.
    #[serde(rename = "inhibitSelling")]
    pub inhibit_selling: bool,

    /// If the Vendor has a faction, this hash will be valid and point to a DestinyFactionDefinition.
    /// The game UI and BNet often mine the faction definition for additional elements and details to place on the screen, such as the faction's Progression status (aka "Reputation").
    #[serde(rename = "factionHash")]
    pub faction_hash: u32,

    /// A number used for calculating the frequency of a vendor's inventory resetting/refreshing.
    /// Don't worry about calculating this - we do it on the server side and send you the next refresh date with the live data.
    #[serde(rename = "resetIntervalMinutes")]
    pub reset_interval_minutes: i32,

    /// Again, used for reset/refreshing of inventory. Don't worry too much about it. Unless you want to.
    #[serde(rename = "resetOffsetMinutes")]
    pub reset_offset_minutes: i32,

    /// If an item can't be purchased from the vendor, there may be many "custom"/game state specific reasons why not.
    /// This is a list of localized strings with messages for those custom failures. The live BNet data will return a failureIndexes property for items that can't be purchased: using those values to index into this array, you can show the user the appropriate failure message for the item that can't be bought.
    #[serde(rename = "failureStrings")]
    pub failure_strings: Option<Vec<String>>,

    /// If we were able to predict the dates when this Vendor will be visible/available, this will be the list of those date ranges. Sadly, we're not able to predict this very frequently, so this will often be useless data.
    #[serde(rename = "unlockRanges")]
    pub unlock_ranges: Option<Vec<crate::dates::DateRange>>,

    /// The internal identifier for the Vendor. A holdover from the old days of Vendors, but we don't have time to refactor it away.
    #[serde(rename = "vendorIdentifier")]
    pub vendor_identifier: Option<String>,

    /// A portrait of the Vendor's smiling mug. Or frothing tentacles.
    #[serde(rename = "vendorPortrait")]
    pub vendor_portrait: Option<String>,

    /// If the vendor has a custom banner image, that can be found here.
    #[serde(rename = "vendorBanner")]
    pub vendor_banner: Option<String>,

    /// If a vendor is not enabled, we won't even save the vendor's definition, and we won't return any items or info about them. It's as if they don't exist.
    #[serde(rename = "enabled")]
    pub enabled: bool,

    /// If a vendor is not visible, we still have and will give vendor definition info, but we won't use them for things like Advisors or UI.
    #[serde(rename = "visible")]
    pub visible: bool,

    /// The identifier of the VendorCategoryDefinition for this vendor's subcategory.
    #[serde(rename = "vendorSubcategoryIdentifier")]
    pub vendor_subcategory_identifier: Option<String>,

    /// If TRUE, consolidate categories that only differ by trivial properties (such as having minor differences in name)
    #[serde(rename = "consolidateCategories")]
    pub consolidate_categories: bool,

    /// Describes "actions" that can be performed on a vendor. Currently, none of these exist. But theoretically a Vendor could let you interact with it by performing actions. We'll see what these end up looking like if they ever get used.
    #[serde(rename = "actions")]
    pub actions: Option<Vec<crate::destiny::definitions::DestinyVendorActionDefinition>>,

    /// These are the headers for sections of items that the vendor is selling. When you see items organized by category in the header, it is these categories that it is showing.
    /// Well, technically not *exactly* these. On BNet, it doesn't make sense to have categories be "paged" as we do in Destiny, so we run some heuristics to attempt to aggregate pages of categories together.
    /// These are the categories post-concatenation, if the vendor had concatenation applied. If you want the pre-aggregated category data, use originalCategories.
    #[serde(rename = "categories")]
    pub categories: Option<Vec<crate::destiny::definitions::DestinyVendorCategoryEntryDefinition>>,

    /// See the categories property for a description of categories and why originalCategories exists.
    #[serde(rename = "originalCategories")]
    pub original_categories: Option<Vec<crate::destiny::definitions::DestinyVendorCategoryEntryDefinition>>,

    /// Display Categories are different from "categories" in that these are specifically for visual grouping and display of categories in Vendor UI.
    /// The "categories" structure is for validation of the contained items, and can be categorized entirely separately from "Display Categories", there need be and often will be no meaningful relationship between the two.
    #[serde(rename = "displayCategories")]
    pub display_categories: Option<Vec<crate::destiny::definitions::DestinyDisplayCategoryDefinition>>,

    /// In addition to selling items, vendors can have "interactions": UI where you "talk" with the vendor and they offer you a reward, some item, or merely acknowledge via dialog that you did something cool.
    #[serde(rename = "interactions")]
    pub interactions: Option<Vec<crate::destiny::definitions::DestinyVendorInteractionDefinition>>,

    /// If the vendor shows you items from your own inventory - such as the Vault vendor does - this data describes the UI around showing those inventory buckets and which ones get shown.
    #[serde(rename = "inventoryFlyouts")]
    pub inventory_flyouts: Option<Vec<crate::destiny::definitions::DestinyVendorInventoryFlyoutDefinition>>,

    /// If the vendor sells items (or merely has a list of items to show like the "Sack" vendors do), this is the list of those items that the vendor can sell. From this list, only a subset will be available from the vendor at any given time, selected randomly and reset on the vendor's refresh interval.
    /// Note that a vendor can sell the same item multiple ways: for instance, nothing stops a vendor from selling you some specific weapon but using two different currencies, or the same weapon at multiple "item levels".
    #[serde(rename = "itemList")]
    pub item_list: Option<Vec<crate::destiny::definitions::DestinyVendorItemDefinition>>,

    /// BNet doesn't use this data yet, but it appears to be an optional list of flavor text about services that the Vendor can provide.
    #[serde(rename = "services")]
    pub services: Option<Vec<crate::destiny::definitions::DestinyVendorServiceDefinition>>,

    /// If the Vendor is actually a vehicle for the transferring of items (like the Vault and Postmaster vendors), this defines the list of source->destination buckets for transferring.
    #[serde(rename = "acceptedItems")]
    pub accepted_items: Option<Vec<crate::destiny::definitions::DestinyVendorAcceptedItemDefinition>>,

    /// As many of you know, Vendor data has historically been pretty brutal on the BNet servers. In an effort to reduce this workload, only Vendors with this flag set will be returned on Vendor requests. This allows us to filter out Vendors that don't dynamic data that's particularly useful: things like "Preview/Sack" vendors, for example, that you can usually suss out the details for using just the definitions themselves.
    #[serde(rename = "returnWithVendorRequest")]
    pub return_with_vendor_request: bool,

    /// A vendor can be at different places in the world depending on the game/character/account state. This is the list of possible locations for the vendor, along with conditions we use to determine which one is currently active.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<crate::destiny::definitions::vendors::DestinyVendorLocationDefinition>>,

    /// A vendor can be a part of 0 or 1 "groups" at a time: a group being a collection of Vendors related by either location or function/purpose. It's used for our our Companion Vendor UI. Only one of these can be active for a Vendor at a time.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::destiny::definitions::DestinyVendorGroupReference>>,

    /// Some items don't make sense to return in the API, for example because they represent an action to be performed rather than an item being sold. I'd rather we not do this, but at least in the short term this is a workable workaround.
    #[serde(rename = "ignoreSaleItemHashes")]
    pub ignore_sale_item_hashes: Option<Vec<u32>>,

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
pub struct DestinyVendorDisplayPropertiesDefinition {
    /// I regret calling this a "large icon". It's more like a medium-sized image with a picture of the vendor's mug on it, trying their best to look cool. Not what one would call an icon.
    #[serde(rename = "largeIcon")]
    pub large_icon: Option<String>,

    #[serde(rename = "subtitle")]
    pub subtitle: Option<String>,

    /// If we replaced the icon with something more glitzy, this is the original icon that the vendor had according to the game's content. It may be more lame and/or have less razzle-dazzle. But who am I to tell you which icon to use.
    #[serde(rename = "originalIcon")]
    pub original_icon: Option<String>,

    /// Vendors, in addition to expected display property data, may also show some "common requirements" as statically defined definition data. This might be when a vendor accepts a single type of currency, or when the currency is unique to the vendor and the designers wanted to show that currency when you interact with the vendor.
    #[serde(rename = "requirementsDisplay")]
    pub requirements_display: Option<Vec<crate::destiny::definitions::DestinyVendorRequirementDisplayEntryDefinition>>,

    /// This is the icon used in parts of the game UI such as the vendor's waypoint.
    #[serde(rename = "smallTransparentIcon")]
    pub small_transparent_icon: Option<String>,

    /// This is the icon used in the map overview, when the vendor is located on the map.
    #[serde(rename = "mapIcon")]
    pub map_icon: Option<String>,

    /// This is apparently the "Watermark". I am not certain offhand where this is actually used in the Game UI, but some people may find it useful.
    #[serde(rename = "largeTransparentIcon")]
    pub large_transparent_icon: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Note that "icon" is sometimes misleading, and should be interpreted in the context of the entity. For instance, in Destiny 1 the DestinyRecordBookDefinition's icon was a big picture of a book.
    /// But usually, it will be a small square image that you can use as... well, an icon.
    /// They are currently represented as 96px x 96px images.
    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "iconSequences")]
    pub icon_sequences: Option<Vec<crate::destiny::definitions::common::DestinyIconSequenceDefinition>>,

    /// If this item has a high-res icon (at least for now, many things won't), then the path to that icon will be here.
    #[serde(rename = "highResIcon")]
    pub high_res_icon: Option<String>,

    #[serde(rename = "hasIcon")]
    pub has_icon: bool,
}

/// The localized properties of the requirementsDisplay, allowing information about the requirement or item being featured to be seen.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorRequirementDisplayEntryDefinition {
    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "source")]
    pub source: Option<String>,

    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// If a vendor can ever end up performing actions, these are the properties that will be related to those actions. I'm not going to bother documenting this yet, as it is unused and unclear if it will ever be used... but in case it is ever populated and someone finds it useful, it is defined here.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorActionDefinition {
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "executeSeconds")]
    pub execute_seconds: i32,

    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "verb")]
    pub verb: Option<String>,

    #[serde(rename = "isPositive")]
    pub is_positive: bool,

    #[serde(rename = "actionId")]
    pub action_id: Option<String>,

    #[serde(rename = "actionHash")]
    pub action_hash: u32,

    #[serde(rename = "autoPerformAction")]
    pub auto_perform_action: bool,
}

/// This is the definition for a single Vendor Category, into which Sale Items are grouped.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorCategoryEntryDefinition {
    /// The index of the category in the original category definitions for the vendor.
    #[serde(rename = "categoryIndex")]
    pub category_index: i32,

    /// Used in sorting items in vendors... but there's a lot more to it. Just go with the order provided in the itemIndexes property on the DestinyVendorCategoryComponent instead, it should be more reliable than trying to recalculate it yourself.
    #[serde(rename = "sortValue")]
    pub sort_value: i32,

    /// The hashed identifier for the category.
    #[serde(rename = "categoryHash")]
    pub category_hash: u32,

    /// The amount of items that will be available when this category is shown.
    #[serde(rename = "quantityAvailable")]
    pub quantity_available: i32,

    /// If items aren't up for sale in this category, should we still show them (greyed out)?
    #[serde(rename = "showUnavailableItems")]
    pub show_unavailable_items: bool,

    /// If you don't have the currency required to buy items from this category, should the items be hidden?
    #[serde(rename = "hideIfNoCurrency")]
    pub hide_if_no_currency: bool,

    /// True if this category doesn't allow purchases.
    #[serde(rename = "hideFromRegularPurchase")]
    pub hide_from_regular_purchase: bool,

    /// The localized string for making purchases from this category, if it is different from the vendor's string for purchasing.
    #[serde(rename = "buyStringOverride")]
    pub buy_string_override: Option<String>,

    /// If the category is disabled, this is the localized description to show.
    #[serde(rename = "disabledDescription")]
    pub disabled_description: Option<String>,

    /// The localized title of the category.
    #[serde(rename = "displayTitle")]
    pub display_title: Option<String>,

    /// If this category has an overlay prompt that should appear, this contains the details of that prompt.
    #[serde(rename = "overlay")]
    pub overlay: Option<crate::destiny::definitions::DestinyVendorCategoryOverlayDefinition>,

    /// A shortcut for the vendor item indexes sold under this category. Saves us from some expensive reorganization at runtime.
    #[serde(rename = "vendorItemIndexes")]
    pub vendor_item_indexes: Option<Vec<i32>>,

    /// Sometimes a category isn't actually used to sell items, but rather to preview them. This implies different UI (and manual placement of the category in the UI) in the game, and special treatment.
    #[serde(rename = "isPreview")]
    pub is_preview: bool,

    /// If true, this category only displays items: you can't purchase anything in them.
    #[serde(rename = "isDisplayOnly")]
    pub is_display_only: bool,

    #[serde(rename = "resetIntervalMinutesOverride")]
    pub reset_interval_minutes_override: i32,

    #[serde(rename = "resetOffsetMinutesOverride")]
    pub reset_offset_minutes_override: i32,
}

/// The details of an overlay prompt to show to a user. They are all fairly self-explanatory localized strings that can be shown.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorCategoryOverlayDefinition {
    #[serde(rename = "choiceDescription")]
    pub choice_description: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    /// If this overlay has a currency item that it features, this is said featured item.
    #[serde(rename = "currencyItemHash")]
    pub currency_item_hash: Option<u32>,
}

/// Display Categories are different from "categories" in that these are specifically for visual grouping and display of categories in Vendor UI. The "categories" structure is for validation of the contained items, and can be categorized entirely separately from "Display Categories", there need be and often will be no meaningful relationship between the two.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyDisplayCategoryDefinition {
    #[serde(rename = "index")]
    pub index: i32,

    /// A string identifier for the display category.
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    #[serde(rename = "displayCategoryHash")]
    pub display_category_hash: u32,

    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If true, this category should be displayed in the "Banner" section of the vendor's UI.
    #[serde(rename = "displayInBanner")]
    pub display_in_banner: bool,

    /// If it exists, this is the hash identifier of a DestinyProgressionDefinition that represents the progression to show on this display category.
    /// Specific categories can now have thier own distinct progression, apparently. So that's cool.
    #[serde(rename = "progressionHash")]
    pub progression_hash: Option<u32>,

    /// If this category sorts items in a nonstandard way, this will be the way we sort.
    #[serde(rename = "sortOrder")]
    pub sort_order: crate::destiny::VendorDisplayCategorySortOrder,

    /// An indicator of how the category will be displayed in the UI. It's up to you to do something cool or interesting in response to this, or just to treat it as a normal category.
    #[serde(rename = "displayStyleHash")]
    pub display_style_hash: Option<u32>,

    /// An indicator of how the category will be displayed in the UI. It's up to you to do something cool or interesting in response to this, or just to treat it as a normal category.
    #[serde(rename = "displayStyleIdentifier")]
    pub display_style_identifier: Option<String>,
}

/// A Vendor Interaction is a dialog shown by the vendor other than sale items or transfer screens. The vendor is showing you something, and asking you to reply to it by choosing an option or reward.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorInteractionDefinition {
    /// The position of this interaction in its parent array. Note that this is NOT content agnostic, and should not be used as such.
    #[serde(rename = "interactionIndex")]
    pub interaction_index: i32,

    /// The potential replies that the user can make to the interaction.
    #[serde(rename = "replies")]
    pub replies: Option<Vec<crate::destiny::definitions::DestinyVendorInteractionReplyDefinition>>,

    /// If >= 0, this is the category of sale items to show along with this interaction dialog.
    #[serde(rename = "vendorCategoryIndex")]
    pub vendor_category_index: i32,

    /// If this interaction dialog is about a quest, this is the questline related to the interaction. You can use this to show the quest overview, or even the character's status with the quest if you use it to find the character's current Quest Step by checking their inventory against this questlineItemHash's DestinyInventoryItemDefinition.setData.
    #[serde(rename = "questlineItemHash")]
    pub questline_item_hash: u32,

    /// If this interaction is meant to show you sacks, this is the list of types of sacks to be shown. If empty, the interaction is not meant to show sacks.
    #[serde(rename = "sackInteractionList")]
    pub sack_interaction_list: Option<Vec<crate::destiny::definitions::DestinyVendorInteractionSackEntryDefinition>>,

    /// A UI hint for the behavior of the interaction screen. This is useful to determine what type of interaction is occurring, such as a prompt to receive a rank up reward or a prompt to choose a reward for completing a quest. The hash isn't as useful as the Enum in retrospect, well what can you do. Try using interactionType instead.
    #[serde(rename = "uiInteractionType")]
    pub ui_interaction_type: u32,

    /// The enumerated version of the possible UI hints for vendor interactions, which is a little easier to grok than the hash found in uiInteractionType.
    #[serde(rename = "interactionType")]
    pub interaction_type: crate::destiny::VendorInteractionType,

    /// If this interaction is displaying rewards, this is the text to use for the header of the reward-displaying section of the interaction.
    #[serde(rename = "rewardBlockLabel")]
    pub reward_block_label: Option<String>,

    /// If the vendor's reward list is sourced from one of his categories, this is the index into the category array of items to show.
    #[serde(rename = "rewardVendorCategoryIndex")]
    pub reward_vendor_category_index: i32,

    /// If the vendor interaction has flavor text, this is some of it.
    #[serde(rename = "flavorLineOne")]
    pub flavor_line_one: Option<String>,

    /// If the vendor interaction has flavor text, this is the rest of it.
    #[serde(rename = "flavorLineTwo")]
    pub flavor_line_two: Option<String>,

    /// The header for the interaction dialog.
    #[serde(rename = "headerDisplayProperties")]
    pub header_display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The localized text telling the player what to do when they see this dialog.
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
}

/// When the interaction is replied to, Reward sites will fire and items potentially selected based on whether the given unlock expression is TRUE.
/// You can potentially choose one from multiple replies when replying to an interaction: this is how you get either/or rewards from vendors.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorInteractionReplyDefinition {
    /// The rewards granted upon responding to the vendor.
    #[serde(rename = "itemRewardsSelection")]
    pub item_rewards_selection: crate::destiny::DestinyVendorInteractionRewardSelection,

    /// The localized text for the reply.
    #[serde(rename = "reply")]
    pub reply: Option<String>,

    /// An enum indicating the type of reply being made.
    #[serde(rename = "replyType")]
    pub reply_type: crate::destiny::DestinyVendorReplyType,
}

/// Compare this sackType to the sack identifier in the DestinyInventoryItemDefinition.vendorSackType property of items. If they match, show this sack with this interaction.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorInteractionSackEntryDefinition {
    #[serde(rename = "sackType")]
    pub sack_type: u32,
}

/// The definition for an "inventory flyout": a UI screen where we show you part of an otherwise hidden vendor inventory: like the Vault inventory buckets.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorInventoryFlyoutDefinition {
    /// If the flyout is locked, this is the reason why.
    #[serde(rename = "lockedDescription")]
    pub locked_description: Option<String>,

    /// The title and other common properties of the flyout.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// A list of inventory buckets and other metadata to show on the screen.
    #[serde(rename = "buckets")]
    pub buckets: Option<Vec<crate::destiny::definitions::DestinyVendorInventoryFlyoutBucketDefinition>>,

    /// An identifier for the flyout, in case anything else needs to refer to them.
    #[serde(rename = "flyoutId")]
    pub flyout_id: u32,

    /// If this is true, don't show any of the glistening "this is a new item" UI elements, like we show on the inventory items themselves in in-game UI.
    #[serde(rename = "suppressNewness")]
    pub suppress_newness: bool,

    /// If this flyout is meant to show you the contents of the player's equipment slot, this is the slot to show.
    #[serde(rename = "equipmentSlotHash")]
    pub equipment_slot_hash: Option<u32>,
}

/// Information about a single inventory bucket in a vendor flyout UI and how it is shown.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorInventoryFlyoutBucketDefinition {
    /// If true, the inventory bucket should be able to be collapsed visually.
    #[serde(rename = "collapsible")]
    pub collapsible: bool,

    /// The inventory bucket whose contents should be shown.
    #[serde(rename = "inventoryBucketHash")]
    pub inventory_bucket_hash: u32,

    /// The methodology to use for sorting items from the flyout.
    #[serde(rename = "sortItemsBy")]
    pub sort_items_by: crate::destiny::DestinyItemSortType,
}

/// This represents an item being sold by the vendor.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorItemDefinition {
    /// The index into the DestinyVendorDefinition.saleList. This is what we use to refer to items being sold throughout live and definition data.
    #[serde(rename = "vendorItemIndex")]
    pub vendor_item_index: i32,

    /// The hash identifier of the item being sold (DestinyInventoryItemDefinition).
    /// Note that a vendor can sell the same item in multiple ways, so don't assume that itemHash is a unique identifier for this entity.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// The amount you will recieve of the item described in itemHash if you make the purchase.
    #[serde(rename = "quantity")]
    pub quantity: i32,

    /// An list of indexes into the DestinyVendorDefinition.failureStrings array, indicating the possible failure strings that can be relevant for this item.
    #[serde(rename = "failureIndexes")]
    pub failure_indexes: Option<Vec<i32>>,

    /// This is a pre-compiled aggregation of item value and priceOverrideList, so that we have one place to check for what the purchaser must pay for the item. Use this instead of trying to piece together the price separately.
    /// The somewhat crappy part about this is that, now that item quantity overrides have dynamic modifiers, this will not necessarily be statically true. If you were using this instead of live data, switch to using live data.
    #[serde(rename = "currencies")]
    pub currencies: Option<Vec<crate::destiny::definitions::DestinyVendorItemQuantity>>,

    /// If this item can be refunded, this is the policy for what will be refundd, how, and in what time period.
    #[serde(rename = "refundPolicy")]
    pub refund_policy: crate::destiny::DestinyVendorItemRefundPolicy,

    /// The amount of time before refundability of the newly purchased item will expire.
    #[serde(rename = "refundTimeLimit")]
    pub refund_time_limit: i32,

    /// The Default level at which the item will spawn. Almost always driven by an adjusto these days. Ideally should be singular. It's a long story how this ended up as a list, but there is always either going to be 0:1 of these entities.
    #[serde(rename = "creationLevels")]
    pub creation_levels: Option<Vec<crate::destiny::definitions::DestinyItemCreationEntryLevelDefinition>>,

    /// This is an index specifically into the display category, as opposed to the server-side Categories (which do not need to match or pair with each other in any way: server side categories are really just structures for common validation. Display Category will let us more easily categorize items visually)
    #[serde(rename = "displayCategoryIndex")]
    pub display_category_index: i32,

    /// The index into the DestinyVendorDefinition.categories array, so you can find the category associated with this item.
    #[serde(rename = "categoryIndex")]
    pub category_index: i32,

    /// Same as above, but for the original category indexes.
    #[serde(rename = "originalCategoryIndex")]
    pub original_category_index: i32,

    /// The minimum character level at which this item is available for sale.
    #[serde(rename = "minimumLevel")]
    pub minimum_level: i32,

    /// The maximum character level at which this item is available for sale.
    #[serde(rename = "maximumLevel")]
    pub maximum_level: i32,

    /// The action to be performed when purchasing the item, if it's not just "buy".
    #[serde(rename = "action")]
    pub action: Option<crate::destiny::definitions::DestinyVendorSaleItemActionBlockDefinition>,

    /// The string identifier for the category selling this item.
    #[serde(rename = "displayCategory")]
    pub display_category: Option<String>,

    /// The inventory bucket into which this item will be placed upon purchase.
    #[serde(rename = "inventoryBucketHash")]
    pub inventory_bucket_hash: u32,

    /// The most restrictive scope that determines whether the item is available in the Vendor's inventory. See DestinyGatingScope's documentation for more information.
    /// This can be determined by Unlock gating, or by whether or not the item has purchase level requirements (minimumLevel and maximumLevel properties).
    #[serde(rename = "visibilityScope")]
    pub visibility_scope: crate::destiny::DestinyGatingScope,

    /// Similar to visibilityScope, it represents the most restrictive scope that determines whether the item can be purchased. It will at least be as restrictive as visibilityScope, but could be more restrictive if the item has additional purchase requirements beyond whether it is merely visible or not.
    /// See DestinyGatingScope's documentation for more information.
    #[serde(rename = "purchasableScope")]
    pub purchasable_scope: crate::destiny::DestinyGatingScope,

    /// If this item can only be purchased by a given platform, this indicates the platform to which it is restricted.
    #[serde(rename = "exclusivity")]
    pub exclusivity: crate::BungieMembershipType,

    /// If this sale can only be performed as the result of an offer check, this is true.
    #[serde(rename = "isOffer")]
    pub is_offer: Option<bool>,

    /// If this sale can only be performed as the result of receiving a CRM offer, this is true.
    #[serde(rename = "isCrm")]
    pub is_crm: Option<bool>,

    /// *if* the category this item is in supports non-default sorting, this value should represent the sorting value to use, pre-processed and ready to go.
    #[serde(rename = "sortValue")]
    pub sort_value: i32,

    /// If this item can expire, this is the tooltip message to show with its expiration info.
    #[serde(rename = "expirationTooltip")]
    pub expiration_tooltip: Option<String>,

    /// If this is populated, the purchase of this item should redirect to purchasing these other items instead.
    #[serde(rename = "redirectToSaleIndexes")]
    pub redirect_to_sale_indexes: Option<Vec<i32>>,

    #[serde(rename = "socketOverrides")]
    pub socket_overrides: Option<Vec<crate::destiny::definitions::DestinyVendorItemSocketOverride>>,

    /// If true, this item is some sort of dummy sale item that cannot actually be purchased. It may be a display only item, or some fluff left by a content designer for testing purposes, or something that got disabled because it was a terrible idea. You get the picture. We won't know *why* it can't be purchased, only that it can't be. Sorry.
    /// This is also only whether it's unpurchasable as a static property according to game content. There are other reasons why an item may or may not be purchasable at runtime, so even if this isn't set to True you should trust the runtime value for this sale item over the static definition if this is unset.
    #[serde(rename = "unpurchasable")]
    pub unpurchasable: Option<bool>,
}

/// In addition to item quantity information for vendor prices, this also has any optional information that may exist about how the item's quantity can be modified. (unfortunately not information that is able to be read outside of the BNet servers, but it's there)
#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorItemQuantity {
    /// The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// If this quantity is referring to a specific instance of an item, this will have the item's instance ID. Normally, this will be null.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: Option<i64>,

    /// The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used.
    #[serde(rename = "quantity")]
    pub quantity: i32,

    /// Indicates that this item quantity may be conditionally shown or hidden, based on various sources of state. For example: server flags, account state, or character progress.
    #[serde(rename = "hasConditionalVisibility")]
    pub has_conditional_visibility: bool,
}

/// An overly complicated wrapper for the item level at which the item should spawn.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemCreationEntryLevelDefinition {
    #[serde(rename = "level")]
    pub level: i32,
}

/// Not terribly useful, some basic cooldown interaction info.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorSaleItemActionBlockDefinition {
    #[serde(rename = "executeSeconds")]
    pub execute_seconds: f32,

    #[serde(rename = "isPositive")]
    pub is_positive: bool,
}

/// The information for how the vendor purchase should override a given socket with custom plug data.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorItemSocketOverride {
    /// If this is populated, the socket will be overridden with a specific plug.
    /// If this isn't populated, it's being overridden by something more complicated that is only known by the Game Server and God, which means we can't tell you in advance what it'll be.
    #[serde(rename = "singleItemHash")]
    pub single_item_hash: Option<u32>,

    /// If this is greater than -1, the number of randomized plugs on this socket will be set to this quantity instead of whatever it's set to by default.
    #[serde(rename = "randomizedOptionsCount")]
    pub randomized_options_count: i32,

    /// This appears to be used to select which socket ultimately gets the override defined here.
    #[serde(rename = "socketTypeHash")]
    pub socket_type_hash: u32,
}

/// When a vendor provides services, this is the localized name of those services.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorServiceDefinition {
    /// The localized name of a service provided.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// If you ever wondered how the Vault works, here it is.
/// The Vault is merely a set of inventory buckets that exist on your Profile/Account level. When you transfer items in the Vault, the game is using the Vault Vendor's DestinyVendorAcceptedItemDefinitions to see where the appropriate destination bucket is for the source bucket from whence your item is moving. If it finds such an entry, it transfers the item to the other bucket.
/// The mechanics for Postmaster works similarly, which is also a vendor. All driven by Accepted Items.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorAcceptedItemDefinition {
    /// The "source" bucket for a transfer. When a user wants to transfer an item, the appropriate DestinyVendorDefinition's acceptedItems property is evaluated, looking for an entry where acceptedInventoryBucketHash matches the bucket that the item being transferred is currently located. If it exists, the item will be transferred into whatever bucket is defined by destinationInventoryBucketHash.
    #[serde(rename = "acceptedInventoryBucketHash")]
    pub accepted_inventory_bucket_hash: u32,

    /// This is the bucket where the item being transferred will be put, given that it was being transferred *from* the bucket defined in acceptedInventoryBucketHash.
    #[serde(rename = "destinationInventoryBucketHash")]
    pub destination_inventory_bucket_hash: u32,
}

/// On to one of the more confusing subjects of the API. What is a Destination, and what is the relationship between it, Activities, Locations, and Places?
/// A "Destination" is a specific region/city/area of a larger "Place". For instance, a Place might be Earth where a Destination might be Bellevue, Washington. (Please, pick a more interesting destination if you come to visit Earth).
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyDestinationDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The place that "owns" this Destination. Use this hash to look up the DestinyPlaceDefinition.
    #[serde(rename = "placeHash")]
    pub place_hash: u32,

    /// If this Destination has a default Free-Roam activity, this is the hash for that Activity. Use it to look up the DestinyActivityDefintion.
    #[serde(rename = "defaultFreeroamActivityHash")]
    pub default_freeroam_activity_hash: u32,

    /// If the Destination has default Activity Graphs (i.e. "Map") that should be shown in the director, this is the list of those Graphs. At most, only one should be active at any given time for a Destination: these would represent, for example, different variants on a Map if the Destination is changing on a macro level based on game state.
    #[serde(rename = "activityGraphEntries")]
    pub activity_graph_entries: Option<Vec<crate::destiny::definitions::DestinyActivityGraphListEntryDefinition>>,

    /// A Destination may have many "Bubbles" zones with human readable properties.
    /// We don't get as much info as I'd like about them - I'd love to return info like where on the map they are located - but at least this gives you the name of those bubbles. bubbleSettings and bubbles both have the identical number of entries, and you should match up their indexes to provide matching bubble and bubbleSettings data.
    /// DEPRECATED - Just use bubbles, it now has this data.
    #[serde(rename = "bubbleSettings")]
    pub bubble_settings: Option<Vec<crate::destiny::definitions::DestinyDestinationBubbleSettingDefinition>>,

    /// This provides the unique identifiers for every bubble in the destination (only guaranteed unique within the destination), and any intrinsic properties of the bubble.
    /// bubbleSettings and bubbles both have the identical number of entries, and you should match up their indexes to provide matching bubble and bubbleSettings data.
    #[serde(rename = "bubbles")]
    pub bubbles: Option<Vec<crate::destiny::definitions::DestinyBubbleDefinition>>,

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

/// Destinations and Activities may have default Activity Graphs that should be shown when you bring up the Director and are playing in either.
/// This contract defines the graph referred to and the gating for when it is relevant.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityGraphListEntryDefinition {
    /// The hash identifier of the DestinyActivityGraphDefinition that should be shown when opening the director.
    #[serde(rename = "activityGraphHash")]
    pub activity_graph_hash: u32,
}

/// The static data about Activities in Destiny 2.
/// Note that an Activity must be combined with an ActivityMode to know - from a Gameplay perspective - what the user is "Playing".
/// In most PvE activities, this is fairly straightforward. A Story Activity can only be played in the Story Activity Mode.
/// However, in PvP activities, the Activity alone only tells you the map being played, or the Playlist that the user chose to enter. You'll need to know the Activity Mode they're playing to know that they're playing Mode X on Map Y.
/// Activity Definitions tell a great deal of information about what *could* be relevant to a user: what rewards they can earn, what challenges could be performed, what modifiers could be applied. To figure out which of these properties is actually live, you'll need to combine the definition with "Live" data from one of the Destiny endpoints.
/// Activities also have Activity Types, but unfortunately in Destiny 2 these are even less reliable of a source of information than they were in Destiny 1. I will be looking into ways to provide more reliable sources for type information as time goes on, but for now we're going to have to deal with the limitations. See DestinyActivityTypeDefinition for more information.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityDefinition {
    /// The title, subtitle, and icon for the activity. We do a little post-processing on this to try and account for Activities where the designers have left this data too minimal to determine what activity is actually being played.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The unadulterated form of the display properties, as they ought to be shown in the Director (if the activity appears in the director).
    #[serde(rename = "originalDisplayProperties")]
    pub original_display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The title, subtitle, and icon for the activity as determined by Selection Screen data, if there is any for this activity. There won't be data in this field if the activity is never shown in a selection/options screen.
    #[serde(rename = "selectionScreenDisplayProperties")]
    pub selection_screen_display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If the activity has an icon associated with a specific release (such as a DLC), this is the path to that release's icon.
    #[serde(rename = "releaseIcon")]
    pub release_icon: Option<String>,

    /// If the activity will not be visible until a specific and known time, this will be the seconds since the Epoch when it will become visible.
    #[serde(rename = "releaseTime")]
    pub release_time: i32,

    /// The recommended light level for this activity.
    #[serde(rename = "activityLightLevel")]
    pub activity_light_level: i32,

    /// The hash identifier for the Destination on which this Activity is played. Use it to look up the DestinyDestinationDefinition for human readable info about the destination. A Destination can be thought of as a more specific location than a "Place". For instance, if the "Place" is Earth, the "Destination" would be a specific city or region on Earth.
    #[serde(rename = "destinationHash")]
    pub destination_hash: u32,

    /// The hash identifier for the "Place" on which this Activity is played. Use it to look up the DestinyPlaceDefinition for human readable info about the Place. A Place is the largest-scoped concept for location information. For instance, if the "Place" is Earth, the "Destination" would be a specific city or region on Earth.
    #[serde(rename = "placeHash")]
    pub place_hash: u32,

    /// The hash identifier for the Activity Type of this Activity. You may use it to look up the DestinyActivityTypeDefinition for human readable info, but be forewarned: Playlists and many PVP Map Activities will map to generic Activity Types. You'll have to use your knowledge of the Activity Mode being played to get more specific information about what the user is playing.
    #[serde(rename = "activityTypeHash")]
    pub activity_type_hash: u32,

    /// The difficulty tier of the activity.
    #[serde(rename = "tier")]
    pub tier: i32,

    /// When Activities are completed, we generate a "Post-Game Carnage Report", or PGCR, with details about what happened in that activity (how many kills someone got, which team won, etc...) We use this image as the background when displaying PGCR information, and often use it when we refer to the Activity in general.
    #[serde(rename = "pgcrImage")]
    pub pgcr_image: Option<String>,

    /// The expected possible rewards for the activity. These rewards may or may not be accessible for an individual player based on their character state, the account state, and even the game's state overall. But it is a useful reference for possible rewards you can earn in the activity. These match up to rewards displayed when you hover over the Activity in the in-game Director, and often refer to Placeholder or "Dummy" items: items that tell you what you can earn in vague terms rather than what you'll specifically be earning (partly because the game doesn't even know what you'll earn specifically until you roll for it at the end)
    #[serde(rename = "rewards")]
    pub rewards: Option<Vec<crate::destiny::definitions::DestinyActivityRewardDefinition>>,

    /// Activities can have Modifiers, as defined in DestinyActivityModifierDefinition. These are references to the modifiers that *can* be applied to that activity, along with data that we use to determine if that modifier is actually active at any given point in time.
    #[serde(rename = "modifiers")]
    pub modifiers: Option<Vec<crate::destiny::definitions::DestinyActivityModifierReferenceDefinition>>,

    /// If True, this Activity is actually a Playlist that refers to multiple possible specific Activities and Activity Modes. For instance, a Crucible Playlist may have references to multiple Activities (Maps) with multiple Activity Modes (specific PvP gameplay modes). If this is true, refer to the playlistItems property for the specific entries in the playlist.
    #[serde(rename = "isPlaylist")]
    pub is_playlist: bool,

    /// An activity can have many Challenges, of which any subset of them may be active for play at any given period of time. This gives the information about the challenges and data that we use to understand when they're active and what rewards they provide. Sadly, at the moment there's no central definition for challenges: much like "Skulls" were in Destiny 1, these are defined on individual activities and there can be many duplicates/near duplicates across the Destiny 2 ecosystem. I have it in mind to centralize these in a future revision of the API, but we are out of time.
    #[serde(rename = "challenges")]
    pub challenges: Option<Vec<crate::destiny::definitions::DestinyActivityChallengeDefinition>>,

    /// If there are status strings related to the activity and based on internal state of the game, account, or character, then this will be the definition of those strings and the states needed in order for the strings to be shown.
    #[serde(rename = "optionalUnlockStrings")]
    pub optional_unlock_strings: Option<Vec<crate::destiny::definitions::DestinyActivityUnlockStringDefinition>>,

    /// Represents all of the possible activities that could be played in the Playlist, along with information that we can use to determine if they are active at the present time.
    #[serde(rename = "playlistItems")]
    pub playlist_items: Option<Vec<crate::destiny::definitions::DestinyActivityPlaylistItemDefinition>>,

    /// Unfortunately, in practice this is almost never populated. In theory, this is supposed to tell which Activity Graph to show if you bring up the director while in this activity.
    #[serde(rename = "activityGraphList")]
    pub activity_graph_list: Option<Vec<crate::destiny::definitions::DestinyActivityGraphListEntryDefinition>>,

    /// This block of data provides information about the Activity's matchmaking attributes: how many people can join and such.
    #[serde(rename = "matchmaking")]
    pub matchmaking: Option<crate::destiny::definitions::DestinyActivityMatchmakingBlockDefinition>,

    /// This block of data, if it exists, provides information about the guided game experience and restrictions for this activity. If it doesn't exist, the game is not able to be played as a guided game.
    #[serde(rename = "guidedGame")]
    pub guided_game: Option<crate::destiny::definitions::DestinyActivityGuidedBlockDefinition>,

    /// If this activity had an activity mode directly defined on it, this will be the hash of that mode.
    #[serde(rename = "directActivityModeHash")]
    pub direct_activity_mode_hash: Option<u32>,

    /// If the activity had an activity mode directly defined on it, this will be the enum value of that mode.
    #[serde(rename = "directActivityModeType")]
    pub direct_activity_mode_type: Option<i32>,

    /// The set of all possible loadout requirements that could be active for this activity. Only one will be active at any given time, and you can discover which one through activity-associated data such as Milestones that have activity info on them.
    #[serde(rename = "loadouts")]
    pub loadouts: Option<Vec<crate::destiny::definitions::DestinyActivityLoadoutRequirementSet>>,

    /// The hash identifiers for Activity Modes relevant to this activity.  Note that if this is a playlist, the specific playlist entry chosen will determine the actual activity modes that end up being relevant.
    #[serde(rename = "activityModeHashes")]
    pub activity_mode_hashes: Option<Vec<u32>>,

    /// The activity modes - if any - in enum form. Because we can't seem to escape the enums.
    #[serde(rename = "activityModeTypes")]
    pub activity_mode_types: Option<Vec<crate::destiny::historical_stats::definitions::DestinyActivityModeType>>,

    /// If true, this activity is a PVP activity or playlist.
    #[serde(rename = "isPvP")]
    pub is_pv_p: bool,

    /// The list of phases or points of entry into an activity, along with information we can use to determine their gating and availability.
    #[serde(rename = "insertionPoints")]
    pub insertion_points: Option<Vec<crate::destiny::definitions::DestinyActivityInsertionPointDefinition>>,

    /// A list of location mappings that are affected by this activity. Pulled out of DestinyLocationDefinitions for our/your lookup convenience.
    #[serde(rename = "activityLocationMappings")]
    pub activity_location_mappings: Option<Vec<crate::destiny::constants::DestinyEnvironmentLocationMapping>>,

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

/// Activities can refer to one or more sets of tooltip-friendly reward data. These are the definitions for those tooltip friendly rewards.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityRewardDefinition {
    /// The header for the reward set, if any.
    #[serde(rename = "rewardText")]
    pub reward_text: Option<String>,

    /// The "Items provided" in the reward. This is almost always a pointer to a DestinyInventoryItemDefintion for an item that you can't actually earn in-game, but that has name/description/icon information for the vague concept of the rewards you will receive. This is because the actual reward generation is non-deterministic and extremely complicated, so the best the game can do is tell you what you'll get in vague terms. And so too shall we.
    /// Interesting trivia: you actually *do* earn these items when you complete the activity. They go into a single-slot bucket on your profile, which is how you see the pop-ups of these rewards when you complete an activity that match these "dummy" items. You can even see them if you look at the last one you earned in your profile-level inventory through the BNet API! Who said reading documentation is a waste of time?
    #[serde(rename = "rewardItems")]
    pub reward_items: Option<Vec<crate::destiny::DestinyItemQuantity>>,
}

/// A reference to an Activity Modifier from another entity, such as an Activity (for now, just Activities).
/// This defines some
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityModifierReferenceDefinition {
    /// The hash identifier for the DestinyActivityModifierDefinition referenced by this activity.
    #[serde(rename = "activityModifierHash")]
    pub activity_modifier_hash: u32,
}

/// Represents a reference to a Challenge, which for now is just an Objective.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityChallengeDefinition {
    /// The hash for the Objective that matches this challenge. Use it to look up the DestinyObjectiveDefinition.
    #[serde(rename = "objectiveHash")]
    pub objective_hash: u32,

    /// The rewards as they're represented in the UI. Note that they generally link to "dummy" items that give a summary of rewards rather than direct, real items themselves.
    /// If the quantity is 0, don't show the quantity.
    #[serde(rename = "dummyRewards")]
    pub dummy_rewards: Option<Vec<crate::destiny::DestinyItemQuantity>>,
}

/// Defines an "Objective".
/// An objective is a specific task you should accomplish in the game. These are referred to by:
/// - Quest Steps (which are DestinyInventoryItemDefinition entities with Objectives)
/// - Challenges (which are Objectives defined on an DestinyActivityDefintion)
/// - Milestones (which refer to Objectives that are defined on both Quest Steps and Activities)
/// - Anything else that the designers decide to do later.
/// Objectives have progress, a notion of having been Completed, human readable data describing the task to be accomplished, and a lot of optional tack-on data that can enhance the information provided about the task.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyObjectiveDefinition {
    /// Ideally, this should tell you what your task is. I'm not going to lie to you though. Sometimes this doesn't have useful information at all. Which sucks, but there's nothing either of us can do about it.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The value that the unlock value defined in unlockValueHash must reach in order for the objective to be considered Completed. Used in calculating progress and completion status.
    #[serde(rename = "completionValue")]
    pub completion_value: i32,

    /// A shortcut for determining the most restrictive gating that this Objective is set to use. This includes both the dynamic determination of progress and of completion values. See the DestinyGatingScope enum's documentation for more details.
    #[serde(rename = "scope")]
    pub scope: crate::destiny::DestinyGatingScope,

    /// OPTIONAL: a hash identifier for the location at which this objective must be accomplished, if there is a location defined. Look up the DestinyLocationDefinition for this hash for that additional location info.
    #[serde(rename = "locationHash")]
    pub location_hash: u32,

    /// If true, the value is allowed to go negative.
    #[serde(rename = "allowNegativeValue")]
    pub allow_negative_value: bool,

    /// If true, you can effectively "un-complete" this objective if you lose progress after crossing the completion threshold.
    /// If False, once you complete the task it will remain completed forever by locking the value.
    #[serde(rename = "allowValueChangeWhenCompleted")]
    pub allow_value_change_when_completed: bool,

    /// If true, completion means having an unlock value less than or equal to the completionValue.
    /// If False, completion means having an unlock value greater than or equal to the completionValue.
    #[serde(rename = "isCountingDownward")]
    pub is_counting_downward: bool,

    /// The UI style applied to the objective. It's an enum, take a look at DestinyUnlockValueUIStyle for details of the possible styles. Use this info as you wish to customize your UI.
    /// DEPRECATED: This is no longer populated by Destiny 2 game content. Please use inProgressValueStyle and completedValueStyle instead.
    #[serde(rename = "valueStyle")]
    pub value_style: crate::destiny::DestinyUnlockValueUIStyle,

    /// Text to describe the progress bar.
    #[serde(rename = "progressDescription")]
    pub progress_description: Option<String>,

    /// If this objective enables Perks intrinsically, the conditions for that enabling are defined here.
    #[serde(rename = "perks")]
    pub perks: Option<crate::destiny::definitions::DestinyObjectivePerkEntryDefinition>,

    /// If this objective enables modifications on a player's stats intrinsically, the conditions are defined here.
    #[serde(rename = "stats")]
    pub stats: Option<crate::destiny::definitions::DestinyObjectiveStatEntryDefinition>,

    /// If nonzero, this is the minimum value at which the objective's progression should be shown. Otherwise, don't show it yet.
    #[serde(rename = "minimumVisibilityThreshold")]
    pub minimum_visibility_threshold: i32,

    /// If True, the progress will continue even beyond the point where the objective met its minimum completion requirements. Your UI will have to accommodate it.
    #[serde(rename = "allowOvercompletion")]
    pub allow_overcompletion: bool,

    /// If True, you should continue showing the progression value in the UI after it's complete. I mean, we already do that in BNet anyways, but if you want to be better behaved than us you could honor this flag.
    #[serde(rename = "showValueOnComplete")]
    pub show_value_on_complete: bool,

    /// The style to use when the objective is completed.
    #[serde(rename = "completedValueStyle")]
    pub completed_value_style: crate::destiny::DestinyUnlockValueUIStyle,

    /// The style to use when the objective is still in progress.
    #[serde(rename = "inProgressValueStyle")]
    pub in_progress_value_style: crate::destiny::DestinyUnlockValueUIStyle,

    /// Objectives can have arbitrary UI-defined identifiers that define the style applied to objectives. For convenience, known UI labels will be defined in the uiStyle enum value.
    #[serde(rename = "uiLabel")]
    pub ui_label: Option<String>,

    /// If the objective has a known UI label value, this property will represent it.
    #[serde(rename = "uiStyle")]
    pub ui_style: crate::destiny::DestinyObjectiveUiStyle,

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

/// Defines the conditions under which an intrinsic perk is applied while participating in an Objective.
/// These perks will generally not be benefit-granting perks, but rather a perk that modifies gameplay in some interesting way.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyObjectivePerkEntryDefinition {
    /// The hash identifier of the DestinySandboxPerkDefinition that will be applied to the character.
    #[serde(rename = "perkHash")]
    pub perk_hash: u32,

    /// An enumeration indicating whether it will be applied as long as the Objective is active, when it's completed, or until it's completed.
    #[serde(rename = "style")]
    pub style: crate::destiny::DestinyObjectiveGrantStyle,
}

/// Perks are modifiers to a character or item that can be applied situationally.
/// - Perks determine a weapons' damage type.
/// - Perks put the Mods in Modifiers (they are literally the entity that bestows the Sandbox benefit for whatever fluff text about the modifier in the Socket, Plug or Talent Node)
/// - Perks are applied for unique alterations of state in Objectives
/// Anyways, I'm sure you can see why perks are so interesting.
/// What Perks often don't have is human readable information, so we attempt to reverse engineer that by pulling that data from places that uniquely refer to these perks: namely, Talent Nodes and Plugs. That only gives us a subset of perks that are human readable, but those perks are the ones people generally care about anyways. The others are left as a mystery, their true purpose mostly unknown and undocumented.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinySandboxPerkDefinition {
    /// These display properties are by no means guaranteed to be populated. Usually when it is, it's only because we back-filled them with the displayProperties of some Talent Node or Plug item that happened to be uniquely providing that perk.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The string identifier for the perk.
    #[serde(rename = "perkIdentifier")]
    pub perk_identifier: Option<String>,

    /// If true, you can actually show the perk in the UI. Otherwise, it doesn't have useful player-facing information.
    #[serde(rename = "isDisplayable")]
    pub is_displayable: bool,

    /// If this perk grants a damage type to a weapon, the damage type will be defined here.
    /// Unless you have a compelling reason to use this enum value, use the damageTypeHash instead to look up the actual DestinyDamageTypeDefinition.
    #[serde(rename = "damageType")]
    pub damage_type: crate::destiny::DamageType,

    /// The hash identifier for looking up the DestinyDamageTypeDefinition, if this perk has a damage type.
    /// This is preferred over using the damageType enumeration value, which has been left purely because it is occasionally convenient.
    #[serde(rename = "damageTypeHash")]
    pub damage_type_hash: Option<u32>,

    /// An old holdover from the original Armory, this was an attempt to group perks by functionality.
    /// It is as yet unpopulated, and there will be quite a bit of work needed to restore it to its former working order.
    #[serde(rename = "perkGroups")]
    pub perk_groups: Option<crate::destiny::definitions::DestinyTalentNodeStepGroups>,

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

/// These properties are an attempt to categorize talent node steps by certain common properties. See the related enumerations for the type of properties being categorized.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyTalentNodeStepGroups {
    #[serde(rename = "weaponPerformance")]
    pub weapon_performance: enumflags2::BitFlags<crate::destiny::definitions::DestinyTalentNodeStepWeaponPerformances>,

    #[serde(rename = "impactEffects")]
    pub impact_effects: enumflags2::BitFlags<crate::destiny::definitions::DestinyTalentNodeStepImpactEffects>,

    #[serde(rename = "guardianAttributes")]
    pub guardian_attributes: enumflags2::BitFlags<crate::destiny::definitions::DestinyTalentNodeStepGuardianAttributes>,

    #[serde(rename = "lightAbilities")]
    pub light_abilities: enumflags2::BitFlags<crate::destiny::definitions::DestinyTalentNodeStepLightAbilities>,

    #[serde(rename = "damageTypes")]
    pub damage_types: enumflags2::BitFlags<crate::destiny::definitions::DestinyTalentNodeStepDamageTypes>,
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinyTalentNodeStepWeaponPerformances {
    RateOfFire = 1,
    Damage = 2,
    Accuracy = 4,
    Range = 8,
    Zoom = 16,
    Recoil = 32,
    Ready = 64,
    Reload = 128,
    HairTrigger = 256,
    AmmoAndMagazine = 512,
    TrackingAndDetonation = 1024,
    ShotgunSpread = 2048,
    ChargeTime = 4096,
}

impl Display for DestinyTalentNodeStepWeaponPerformances {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for DestinyTalentNodeStepWeaponPerformances {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "RateOfFire" => Ok(DestinyTalentNodeStepWeaponPerformances::RateOfFire),
            "Damage" => Ok(DestinyTalentNodeStepWeaponPerformances::Damage),
            "Accuracy" => Ok(DestinyTalentNodeStepWeaponPerformances::Accuracy),
            "Range" => Ok(DestinyTalentNodeStepWeaponPerformances::Range),
            "Zoom" => Ok(DestinyTalentNodeStepWeaponPerformances::Zoom),
            "Recoil" => Ok(DestinyTalentNodeStepWeaponPerformances::Recoil),
            "Ready" => Ok(DestinyTalentNodeStepWeaponPerformances::Ready),
            "Reload" => Ok(DestinyTalentNodeStepWeaponPerformances::Reload),
            "HairTrigger" => Ok(DestinyTalentNodeStepWeaponPerformances::HairTrigger),
            "AmmoAndMagazine" => Ok(DestinyTalentNodeStepWeaponPerformances::AmmoAndMagazine),
            "TrackingAndDetonation" => Ok(DestinyTalentNodeStepWeaponPerformances::TrackingAndDetonation),
            "ShotgunSpread" => Ok(DestinyTalentNodeStepWeaponPerformances::ShotgunSpread),
            "ChargeTime" => Ok(DestinyTalentNodeStepWeaponPerformances::ChargeTime),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyTalentNodeStepWeaponPerformances", s)),
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinyTalentNodeStepImpactEffects {
    ArmorPiercing = 1,
    Ricochet = 2,
    Flinch = 4,
    CollateralDamage = 8,
    Disorient = 16,
    HighlightTarget = 32,
}

impl Display for DestinyTalentNodeStepImpactEffects {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for DestinyTalentNodeStepImpactEffects {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "ArmorPiercing" => Ok(DestinyTalentNodeStepImpactEffects::ArmorPiercing),
            "Ricochet" => Ok(DestinyTalentNodeStepImpactEffects::Ricochet),
            "Flinch" => Ok(DestinyTalentNodeStepImpactEffects::Flinch),
            "CollateralDamage" => Ok(DestinyTalentNodeStepImpactEffects::CollateralDamage),
            "Disorient" => Ok(DestinyTalentNodeStepImpactEffects::Disorient),
            "HighlightTarget" => Ok(DestinyTalentNodeStepImpactEffects::HighlightTarget),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyTalentNodeStepImpactEffects", s)),
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinyTalentNodeStepGuardianAttributes {
    Stats = 1,
    Shields = 2,
    Health = 4,
    Revive = 8,
    AimUnderFire = 16,
    Radar = 32,
    Invisibility = 64,
    Reputations = 128,
}

impl Display for DestinyTalentNodeStepGuardianAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for DestinyTalentNodeStepGuardianAttributes {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Stats" => Ok(DestinyTalentNodeStepGuardianAttributes::Stats),
            "Shields" => Ok(DestinyTalentNodeStepGuardianAttributes::Shields),
            "Health" => Ok(DestinyTalentNodeStepGuardianAttributes::Health),
            "Revive" => Ok(DestinyTalentNodeStepGuardianAttributes::Revive),
            "AimUnderFire" => Ok(DestinyTalentNodeStepGuardianAttributes::AimUnderFire),
            "Radar" => Ok(DestinyTalentNodeStepGuardianAttributes::Radar),
            "Invisibility" => Ok(DestinyTalentNodeStepGuardianAttributes::Invisibility),
            "Reputations" => Ok(DestinyTalentNodeStepGuardianAttributes::Reputations),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyTalentNodeStepGuardianAttributes", s)),
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinyTalentNodeStepLightAbilities {
    Grenades = 1,
    Melee = 2,
    MovementModes = 4,
    Orbs = 8,
    SuperEnergy = 16,
    SuperMods = 32,
}

impl Display for DestinyTalentNodeStepLightAbilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for DestinyTalentNodeStepLightAbilities {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Grenades" => Ok(DestinyTalentNodeStepLightAbilities::Grenades),
            "Melee" => Ok(DestinyTalentNodeStepLightAbilities::Melee),
            "MovementModes" => Ok(DestinyTalentNodeStepLightAbilities::MovementModes),
            "Orbs" => Ok(DestinyTalentNodeStepLightAbilities::Orbs),
            "SuperEnergy" => Ok(DestinyTalentNodeStepLightAbilities::SuperEnergy),
            "SuperMods" => Ok(DestinyTalentNodeStepLightAbilities::SuperMods),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyTalentNodeStepLightAbilities", s)),
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinyTalentNodeStepDamageTypes {
    Kinetic = 1,
    Arc = 2,
    Solar = 4,
    Void = 8,
}

impl Display for DestinyTalentNodeStepDamageTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for DestinyTalentNodeStepDamageTypes {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Kinetic" => Ok(DestinyTalentNodeStepDamageTypes::Kinetic),
            "Arc" => Ok(DestinyTalentNodeStepDamageTypes::Arc),
            "Solar" => Ok(DestinyTalentNodeStepDamageTypes::Solar),
            "Void" => Ok(DestinyTalentNodeStepDamageTypes::Void),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyTalentNodeStepDamageTypes", s)),
        }
    }
}

/// All damage types that are possible in the game are defined here, along with localized info and icons as needed.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyDamageTypeDefinition {
    /// The description of the damage type, icon etc...
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// A variant of the icon that is transparent and colorless.
    #[serde(rename = "transparentIconPath")]
    pub transparent_icon_path: Option<String>,

    /// If TRUE, the game shows this damage type's icon. Otherwise, it doesn't. Whether you show it or not is up to you.
    #[serde(rename = "showIcon")]
    pub show_icon: bool,

    /// We have an enumeration for damage types for quick reference. This is the current definition's damage type enum value.
    #[serde(rename = "enumValue")]
    pub enum_value: crate::destiny::DamageType,

    /// A color associated with the damage type. The displayProperties icon is tinted with a color close to this.
    #[serde(rename = "color")]
    pub color: Option<crate::destiny::misc::DestinyColor>,

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

/// Defines the conditions under which stat modifications will be applied to a Character while participating in an objective.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyObjectiveStatEntryDefinition {
    /// The stat being modified, and the value used.
    #[serde(rename = "stat")]
    pub stat: Option<crate::destiny::definitions::DestinyItemInvestmentStatDefinition>,

    /// Whether it will be applied as long as the objective is active, when it's completed, or until it's completed.
    #[serde(rename = "style")]
    pub style: crate::destiny::DestinyObjectiveGrantStyle,
}

/// Represents a "raw" investment stat, before calculated stats are calculated and before any DestinyStatGroupDefinition is applied to transform the stat into something closer to what you see in-game.
/// Because these won't match what you see in-game, consider carefully whether you really want to use these stats. I have left them in case someone can do something useful or interesting with the pre-processed statistics.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemInvestmentStatDefinition {
    /// The hash identifier for the DestinyStatDefinition defining this stat.
    #[serde(rename = "statTypeHash")]
    pub stat_type_hash: u32,

    /// The raw "Investment" value for the stat, before transformations are performed to turn this raw stat into stats that are displayed in the game UI.
    #[serde(rename = "value")]
    pub value: i32,

    /// If this is true, the stat will only be applied on the item in certain game state conditions, and we can't know statically whether or not this stat will be applied. Check the "live" API data instead for whether this value is being applied on a specific instance of the item in question, and you can use this to decide whether you want to show the stat on the generic view of the item, or whether you want to show some kind of caveat or warning about the stat value being conditional on game state.
    #[serde(rename = "isConditionallyActive")]
    pub is_conditionally_active: bool,
}

/// A "Location" is a sort of shortcut for referring to a specific combination of Activity, Destination, Place, and even Bubble or NavPoint within a space.
/// Most of this data isn't intrinsically useful to us, but Objectives refer to locations, and through that we can at least infer the Activity, Destination, and Place being referred to by the Objective.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLocationDefinition {
    /// If the location has a Vendor on it, this is the hash identifier for that Vendor. Look them up with DestinyVendorDefinition.
    #[serde(rename = "vendorHash")]
    pub vendor_hash: u32,

    /// A Location may refer to different specific spots in the world based on the world's current state. This is a list of those potential spots, and the data we can use at runtime to determine which one of the spots is the currently valid one.
    #[serde(rename = "locationReleases")]
    pub location_releases: Option<Vec<crate::destiny::definitions::DestinyLocationReleaseDefinition>>,

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

/// A specific "spot" referred to by a location. Only one of these can be active at a time for a given Location.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLocationReleaseDefinition {
    /// Sadly, these don't appear to be populated anymore (ever?)
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "smallTransparentIcon")]
    pub small_transparent_icon: Option<String>,

    #[serde(rename = "mapIcon")]
    pub map_icon: Option<String>,

    #[serde(rename = "largeTransparentIcon")]
    pub large_transparent_icon: Option<String>,

    /// If we had map information, this spawnPoint would be interesting. But sadly, we don't have that info.
    #[serde(rename = "spawnPoint")]
    pub spawn_point: u32,

    /// The Destination being pointed to by this location.
    #[serde(rename = "destinationHash")]
    pub destination_hash: u32,

    /// The Activity being pointed to by this location.
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,

    /// The Activity Graph being pointed to by this location.
    #[serde(rename = "activityGraphHash")]
    pub activity_graph_hash: u32,

    /// The Activity Graph Node being pointed to by this location. (Remember that Activity Graph Node hashes are only unique within an Activity Graph: so use the combination to find the node being spoken of)
    #[serde(rename = "activityGraphNodeHash")]
    pub activity_graph_node_hash: u32,

    /// The Activity Bubble within the Destination. Look this up in the DestinyDestinationDefinition's bubbles and bubbleSettings properties.
    #[serde(rename = "activityBubbleName")]
    pub activity_bubble_name: u32,

    /// If we had map information, this would tell us something cool about the path this location wants you to take. I wish we had map information.
    #[serde(rename = "activityPathBundle")]
    pub activity_path_bundle: u32,

    /// If we had map information, this would tell us about path information related to destination on the map. Sad. Maybe you can do something cool with it. Go to town man.
    #[serde(rename = "activityPathDestination")]
    pub activity_path_destination: u32,

    /// The type of Nav Point that this represents. See the enumeration for more info.
    #[serde(rename = "navPointType")]
    pub nav_point_type: crate::destiny::DestinyActivityNavPointType,

    /// Looks like it should be the position on the map, but sadly it does not look populated... yet?
    #[serde(rename = "worldPosition")]
    pub world_position: Option<Vec<i32>>,
}

/// Represents a status string that could be conditionally displayed about an activity. Note that externally, you can only see the strings themselves. Internally we combine this information with server state to determine which strings should be shown.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityUnlockStringDefinition {
    /// The string to be displayed if the conditions are met.
    #[serde(rename = "displayString")]
    pub display_string: Option<String>,
}

/// If the activity is a playlist, this is the definition for a specific entry in the playlist: a single possible combination of Activity and Activity Mode that can be chosen.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityPlaylistItemDefinition {
    /// The hash identifier of the Activity that can be played. Use it to look up the DestinyActivityDefinition.
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,

    /// If this playlist entry had an activity mode directly defined on it, this will be the hash of that mode.
    #[serde(rename = "directActivityModeHash")]
    pub direct_activity_mode_hash: Option<u32>,

    /// If the playlist entry had an activity mode directly defined on it, this will be the enum value of that mode.
    #[serde(rename = "directActivityModeType")]
    pub direct_activity_mode_type: Option<i32>,

    /// The hash identifiers for Activity Modes relevant to this entry.
    #[serde(rename = "activityModeHashes")]
    pub activity_mode_hashes: Option<Vec<u32>>,

    /// The activity modes - if any - in enum form. Because we can't seem to escape the enums.
    #[serde(rename = "activityModeTypes")]
    pub activity_mode_types: Option<Vec<crate::destiny::historical_stats::definitions::DestinyActivityModeType>>,
}

/// This definition represents an "Activity Mode" as it exists in the Historical Stats endpoints. An individual Activity Mode represents a collection of activities that are played in a certain way. For example, Nightfall Strikes are part of a "Nightfall" activity mode, and any activities played as the PVP mode "Clash" are part of the "Clash activity mode.
/// Activity modes are nested under each other in a hierarchy, so that if you ask for - for example - "AllPvP", you will get any PVP activities that the user has played, regardless of what specific PVP mode was being played.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityModeDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If this activity mode has a related PGCR image, this will be the path to said image.
    #[serde(rename = "pgcrImage")]
    pub pgcr_image: Option<String>,

    /// The Enumeration value for this Activity Mode. Pass this identifier into Stats endpoints to get aggregate stats for this mode.
    #[serde(rename = "modeType")]
    pub mode_type: crate::destiny::historical_stats::definitions::DestinyActivityModeType,

    /// The type of play being performed in broad terms (PVP, PVE)
    #[serde(rename = "activityModeCategory")]
    pub activity_mode_category: crate::destiny::DestinyActivityModeCategory,

    /// If True, this mode has oppositional teams fighting against each other rather than "Free-For-All" or Co-operative modes of play.
    /// Note that Aggregate modes are never marked as team based, even if they happen to be team based at the moment. At any time, an aggregate whose subordinates are only team based could be changed so that one or more aren't team based, and then this boolean won't make much sense (the aggregation would become "sometimes team based"). Let's not deal with that right now.
    #[serde(rename = "isTeamBased")]
    pub is_team_based: bool,

    /// If true, this mode is an aggregation of other, more specific modes rather than being a mode in itself. This includes modes that group Features/Events rather than Gameplay, such as Trials of The Nine: Trials of the Nine being an Event that is interesting to see aggregate data for, but when you play the activities within Trials of the Nine they are more specific activity modes such as Clash.
    #[serde(rename = "isAggregateMode")]
    pub is_aggregate_mode: bool,

    /// The hash identifiers of the DestinyActivityModeDefinitions that represent all of the "parent" modes for this mode. For instance, the Nightfall Mode is also a member of AllStrikes and AllPvE.
    #[serde(rename = "parentHashes")]
    pub parent_hashes: Option<Vec<u32>>,

    /// A Friendly identifier you can use for referring to this Activity Mode. We really only used this in our URLs, so... you know, take that for whatever it's worth.
    #[serde(rename = "friendlyName")]
    pub friendly_name: Option<String>,

    /// If this exists, the mode has specific Activities (referred to by the Key) that should instead map to other Activity Modes when they are played. This was useful in D1 for Private Matches, where we wanted to have Private Matches as an activity mode while still referring to the specific mode being played.
    #[serde(rename = "activityModeMappings")]
    pub activity_mode_mappings: Option<HashMap<u32, crate::destiny::historical_stats::definitions::DestinyActivityModeType>>,

    /// If FALSE, we want to ignore this type when we're showing activity modes in BNet UI. It will still be returned in case 3rd parties want to use it for any purpose.
    #[serde(rename = "display")]
    pub display: bool,

    /// The relative ordering of activity modes.
    #[serde(rename = "order")]
    pub order: i32,

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

/// Information about matchmaking and party size for the activity.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityMatchmakingBlockDefinition {
    /// If TRUE, the activity is matchmade. Otherwise, it requires explicit forming of a party.
    #[serde(rename = "isMatchmade")]
    pub is_matchmade: bool,

    /// The minimum # of people in the fireteam for the activity to launch.
    #[serde(rename = "minParty")]
    pub min_party: i32,

    /// The maximum # of people allowed in a Fireteam.
    #[serde(rename = "maxParty")]
    pub max_party: i32,

    /// The maximum # of people allowed across all teams in the activity.
    #[serde(rename = "maxPlayers")]
    pub max_players: i32,

    /// If true, you have to Solemnly Swear to be up to Nothing But Good(tm) to play.
    #[serde(rename = "requiresGuardianOath")]
    pub requires_guardian_oath: bool,
}

/// Guided Game information for this activity.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityGuidedBlockDefinition {
    /// The maximum amount of people that can be in the waiting lobby.
    #[serde(rename = "guidedMaxLobbySize")]
    pub guided_max_lobby_size: i32,

    /// The minimum amount of people that can be in the waiting lobby.
    #[serde(rename = "guidedMinLobbySize")]
    pub guided_min_lobby_size: i32,

    /// If -1, the guided group cannot be disbanded. Otherwise, take the total # of players in the activity and subtract this number: that is the total # of votes needed for the guided group to disband.
    #[serde(rename = "guidedDisbandCount")]
    pub guided_disband_count: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityLoadoutRequirementSet {
    /// The set of requirements that will be applied on the activity if this requirement set is active.
    #[serde(rename = "requirements")]
    pub requirements: Option<Vec<crate::destiny::definitions::DestinyActivityLoadoutRequirement>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityLoadoutRequirement {
    #[serde(rename = "equipmentSlotHash")]
    pub equipment_slot_hash: u32,

    #[serde(rename = "allowedEquippedItemHashes")]
    pub allowed_equipped_item_hashes: Option<Vec<u32>>,

    #[serde(rename = "allowedWeaponSubTypes")]
    pub allowed_weapon_sub_types: Option<Vec<crate::destiny::DestinyItemSubType>>,
}

/// A point of entry into an activity, gated by an unlock flag and with some more-or-less useless (for our purposes) phase information. I'm including it in case we end up being able to bolt more useful information onto it in the future.
/// UPDATE: Turns out this information isn't actually useless, and is in fact actually useful for people. Who would have thought? We still don't have localized info for it, but at least this will help people when they're looking at phase indexes in stats data, or when they want to know what phases have been completed on a weekly achievement.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityInsertionPointDefinition {
    /// A unique hash value representing the phase. This can be useful for, for example, comparing how different instances of Raids have phases in different orders!
    #[serde(rename = "phaseHash")]
    pub phase_hash: u32,
}

/// Okay, so Activities (DestinyActivityDefinition) take place in Destinations (DestinyDestinationDefinition). Destinations are part of larger locations known as Places (you're reading its documentation right now).
/// Places are more on the planetary scale, like "Earth" and "Your Mom."
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyPlaceDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

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

/// The definition for an Activity Type.
/// In Destiny 2, an Activity Type represents a conceptual categorization of Activities.
/// These are most commonly used in the game for the subtitle under Activities, but BNet uses them extensively to identify and group activities by their common properties.
/// Unfortunately, there has been a movement away from providing the richer data in Destiny 2 that we used to get in Destiny 1 for Activity Types. For instance, Nightfalls are grouped under the same Activity Type as regular Strikes.
/// For this reason, BNet will eventually migrate toward Activity Modes as a better indicator of activity category. But for the time being, it is still referred to in many places across our codebase.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActivityTypeDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

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

/// Where the sausage gets made. Unlock Expressions are the foundation of the game's gating mechanics and investment-related restrictions. They can test Unlock Flags and Unlock Values for certain states, using a sufficient amount of logical operators such that unlock expressions are effectively Turing complete.
/// Use UnlockExpressionParser to evaluate expressions using an IUnlockContext parsed from Babel.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyUnlockExpressionDefinition {
    /// A shortcut for determining the most restrictive gating that this expression performs. See the DestinyGatingScope enum's documentation for more details.
    #[serde(rename = "scope")]
    pub scope: crate::destiny::DestinyGatingScope,
}

/// Human readable data about the bubble. Combine with DestinyBubbleDefinition - see DestinyDestinationDefinition.bubbleSettings for more information.
/// DEPRECATED - Just use bubbles.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyDestinationBubbleSettingDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,
}

/// Basic identifying data about the bubble. Combine with DestinyDestinationBubbleSettingDefinition - see DestinyDestinationDefinition.bubbleSettings for more information.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyBubbleDefinition {
    /// The identifier for the bubble: only guaranteed to be unique within the Destination.
    #[serde(rename = "hash")]
    pub hash: u32,

    /// The display properties of this bubble, so you don't have to look them up in a separate list anymore.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorGroupReference {
    /// The DestinyVendorGroupDefinition to which this Vendor can belong.
    #[serde(rename = "vendorGroupHash")]
    pub vendor_group_hash: u32,
}

/// BNet attempts to group vendors into similar collections. These groups aren't technically game canonical, but they are helpful for filtering vendors or showing them organized into a clean view on a webpage or app.
/// These definitions represent the groups we've built. Unlike in Destiny 1, a Vendors' group may change dynamically as the game state changes: thus, you will want to check DestinyVendorComponent responses to find a vendor's currently active Group (if you care).
/// Using this will let you group your vendors in your UI in a similar manner to how we will do grouping in the Companion.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorGroupDefinition {
    /// The recommended order in which to render the groups, Ascending order.
    #[serde(rename = "order")]
    pub order: i32,

    /// For now, a group just has a name.
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,

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

/// These definitions represent Factions in the game. Factions have ended up unilaterally being related to Vendors that represent them, but that need not necessarily be the case.
/// A Faction is really just an entity that has a related progression for which a character can gain experience. In Destiny 1, Dead Orbit was an example of a Faction: there happens to be a Vendor that represents Dead Orbit (and indeed, DestinyVendorDefinition.factionHash defines to this relationship), but Dead Orbit could theoretically exist without the Vendor that provides rewards.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyFactionDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The hash identifier for the DestinyProgressionDefinition that indicates the character's relationship with this faction in terms of experience and levels.
    #[serde(rename = "progressionHash")]
    pub progression_hash: u32,

    /// The faction token item hashes, and their respective progression values.
    #[serde(rename = "tokenValues")]
    pub token_values: Option<HashMap<u32, u32>>,

    /// The faction reward item hash, usually an engram.
    #[serde(rename = "rewardItemHash")]
    pub reward_item_hash: u32,

    /// The faction reward vendor hash, used for faction engram previews.
    #[serde(rename = "rewardVendorHash")]
    pub reward_vendor_hash: u32,

    /// List of vendors that are associated with this faction. The last vendor that passes the unlock flag checks is the one that should be shown.
    #[serde(rename = "vendors")]
    pub vendors: Option<Vec<crate::destiny::definitions::DestinyFactionVendorDefinition>>,

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

/// These definitions represent faction vendors at different points in the game.
/// A single faction may contain multiple vendors, or the same vendor available at two different locations.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyFactionVendorDefinition {
    /// The faction vendor hash.
    #[serde(rename = "vendorHash")]
    pub vendor_hash: u32,

    /// The hash identifier for a Destination at which this vendor may be located. Each destination where a Vendor may exist will only ever have a single entry.
    #[serde(rename = "destinationHash")]
    pub destination_hash: u32,

    /// The relative path to the background image representing this Vendor at this location, for use in a banner.
    #[serde(rename = "backgroundImagePath")]
    pub background_image_path: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinySandboxPatternDefinition {
    #[serde(rename = "patternHash")]
    pub pattern_hash: u32,

    #[serde(rename = "patternGlobalTagIdHash")]
    pub pattern_global_tag_id_hash: u32,

    #[serde(rename = "weaponContentGroupHash")]
    pub weapon_content_group_hash: u32,

    #[serde(rename = "weaponTranslationGroupHash")]
    pub weapon_translation_group_hash: u32,

    #[serde(rename = "weaponTypeHash")]
    pub weapon_type_hash: Option<u32>,

    #[serde(rename = "weaponType")]
    pub weapon_type: crate::destiny::DestinyItemSubType,

    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::destiny::definitions::DestinyArrangementRegionFilterDefinition>>,

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
pub struct DestinyArrangementRegionFilterDefinition {
    #[serde(rename = "artArrangementRegionHash")]
    pub art_arrangement_region_hash: u32,

    #[serde(rename = "artArrangementRegionIndex")]
    pub art_arrangement_region_index: i32,

    #[serde(rename = "statHash")]
    pub stat_hash: u32,

    #[serde(rename = "arrangementIndexByStatValue")]
    pub arrangement_index_by_stat_value: Option<HashMap<i32, i32>>,
}

/// Items like Sacks or Boxes can have items that it shows in-game when you view details that represent the items you can obtain if you use or acquire the item.
/// This defines those categories, and gives some insights into that data's source.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemPreviewBlockDefinition {
    /// A string that the game UI uses as a hint for which detail screen to show for the item. You, too, can leverage this for your own custom screen detail views. Note, however, that these are arbitrarily defined by designers: there's no guarantees of a fixed, known number of these - so fall back to something reasonable if you don't recognize it.
    #[serde(rename = "screenStyle")]
    pub screen_style: Option<String>,

    /// If the preview data is derived from a fake "Preview" Vendor, this will be the hash identifier for the DestinyVendorDefinition of that fake vendor.
    #[serde(rename = "previewVendorHash")]
    pub preview_vendor_hash: u32,

    /// If this item should show you Artifact information when you preview it, this is the hash identifier of the DestinyArtifactDefinition for the artifact whose data should be shown.
    #[serde(rename = "artifactHash")]
    pub artifact_hash: Option<u32>,

    /// If the preview has an associated action (like "Open"), this will be the localized string for that action.
    #[serde(rename = "previewActionString")]
    pub preview_action_string: Option<String>,

    /// This is a list of the items being previewed, categorized in the same way as they are in the preview UI.
    #[serde(rename = "derivedItemCategories")]
    pub derived_item_categories: Option<Vec<crate::destiny::definitions::items::DestinyDerivedItemCategoryDefinition>>,
}

/// An item's "Quality" determines its calculated stats. The Level at which the item spawns is combined with its "qualityLevel" along with some additional calculations to determine the value of those stats.
/// In Destiny 2, most items don't have default item levels and quality, making this property less useful: these apparently are almost always determined by the complex mechanisms of the Reward system rather than statically. They are still provided here in case they are still useful for people. This also contains some information about Infusion.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemQualityBlockDefinition {
    /// The "base" defined level of an item. This is a list because, in theory, each Expansion could define its own base level for an item.
    /// In practice, not only was that never done in Destiny 1, but now this isn't even populated at all. When it's not populated, the level at which it spawns has to be inferred by Reward information, of which BNet receives an imperfect view and will only be reliable on instanced data as a result.
    #[serde(rename = "itemLevels")]
    pub item_levels: Option<Vec<i32>>,

    /// qualityLevel is used in combination with the item's level to calculate stats like Attack and Defense. It plays a role in that calculation, but not nearly as large as itemLevel does.
    #[serde(rename = "qualityLevel")]
    pub quality_level: i32,

    /// The string identifier for this item's "infusability", if any.
    /// Items that match the same infusionCategoryName are allowed to infuse with each other.
    /// DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead.
    #[serde(rename = "infusionCategoryName")]
    pub infusion_category_name: Option<String>,

    /// The hash identifier for the infusion. It does not map to a Definition entity.
    /// DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead.
    #[serde(rename = "infusionCategoryHash")]
    pub infusion_category_hash: u32,

    /// If any one of these hashes matches any value in another item's infusionCategoryHashes, the two can infuse with each other.
    #[serde(rename = "infusionCategoryHashes")]
    pub infusion_category_hashes: Option<Vec<u32>>,

    /// An item can refer to pre-set level requirements. They are defined in DestinyProgressionLevelRequirementDefinition, and you can use this hash to find the appropriate definition.
    #[serde(rename = "progressionLevelRequirementHash")]
    pub progression_level_requirement_hash: u32,

    /// The latest version available for this item.
    #[serde(rename = "currentVersion")]
    pub current_version: u32,

    /// The list of versions available for this item.
    #[serde(rename = "versions")]
    pub versions: Option<Vec<crate::destiny::definitions::DestinyItemVersionDefinition>>,

    /// Icon overlays to denote the item version and power cap status.
    #[serde(rename = "displayVersionWatermarkIcons")]
    pub display_version_watermark_icons: Option<Vec<String>>,
}

/// The version definition currently just holds a reference to the power cap.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemVersionDefinition {
    /// A reference to the power cap for this item version.
    #[serde(rename = "powerCapHash")]
    pub power_cap_hash: u32,
}

/// This defines an item's "Value". Unfortunately, this appears to be used in different ways depending on the way that the item itself is used.
/// For items being sold at a Vendor, this is the default "sale price" of the item. These days, the vendor itself almost always sets the price, but it still possible for the price to fall back to this value. For quests, it is a preview of rewards you can gain by completing the quest. For dummy items, if the itemValue refers to an Emblem, it is the emblem that should be shown as the reward. (jeez louise)
/// It will likely be used in a number of other ways in the future, it appears to be a bucket where they put arbitrary items and quantities into the item.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemValueBlockDefinition {
    /// References to the items that make up this item's "value", and the quantity.
    #[serde(rename = "itemValue")]
    pub item_value: Option<Vec<crate::destiny::DestinyItemQuantity>>,

    /// If there's a localized text description of the value provided, this will be said description.
    #[serde(rename = "valueDescription")]
    pub value_description: Option<String>,
}

/// Data about an item's "sources": ways that the item can be obtained.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSourceBlockDefinition {
    /// The list of hash identifiers for Reward Sources that hint where the item can be found (DestinyRewardSourceDefinition).
    #[serde(rename = "sourceHashes")]
    pub source_hashes: Option<Vec<u32>>,

    /// A collection of details about the stats that were computed for the ways we found that the item could be spawned.
    #[serde(rename = "sources")]
    pub sources: Option<Vec<crate::destiny::definitions::sources::DestinyItemSourceDefinition>>,

    /// If we found that this item is exclusive to a specific platform, this will be set to the BungieMembershipType enumeration that matches that platform.
    #[serde(rename = "exclusive")]
    pub exclusive: crate::BungieMembershipType,

    /// A denormalized reference back to vendors that potentially sell this item.
    #[serde(rename = "vendorSources")]
    pub vendor_sources: Option<Vec<crate::destiny::definitions::DestinyItemVendorSourceReference>>,
}

/// Represents a heuristically-determined "item source" according to Bungie.net. These item sources are non-canonical: we apply a combination of special configuration and often-fragile heuristics to attempt to discern whether an item should be part of a given "source," but we have known cases of false positives and negatives due to our imperfect heuristics.
/// Still, they provide a decent approximation for people trying to figure out how an item can be obtained. DestinyInventoryItemDefinition refers to sources in the sourceDatas.sourceHashes property for all sources we determined the item could spawn from.
/// An example in Destiny 1 of a Source would be "Nightfall". If an item has the "Nightfall" source associated with it, it's extremely likely that you can earn that item while playing Nightfall, either during play or as an after-completion reward.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyRewardSourceDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// Sources are grouped into categories: common ways that items are provided. I hope to see this expand in Destiny 2 once we have time to generate accurate reward source data.
    #[serde(rename = "category")]
    pub category: crate::destiny::definitions::DestinyRewardSourceCategory,

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

/// BNet's custom categorization of reward sources. We took a look at the existing ways that items could be spawned, and tried to make high-level categorizations of them. This needs to be re-evaluated for Destiny 2.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinyRewardSourceCategory {
    /// The source doesn't fit well into any of the other types.
    None = 0,
    /// The source is directly related to the rewards gained by playing an activity or set of activities. This currently includes Quests and other action in-game.
    Activity = 1,
    /// This source is directly related to items that Vendors sell.
    Vendor = 2,
    /// This source is a custom aggregation of items that can be earned in many ways, but that share some other property in common that is useful to share. For instance, in Destiny 1 we would make "Reward Sources" for every game expansion: that way, you could search reward sources to see what items became available with any given Expansion.
    Aggregate = 3,
}

impl Display for DestinyRewardSourceCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for DestinyRewardSourceCategory {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(DestinyRewardSourceCategory::None),
            "Activity" => Ok(DestinyRewardSourceCategory::Activity),
            "Vendor" => Ok(DestinyRewardSourceCategory::Vendor),
            "Aggregate" => Ok(DestinyRewardSourceCategory::Aggregate),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyRewardSourceCategory", s)),
        }
    }
}

/// Represents that a vendor could sell this item, and provides a quick link to that vendor and sale item.
/// Note that we do not and cannot make a guarantee that the vendor will ever *actually* sell this item, only that the Vendor has a definition that indicates it *could* be sold.
/// Note also that a vendor may sell the same item in multiple "ways", which means there may be multiple vendorItemIndexes for a single Vendor hash.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemVendorSourceReference {
    /// The identifier for the vendor that may sell this item.
    #[serde(rename = "vendorHash")]
    pub vendor_hash: u32,

    /// The Vendor sale item indexes that represent the sale information for this item. The same vendor may sell an item in multiple "ways", hence why this is a list. (for instance, a weapon may be "sold" as a reward in a quest, for Glimmer, and for Masterwork Cores: each of those ways would be represented by a different vendor sale item with a different index)
    #[serde(rename = "vendorItemIndexes")]
    pub vendor_item_indexes: Option<Vec<i32>>,
}

/// An item can have objectives on it. In practice, these are the exclusive purview of "Quest Step" items: DestinyInventoryItemDefinitions that represent a specific step in a Quest.
/// Quest steps have 1:M objectives that we end up processing and returning in live data as DestinyQuestStatus data, and other useful information.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemObjectiveBlockDefinition {
    /// The hashes to Objectives (DestinyObjectiveDefinition) that are part of this Quest Step, in the order that they should be rendered.
    #[serde(rename = "objectiveHashes")]
    pub objective_hashes: Option<Vec<u32>>,

    /// For every entry in objectiveHashes, there is a corresponding entry in this array at the same index. If the objective is meant to be associated with a specific DestinyActivityDefinition, there will be a valid hash at that index. Otherwise, it will be invalid (0).
    /// Rendered somewhat obsolete by perObjectiveDisplayProperties, which currently has much the same information but may end up with more info in the future.
    #[serde(rename = "displayActivityHashes")]
    pub display_activity_hashes: Option<Vec<u32>>,

    /// If True, all objectives must be completed for the step to be completed. If False, any one objective can be completed for the step to be completed.
    #[serde(rename = "requireFullObjectiveCompletion")]
    pub require_full_objective_completion: bool,

    /// The hash for the DestinyInventoryItemDefinition representing the Quest to which this Quest Step belongs.
    #[serde(rename = "questlineItemHash")]
    pub questline_item_hash: u32,

    /// The localized string for narrative text related to this quest step, if any.
    #[serde(rename = "narrative")]
    pub narrative: Option<String>,

    /// The localized string describing an action to be performed associated with the objectives, if any.
    #[serde(rename = "objectiveVerbName")]
    pub objective_verb_name: Option<String>,

    /// The identifier for the type of quest being performed, if any. Not associated with any fixed definition, yet.
    #[serde(rename = "questTypeIdentifier")]
    pub quest_type_identifier: Option<String>,

    /// A hashed value for the questTypeIdentifier, because apparently I like to be redundant.
    #[serde(rename = "questTypeHash")]
    pub quest_type_hash: u32,

    /// One entry per Objective on the item, it will have related display information.
    #[serde(rename = "perObjectiveDisplayProperties")]
    pub per_objective_display_properties: Option<Vec<crate::destiny::definitions::DestinyObjectiveDisplayProperties>>,

    #[serde(rename = "displayAsStatTracker")]
    pub display_as_stat_tracker: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyObjectiveDisplayProperties {
    /// The activity associated with this objective in the context of this item, if any.
    #[serde(rename = "activityHash")]
    pub activity_hash: Option<u32>,

    /// If true, the game shows this objective on item preview screens.
    #[serde(rename = "displayOnItemPreviewScreen")]
    pub display_on_item_preview_screen: bool,
}

/// The metrics available for display and selection on an item.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemMetricBlockDefinition {
    /// Hash identifiers for any DestinyPresentationNodeDefinition entry that can be used to list available metrics. Any metric listed directly below these nodes, or in any of these nodes' children will be made available for selection.
    #[serde(rename = "availableMetricCategoryNodeHashes")]
    pub available_metric_category_node_hashes: Option<Vec<u32>>,
}

/// An Unlock Value is an internal integer value, stored on the server and used in a variety of ways, most frequently for the gating/requirement checks that the game performs across all of its main features. They can also be used as the storage data for mapped Progressions, Objectives, and other features that require storage of variable numeric values.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyUnlockValueDefinition {
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

/// If an item has a related gearset, this is the list of items in that set, and an unlock expression that evaluates to a number representing the progress toward gearset completion (a very rare use for unlock expressions!)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemGearsetBlockDefinition {
    /// The maximum possible number of items that can be collected.
    #[serde(rename = "trackingValueMax")]
    pub tracking_value_max: i32,

    /// The list of hashes for items in the gearset. Use them to look up DestinyInventoryItemDefinition entries for the items in the set.
    #[serde(rename = "itemList")]
    pub item_list: Option<Vec<u32>>,
}

/// Some items are "sacks" - they can be "opened" to produce other items. This is information related to its sack status, mostly UI strings. Engrams are an example of items that are considered to be "Sacks".
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSackBlockDefinition {
    /// A description of what will happen when you open the sack. As far as I can tell, this is blank currently. Unknown whether it will eventually be populated with useful info.
    #[serde(rename = "detailAction")]
    pub detail_action: Option<String>,

    /// The localized name of the action being performed when you open the sack.
    #[serde(rename = "openAction")]
    pub open_action: Option<String>,

    #[serde(rename = "selectItemCount")]
    pub select_item_count: i32,

    #[serde(rename = "vendorSackType")]
    pub vendor_sack_type: Option<String>,

    #[serde(rename = "openOnAcquire")]
    pub open_on_acquire: bool,
}

/// If defined, the item has at least one socket.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSocketBlockDefinition {
    /// This was supposed to be a string that would give per-item details about sockets. In practice, it turns out that all this ever has is the localized word "details". ... that's lame, but perhaps it will become something cool in the future.
    #[serde(rename = "detail")]
    pub detail: Option<String>,

    /// Each non-intrinsic (or mutable) socket on an item is defined here. Check inside for more info.
    #[serde(rename = "socketEntries")]
    pub socket_entries: Option<Vec<crate::destiny::definitions::DestinyItemSocketEntryDefinition>>,

    /// Each intrinsic (or immutable/permanent) socket on an item is defined here, along with the plug that is permanently affixed to the socket.
    #[serde(rename = "intrinsicSockets")]
    pub intrinsic_sockets: Option<Vec<crate::destiny::definitions::DestinyItemIntrinsicSocketEntryDefinition>>,

    /// A convenience property, that refers to the sockets in the "sockets" property, pre-grouped by category and ordered in the manner that they should be grouped in the UI. You could form this yourself with the existing data, but why would you want to? Enjoy life man.
    #[serde(rename = "socketCategories")]
    pub socket_categories: Option<Vec<crate::destiny::definitions::DestinyItemSocketCategoryDefinition>>,
}

/// The definition information for a specific socket on an item. This will determine how the socket behaves in-game.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSocketEntryDefinition {
    /// All sockets have a type, and this is the hash identifier for this particular type. Use it to look up the DestinySocketTypeDefinition: read there for more information on how socket types affect the behavior of the socket.
    #[serde(rename = "socketTypeHash")]
    pub socket_type_hash: u32,

    /// If a valid hash, this is the hash identifier for the DestinyInventoryItemDefinition representing the Plug that will be initially inserted into the item on item creation. Otherwise, this Socket will either start without a plug inserted, or will have one randomly inserted.
    #[serde(rename = "singleInitialItemHash")]
    pub single_initial_item_hash: u32,

    /// This is a list of pre-determined plugs that can *always* be plugged into this socket, without the character having the plug in their inventory.
    /// If this list is populated, you will not be allowed to plug an arbitrary item in the socket: you will only be able to choose from one of these reusable plugs.
    #[serde(rename = "reusablePlugItems")]
    pub reusable_plug_items: Option<Vec<crate::destiny::definitions::DestinyItemSocketEntryPlugItemDefinition>>,

    /// If this is true, then the socket will not be initialized with a plug if the item is purchased from a Vendor.
    /// Remember that Vendors are much more than conceptual vendors: they include "Collection Kiosks" and other entities. See DestinyVendorDefinition for more information.
    #[serde(rename = "preventInitializationOnVendorPurchase")]
    pub prevent_initialization_on_vendor_purchase: bool,

    /// If this is true, the perks provided by this socket shouldn't be shown in the item's tooltip. This might be useful if it's providing a hidden bonus, or if the bonus is less important than other benefits on the item.
    #[serde(rename = "hidePerksInItemTooltip")]
    pub hide_perks_in_item_tooltip: bool,

    /// Indicates where you should go to get plugs for this socket. This will affect how you populate your UI, as well as what plugs are valid for this socket. It's an alternative to having to check for the existence of certain properties (reusablePlugItems for example) to infer where plugs should come from.
    #[serde(rename = "plugSources")]
    pub plug_sources: enumflags2::BitFlags<crate::destiny::SocketPlugSources>,

    /// If this socket's plugs come from a reusable DestinyPlugSetDefinition, this is the identifier for that set. We added this concept to reduce some major duplication that's going to come from sockets as replacements for what was once implemented as large sets of items and kiosks (like Emotes).
    /// As of Shadowkeep, these will come up much more frequently and be driven by game content rather than custom curation.
    #[serde(rename = "reusablePlugSetHash")]
    pub reusable_plug_set_hash: Option<u32>,

    /// This field replaces "randomizedPlugItems" as of Shadowkeep launch. If a socket has randomized plugs, this is a pointer to the set of plugs that could be used, as defined in DestinyPlugSetDefinition.
    /// If null, the item has no randomized plugs.
    #[serde(rename = "randomizedPlugSetHash")]
    pub randomized_plug_set_hash: Option<u32>,

    /// If true, then this socket is visible in the item's "default" state. If you have an instance, you should always check the runtime state, as that can override this visibility setting: but if you're looking at the item on a conceptual level, this property can be useful for hiding data such as legacy sockets - which remain defined on items for infrastructure purposes, but can be confusing for users to see.
    #[serde(rename = "defaultVisible")]
    pub default_visible: bool,
}

/// The definition of a known, reusable plug that can be applied to a socket.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSocketEntryPlugItemDefinition {
    /// The hash identifier of a DestinyInventoryItemDefinition representing the plug that can be inserted.
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSocketEntryPlugItemRandomizedDefinition {
    #[serde(rename = "craftingRequirements")]
    pub crafting_requirements: Option<crate::destiny::definitions::DestinyPlugItemCraftingRequirements>,

    /// Indicates if the plug can be rolled on the current version of the item. For example, older versions of weapons may have plug rolls that are no longer possible on the current versions.
    #[serde(rename = "currentlyCanRoll")]
    pub currently_can_roll: bool,

    /// The hash identifier of a DestinyInventoryItemDefinition representing the plug that can be inserted.
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyPlugItemCraftingRequirements {
    #[serde(rename = "unlockRequirements")]
    pub unlock_requirements: Option<Vec<crate::destiny::definitions::DestinyPlugItemCraftingUnlockRequirement>>,

    /// If the plug has a known level requirement, it'll be available here.
    #[serde(rename = "requiredLevel")]
    pub required_level: Option<i32>,

    #[serde(rename = "materialRequirementHashes")]
    pub material_requirement_hashes: Option<Vec<u32>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyPlugItemCraftingUnlockRequirement {
    #[serde(rename = "failureDescription")]
    pub failure_description: Option<String>,
}

/// Represents a socket that has a plug associated with it intrinsically. This is useful for situations where the weapon needs to have a visual plug/Mod on it, but that plug/Mod should never change.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemIntrinsicSocketEntryDefinition {
    /// Indicates the plug that is intrinsically inserted into this socket.
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,

    /// Indicates the type of this intrinsic socket.
    #[serde(rename = "socketTypeHash")]
    pub socket_type_hash: u32,

    /// If true, then this socket is visible in the item's "default" state. If you have an instance, you should always check the runtime state, as that can override this visibility setting: but if you're looking at the item on a conceptual level, this property can be useful for hiding data such as legacy sockets - which remain defined on items for infrastructure purposes, but can be confusing for users to see.
    #[serde(rename = "defaultVisible")]
    pub default_visible: bool,
}

/// Sockets are grouped into categories in the UI. These define which category and which sockets are under that category.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSocketCategoryDefinition {
    /// The hash for the Socket Category: a quick way to go get the header display information for the category. Use it to look up DestinySocketCategoryDefinition info.
    #[serde(rename = "socketCategoryHash")]
    pub socket_category_hash: u32,

    /// Use these indexes to look up the sockets in the "sockets.socketEntries" property on the item definition. These are the indexes under the category, in game-rendered order.
    #[serde(rename = "socketIndexes")]
    pub socket_indexes: Option<Vec<i32>>,
}

/// This appears to be information used when rendering rewards. We don't currently use it on BNet.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSummaryBlockDefinition {
    /// Apparently when rendering an item in a reward, this should be used as a sort priority. We're not doing it presently.
    #[serde(rename = "sortPriority")]
    pub sort_priority: i32,
}

/// This defines information that can only come from a talent grid on an item. Items mostly have negligible talent grid data these days, but instanced items still retain grids as a source for some of this common information.
/// Builds/Subclasses are the only items left that still have talent grids with meaningful Nodes.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemTalentGridBlockDefinition {
    /// The hash identifier of the DestinyTalentGridDefinition attached to this item.
    #[serde(rename = "talentGridHash")]
    pub talent_grid_hash: u32,

    /// This is meant to be a subtitle for looking at the talent grid. In practice, somewhat frustratingly, this always merely says the localized word for "Details". Great. Maybe it'll have more if talent grids ever get used for more than builds and subclasses again.
    #[serde(rename = "itemDetailString")]
    pub item_detail_string: Option<String>,

    /// A shortcut string identifier for the "build" in question, if this talent grid has an associated build. Doesn't map to anything we can expose at the moment.
    #[serde(rename = "buildName")]
    pub build_name: Option<String>,

    /// If the talent grid implies a damage type, this is the enum value for that damage type.
    #[serde(rename = "hudDamageType")]
    pub hud_damage_type: crate::destiny::DamageType,

    /// If the talent grid has a special icon that's shown in the game UI (like builds, funny that), this is the identifier for that icon. Sadly, we don't actually get that icon right now. I'll be looking to replace this with a path to the actual icon itself.
    #[serde(rename = "hudIcon")]
    pub hud_icon: Option<String>,
}

/// The time has unfortunately come to talk about Talent Grids.
/// Talent Grids are the most complex and unintuitive part of the Destiny Definition data. Grab a cup of coffee before we begin, I can wait.
/// Talent Grids were the primary way that items could be customized in Destiny 1. In Destiny 2, for now, talent grids have become exclusively used by Subclass/Build items: but the system is still in place for it to be used by items should the direction change back toward talent grids.
/// Talent Grids have Nodes: the visual circles on the talent grid detail screen that have icons and can be activated if you meet certain requirements and pay costs. The actual visual data and effects, however, are driven by the "Steps" on Talent Nodes. Any given node will have 1:M of these steps, and the specific step that will be considered the "current" step (and thus the dictator of all benefits, visual state, and activation requirements on the Node) will almost always not be determined until an instance of the item is created. This is how, in Destiny 1, items were able to have such a wide variety of what users saw as "Perks": they were actually Talent Grids with nodes that had a wide variety of Steps, randomly chosen at the time of item creation.
/// Now that Talent Grids are used exclusively by subclasses and builds, all of the properties within still apply: but there are additional visual elements on the Subclass/Build screens that are superimposed on top of the talent nodes. Unfortunately, BNet doesn't have this data: if you want to build a subclass screen, you will have to provide your own "decorative" assets, such as the visual connectors between nodes and the fancy colored-fire-bathed character standing behind the nodes.
/// DestinyInventoryItem.talentGrid.talentGridHash defines an item's linked Talent Grid, which brings you to this definition that contains enough satic data about talent grids to make your head spin. These *must* be combined with instanced data - found when live data returns DestinyItemTalentGridComponent - in order to derive meaning. The instanced data will reference nodes and steps within these definitions, which you will then have to look up in the definition and combine with the instanced data to give the user the visual representation of their item's talent grid.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyTalentGridDefinition {
    /// The maximum possible level of the Talent Grid: at this level, any nodes are allowed to be activated.
    #[serde(rename = "maxGridLevel")]
    pub max_grid_level: i32,

    /// The meaning of this has been lost in the sands of time: it still exists as a property, but appears to be unused in the modern UI of talent grids. It used to imply that each visual "column" of talent nodes required identical progression levels in order to be activated. Returning this value in case it is still useful to someone? Perhaps it's just a bit of interesting history.
    #[serde(rename = "gridLevelPerColumn")]
    pub grid_level_per_column: i32,

    /// The hash identifier of the Progression (DestinyProgressionDefinition) that drives whether and when Talent Nodes can be activated on the Grid. Items will have instances of this Progression, and will gain experience that will eventually cause the grid to increase in level. As the grid's level increases, it will cross the threshold where nodes can be activated. See DestinyTalentGridStepDefinition's activation requirements for more information.
    #[serde(rename = "progressionHash")]
    pub progression_hash: u32,

    /// The list of Talent Nodes on the Grid (recall that Nodes themselves are really just locations in the UI to show whatever their current Step is. You will only know the current step for a node by retrieving instanced data through platform calls to the API that return DestinyItemTalentGridComponent).
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<crate::destiny::definitions::DestinyTalentNodeDefinition>>,

    /// Talent Nodes can exist in "exclusive sets": these are sets of nodes in which only a single node in the set can be activated at any given time. Activating a node in this set will automatically deactivate the other nodes in the set (referred to as a "Swap").
    /// If a node in the exclusive set has already been activated, the game will not charge you materials to activate another node in the set, even if you have never activated it before, because you already paid the cost to activate one node in the set.
    /// Not to be confused with Exclusive Groups. (how the heck do we NOT get confused by that? Jeez) See the groups property for information about that only-tangentially-related concept.
    #[serde(rename = "exclusiveSets")]
    pub exclusive_sets: Option<Vec<crate::destiny::definitions::DestinyTalentNodeExclusiveSetDefinition>>,

    /// This is a quick reference to the indexes of nodes that are not part of exclusive sets. Handy for knowing which talent nodes can only be activated directly, rather than via swapping.
    #[serde(rename = "independentNodeIndexes")]
    pub independent_node_indexes: Option<Vec<i32>>,

    /// Talent Nodes can have "Exclusive Groups". These are not to be confused with Exclusive Sets (see exclusiveSets property).
    /// Look at the definition of DestinyTalentExclusiveGroup for more information and how they work. These groups are keyed by the "groupHash" from DestinyTalentExclusiveGroup.
    #[serde(rename = "groups")]
    pub groups: Option<HashMap<u32, crate::destiny::definitions::DestinyTalentExclusiveGroup>>,

    /// BNet wants to show talent nodes grouped by similar purpose with localized titles. This is the ordered list of those categories: if you want to show nodes by category, you can iterate over this list, render the displayProperties for the category as the title, and then iterate over the talent nodes referenced by the category to show the related nodes.
    /// Note that this is different from Exclusive Groups or Sets, because these categories also incorporate "Independent" nodes that belong to neither sets nor groups. These are purely for visual grouping of nodes rather than functional grouping.
    #[serde(rename = "nodeCategories")]
    pub node_categories: Option<Vec<crate::destiny::definitions::DestinyTalentNodeCategory>>,

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

/// Talent Grids on items have Nodes. These nodes have positions in the talent grid's UI, and contain "Steps" (DestinyTalentNodeStepDefinition), one of whom will be the "Current" step.
/// The Current Step determines the visual properties of the node, as well as what the node grants when it is activated.
/// See DestinyTalentGridDefinition for a more complete overview of how Talent Grids work, and how they are used in Destiny 2 (and how they were used in Destiny 1).
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyTalentNodeDefinition {
    /// The index into the DestinyTalentGridDefinition's "nodes" property where this node is located. Used to uniquely identify the node within the Talent Grid. Note that this is content version dependent: make sure you have the latest version of content before trying to use these properties.
    #[serde(rename = "nodeIndex")]
    pub node_index: i32,

    /// The hash identifier for the node, which unfortunately is also content version dependent but can be (and ideally, should be) used instead of the nodeIndex to uniquely identify the node.
    /// The two exist side-by-side for backcompat reasons due to the Great Talent Node Restructuring of Destiny 1, and I ran out of time to remove one of them and standardize on the other. Sorry!
    #[serde(rename = "nodeHash")]
    pub node_hash: u32,

    /// The visual "row" where the node should be shown in the UI. If negative, then the node is hidden.
    #[serde(rename = "row")]
    pub row: i32,

    /// The visual "column" where the node should be shown in the UI. If negative, the node is hidden.
    #[serde(rename = "column")]
    pub column: i32,

    /// Indexes into the DestinyTalentGridDefinition.nodes property for any nodes that must be activated before this one is allowed to be activated.
    /// I would have liked to change this to hashes for Destiny 2, but we have run out of time.
    #[serde(rename = "prerequisiteNodeIndexes")]
    pub prerequisite_node_indexes: Option<Vec<i32>>,

    /// At one point, Talent Nodes supported the idea of "Binary Pairs": nodes that overlapped each other visually, and where activating one deactivated the other. They ended up not being used, mostly because Exclusive Sets are *almost* a superset of this concept, but the potential for it to be used still exists in theory.
    /// If this is ever used, this will be the index into the DestinyTalentGridDefinition.nodes property for the node that is the binary pair match to this node. Activating one deactivates the other.
    #[serde(rename = "binaryPairNodeIndex")]
    pub binary_pair_node_index: i32,

    /// If true, this node will automatically unlock when the Talent Grid's level reaches the required level of the current step of this node.
    #[serde(rename = "autoUnlocks")]
    pub auto_unlocks: bool,

    /// At one point, Nodes were going to be able to be activated multiple times, changing the current step and potentially piling on multiple effects from the previously activated steps. This property would indicate if the last step could be activated multiple times.
    /// This is not currently used, but it isn't out of the question that this could end up being used again in a theoretical future.
    #[serde(rename = "lastStepRepeats")]
    pub last_step_repeats: bool,

    /// If this is true, the node's step is determined randomly rather than the first step being chosen.
    #[serde(rename = "isRandom")]
    pub is_random: bool,

    /// At one point, you were going to be able to repurchase talent nodes that had random steps, to "re-roll" the current step of the node (and thus change the properties of your item). This was to be the activation requirement for performing that re-roll.
    /// The system still exists to do this, as far as I know, so it may yet come back around!
    #[serde(rename = "randomActivationRequirement")]
    pub random_activation_requirement: Option<crate::destiny::definitions::DestinyNodeActivationRequirement>,

    /// If this is true, the node can be "re-rolled" to acquire a different random current step. This is not used, but still exists for a theoretical future of talent grids.
    #[serde(rename = "isRandomRepurchasable")]
    pub is_random_repurchasable: bool,

    /// At this point, "steps" have been obfuscated into conceptual entities, aggregating the underlying notions of "properties" and "true steps".
    /// If you need to know a step as it truly exists - such as when recreating Node logic when processing Vendor data - you'll have to use the "realSteps" property below.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::destiny::definitions::DestinyNodeStepDefinition>>,

    /// The nodeHash values for nodes that are in an Exclusive Set with this node.
    /// See DestinyTalentGridDefinition.exclusiveSets for more info about exclusive sets.
    /// Again, note that these are nodeHashes and *not* nodeIndexes.
    #[serde(rename = "exclusiveWithNodeHashes")]
    pub exclusive_with_node_hashes: Option<Vec<u32>>,

    /// If the node's step is randomly selected, this is the amount of the Talent Grid's progression experience at which the progression bar for the node should be shown.
    #[serde(rename = "randomStartProgressionBarAtProgression")]
    pub random_start_progression_bar_at_progression: i32,

    /// A string identifier for a custom visual layout to apply to this talent node. Unfortunately, we do not have any data for rendering these custom layouts. It will be up to you to interpret these strings and change your UI if you want to have custom UI matching these layouts.
    #[serde(rename = "layoutIdentifier")]
    pub layout_identifier: Option<String>,

    /// As of Destiny 2, nodes can exist as part of "Exclusive Groups". These differ from exclusive sets in that, within the group, many nodes can be activated. But the act of activating any node in the group will cause "opposing" nodes (nodes in groups that are not allowed to be activated at the same time as this group) to deactivate.
    /// See DestinyTalentExclusiveGroup for more information on the details. This is an identifier for this node's group, if it is part of one.
    #[serde(rename = "groupHash")]
    pub group_hash: Option<u32>,

    /// Talent nodes can be associated with a piece of Lore, generally rendered in a tooltip. This is the hash identifier of the lore element to show, if there is one to be show.
    #[serde(rename = "loreHash")]
    pub lore_hash: Option<u32>,

    /// Comes from the talent grid node style: this identifier should be used to determine how to render the node in the UI.
    #[serde(rename = "nodeStyleIdentifier")]
    pub node_style_identifier: Option<String>,

    /// Comes from the talent grid node style: if true, then this node should be ignored for determining whether the grid is complete.
    #[serde(rename = "ignoreForCompletion")]
    pub ignore_for_completion: bool,
}

/// Talent nodes have requirements that must be met before they can be activated.
/// This describes the material costs, the Level of the Talent Grid's progression required, and other conditional information that limits whether a talent node can be activated.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyNodeActivationRequirement {
    /// The Progression level on the Talent Grid required to activate this node.
    /// See DestinyTalentGridDefinition.progressionHash for the related Progression, and read DestinyProgressionDefinition's documentation to learn more about Progressions.
    #[serde(rename = "gridLevel")]
    pub grid_level: i32,

    /// The list of hash identifiers for material requirement sets: materials that are required for the node to be activated. See DestinyMaterialRequirementSetDefinition for more information about material requirements.
    /// In this case, only a single DestinyMaterialRequirementSetDefinition will be chosen from this list, and we won't know which one will be chosen until an instance of the item is created.
    #[serde(rename = "materialRequirementHashes")]
    pub material_requirement_hashes: Option<Vec<u32>>,
}

/// This defines the properties of a "Talent Node Step". When you see a talent node in game, the actual visible properties that you see (its icon, description, the perks and stats it provides) are not provided by the Node itself, but rather by the currently active Step on the node.
/// When a Talent Node is activated, the currently active step's benefits are conferred upon the item and character.
/// The currently active step on talent nodes are determined when an item is first instantiated. Sometimes it is random, sometimes it is more deterministic (particularly when a node has only a single step).
/// Note that, when dealing with Talent Node Steps, you must ensure that you have the latest version of content. stepIndex and nodeStepHash - two ways of identifying the step within a node - are both content version dependent, and thus are subject to change between content updates.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyNodeStepDefinition {
    /// These are the display properties actually used to render the Talent Node. The currently active step's displayProperties are shown.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The index of this step in the list of Steps on the Talent Node.
    /// Unfortunately, this is the closest thing we have to an identifier for the Step: steps are not provided a content version agnostic identifier. This means that, when you are dealing with talent nodes, you will need to first ensure that you have the latest version of content.
    #[serde(rename = "stepIndex")]
    pub step_index: i32,

    /// The hash of this node step. Unfortunately, while it can be used to uniquely identify the step within a node, it is also content version dependent and should not be relied on without ensuring you have the latest vesion of content.
    #[serde(rename = "nodeStepHash")]
    pub node_step_hash: u32,

    /// If you can interact with this node in some way, this is the localized description of that interaction.
    #[serde(rename = "interactionDescription")]
    pub interaction_description: Option<String>,

    /// An enum representing a damage type granted by activating this step, if any.
    #[serde(rename = "damageType")]
    pub damage_type: crate::destiny::DamageType,

    /// If the step provides a damage type, this will be the hash identifier used to look up the damage type's DestinyDamageTypeDefinition.
    #[serde(rename = "damageTypeHash")]
    pub damage_type_hash: Option<u32>,

    /// If the step has requirements for activation (they almost always do, if nothing else than for the Talent Grid's Progression to have reached a certain level), they will be defined here.
    #[serde(rename = "activationRequirement")]
    pub activation_requirement: Option<crate::destiny::definitions::DestinyNodeActivationRequirement>,

    /// There was a time when talent nodes could be activated multiple times, and the effects of subsequent Steps would be compounded on each other, essentially "upgrading" the node. We have moved away from this, but theoretically the capability still exists.
    /// I continue to return this in case it is used in the future: if true and this step is the current step in the node, you are allowed to activate the node a second time to receive the benefits of the next step in the node, which will then become the active step.
    #[serde(rename = "canActivateNextStep")]
    pub can_activate_next_step: bool,

    /// The stepIndex of the next step in the talent node, or -1 if this is the last step or if the next step to be chosen is random.
    /// This doesn't really matter anymore unless canActivateNextStep begins to be used again.
    #[serde(rename = "nextStepIndex")]
    pub next_step_index: i32,

    /// If true, the next step to be chosen is random, and if you're allowed to activate the next step. (if canActivateNextStep = true)
    #[serde(rename = "isNextStepRandom")]
    pub is_next_step_random: bool,

    /// The list of hash identifiers for Perks (DestinySandboxPerkDefinition) that are applied when this step is active. Perks provide a variety of benefits and modifications - examine DestinySandboxPerkDefinition to learn more.
    #[serde(rename = "perkHashes")]
    pub perk_hashes: Option<Vec<u32>>,

    /// When the Talent Grid's progression reaches this value, the circular "progress bar" that surrounds the talent node should be shown.
    /// This also indicates the lower bound of said progress bar, with the upper bound being the progress required to reach activationRequirement.gridLevel. (at some point I should precalculate the upper bound and put it in the definition to save people time)
    #[serde(rename = "startProgressionBarAtProgress")]
    pub start_progression_bar_at_progress: i32,

    /// When the step provides stat benefits on the item or character, this is the list of hash identifiers for stats (DestinyStatDefinition) that are provided.
    #[serde(rename = "statHashes")]
    pub stat_hashes: Option<Vec<u32>>,

    /// If this is true, the step affects the item's Quality in some way. See DestinyInventoryItemDefinition for more information about the meaning of Quality. I already made a joke about Zen and the Art of Motorcycle Maintenance elsewhere in the documentation, so I will avoid doing it again. Oops too late
    #[serde(rename = "affectsQuality")]
    pub affects_quality: bool,

    /// In Destiny 1, the Armory's Perk Filtering was driven by a concept of TalentNodeStepGroups: categorizations of talent nodes based on their functionality. While the Armory isn't a BNet-facing thing for now, and the new Armory will need to account for Sockets rather than Talent Nodes, this categorization capability feels useful enough to still keep around.
    #[serde(rename = "stepGroups")]
    pub step_groups: Option<crate::destiny::definitions::DestinyTalentNodeStepGroups>,

    /// If true, this step can affect the level of the item. See DestinyInventoryItemDefintion for more information about item levels and their effect on stats.
    #[serde(rename = "affectsLevel")]
    pub affects_level: bool,

    /// If this step is activated, this will be a list of information used to replace socket items with new Plugs. See DestinyInventoryItemDefinition for more information about sockets and plugs.
    #[serde(rename = "socketReplacements")]
    pub socket_replacements: Option<Vec<crate::destiny::definitions::DestinyNodeSocketReplaceResponse>>,
}

/// This is a bit of an odd duck. Apparently, if talent nodes steps have this data, the game will go through on step activation and alter the first Socket it finds on the item that has a type matching the given socket type, inserting the indicated plug item.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyNodeSocketReplaceResponse {
    /// The hash identifier of the socket type to find amidst the item's sockets (the item to which this talent grid is attached). See DestinyInventoryItemDefinition.sockets.socketEntries to find the socket type of sockets on the item in question.
    #[serde(rename = "socketTypeHash")]
    pub socket_type_hash: u32,

    /// The hash identifier of the plug item that will be inserted into the socket found.
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,
}

/// The list of indexes into the Talent Grid's "nodes" property for nodes in this exclusive set. (See DestinyTalentNodeDefinition.nodeIndex)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyTalentNodeExclusiveSetDefinition {
    /// The list of node indexes for the exclusive set. Historically, these were indexes. I would have liked to replace this with nodeHashes for consistency, but it's way too late for that. (9:09 PM, he's right!)
    #[serde(rename = "nodeIndexes")]
    pub node_indexes: Option<Vec<i32>>,
}

/// As of Destiny 2, nodes can exist as part of "Exclusive Groups". These differ from exclusive sets in that, within the group, many nodes can be activated. But the act of activating any node in the group will cause "opposing" nodes (nodes in groups that are not allowed to be activated at the same time as this group) to deactivate.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyTalentExclusiveGroup {
    /// The identifier for this exclusive group. Only guaranteed unique within the talent grid, not globally.
    #[serde(rename = "groupHash")]
    pub group_hash: u32,

    /// If this group has an associated piece of lore to show next to it, this will be the identifier for that DestinyLoreDefinition.
    #[serde(rename = "loreHash")]
    pub lore_hash: Option<u32>,

    /// A quick reference of the talent nodes that are part of this group, by their Talent Node hashes. (See DestinyTalentNodeDefinition.nodeHash)
    #[serde(rename = "nodeHashes")]
    pub node_hashes: Option<Vec<u32>>,

    /// A quick reference of Groups whose nodes will be deactivated if any node in this group is activated.
    #[serde(rename = "opposingGroupHashes")]
    pub opposing_group_hashes: Option<Vec<u32>>,

    /// A quick reference of Nodes that will be deactivated if any node in this group is activated, by their Talent Node hashes. (See DestinyTalentNodeDefinition.nodeHash)
    #[serde(rename = "opposingNodeHashes")]
    pub opposing_node_hashes: Option<Vec<u32>>,
}

/// An artificial construct provided by Bungie.Net, where we attempt to group talent nodes by functionality.
/// This is a single set of references to Talent Nodes that share a common trait or purpose.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyTalentNodeCategory {
    /// Mostly just for debug purposes, but if you find it useful you can have it. This is BNet's manually created identifier for this category.
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    /// If true, we found the localized content in a related DestinyLoreDefinition instead of local BNet localization files. This is mostly for ease of my own future investigations.
    #[serde(rename = "isLoreDriven")]
    pub is_lore_driven: bool,

    /// Will contain at least the "name", which will be the title of the category. We will likely not have description and an icon yet, but I'm going to keep my options open.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The set of all hash identifiers for Talent Nodes (DestinyTalentNodeDefinition) in this Talent Grid that are part of this Category.
    #[serde(rename = "nodeHashes")]
    pub node_hashes: Option<Vec<u32>>,
}

/// An intrinsic perk on an item, and the requirements for it to be activated.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemPerkEntryDefinition {
    /// If this perk is not active, this is the string to show for why it's not providing its benefits.
    #[serde(rename = "requirementDisplayString")]
    pub requirement_display_string: Option<String>,

    /// A hash identifier for the DestinySandboxPerkDefinition being provided on the item.
    #[serde(rename = "perkHash")]
    pub perk_hash: u32,

    /// Indicates whether this perk should be shown, or if it should be shown disabled.
    #[serde(rename = "perkVisibility")]
    pub perk_visibility: crate::destiny::ItemPerkVisibility,
}

/// In an attempt to categorize items by type, usage, and other interesting properties, we created DestinyItemCategoryDefinition: information about types that is assembled using a set of heuristics that examine the properties of an item such as what inventory bucket it's in, its item type name, and whether it has or is missing certain blocks of data.
/// This heuristic is imperfect, however. If you find an item miscategorized, let us know on the Bungie API forums!
/// We then populate all of the categories that we think an item belongs to in its DestinyInventoryItemDefinition.itemCategoryHashes property. You can use that to provide your own custom item filtering, sorting, aggregating... go nuts on it! And let us know if you see more categories that you wish would be added!
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemCategoryDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If True, this category should be visible in UI. Sometimes we make categories that we don't think are interesting externally. It's up to you if you want to skip on showing them.
    #[serde(rename = "visible")]
    pub visible: bool,

    /// If True, this category has been deprecated: it may have no items left, or there may be only legacy items that remain in it which are no longer relevant to the game.
    #[serde(rename = "deprecated")]
    pub deprecated: bool,

    /// A shortened version of the title. The reason why we have this is because the Armory in German had titles that were too long to display in our UI, so these were localized abbreviated versions of those categories. The property still exists today, even though the Armory doesn't exist for D2... yet.
    #[serde(rename = "shortTitle")]
    pub short_title: Option<String>,

    /// The janky regular expression we used against the item type to try and discern whether the item belongs to this category.
    #[serde(rename = "itemTypeRegex")]
    pub item_type_regex: Option<String>,

    /// If the item in question has this category, it also should have this breaker type.
    #[serde(rename = "grantDestinyBreakerType")]
    pub grant_destiny_breaker_type: crate::destiny::DestinyBreakerType,

    /// If the item is a plug, this is the identifier we expect to find associated with it if it is in this category.
    #[serde(rename = "plugCategoryIdentifier")]
    pub plug_category_identifier: Option<String>,

    /// If the item type matches this janky regex, it does *not* belong to this category.
    #[serde(rename = "itemTypeRegexNot")]
    pub item_type_regex_not: Option<String>,

    /// If the item belongs to this bucket, it does belong to this category.
    #[serde(rename = "originBucketIdentifier")]
    pub origin_bucket_identifier: Option<String>,

    /// If an item belongs to this category, it will also receive this item type. This is now how DestinyItemType is populated for items: it used to be an even jankier process, but that's a story that requires more alcohol.
    #[serde(rename = "grantDestinyItemType")]
    pub grant_destiny_item_type: crate::destiny::DestinyItemType,

    /// If an item belongs to this category, it will also receive this subtype enum value.
    /// I know what you're thinking - what if it belongs to multiple categories that provide sub-types?
    /// The last one processed wins, as is the case with all of these "grant" enums. Now you can see one reason why we moved away from these enums... but they're so convenient when they work, aren't they?
    #[serde(rename = "grantDestinySubType")]
    pub grant_destiny_sub_type: crate::destiny::DestinyItemSubType,

    /// If an item belongs to this category, it will also get this class restriction enum value.
    /// See the other "grant"-prefixed properties on this definition for my color commentary.
    #[serde(rename = "grantDestinyClass")]
    pub grant_destiny_class: crate::destiny::DestinyClass,

    /// The traitId that can be found on items that belong to this category.
    #[serde(rename = "traitId")]
    pub trait_id: Option<String>,

    /// If this category is a "parent" category of other categories, those children will have their hashes listed in rendering order here, and can be looked up using these hashes against DestinyItemCategoryDefinition.
    /// In this way, you can build up a visual hierarchy of item categories. That's what we did, and you can do it too. I believe in you. Yes, you, Carl.
    /// (I hope someone named Carl reads this someday)
    #[serde(rename = "groupedCategoryHashes")]
    pub grouped_category_hashes: Option<Vec<u32>>,

    /// All item category hashes of "parent" categories: categories that contain this as a child through the hierarchy of groupedCategoryHashes. It's a bit redundant, but having this child-centric list speeds up some calculations.
    #[serde(rename = "parentCategoryHashes")]
    pub parent_category_hashes: Option<Vec<u32>>,

    /// If true, this category is only used for grouping, and should not be evaluated with its own checks. Rather, the item only has this category if it has one of its child categories.
    #[serde(rename = "groupCategoryOnly")]
    pub group_category_only: bool,

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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProgressionRewardItemQuantity {
    #[serde(rename = "rewardedAtProgressionLevel")]
    pub rewarded_at_progression_level: i32,

    #[serde(rename = "acquisitionBehavior")]
    pub acquisition_behavior: crate::destiny::DestinyProgressionRewardItemAcquisitionBehavior,

    #[serde(rename = "uiDisplayStyle")]
    pub ui_display_style: Option<String>,

    #[serde(rename = "claimUnlockDisplayStrings")]
    pub claim_unlock_display_strings: Option<Vec<String>>,

    /// The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// If this quantity is referring to a specific instance of an item, this will have the item's instance ID. Normally, this will be null.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: Option<i64>,

    /// The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used.
    #[serde(rename = "quantity")]
    pub quantity: i32,

    /// Indicates that this item quantity may be conditionally shown or hidden, based on various sources of state. For example: server flags, account state, or character progress.
    #[serde(rename = "hasConditionalVisibility")]
    pub has_conditional_visibility: bool,
}

/// In Destiny, "Races" are really more like "Species". Sort of. I mean, are the Awoken a separate species from humans? I'm not sure. But either way, they're defined here. You'll see Exo, Awoken, and Human as examples of these Species. Players will choose one for their character.
#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyRaceDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// An enumeration defining the existing, known Races/Species for player characters. This value will be the enum value matching this definition.
    #[serde(rename = "raceType")]
    pub race_type: crate::destiny::DestinyRace,

    /// A localized string referring to the singular form of the Race's name when referred to in gendered form. Keyed by the DestinyGender.
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "genderedRaceNames")]
    pub gendered_race_names: Option<HashMap<crate::destiny::DestinyGender, String>>,

    #[serde(rename = "genderedRaceNamesByGenderHash")]
    pub gendered_race_names_by_gender_hash: Option<HashMap<u32, String>>,

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

/// Unlock Flags are small bits (literally, a bit, as in a boolean value) that the game server uses for an extremely wide range of state checks, progress storage, and other interesting tidbits of information.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyUnlockDefinition {
    /// Sometimes, but not frequently, these unlock flags also have human readable information: usually when they are being directly tested for some requirement, in which case the string is a localized description of why the requirement check failed.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

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

/// An artificial construct of our own creation, to try and put some order on top of Medals and keep them from being one giant, unmanageable and unsorted blob of stats.
/// Unfortunately, we haven't had time to do this evaluation yet in Destiny 2, so we're short on Medal Tiers. This will hopefully be updated over time, if Medals continue to exist.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyMedalTierDefinition {
    /// The name of the tier.
    #[serde(rename = "tierName")]
    pub tier_name: Option<String>,

    /// If you're rendering medals by tier, render them in this order (ascending)
    #[serde(rename = "order")]
    pub order: i32,

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

/// The results of a search for Destiny content. This will be improved on over time, I've been doing some experimenting to see what might be useful.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyEntitySearchResult {
    /// A list of suggested words that might make for better search results, based on the text searched for.
    #[serde(rename = "suggestedWords")]
    pub suggested_words: Option<Vec<String>>,

    /// The items found that are matches/near matches for the searched-for term, sorted by something vaguely resembling "relevance". Hopefully this will get better in the future.
    #[serde(rename = "results")]
    pub results: Option<crate::SearchResultOfDestinyEntitySearchResultItem>,
}

/// An individual Destiny Entity returned from the entity search.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyEntitySearchResultItem {
    /// The hash identifier of the entity. You will use this to look up the DestinyDefinition relevant for the entity found.
    #[serde(rename = "hash")]
    pub hash: u32,

    /// The type of entity, returned as a string matching the DestinyDefinition's contract class name. You'll have to have your own mapping from class names to actually looking up those definitions in the manifest databases.
    #[serde(rename = "entityType")]
    pub entity_type: Option<String>,

    /// Basic display properties on the entity, so you don't have to look up the definition to show basic results for the item.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The ranking value for sorting that we calculated using our relevance formula. This will hopefully get better with time and iteration.
    #[serde(rename = "weight")]
    pub weight: f64,
}
