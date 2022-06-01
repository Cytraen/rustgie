use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// For historical reasons, this list will have both D1 and D2-relevant Activity Modes in it. Please don't take this to mean that some D1-only feature is coming back!
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyActivityModeType {
    None = 0,
    Story = 2,
    Strike = 3,
    Raid = 4,
    AllPvP = 5,
    Patrol = 6,
    AllPvE = 7,
    Reserved9 = 9,
    Control = 10,
    Reserved11 = 11,
    /// Clash -> Destiny's name for Team Deathmatch. 4v4 combat, the team with the highest kills at the end of time wins.
    Clash = 12,
    Reserved13 = 13,
    CrimsonDoubles = 15,
    Nightfall = 16,
    HeroicNightfall = 17,
    AllStrikes = 18,
    IronBanner = 19,
    Reserved20 = 20,
    Reserved21 = 21,
    Reserved22 = 22,
    Reserved24 = 24,
    AllMayhem = 25,
    Reserved26 = 26,
    Reserved27 = 27,
    Reserved28 = 28,
    Reserved29 = 29,
    Reserved30 = 30,
    Supremacy = 31,
    PrivateMatchesAll = 32,
    Survival = 37,
    Countdown = 38,
    TrialsOfTheNine = 39,
    Social = 40,
    TrialsCountdown = 41,
    TrialsSurvival = 42,
    IronBannerControl = 43,
    IronBannerClash = 44,
    IronBannerSupremacy = 45,
    ScoredNightfall = 46,
    ScoredHeroicNightfall = 47,
    Rumble = 48,
    AllDoubles = 49,
    Doubles = 50,
    PrivateMatchesClash = 51,
    PrivateMatchesControl = 52,
    PrivateMatchesSupremacy = 53,
    PrivateMatchesCountdown = 54,
    PrivateMatchesSurvival = 55,
    PrivateMatchesMayhem = 56,
    PrivateMatchesRumble = 57,
    HeroicAdventure = 58,
    Showdown = 59,
    Lockdown = 60,
    Scorched = 61,
    ScorchedTeam = 62,
    Gambit = 63,
    AllPvECompetitive = 64,
    Breakthrough = 65,
    BlackArmoryRun = 66,
    Salvage = 67,
    IronBannerSalvage = 68,
    PvPCompetitive = 69,
    PvPQuickplay = 70,
    ClashQuickplay = 71,
    ClashCompetitive = 72,
    ControlQuickplay = 73,
    ControlCompetitive = 74,
    GambitPrime = 75,
    Reckoning = 76,
    Menagerie = 77,
    VexOffensive = 78,
    NightmareHunt = 79,
    Elimination = 80,
    Momentum = 81,
    Dungeon = 82,
    Sundial = 83,
    TrialsOfOsiris = 84,
    Dares = 85,
    Offensive = 86,
    LostSector = 87,
    Rift = 88,
    ZoneControl = 89,
    IronBannerRift = 90,
}

impl Display for DestinyActivityModeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsDefinition {
    /// Unique programmer friendly ID for this stat
    #[serde(rename = "statId")]
    pub stat_id: Option<String>,

    /// Statistic group
    #[serde(rename = "group")]
    pub group: crate::destiny::historical_stats::definitions::DestinyStatsGroupType,

    /// Time periods the statistic covers
    #[serde(rename = "periodTypes")]
    pub period_types: Option<Vec<crate::destiny::historical_stats::definitions::PeriodType>>,

    /// Game modes where this statistic can be reported.
    #[serde(rename = "modes")]
    pub modes: Option<Vec<crate::destiny::historical_stats::definitions::DestinyActivityModeType>>,

    /// Category for the stat.
    #[serde(rename = "category")]
    pub category: crate::destiny::historical_stats::definitions::DestinyStatsCategoryType,

    /// Display name
    #[serde(rename = "statName")]
    pub stat_name: Option<String>,

    /// Display name abbreviated
    #[serde(rename = "statNameAbbr")]
    pub stat_name_abbr: Option<String>,

    /// Description of a stat if applicable.
    #[serde(rename = "statDescription")]
    pub stat_description: Option<String>,

    /// Unit, if any, for the statistic
    #[serde(rename = "unitType")]
    pub unit_type: crate::destiny::historical_stats::definitions::UnitType,

    /// Optional URI to an icon for the statistic
    #[serde(rename = "iconImage")]
    pub icon_image: Option<String>,

    /// Optional icon for the statistic
    #[serde(rename = "mergeMethod")]
    pub merge_method: Option<i32>,

    /// Localized Unit Name for the stat.
    #[serde(rename = "unitLabel")]
    pub unit_label: Option<String>,

    /// Weight assigned to this stat indicating its relative impressiveness.
    #[serde(rename = "weight")]
    pub weight: i32,

    /// The tier associated with this medal - be it implicitly or explicitly.
    #[serde(rename = "medalTierHash")]
    pub medal_tier_hash: Option<u32>,
}

/// If the enum value is > 100, it is a "special" group that cannot be queried for directly (special cases apply to when they are returned, and are not relevant in general cases)
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyStatsGroupType {
    None = 0,
    General = 1,
    Weapons = 2,
    Medals = 3,
    /// This is purely to serve as the dividing line between filterable and un-filterable groups. Below this number is a group you can pass as a filter. Above it are groups used in very specific circumstances and not relevant for filtering.
    ReservedGroups = 100,
    /// Only applicable while generating leaderboards.
    Leaderboard = 101,
    /// These will *only* be consumed by GetAggregateStatsByActivity
    Activity = 102,
    /// These are only consumed and returned by GetUniqueWeaponHistory
    UniqueWeapon = 103,
    Internal = 104,
}

impl Display for DestinyStatsGroupType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyStatsCategoryType {
    None = 0,
    Kills = 1,
    Assists = 2,
    Deaths = 3,
    Criticals = 4,
    KDa = 5,
    KD = 6,
    Score = 7,
    Entered = 8,
    TimePlayed = 9,
    MedalWins = 10,
    MedalGame = 11,
    MedalSpecialKills = 12,
    MedalSprees = 13,
    MedalMultiKills = 14,
    MedalAbilities = 15,
}

impl Display for DestinyStatsCategoryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnitType {
    None = 0,
    /// Indicates the statistic is a simple count of something.
    Count = 1,
    /// Indicates the statistic is a per game average.
    PerGame = 2,
    /// Indicates the number of seconds
    Seconds = 3,
    /// Indicates the number of points earned
    Points = 4,
    /// Values represents a team ID
    Team = 5,
    /// Values represents a distance (units to-be-determined)
    Distance = 6,
    /// Ratio represented as a whole value from 0 to 100.
    Percent = 7,
    /// Ratio of something, shown with decimal places
    Ratio = 8,
    /// True or false
    Boolean = 9,
    /// The stat is actually a weapon type.
    WeaponType = 10,
    /// Indicates victory, defeat, or something in between.
    Standing = 11,
    /// Number of milliseconds some event spanned. For example, race time, or lap time.
    Milliseconds = 12,
    /// The value is a enumeration of the Completion Reason type.
    CompletionReason = 13,
}

impl Display for UnitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyStatsMergeMethod {
    /// When collapsing multiple instances of the stat together, add the values.
    Add = 0,
    /// When collapsing multiple instances of the stat together, take the lower value.
    Min = 1,
    /// When collapsing multiple instances of the stat together, take the higher value.
    Max = 2,
}

impl Display for DestinyStatsMergeMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PeriodType {
    None = 0,
    Daily = 1,
    AllTime = 2,
    Activity = 3,
}

impl Display for PeriodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}
