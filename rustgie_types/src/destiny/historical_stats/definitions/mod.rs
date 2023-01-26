use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// For historical reasons, this list will have both D1 and D2-relevant Activity Modes in it. Please don't take this to mean that some D1-only feature is coming back!
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    IronBannerZoneControl = 91,
}

impl Display for DestinyActivityModeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for DestinyActivityModeType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(DestinyActivityModeType::None),
            "Story" => Ok(DestinyActivityModeType::Story),
            "Strike" => Ok(DestinyActivityModeType::Strike),
            "Raid" => Ok(DestinyActivityModeType::Raid),
            "AllPvP" => Ok(DestinyActivityModeType::AllPvP),
            "Patrol" => Ok(DestinyActivityModeType::Patrol),
            "AllPvE" => Ok(DestinyActivityModeType::AllPvE),
            "Reserved9" => Ok(DestinyActivityModeType::Reserved9),
            "Control" => Ok(DestinyActivityModeType::Control),
            "Reserved11" => Ok(DestinyActivityModeType::Reserved11),
            "Clash" => Ok(DestinyActivityModeType::Clash),
            "Reserved13" => Ok(DestinyActivityModeType::Reserved13),
            "CrimsonDoubles" => Ok(DestinyActivityModeType::CrimsonDoubles),
            "Nightfall" => Ok(DestinyActivityModeType::Nightfall),
            "HeroicNightfall" => Ok(DestinyActivityModeType::HeroicNightfall),
            "AllStrikes" => Ok(DestinyActivityModeType::AllStrikes),
            "IronBanner" => Ok(DestinyActivityModeType::IronBanner),
            "Reserved20" => Ok(DestinyActivityModeType::Reserved20),
            "Reserved21" => Ok(DestinyActivityModeType::Reserved21),
            "Reserved22" => Ok(DestinyActivityModeType::Reserved22),
            "Reserved24" => Ok(DestinyActivityModeType::Reserved24),
            "AllMayhem" => Ok(DestinyActivityModeType::AllMayhem),
            "Reserved26" => Ok(DestinyActivityModeType::Reserved26),
            "Reserved27" => Ok(DestinyActivityModeType::Reserved27),
            "Reserved28" => Ok(DestinyActivityModeType::Reserved28),
            "Reserved29" => Ok(DestinyActivityModeType::Reserved29),
            "Reserved30" => Ok(DestinyActivityModeType::Reserved30),
            "Supremacy" => Ok(DestinyActivityModeType::Supremacy),
            "PrivateMatchesAll" => Ok(DestinyActivityModeType::PrivateMatchesAll),
            "Survival" => Ok(DestinyActivityModeType::Survival),
            "Countdown" => Ok(DestinyActivityModeType::Countdown),
            "TrialsOfTheNine" => Ok(DestinyActivityModeType::TrialsOfTheNine),
            "Social" => Ok(DestinyActivityModeType::Social),
            "TrialsCountdown" => Ok(DestinyActivityModeType::TrialsCountdown),
            "TrialsSurvival" => Ok(DestinyActivityModeType::TrialsSurvival),
            "IronBannerControl" => Ok(DestinyActivityModeType::IronBannerControl),
            "IronBannerClash" => Ok(DestinyActivityModeType::IronBannerClash),
            "IronBannerSupremacy" => Ok(DestinyActivityModeType::IronBannerSupremacy),
            "ScoredNightfall" => Ok(DestinyActivityModeType::ScoredNightfall),
            "ScoredHeroicNightfall" => Ok(DestinyActivityModeType::ScoredHeroicNightfall),
            "Rumble" => Ok(DestinyActivityModeType::Rumble),
            "AllDoubles" => Ok(DestinyActivityModeType::AllDoubles),
            "Doubles" => Ok(DestinyActivityModeType::Doubles),
            "PrivateMatchesClash" => Ok(DestinyActivityModeType::PrivateMatchesClash),
            "PrivateMatchesControl" => Ok(DestinyActivityModeType::PrivateMatchesControl),
            "PrivateMatchesSupremacy" => Ok(DestinyActivityModeType::PrivateMatchesSupremacy),
            "PrivateMatchesCountdown" => Ok(DestinyActivityModeType::PrivateMatchesCountdown),
            "PrivateMatchesSurvival" => Ok(DestinyActivityModeType::PrivateMatchesSurvival),
            "PrivateMatchesMayhem" => Ok(DestinyActivityModeType::PrivateMatchesMayhem),
            "PrivateMatchesRumble" => Ok(DestinyActivityModeType::PrivateMatchesRumble),
            "HeroicAdventure" => Ok(DestinyActivityModeType::HeroicAdventure),
            "Showdown" => Ok(DestinyActivityModeType::Showdown),
            "Lockdown" => Ok(DestinyActivityModeType::Lockdown),
            "Scorched" => Ok(DestinyActivityModeType::Scorched),
            "ScorchedTeam" => Ok(DestinyActivityModeType::ScorchedTeam),
            "Gambit" => Ok(DestinyActivityModeType::Gambit),
            "AllPvECompetitive" => Ok(DestinyActivityModeType::AllPvECompetitive),
            "Breakthrough" => Ok(DestinyActivityModeType::Breakthrough),
            "BlackArmoryRun" => Ok(DestinyActivityModeType::BlackArmoryRun),
            "Salvage" => Ok(DestinyActivityModeType::Salvage),
            "IronBannerSalvage" => Ok(DestinyActivityModeType::IronBannerSalvage),
            "PvPCompetitive" => Ok(DestinyActivityModeType::PvPCompetitive),
            "PvPQuickplay" => Ok(DestinyActivityModeType::PvPQuickplay),
            "ClashQuickplay" => Ok(DestinyActivityModeType::ClashQuickplay),
            "ClashCompetitive" => Ok(DestinyActivityModeType::ClashCompetitive),
            "ControlQuickplay" => Ok(DestinyActivityModeType::ControlQuickplay),
            "ControlCompetitive" => Ok(DestinyActivityModeType::ControlCompetitive),
            "GambitPrime" => Ok(DestinyActivityModeType::GambitPrime),
            "Reckoning" => Ok(DestinyActivityModeType::Reckoning),
            "Menagerie" => Ok(DestinyActivityModeType::Menagerie),
            "VexOffensive" => Ok(DestinyActivityModeType::VexOffensive),
            "NightmareHunt" => Ok(DestinyActivityModeType::NightmareHunt),
            "Elimination" => Ok(DestinyActivityModeType::Elimination),
            "Momentum" => Ok(DestinyActivityModeType::Momentum),
            "Dungeon" => Ok(DestinyActivityModeType::Dungeon),
            "Sundial" => Ok(DestinyActivityModeType::Sundial),
            "TrialsOfOsiris" => Ok(DestinyActivityModeType::TrialsOfOsiris),
            "Dares" => Ok(DestinyActivityModeType::Dares),
            "Offensive" => Ok(DestinyActivityModeType::Offensive),
            "LostSector" => Ok(DestinyActivityModeType::LostSector),
            "Rift" => Ok(DestinyActivityModeType::Rift),
            "ZoneControl" => Ok(DestinyActivityModeType::ZoneControl),
            "IronBannerRift" => Ok(DestinyActivityModeType::IronBannerRift),
            "IronBannerZoneControl" => Ok(DestinyActivityModeType::IronBannerZoneControl),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyActivityModeType", s)),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
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
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for DestinyStatsGroupType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(DestinyStatsGroupType::None),
            "General" => Ok(DestinyStatsGroupType::General),
            "Weapons" => Ok(DestinyStatsGroupType::Weapons),
            "Medals" => Ok(DestinyStatsGroupType::Medals),
            "ReservedGroups" => Ok(DestinyStatsGroupType::ReservedGroups),
            "Leaderboard" => Ok(DestinyStatsGroupType::Leaderboard),
            "Activity" => Ok(DestinyStatsGroupType::Activity),
            "UniqueWeapon" => Ok(DestinyStatsGroupType::UniqueWeapon),
            "Internal" => Ok(DestinyStatsGroupType::Internal),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyStatsGroupType", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for DestinyStatsCategoryType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(DestinyStatsCategoryType::None),
            "Kills" => Ok(DestinyStatsCategoryType::Kills),
            "Assists" => Ok(DestinyStatsCategoryType::Assists),
            "Deaths" => Ok(DestinyStatsCategoryType::Deaths),
            "Criticals" => Ok(DestinyStatsCategoryType::Criticals),
            "KDa" => Ok(DestinyStatsCategoryType::KDa),
            "KD" => Ok(DestinyStatsCategoryType::KD),
            "Score" => Ok(DestinyStatsCategoryType::Score),
            "Entered" => Ok(DestinyStatsCategoryType::Entered),
            "TimePlayed" => Ok(DestinyStatsCategoryType::TimePlayed),
            "MedalWins" => Ok(DestinyStatsCategoryType::MedalWins),
            "MedalGame" => Ok(DestinyStatsCategoryType::MedalGame),
            "MedalSpecialKills" => Ok(DestinyStatsCategoryType::MedalSpecialKills),
            "MedalSprees" => Ok(DestinyStatsCategoryType::MedalSprees),
            "MedalMultiKills" => Ok(DestinyStatsCategoryType::MedalMultiKills),
            "MedalAbilities" => Ok(DestinyStatsCategoryType::MedalAbilities),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyStatsCategoryType", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for UnitType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(UnitType::None),
            "Count" => Ok(UnitType::Count),
            "PerGame" => Ok(UnitType::PerGame),
            "Seconds" => Ok(UnitType::Seconds),
            "Points" => Ok(UnitType::Points),
            "Team" => Ok(UnitType::Team),
            "Distance" => Ok(UnitType::Distance),
            "Percent" => Ok(UnitType::Percent),
            "Ratio" => Ok(UnitType::Ratio),
            "Boolean" => Ok(UnitType::Boolean),
            "WeaponType" => Ok(UnitType::WeaponType),
            "Standing" => Ok(UnitType::Standing),
            "Milliseconds" => Ok(UnitType::Milliseconds),
            "CompletionReason" => Ok(UnitType::CompletionReason),
            _ => Err(anyhow!("Could not deserialize string '{}' to UnitType", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinyStatsMergeMethod {
    /// When collapsing multiple instances of the stat together, add the values.
    Add = 0,
    /// When collapsing multiple instances of the stat together, take the lower value.
    Min = 1,
    /// When collapsing multiple instances of the stat together, take the higher value.
    Max = 2,
}

impl Display for DestinyStatsMergeMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for DestinyStatsMergeMethod {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Add" => Ok(DestinyStatsMergeMethod::Add),
            "Min" => Ok(DestinyStatsMergeMethod::Min),
            "Max" => Ok(DestinyStatsMergeMethod::Max),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinyStatsMergeMethod", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PeriodType {
    None = 0,
    Daily = 1,
    AllTime = 2,
    Activity = 3,
}

impl Display for PeriodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for PeriodType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(PeriodType::None),
            "Daily" => Ok(PeriodType::Daily),
            "AllTime" => Ok(PeriodType::AllTime),
            "Activity" => Ok(PeriodType::Activity),
            _ => Err(anyhow!("Could not deserialize string '{}' to PeriodType", s)),
        }
    }
}
