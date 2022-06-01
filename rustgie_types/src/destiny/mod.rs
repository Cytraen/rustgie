pub mod activities;
pub mod advanced;
pub mod artifacts;
pub mod challenges;
pub mod character;
pub mod components;
pub mod config;
pub mod constants;
pub mod definitions;
pub mod entities;
pub mod historical_stats;
pub mod milestones;
pub mod misc;
pub mod perks;
pub mod progression;
pub mod quests;
pub mod reporting;
pub mod requests;
pub mod responses;
pub mod sockets;
pub mod vendors;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Information about a current character's status with a Progression. A progression is a value that can increase with activity and has levels. Think Character Level and Reputation Levels. Combine this "live" data with the related DestinyProgressionDefinition for a full picture of the Progression.
#[derive(Deserialize, Serialize)]
pub struct DestinyProgression {
    /// The hash identifier of the Progression in question. Use it to look up the DestinyProgressionDefinition in static data.
    #[serde(rename = "progressionHash")]
    pub progression_hash: u32,

    /// The amount of progress earned today for this progression.
    #[serde(rename = "dailyProgress")]
    pub daily_progress: i32,

    /// If this progression has a daily limit, this is that limit.
    #[serde(rename = "dailyLimit")]
    pub daily_limit: i32,

    /// The amount of progress earned toward this progression in the current week.
    #[serde(rename = "weeklyProgress")]
    pub weekly_progress: i32,

    /// If this progression has a weekly limit, this is that limit.
    #[serde(rename = "weeklyLimit")]
    pub weekly_limit: i32,

    /// This is the total amount of progress obtained overall for this progression (for instance, the total amount of Character Level experience earned)
    #[serde(rename = "currentProgress")]
    pub current_progress: i32,

    /// This is the level of the progression (for instance, the Character Level).
    #[serde(rename = "level")]
    pub level: i32,

    /// This is the maximum possible level you can achieve for this progression (for example, the maximum character level obtainable)
    #[serde(rename = "levelCap")]
    pub level_cap: i32,

    /// Progressions define their levels in "steps". Since the last step may be repeatable, the user may be at a higher level than the actual Step achieved in the progression. Not necessarily useful, but potentially interesting for those cruising the API. Relate this to the "steps" property of the DestinyProgression to see which step the user is on, if you care about that. (Note that this is Content Version dependent since it refers to indexes.)
    #[serde(rename = "stepIndex")]
    pub step_index: i32,

    /// The amount of progression (i.e. "Experience") needed to reach the next level of this Progression. Jeez, progression is such an overloaded word.
    #[serde(rename = "progressToNextLevel")]
    pub progress_to_next_level: i32,

    /// The total amount of progression (i.e. "Experience") needed in order to reach the next level.
    #[serde(rename = "nextLevelAt")]
    pub next_level_at: i32,

    /// The number of resets of this progression you've executed this season, if applicable to this progression.
    #[serde(rename = "currentResetCount")]
    pub current_reset_count: Option<i32>,

    /// Information about historical resets of this progression, if there is any data for it.
    #[serde(rename = "seasonResets")]
    pub season_resets: Option<Vec<crate::destiny::DestinyProgressionResetEntry>>,

    /// Information about historical rewards for this progression, if there is any data for it.
    #[serde(rename = "rewardItemStates")]
    pub reward_item_states: Option<Vec<crate::destiny::DestinyProgressionRewardItemState>>,
}

/// Represents a season and the number of resets you had in that season.
/// We do not necessarily - even for progressions with resets - track it over all seasons. So be careful and check the season numbers being returned.
#[derive(Deserialize, Serialize)]
pub struct DestinyProgressionResetEntry {
    #[serde(rename = "season")]
    pub season: i32,

    #[serde(rename = "resets")]
    pub resets: i32,
}

/// Represents the different states a progression reward item can be in.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyProgressionRewardItemState {
    None = 0,
    /// If this is set, the reward should be hidden.
    Invisible = 1,
    /// If this is set, the reward has been earned.
    Earned = 2,
    /// If this is set, the reward has been claimed.
    Claimed = 4,
    /// If this is set, the reward is allowed to be claimed by this Character. An item can be earned but still can't be claimed in certain circumstances, like if it's only allowed for certain subclasses. It also might not be able to be claimed if you already claimed it!
    ClaimAllowed = 8,
}

impl Display for DestinyProgressionRewardItemState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// There are many Progressions in Destiny (think Character Level, or Reputation). These are the various "Scopes" of Progressions, which affect many things: * Where/if they are stored * How they are calculated * Where they can be used in other game logic
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyProgressionScope {
    Account = 0,
    Character = 1,
    Clan = 2,
    Item = 3,
    ImplicitFromEquipment = 4,
    Mapped = 5,
    MappedAggregate = 6,
    MappedStat = 7,
    MappedUnlockValue = 8,
}

impl Display for DestinyProgressionScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// If progression is earned, this determines whether the progression shows visual effects on the character or its item - or neither.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyProgressionStepDisplayEffect {
    None = 0,
    Character = 1,
    Item = 2,
}

impl Display for DestinyProgressionStepDisplayEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Used in a number of Destiny contracts to return data about an item stack and its quantity. Can optionally return an itemInstanceId if the item is instanced - in which case, the quantity returned will be 1. If it's not... uh, let me know okay? Thanks.
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyItemQuantity {
    /// The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// If this quantity is referring to a specific instance of an item, this will have the item's instance ID. Normally, this will be null.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: Option<i64>,

    /// The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used.
    #[serde(rename = "quantity")]
    pub quantity: i32,

    /// Indicates that this item quantity may be conditionally shown or hidden, based on various sources of state. For example: server flags, account state, or character progress.
    #[serde(rename = "hasConditionalVisibility")]
    pub has_conditional_visibility: bool,
}

/// Indicates the type of actions that can be performed
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SocketTypeActionType {
    InsertPlug = 0,
    InfuseItem = 1,
    ReinitializeSocket = 2,
}

impl Display for SocketTypeActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinySocketVisibility {
    Visible = 0,
    Hidden = 1,
    HiddenWhenEmpty = 2,
    HiddenIfNoPlugsAvailable = 3,
}

impl Display for DestinySocketVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Represents the possible and known UI styles used by the game for rendering Socket Categories.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinySocketCategoryStyle {
    Unknown = 0,
    Reusable = 1,
    Consumable = 2,
    Unlockable = 3,
    Intrinsic = 4,
    EnergyMeter = 5,
    LargePerk = 6,
    Abilities = 7,
    Supers = 8,
}

impl Display for DestinySocketCategoryStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TierType {
    Unknown = 0,
    Currency = 1,
    Basic = 2,
    Common = 3,
    Rare = 4,
    Superior = 5,
    Exotic = 6,
}

impl Display for TierType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BucketScope {
    Character = 0,
    Account = 1,
}

impl Display for BucketScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BucketCategory {
    Invisible = 0,
    Item = 1,
    Currency = 2,
    Equippable = 3,
    Ignored = 4,
}

impl Display for BucketCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ItemLocation {
    Unknown = 0,
    Inventory = 1,
    Vault = 2,
    Vendor = 3,
    Postmaster = 4,
}

impl Display for ItemLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// When a Stat (DestinyStatDefinition) is aggregated, this is the rules used for determining the level and formula used for aggregation.
/// * CharacterAverage = apply a weighted average using the related DestinyStatGroupDefinition on the DestinyInventoryItemDefinition across the character's equipped items. See both of those definitions for details. * Character = don't aggregate: the stat should be located and used directly on the character. * Item = don't aggregate: the stat should be located and used directly on the item.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyStatAggregationType {
    CharacterAverage = 0,
    Character = 1,
    Item = 2,
}

impl Display for DestinyStatAggregationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// At last, stats have categories. Use this for whatever purpose you might wish.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyStatCategory {
    Gameplay = 0,
    Weapon = 1,
    Defense = 2,
    Primary = 3,
}

impl Display for DestinyStatCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EquippingItemBlockAttributes {
    None = 0,
    EquipOnAcquire = 1,
}

impl Display for EquippingItemBlockAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyAmmunitionType {
    None = 0,
    Primary = 1,
    Special = 2,
    Heavy = 3,
    Unknown = 4,
}

impl Display for DestinyAmmunitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[derive(Deserialize, Serialize)]
pub struct DyeReference {
    #[serde(rename = "channelHash")]
    pub channel_hash: u32,

    #[serde(rename = "dyeHash")]
    pub dye_hash: u32,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 3,
}

impl Display for DestinyClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyGender {
    Male = 0,
    Female = 1,
    Unknown = 2,
}

impl Display for DestinyGender {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Describes the type of progression that a vendor has.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyVendorProgressionType {
    /// The original rank progression from token redemption.
    Default = 0,
    /// Progression from ranks in ritual content. For example: Crucible (Shaxx), Gambit (Drifter), and Season 13 Battlegrounds (War Table).
    Ritual = 1,
    /// A vendor progression with no seasonal refresh. For example: Xur in the Eternity destination for the 30th Anniversary.
    NoSeasonalRefresh = 2,
}

impl Display for DestinyVendorProgressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Display categories can have custom sort orders. These are the possible options.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VendorDisplayCategorySortOrder {
    Default = 0,
    SortByTier = 1,
}

impl Display for VendorDisplayCategorySortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// When a Vendor Interaction provides rewards, they'll either let you choose one or let you have all of them. This determines which it will be.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyVendorInteractionRewardSelection {
    None = 0,
    One = 1,
    All = 2,
}

impl Display for DestinyVendorInteractionRewardSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// This determines the type of reply that a Vendor will have during an Interaction.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyVendorReplyType {
    Accept = 0,
    Decline = 1,
    Complete = 2,
}

impl Display for DestinyVendorReplyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// An enumeration of the known UI interactions for Vendors.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VendorInteractionType {
    Unknown = 0,
    /// An empty interaction. If this ends up in content, it is probably a game bug.
    Undefined = 1,
    /// An interaction shown when you complete a quest and receive a reward.
    QuestComplete = 2,
    /// An interaction shown when you talk to a Vendor as an intermediary step of a quest.
    QuestContinue = 3,
    /// An interaction shown when you are previewing the vendor's reputation rewards.
    ReputationPreview = 4,
    /// An interaction shown when you rank up with the vendor.
    RankUpReward = 5,
    /// An interaction shown when you have tokens to turn in for the vendor.
    TokenTurnIn = 6,
    /// An interaction shown when you're accepting a new quest.
    QuestAccept = 7,
    /// Honestly, this doesn't seem consistent to me. It is used to give you choices in the Cryptarch as well as some reward prompts by the Eververse vendor. I'll have to look into that further at some point.
    ProgressTab = 8,
    /// These seem even less consistent. I don't know what these are.
    End = 9,
    /// Also seem inconsistent. I also don't know what these are offhand.
    Start = 10,
}

impl Display for VendorInteractionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Determines how items are sorted in an inventory bucket.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyItemSortType {
    ItemId = 0,
    Timestamp = 1,
    StackSize = 2,
}

impl Display for DestinyItemSortType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// The action that happens when the user attempts to refund an item.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyVendorItemRefundPolicy {
    NotRefundable = 0,
    DeletesItem = 1,
    RevokesLicense = 2,
}

impl Display for DestinyVendorItemRefundPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// This enumeration represents the most restrictive type of gating that is being performed by an entity. This is useful as a shortcut to avoid a lot of lookups when determining whether the gating on an Entity applies to everyone equally, or to their specific Profile or Character states.
/// None = There is no gating on this item.
/// Global = The gating on this item is based entirely on global game state. It will be gated the same for everyone.
/// Clan = The gating on this item is at the Clan level. For instance, if you're gated by Clan level this will be the case.
/// Profile = The gating includes Profile-specific checks, but not on the Profile's characters. An example of this might be when you acquire an Emblem: the Emblem will be available in your Kiosk for all characters in your Profile from that point onward.
/// Character = The gating includes Character-specific checks, including character level restrictions. An example of this might be an item that you can't purchase from a Vendor until you reach a specific Character Level.
/// Item = The gating includes item-specific checks. For BNet, this generally implies that we'll show this data only on a character level or deeper.
/// AssumedWorstCase = The unlocks and checks being used for this calculation are of an unknown type and are used for unknown purposes. For instance, if some great person decided that an unlock value should be globally scoped, but then the game changes it using character-specific data in a way that BNet doesn't know about. Because of the open-ended potential for this to occur, many unlock checks for "globally" scoped unlock data may be assumed as the worst case unless it has been specifically whitelisted as otherwise. That sucks, but them's the breaks.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyGatingScope {
    None = 0,
    Global = 1,
    Clan = 2,
    Profile = 3,
    Character = 4,
    Item = 5,
    AssumedWorstCase = 6,
}

impl Display for DestinyGatingScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// The various known UI styles in which an item can be highlighted. It'll be up to you to determine what you want to show based on this highlighting, BNet doesn't have any assets that correspond to these states. And yeah, RiseOfIron and Comet have their own special highlight states. Don't ask me, I can't imagine they're still used.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ActivityGraphNodeHighlightType {
    None = 0,
    Normal = 1,
    Hyper = 2,
    Comet = 3,
    RiseOfIron = 4,
}

impl Display for ActivityGraphNodeHighlightType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// If you're showing an unlock value in the UI, this is the format in which it should be shown. You'll have to build your own algorithms on the client side to determine how best to render these options.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyUnlockValueUIStyle {
    /// Generally, Automatic means "Just show the number"
    Automatic = 0,
    /// Show the number as a fractional value. For this to make sense, the value being displayed should have a comparable upper bound, like the progress to the next level of a Progression.
    Fraction = 1,
    /// Show the number as a checkbox. 0 Will mean unchecked, any other value will mean checked.
    Checkbox = 2,
    /// Show the number as a percentage. For this to make sense, the value being displayed should have a comparable upper bound, like the progress to the next level of a Progression.
    Percentage = 3,
    /// Show the number as a date and time. The number will be the number of seconds since the Unix Epoch (January 1st, 1970 at midnight UTC). It'll be up to you to convert this into a date and time format understandable to the user in their time zone.
    DateTime = 4,
    /// Show the number as a floating point value that represents a fraction, where 0 is min and 1 is max. For this to make sense, the value being displayed should have a comparable upper bound, like the progress to the next level of a Progression.
    FractionFloat = 5,
    /// Show the number as a straight-up integer.
    Integer = 6,
    /// Show the number as a time duration. The value will be returned as seconds.
    TimeDuration = 7,
    /// Don't bother showing the value at all, it's not easily human-interpretable, and used for some internal purpose.
    Hidden = 8,
    /// Example: "1.5x"
    Multiplier = 9,
    /// Show the value as a series of green pips, like the wins in a Trials of Osiris score card.
    GreenPips = 10,
    /// Show the value as a series of red pips, like the losses in a Trials of Osiris score card.
    RedPips = 11,
    /// Show the value as a percentage. For example: "51%" - Does no division, only appends '%'
    ExplicitPercentage = 12,
    /// Show the value as a floating-point number. For example: "4.52" NOTE: Passed along from Investment as whole number with last two digits as decimal values (452 -> 4.52)
    RawFloat = 13,
    /// Show the value as a level and a reward.
    LevelAndReward = 14,
}

impl Display for DestinyUnlockValueUIStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Some Objectives provide perks, generally as part of providing some kind of interesting modifier for a Challenge or Quest. This indicates when the Perk is granted.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyObjectiveGrantStyle {
    WhenIncomplete = 0,
    WhenComplete = 1,
    Always = 2,
}

impl Display for DestinyObjectiveGrantStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DamageType {
    None = 0,
    Kinetic = 1,
    Arc = 2,
    Thermal = 3,
    Void = 4,
    Raid = 5,
    Stasis = 6,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// If the objective has a known UI label, this enumeration will represent it.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyObjectiveUiStyle {
    None = 0,
    Highlighted = 1,
    CraftingWeaponLevel = 2,
    CraftingWeaponLevelProgress = 3,
    CraftingWeaponTimestamp = 4,
    CraftingMementos = 5,
    CraftingMementoTitle = 6,
}

impl Display for DestinyObjectiveUiStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyActivityNavPointType {
    Inactive = 0,
    PrimaryObjective = 1,
    SecondaryObjective = 2,
    TravelObjective = 3,
    PublicEventObjective = 4,
    AmmoCache = 5,
    PointTypeFlag = 6,
    CapturePoint = 7,
    DefensiveEncounter = 8,
    GhostInteraction = 9,
    KillAi = 10,
    QuestItem = 11,
    PatrolMission = 12,
    Incoming = 13,
    ArenaObjective = 14,
    AutomationHint = 15,
    TrackedQuest = 16,
}

impl Display for DestinyActivityNavPointType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Activity Modes are grouped into a few possible broad categories.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyActivityModeCategory {
    /// Activities that are neither PVP nor PVE, such as social activities.
    None = 0,
    /// PvE activities, where you shoot aliens in the face.
    PvE = 1,
    /// PvP activities, where you shoot your "friends".
    PvP = 2,
    /// PVE competitive activities, where you shoot whoever you want whenever you want. Or run around collecting small glowing triangles.
    PvECompetitive = 3,
}

impl Display for DestinyActivityModeCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// This Enumeration further classifies items by more specific categorizations than DestinyItemType. The "Sub-Type" is where we classify and categorize items one step further in specificity: "Auto Rifle" instead of just "Weapon" for example, or "Vanguard Bounty" instead of merely "Bounty".
/// These sub-types are provided for historical compatibility with Destiny 1, but an ideal alternative is to use DestinyItemCategoryDefinitions and the DestinyItemDefinition.itemCategories property instead. Item Categories allow for arbitrary hierarchies of specificity, and for items to belong to multiple categories across multiple hierarchies simultaneously. For this enum, we pick a single type as a "best guess" fit.
/// NOTE: This is not all of the item types available, and some of these are holdovers from Destiny 1 that may or may not still exist.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyItemSubType {
    None = 0,
    /// DEPRECATED. Items can be both "Crucible" and something else interesting.
    Crucible = 1,
    /// DEPRECATED. An item can both be "Vanguard" and something else.
    Vanguard = 2,
    /// DEPRECATED. An item can both be Exotic and something else.
    Exotic = 5,
    AutoRifle = 6,
    Shotgun = 7,
    Machinegun = 8,
    HandCannon = 9,
    RocketLauncher = 10,
    FusionRifle = 11,
    SniperRifle = 12,
    PulseRifle = 13,
    ScoutRifle = 14,
    /// DEPRECATED. An item can both be CRM and something else.
    Crm = 16,
    Sidearm = 17,
    Sword = 18,
    Mask = 19,
    Shader = 20,
    Ornament = 21,
    FusionRifleLine = 22,
    GrenadeLauncher = 23,
    SubmachineGun = 24,
    TraceRifle = 25,
    HelmetArmor = 26,
    GauntletsArmor = 27,
    ChestArmor = 28,
    LegArmor = 29,
    ClassArmor = 30,
    Bow = 31,
    DummyRepeatableBounty = 32,
    Glaive = 33,
}

impl Display for DestinyItemSubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Represents a potential state of an Activity Graph node.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyGraphNodeState {
    Hidden = 0,
    Visible = 1,
    Teaser = 2,
    Incomplete = 3,
    Completed = 4,
}

impl Display for DestinyGraphNodeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyPresentationNodeType {
    Default = 0,
    Category = 1,
    Collectibles = 2,
    Records = 3,
    Metric = 4,
    Craftable = 5,
}

impl Display for DestinyPresentationNodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// There's a lot of places where we need to know scope on more than just a profile or character level. For everything else, there's this more generic sense of scope.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyScope {
    Profile = 0,
    Character = 1,
}

impl Display for DestinyScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A hint for how the presentation node should be displayed when shown in a list. How you use this is your UI is up to you.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyPresentationDisplayStyle {
    /// Display the item as a category, through which sub-items are filtered.
    Category = 0,
    Badge = 1,
    Medals = 2,
    Collectible = 3,
    Record = 4,
}

impl Display for DestinyPresentationDisplayStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyRecordValueStyle {
    Integer = 0,
    Percentage = 1,
    Milliseconds = 2,
    Boolean = 3,
    Decimal = 4,
}

impl Display for DestinyRecordValueStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyRecordToastStyle {
    None = 0,
    Record = 1,
    Lore = 2,
    Badge = 3,
    MetaRecord = 4,
    MedalComplete = 5,
    SeasonChallengeComplete = 6,
    GildedTitleComplete = 7,
    CraftingRecipeUnlocked = 8,
}

impl Display for DestinyRecordToastStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A hint for what screen should be shown when this presentation node is clicked into. How you use this is your UI is up to you.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyPresentationScreenStyle {
    /// Use the "default" view for the presentation nodes.
    Default = 0,
    /// Show sub-items as "category sets". In-game, you'd see these as a vertical list of child presentation nodes - armor sets for example - and the icons of items within those sets displayed horizontally.
    CategorySets = 1,
    /// Show sub-items as Badges. (I know, I know. We don't need no stinkin' badges har har har)
    Badge = 2,
}

impl Display for DestinyPresentationScreenStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// If the plug has a specific custom style, this enumeration will represent that style/those styles.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlugUiStyles {
    None = 0,
    Masterwork = 1,
}

impl Display for PlugUiStyles {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// This enum determines whether the plug is available to be inserted.
/// - Normal means that all existing rules for plug insertion apply.
/// - UnavailableIfSocketContainsMatchingPlugCategory means that the plug is only available if the socket does NOT match the plug category.
/// - AvailableIfSocketContainsMatchingPlugCategory means that the plug is only available if the socket DOES match the plug category.
/// For category matching, use the plug's "plugCategoryIdentifier" property, comparing it to
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlugAvailabilityMode {
    Normal = 0,
    UnavailableIfSocketContainsMatchingPlugCategory = 1,
    AvailableIfSocketContainsMatchingPlugCategory = 2,
}

impl Display for PlugAvailabilityMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Represents the socket energy types for Armor 2.0, Ghosts 2.0, and Stasis subclasses.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyEnergyType {
    Any = 0,
    Arc = 1,
    Thermal = 2,
    Void = 3,
    Ghost = 4,
    Subclass = 5,
    Stasis = 6,
}

impl Display for DestinyEnergyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Indicates how a socket is populated, and where you should look for valid plug data.
/// This is a flags enumeration/bitmask field, as you may have to look in multiple sources across multiple components for valid plugs.
/// For instance, a socket could have plugs that are sourced from its own definition, as well as plugs that are sourced from Character-scoped AND profile-scoped Plug Sets. Only by combining plug data for every indicated source will you be able to know all of the plugs available for a socket.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SocketPlugSources {
    /// If there's no way we can detect to insert new plugs.
    None = 0,
    /// Use plugs found in the player's inventory, based on the socket type rules (see DestinySocketTypeDefinition for more info)
    /// Note that a socket - like Shaders - can have *both* reusable plugs and inventory items inserted theoretically.
    InventorySourced = 1,
    /// Use the DestinyItemSocketsComponent.sockets.reusablePlugs property to determine which plugs are valid for this socket. This may have to be combined with other sources, such as plug sets, if those flags are set.
    /// Note that "Reusable" plugs may not necessarily come from a plug set, nor from the "reusablePlugItems" in the socket's Definition data. They can sometimes be "randomized" in which case the only source of truth at the moment is still the runtime DestinyItemSocketsComponent.sockets.reusablePlugs property.
    ReusablePlugItems = 2,
    /// Use the ProfilePlugSets (DestinyProfileResponse.profilePlugSets) component data to determine which plugs are valid for this socket.
    ProfilePlugSet = 4,
    /// Use the CharacterPlugSets (DestinyProfileResponse.characterPlugSets) component data to determine which plugs are valid for this socket.
    CharacterPlugSet = 8,
}

impl Display for SocketPlugSources {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Indicates how a perk should be shown, or if it should be, in the game UI. Maybe useful for those of you trying to filter out internal-use-only perks (or for those of you trying to figure out what they do!)
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ItemPerkVisibility {
    Visible = 0,
    Disabled = 1,
    Hidden = 2,
}

impl Display for ItemPerkVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// As you run into items that need to be classified for Milestone purposes in ways that we cannot infer via direct data, add a new classification here and use a string constant to represent it in the local item config file.
/// NOTE: This is not all of the item types available, and some of these are holdovers from Destiny 1 that may or may not still exist.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpecialItemType {
    None = 0,
    SpecialCurrency = 1,
    Armor = 8,
    Weapon = 9,
    Engram = 23,
    Consumable = 24,
    ExchangeMaterial = 25,
    MissionReward = 27,
    Currency = 29,
}

impl Display for SpecialItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// An enumeration that indicates the high-level "type" of the item, attempting to iron out the context specific differences for specific instances of an entity. For instance, though a weapon may be of various weapon "Types", in DestinyItemType they are all classified as "Weapon". This allows for better filtering on a higher level of abstraction for the concept of types.
/// This enum is provided for historical compatibility with Destiny 1, but an ideal alternative is to use DestinyItemCategoryDefinitions and the DestinyItemDefinition.itemCategories property instead. Item Categories allow for arbitrary hierarchies of specificity, and for items to belong to multiple categories across multiple hierarchies simultaneously. For this enum, we pick a single type as a "best guess" fit.
/// NOTE: This is not all of the item types available, and some of these are holdovers from Destiny 1 that may or may not still exist.
/// I keep updating these because they're so damn convenient. I guess I shouldn't fight it.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyItemType {
    None = 0,
    Currency = 1,
    Armor = 2,
    Weapon = 3,
    Message = 7,
    Engram = 8,
    Consumable = 9,
    ExchangeMaterial = 10,
    MissionReward = 11,
    QuestStep = 12,
    QuestStepComplete = 13,
    Emblem = 14,
    Quest = 15,
    Subclass = 16,
    ClanBanner = 17,
    Aura = 18,
    Mod = 19,
    Dummy = 20,
    Ship = 21,
    Vehicle = 22,
    Emote = 23,
    Ghost = 24,
    Package = 25,
    Bounty = 26,
    Wrapper = 27,
    SeasonalArtifact = 28,
    Finisher = 29,
    Pattern = 30,
}

impl Display for DestinyItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A plug can optionally have a "Breaker Type": a special ability that can affect units in unique ways. Activating this plug can grant one of these types.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyBreakerType {
    None = 0,
    ShieldPiercing = 1,
    Disruption = 2,
    Stagger = 3,
}

impl Display for DestinyBreakerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Represents the different kinds of acquisition behavior for progression reward items.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyProgressionRewardItemAcquisitionBehavior {
    Instant = 0,
    PlayerClaimRequired = 1,
}

impl Display for DestinyProgressionRewardItemAcquisitionBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ItemBindStatus {
    NotBound = 0,
    BoundToCharacter = 1,
    BoundToAccount = 2,
    BoundToGuild = 3,
}

impl Display for ItemBindStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Whether you can transfer an item, and why not if you can't.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TransferStatuses {
    /// The item can be transferred.
    CanTransfer = 0,
    /// You can't transfer the item because it is equipped on a character.
    ItemIsEquipped = 1,
    /// The item is defined as not transferrable in its DestinyInventoryItemDefinition.nonTransferrable property.
    NotTransferrable = 2,
    /// You could transfer the item, but the place you're trying to put it has run out of room! Check your remaining Vault and/or character space.
    NoRoomInDestination = 4,
}

impl Display for TransferStatuses {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A flags enumeration/bitmask where each bit represents a different possible state that the item can be in that may effect how the item is displayed to the user and what actions can be performed against it.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ItemState {
    None = 0,
    /// If this bit is set, the item has been "locked" by the user and cannot be deleted. You may want to represent this visually with a "lock" icon.
    Locked = 1,
    /// If this bit is set, the item is a quest that's being tracked by the user. You may want a visual indicator to show that this is a tracked quest.
    Tracked = 2,
    /// If this bit is set, the item has a Masterwork plug inserted. This usually coincides with having a special "glowing" effect applied to the item's icon.
    Masterwork = 4,
    /// If this bit is set, the item has been 'crafted' by the player. You may want to represent this visually with a "crafted" icon overlay.
    Crafted = 8,
    /// If this bit is set, the item has a 'highlighted' objective. You may want to represent this with an orange-red icon border color.
    HighlightedObjective = 16,
}

impl Display for ItemState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A flags enumeration/bitmask indicating the versions of the game that a given user has purchased.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyGameVersions {
    None = 0,
    Destiny2 = 1,
    DLC1 = 2,
    DLC2 = 4,
    Forsaken = 8,
    YearTwoAnnualPass = 16,
    Shadowkeep = 32,
    BeyondLight = 64,
    Anniversary30th = 128,
    TheWitchQueen = 256,
}

impl Display for DestinyGameVersions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Represents the possible components that can be returned from Destiny "Get" calls such as GetProfile, GetCharacter, GetVendor etc...
/// When making one of these requests, you will pass one or more of these components as a comma separated list in the "?components=" querystring parameter. For instance, if you want baseline Profile data, Character Data, and character progressions, you would pass "?components=Profiles,Characters,CharacterProgressions" You may use either the numerical or string values.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyComponentType {
    None = 0,
    /// Profiles is the most basic component, only relevant when calling GetProfile. This returns basic information about the profile, which is almost nothing: a list of characterIds, some information about the last time you logged in, and that most sobering statistic: how long you've played.
    Profiles = 100,
    /// Only applicable for GetProfile, this will return information about receipts for refundable vendor items.
    VendorReceipts = 101,
    /// Asking for this will get you the profile-level inventories, such as your Vault buckets (yeah, the Vault is really inventory buckets located on your Profile)
    ProfileInventories = 102,
    /// This will get you a summary of items on your Profile that we consider to be "currencies", such as Glimmer. I mean, if there's Glimmer in Destiny 2. I didn't say there was Glimmer.
    ProfileCurrencies = 103,
    /// This will get you any progression-related information that exists on a Profile-wide level, across all characters.
    ProfileProgression = 104,
    /// This will get you information about the silver that this profile has on every platform on which it plays.
    /// You may only request this component for the logged in user's Profile, and will not recieve it if you request it for another Profile.
    PlatformSilver = 105,
    /// This will get you summary info about each of the characters in the profile.
    Characters = 200,
    /// This will get you information about any non-equipped items on the character or character(s) in question, if you're allowed to see it. You have to either be authenticated as that user, or that user must allow anonymous viewing of their non-equipped items in Bungie.Net settings to actually get results.
    CharacterInventories = 201,
    /// This will get you information about the progression (faction, experience, etc... "levels") relevant to each character, if you are the currently authenticated user or the user has elected to allow anonymous viewing of its progression info.
    CharacterProgressions = 202,
    /// This will get you just enough information to be able to render the character in 3D if you have written a 3D rendering library for Destiny Characters, or "borrowed" ours. It's okay, I won't tell anyone if you're using it. I'm no snitch. (actually, we don't care if you use it - go to town)
    CharacterRenderData = 203,
    /// This will return info about activities that a user can see and gating on it, if you are the currently authenticated user or the user has elected to allow anonymous viewing of its progression info. Note that the data returned by this can be unfortunately problematic and relatively unreliable in some cases. We'll eventually work on making it more consistently reliable.
    CharacterActivities = 204,
    /// This will return info about the equipped items on the character(s). Everyone can see this.
    CharacterEquipment = 205,
    /// This will return basic info about instanced items - whether they can be equipped, their tracked status, and some info commonly needed in many places (current damage type, primary stat value, etc)
    ItemInstances = 300,
    /// Items can have Objectives (DestinyObjectiveDefinition) bound to them. If they do, this will return info for items that have such bound objectives.
    ItemObjectives = 301,
    /// Items can have perks (DestinyPerkDefinition). If they do, this will return info for what perks are active on items.
    ItemPerks = 302,
    /// If you just want to render the weapon, this is just enough info to do that rendering.
    ItemRenderData = 303,
    /// Items can have stats, like rate of fire. Asking for this component will return requested item's stats if they have stats.
    ItemStats = 304,
    /// Items can have sockets, where plugs can be inserted. Asking for this component will return all info relevant to the sockets on items that have them.
    ItemSockets = 305,
    /// Items can have talent grids, though that matters a lot less frequently than it used to. Asking for this component will return all relevant info about activated Nodes and Steps on this talent grid, like the good ol' days.
    ItemTalentGrids = 306,
    /// Items that *aren't* instanced still have important information you need to know: how much of it you have, the itemHash so you can look up their DestinyInventoryItemDefinition, whether they're locked, etc... Both instanced and non-instanced items will have these properties. You will get this automatically with Inventory components - you only need to pass this when calling GetItem on a specific item.
    ItemCommonData = 307,
    /// Items that are "Plugs" can be inserted into sockets. This returns statuses about those plugs and why they can/can't be inserted. I hear you giggling, there's nothing funny about inserting plugs. Get your head out of the gutter and pay attention!
    ItemPlugStates = 308,
    /// Sometimes, plugs have objectives on them. This data can get really large, so we split it into its own component. Please, don't grab it unless you need it.
    ItemPlugObjectives = 309,
    /// Sometimes, designers create thousands of reusable plugs and suddenly your response sizes are almost 3MB, and something has to give.
    /// Reusable Plugs were split off as their own component, away from ItemSockets, as a result of the Plug changes in Shadowkeep that made plug data infeasibly large for the most common use cases.
    /// Request this component if and only if you need to know what plugs *could* be inserted into a socket, and need to know it before "drilling" into the details of an item in your application (for instance, if you're doing some sort of interesting sorting or aggregation based on available plugs.
    /// When you get this, you will also need to combine it with "Plug Sets" data if you want a full picture of all of the available plugs: this component will only return plugs that have state data that is per-item. See Plug Sets for available plugs that have Character, Profile, or no state-specific restrictions.
    ItemReusablePlugs = 310,
    /// When obtaining vendor information, this will return summary information about the Vendor or Vendors being returned.
    Vendors = 400,
    /// When obtaining vendor information, this will return information about the categories of items provided by the Vendor.
    VendorCategories = 401,
    /// When obtaining vendor information, this will return the information about items being sold by the Vendor.
    VendorSales = 402,
    /// Asking for this component will return you the account's Kiosk statuses: that is, what items have been filled out/acquired. But only if you are the currently authenticated user or the user has elected to allow anonymous viewing of its progression info.
    Kiosks = 500,
    /// A "shortcut" component that will give you all of the item hashes/quantities of items that the requested character can use to determine if an action (purchasing, socket insertion) has the required currency. (recall that all currencies are just items, and that some vendor purchases require items that you might not traditionally consider to be a "currency", like plugs/mods!)
    CurrencyLookups = 600,
    /// Returns summary status information about all "Presentation Nodes". See DestinyPresentationNodeDefinition for more details, but the gist is that these are entities used by the game UI to bucket Collectibles and Records into a hierarchy of categories. You may ask for and use this data if you want to perform similar bucketing in your own UI: or you can skip it and roll your own.
    PresentationNodes = 700,
    /// Returns summary status information about all "Collectibles". These are records of what items you've discovered while playing Destiny, and some other basic information. For detailed information, you will have to call a separate endpoint devoted to the purpose.
    Collectibles = 800,
    /// Returns summary status information about all "Records" (also known in the game as "Triumphs". I know, it's confusing because there's also "Moments of Triumph" that will themselves be represented as "Triumphs.")
    Records = 900,
    /// Returns information that Bungie considers to be "Transitory": data that may change too frequently or come from a non-authoritative source such that we don't consider the data to be fully trustworthy, but that might prove useful for some limited use cases. We can provide no guarantee of timeliness nor consistency for this data: buyer beware with the Transitory component.
    Transitory = 1000,
    /// Returns summary status information about all "Metrics" (also known in the game as "Stat Trackers").
    Metrics = 1100,
    /// Returns a mapping of localized string variable hashes to values, on a per-account or per-character basis.
    StringVariables = 1200,
    /// Returns summary status information about all "Craftables" aka crafting recipe items.
    Craftables = 1300,
}

impl Display for DestinyComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// I know this doesn't look like a Flags Enumeration/bitmask right now, but I assure you it is. This is the possible states that a Presentation Node can be in, and it is almost certain that its potential states will increase in the future. So don't treat it like a straight up enumeration.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyPresentationNodeState {
    None = 0,
    /// If this is set, the game recommends that you not show this node. But you know your life, do what you've got to do.
    Invisible = 1,
    /// Turns out Presentation Nodes can also be obscured. If they are, this is set.
    Obscured = 2,
}

impl Display for DestinyPresentationNodeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A Flags enumeration/bitmask where each bit represents a possible state that a Record/Triumph can be in.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyRecordState {
    /// If there are no flags set, the record is in a state where it *could* be redeemed, but it has not been yet.
    None = 0,
    /// If this is set, the completed record has been redeemed.
    RecordRedeemed = 1,
    /// If this is set, there's a reward available from this Record but it's unavailable for redemption.
    RewardUnavailable = 2,
    /// If this is set, the objective for this Record has not yet been completed.
    ObjectiveNotCompleted = 4,
    /// If this is set, the game recommends that you replace the display text of this Record with DestinyRecordDefinition.stateInfo.obscuredString.
    Obscured = 8,
    /// If this is set, the game recommends that you not show this record. Do what you will with this recommendation.
    Invisible = 16,
    /// If this is set, you can't complete this record because you lack some permission that's required to complete it.
    EntitlementUnowned = 32,
    /// If this is set, the record has a title (check DestinyRecordDefinition for title info) and you can equip it.
    CanEquipTitle = 64,
}

impl Display for DestinyRecordState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A Flags Enumeration/bitmask where each bit represents a different state that the Collectible can be in. A collectible can be in any number of these states, and you can choose to use or ignore any or all of them when making your own UI that shows Collectible info. Our displays are going to honor them, but we're also the kind of people who only pretend to inhale before quickly passing it to the left. So, you know, do what you got to do.
/// (All joking aside, please note the caveat I mention around the Invisible flag: there are cases where it is in the best interest of your users to honor these flags even if you're a "show all the data" person. Collector-oriented compulsion is a very unfortunate and real thing, and I would hate to instill that compulsion in others through showing them items that they cannot earn. Please consider this when you are making your own apps/sites.)
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyCollectibleState {
    None = 0,
    /// If this flag is set, you have not yet obtained this collectible.
    NotAcquired = 1,
    /// If this flag is set, the item is "obscured" to you: you can/should use the alternate item hash found in DestinyCollectibleDefinition.stateInfo.obscuredOverrideItemHash when displaying this collectible instead of the default display info.
    Obscured = 2,
    /// If this flag is set, the collectible should not be shown to the user.
    /// Please do consider honoring this flag. It is used - for example - to hide items that a person didn't get from the Eververse. I can't prevent these from being returned in definitions, because some people may have acquired them and thus they should show up: but I would hate for people to start feeling some variant of a Collector's Remorse about these items, and thus increasing their purchasing based on that compulsion. That would be a very unfortunate outcome, and one that I wouldn't like to see happen. So please, whether or not I'm your mom, consider honoring this flag and don't show people invisible collectibles.
    Invisible = 4,
    /// If this flag is set, the collectible requires payment for creating an instance of the item, and you are lacking in currency. Bring the benjamins next time. Or spinmetal. Whatever.
    CannotAffordMaterialRequirements = 8,
    /// If this flag is set, you can't pull this item out of your collection because there's no room left in your inventory.
    InventorySpaceUnavailable = 16,
    /// If this flag is set, you already have one of these items and can't have a second one.
    UniquenessViolation = 32,
    /// If this flag is set, the ability to pull this item out of your collection has been disabled.
    PurchaseDisabled = 64,
}

impl Display for DestinyCollectibleState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A flags enumeration that represents a Fireteam Member's status.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyPartyMemberStates {
    None = 0,
    /// This one's pretty obvious - they're on your Fireteam.
    FireteamMember = 1,
    /// I don't know what it means to be in a 'Posse', but apparently this is it.
    PosseMember = 2,
    /// Nor do I understand the difference between them being in a 'Group' vs. a 'Fireteam'.
    /// I'll update these docs once I get more info. If I get more info. If you're reading this, I never got more info. You're on your own, kid.
    GroupMember = 4,
    /// This person is the party leader.
    PartyLeader = 8,
}

impl Display for DestinyPartyMemberStates {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A player can choose to restrict requests to join their Fireteam to specific states. These are the possible states a user can choose.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyGamePrivacySetting {
    Open = 0,
    ClanAndFriendsOnly = 1,
    FriendsOnly = 2,
    InvitationOnly = 3,
    Closed = 4,
}

impl Display for DestinyGamePrivacySetting {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// A Flags enumeration representing the reasons why a person can't join this user's fireteam.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyJoinClosedReasons {
    None = 0,
    /// The user is currently in matchmaking.
    InMatchmaking = 1,
    /// The user is currently in a loading screen.
    Loading = 2,
    /// The user is in an activity that requires solo play.
    SoloMode = 4,
    /// The user can't be joined for one of a variety of internal reasons. Basically, the game can't let you join at this time, but for reasons that aren't under the control of this user.
    InternalReasons = 8,
    /// The user's current activity/quest/other transitory game state is preventing joining.
    DisallowedByGameState = 16,
    /// The user appears to be offline.
    Offline = 32768,
}

impl Display for DestinyJoinClosedReasons {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyRace {
    Human = 0,
    Awoken = 1,
    Exo = 2,
    Unknown = 3,
}

impl Display for DestinyRace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Represents the "Live" data that we can obtain about a Character's status with a specific Activity. This will tell you whether the character can participate in the activity, as well as some other basic mutable information.
/// Meant to be combined with static DestinyActivityDefinition data for a full picture of the Activity.
#[derive(Deserialize, Serialize)]
pub struct DestinyActivity {
    /// The hash identifier of the Activity. Use this to look up the DestinyActivityDefinition of the activity.
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,

    /// If true, then the activity should have a "new" indicator in the Director UI.
    #[serde(rename = "isNew")]
    pub is_new: bool,

    /// If true, the user is allowed to lead a Fireteam into this activity.
    #[serde(rename = "canLead")]
    pub can_lead: bool,

    /// If true, the user is allowed to join with another Fireteam in this activity.
    #[serde(rename = "canJoin")]
    pub can_join: bool,

    /// If true, we both have the ability to know that the user has completed this activity and they have completed it. Unfortunately, we can't necessarily know this for all activities. As such, this should probably only be used if you already know in advance which specific activities you wish to check.
    #[serde(rename = "isCompleted")]
    pub is_completed: bool,

    /// If true, the user should be able to see this activity.
    #[serde(rename = "isVisible")]
    pub is_visible: bool,

    /// The difficulty level of the activity, if applicable.
    #[serde(rename = "displayLevel")]
    pub display_level: Option<i32>,

    /// The recommended light level for the activity, if applicable.
    #[serde(rename = "recommendedLight")]
    pub recommended_light: Option<i32>,

    /// A DestinyActivityDifficultyTier enum value indicating the difficulty of the activity.
    #[serde(rename = "difficultyTier")]
    pub difficulty_tier: crate::destiny::DestinyActivityDifficultyTier,

    #[serde(rename = "challenges")]
    pub challenges: Option<Vec<crate::destiny::challenges::DestinyChallengeStatus>>,

    /// If the activity has modifiers, this will be the list of modifiers that all variants have in common. Perform lookups against DestinyActivityModifierDefinition which defines the modifier being applied to get at the modifier data.
    /// Note that, in the DestiyActivityDefinition, you will see many more modifiers than this being referred to: those are all *possible* modifiers for the activity, not the active ones. Use only the active ones to match what's really live.
    #[serde(rename = "modifierHashes")]
    pub modifier_hashes: Option<Vec<u32>>,

    /// The set of activity options for this activity, keyed by an identifier that's unique for this activity (not guaranteed to be unique between or across all activities, though should be unique for every *variant* of a given *conceptual* activity: for instance, the original D2 Raid has many variant DestinyActivityDefinitions. While other activities could potentially have the same option hashes, for any given D2 base Raid variant the hash will be unique).
    /// As a concrete example of this data, the hashes you get for Raids will correspond to the currently active "Challenge Mode".
    /// We don't have any human readable information for these, but saavy 3rd party app users could manually associate the key (a hash identifier for the "option" that is enabled/disabled) and the value (whether it's enabled or disabled presently)
    /// On our side, we don't necessarily even know what these are used for (the game designers know, but we don't), and we have no human readable data for them. In order to use them, you will have to do some experimentation.
    #[serde(rename = "booleanActivityOptions")]
    pub boolean_activity_options: Option<HashMap<u32, bool>>,

    /// If returned, this is the index into the DestinyActivityDefinition's "loadouts" property, indicating the currently active loadout requirements.
    #[serde(rename = "loadoutRequirementIndex")]
    pub loadout_requirement_index: Option<i32>,
}

/// An enumeration representing the potential difficulty levels of an activity. Their names are... more qualitative than quantitative.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyActivityDifficultyTier {
    Trivial = 0,
    Easy = 1,
    Normal = 2,
    Challenging = 3,
    Hard = 4,
    Brave = 5,
    AlmostImpossible = 6,
    Impossible = 7,
}

impl Display for DestinyActivityDifficultyTier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Represents a stat on an item *or* Character (NOT a Historical Stat, but a physical attribute stat like Attack, Defense etc...)
#[derive(Deserialize, Serialize)]
pub struct DestinyStat {
    /// The hash identifier for the Stat. Use it to look up the DestinyStatDefinition for static data about the stat.
    #[serde(rename = "statHash")]
    pub stat_hash: u32,

    /// The current value of the Stat.
    #[serde(rename = "value")]
    pub value: i32,
}

/// The reasons why an item cannot be equipped, if any. Many flags can be set, or "None" if
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EquipFailureReason {
    /// The item is/was able to be equipped.
    None = 0,
    /// This is not the kind of item that can be equipped. Did you try equipping Glimmer or something?
    ItemUnequippable = 1,
    /// This item is part of a "unique set", and you can't have more than one item of that same set type equipped at once. For instance, if you already have an Exotic Weapon equipped, you can't equip a second one in another weapon slot.
    ItemUniqueEquipRestricted = 2,
    /// This item has state-based gating that prevents it from being equipped in certain circumstances. For instance, an item might be for Warlocks only and you're a Titan, or it might require you to have beaten some special quest that you haven't beaten yet. Use the additional failure data passed on the item itself to get more information about what the specific failure case was (See DestinyInventoryItemDefinition and DestinyItemInstanceComponent)
    ItemFailedUnlockCheck = 4,
    /// This item requires you to have reached a specific character level in order to equip it, and you haven't reached that level yet.
    ItemFailedLevelCheck = 8,
    /// This item can't be equipped on the character requested, because it must be in that character's inventory first. Transfer the item to the character you want to equip it before you attempt to equip it.
    ItemNotOnCharacter = 16,
}

impl Display for EquipFailureReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// I see you've come to find out more about Talent Nodes. I'm so sorry. Talent Nodes are the conceptual, visual nodes that appear on Talent Grids. Talent Grids, in Destiny 1, were found on almost every instanced item: they had Nodes that could be activated to change the properties of the item. In Destiny 2, Talent Grids only exist for Builds/Subclasses, and while the basic concept is the same (Nodes can be activated once you've gained sufficient Experience on the Item, and provide effects), there are some new concepts from Destiny 1. Examine DestinyTalentGridDefinition and its subordinates for more information. This is the "Live" information for the current status of a Talent Node on a specific item. Talent Nodes have many Steps, but only one can be active at any one time: and it is the Step that determines both the visual and the game state-changing properties that the Node provides. Examine this and DestinyTalentNodeStepDefinition carefully. *IMPORTANT NOTE* Talent Nodes are, unfortunately, Content Version DEPENDENT. Though they refer to hashes for Nodes and Steps, those hashes are not guaranteed to be immutable across content versions. This is a source of great exasperation for me, but as a result anyone using Talent Grid data must ensure that the content version of their static content matches that of the server responses before showing or making decisions based on talent grid data.
#[derive(Deserialize, Serialize)]
pub struct DestinyTalentNode {
    /// The index of the Talent Node being referred to (an index into DestinyTalentGridDefinition.nodes[]). CONTENT VERSION DEPENDENT.
    #[serde(rename = "nodeIndex")]
    pub node_index: i32,

    /// The hash of the Talent Node being referred to (in DestinyTalentGridDefinition.nodes). Deceptively CONTENT VERSION DEPENDENT. We have no guarantee of the hash's immutability between content versions.
    #[serde(rename = "nodeHash")]
    pub node_hash: u32,

    /// An DestinyTalentNodeState enum value indicating the node's state: whether it can be activated or swapped, and why not if neither can be performed.
    #[serde(rename = "state")]
    pub state: crate::destiny::DestinyTalentNodeState,

    /// If true, the node is activated: it's current step then provides its benefits.
    #[serde(rename = "isActivated")]
    pub is_activated: bool,

    /// The currently relevant Step for the node. It is this step that has rendering data for the node and the benefits that are provided if the node is activated. (the actual rules for benefits provided are extremely complicated in theory, but with how Talent Grids are being used in Destiny 2 you don't have to worry about a lot of those old Destiny 1 rules.) This is an index into: DestinyTalentGridDefinition.nodes[nodeIndex].steps[stepIndex]
    #[serde(rename = "stepIndex")]
    pub step_index: i32,

    /// If the node has material requirements to be activated, this is the list of those requirements.
    #[serde(rename = "materialsToUpgrade")]
    pub materials_to_upgrade: Option<Vec<crate::destiny::definitions::DestinyMaterialRequirement>>,

    /// The progression level required on the Talent Grid in order to be able to activate this talent node. Talent Grids have their own Progression - similar to Character Level, but in this case it is experience related to the item itself.
    #[serde(rename = "activationGridLevel")]
    pub activation_grid_level: i32,

    /// If you want to show a progress bar or circle for how close this talent node is to being activate-able, this is the percentage to show. It follows the node's underlying rules about when the progress bar should first show up, and when it should be filled.
    #[serde(rename = "progressPercent")]
    pub progress_percent: f32,

    /// Whether or not the talent node is actually visible in the game's UI. Whether you want to show it in your own UI is up to you! I'm not gonna tell you who to sock it to.
    #[serde(rename = "hidden")]
    pub hidden: bool,

    /// This property has some history. A talent grid can provide stats on both the item it's related to and the character equipping the item. This returns data about those stat bonuses.
    #[serde(rename = "nodeStatsBlock")]
    pub node_stats_block: Option<crate::destiny::DestinyTalentNodeStatBlock>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyTalentNodeState {
    Invalid = 0,
    CanUpgrade = 1,
    NoPoints = 2,
    NoPrerequisites = 3,
    NoSteps = 4,
    NoUnlock = 5,
    NoMaterial = 6,
    NoGridLevel = 7,
    SwappingLocked = 8,
    MustSwap = 9,
    Complete = 10,
    Unknown = 11,
    CreationOnly = 12,
    Hidden = 13,
}

impl Display for DestinyTalentNodeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// This property has some history. A talent grid can provide stats on both the item it's related to and the character equipping the item. This returns data about those stat bonuses.
#[derive(Deserialize, Serialize)]
pub struct DestinyTalentNodeStatBlock {
    /// The stat benefits conferred when this talent node is activated for the current Step that is active on the node.
    #[serde(rename = "currentStepStats")]
    pub current_step_stats: Option<Vec<crate::destiny::DestinyStat>>,

    /// This is a holdover from the old days of Destiny 1, when a node could be activated multiple times, conferring multiple steps worth of benefits: you would use this property to show what activating the "next" step on the node would provide vs. what the current step is providing. While Nodes are currently not being used this way, the underlying system for this functionality still exists. I hesitate to remove this property while the ability for designers to make such a talent grid still exists. Whether you want to show it is up to you.
    #[serde(rename = "nextStepStats")]
    pub next_step_stats: Option<Vec<crate::destiny::DestinyStat>>,
}

/// Indicates the type of filter to apply to Vendor results.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyVendorFilter {
    None = 0,
    ApiPurchasable = 1,
}

impl Display for DestinyVendorFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VendorItemStatus {
    Success = 0,
    NoInventorySpace = 1,
    NoFunds = 2,
    NoProgression = 4,
    NoUnlock = 8,
    NoQuantity = 16,
    OutsidePurchaseWindow = 32,
    NotAvailable = 64,
    UniquenessViolation = 128,
    UnknownError = 256,
    AlreadySelling = 512,
    Unsellable = 1024,
    SellingInhibited = 2048,
    AlreadyOwned = 4096,
    DisplayOnly = 8192,
}

impl Display for VendorItemStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Indicates the status of an "Unlock Flag" on a Character or Profile.
/// These are individual bits of state that can be either set or not set, and sometimes provide interesting human-readable information in their related DestinyUnlockDefinition.
#[derive(Deserialize, Serialize)]
pub struct DestinyUnlockStatus {
    /// The hash identifier for the Unlock Flag. Use to lookup DestinyUnlockDefinition for static data. Not all unlocks have human readable data - in fact, most don't. But when they do, it can be very useful to show. Even if they don't have human readable data, you might be able to infer the meaning of an unlock flag with a bit of experimentation...
    #[serde(rename = "unlockHash")]
    pub unlock_hash: u32,

    /// Whether the unlock flag is set.
    #[serde(rename = "isSet")]
    pub is_set: bool,
}

/// The possible states of Destiny Profile Records. IMPORTANT: Any given item can theoretically have many of these states simultaneously: as a result, this was altered to be a flags enumeration/bitmask for v3.2.0.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyVendorItemState {
    /// There are no augments on the item.
    None = 0,
    /// Deprecated forever (probably). There was a time when Records were going to be implemented through Vendors, and this field was relevant. Now they're implemented through Presentation Nodes, and this field doesn't matter anymore.
    Incomplete = 1,
    /// Deprecated forever (probably). See the description of the "Incomplete" value for the juicy scoop.
    RewardAvailable = 2,
    /// Deprecated forever (probably). See the description of the "Incomplete" value for the juicy scoop.
    Complete = 4,
    /// This item is considered to be "newly available", and should have some UI showing how shiny it is.
    New = 8,
    /// This item is being "featured", and should be shiny in a different way from items that are merely new.
    Featured = 16,
    /// This item is only available for a limited time, and that time is approaching.
    Ending = 32,
    /// This item is "on sale". Get it while it's hot.
    OnSale = 64,
    /// This item is already owned.
    Owned = 128,
    /// This item should be shown with a "wide view" instead of normal icon view.
    WideView = 256,
    /// This indicates that you should show some kind of attention-requesting indicator on the item, in a similar manner to items in the nexus that have such notifications.
    NexusAttention = 512,
    /// This indicates that the item has some sort of a 'set' discount.
    SetDiscount = 1024,
    /// This indicates that the item has a price drop.
    PriceDrop = 2048,
    /// This indicates that the item is a daily offer.
    DailyOffer = 4096,
    /// This indicates that the item is for charity.
    Charity = 8192,
    /// This indicates that the item has a seasonal reward expiration.
    SeasonalRewardExpiration = 16384,
    /// This indicates that the sale item is the best deal among different choices.
    BestDeal = 32768,
    /// This indicates that the sale item is popular.
    Popular = 65536,
    /// This indicates that the sale item is free.
    Free = 131072,
    /// This indicates that the sale item is locked.
    Locked = 262144,
}

impl Display for DestinyVendorItemState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// The results of a bulk Equipping operation performed through the Destiny API.
#[derive(Deserialize, Serialize)]
pub struct DestinyEquipItemResults {
    #[serde(rename = "equipResults")]
    pub equip_results: Option<Vec<crate::destiny::DestinyEquipItemResult>>,
}

/// The results of an Equipping operation performed through the Destiny API.
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyEquipItemResult {
    /// The instance ID of the item in question (all items that can be equipped must, but definition, be Instanced and thus have an Instance ID that you can use to refer to them)
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: i64,

    /// A PlatformErrorCodes enum indicating whether it succeeded, and if it failed why.
    #[serde(rename = "equipStatus")]
    pub equip_status: crate::exceptions::PlatformErrorCodes,
}
