pub mod definitions;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize)]
pub struct DestinyPostGameCarnageReportData {
    /// Date and time for the activity.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "period")]
    pub period: OffsetDateTime,

    /// If this activity has "phases", this is the phase at which the activity was started. This value is only valid for activities before the Beyond Light expansion shipped. Subsequent activities will not have a valid value here.
    #[serde(rename = "startingPhaseIndex")]
    pub starting_phase_index: Option<i32>,

    /// True if the activity was started from the beginning, if that information is available and the activity was played post Witch Queen release.
    #[serde(rename = "activityWasStartedFromBeginning")]
    pub activity_was_started_from_beginning: Option<bool>,

    /// Details about the activity.
    #[serde(rename = "activityDetails")]
    pub activity_details: Option<crate::destiny::historical_stats::DestinyHistoricalStatsActivity>,

    /// Collection of players and their data for this activity.
    #[serde(rename = "entries")]
    pub entries: Option<Vec<crate::destiny::historical_stats::DestinyPostGameCarnageReportEntry>>,

    /// Collection of stats for the player in this activity.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<crate::destiny::historical_stats::DestinyPostGameCarnageReportTeamEntry>>,
}

/// Summary information about the activity that was played.
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsActivity {
    /// The unique hash identifier of the DestinyActivityDefinition that was played. If I had this to do over, it'd be named activityHash. Too late now.
    #[serde(rename = "referenceId")]
    pub reference_id: u32,

    /// The unique hash identifier of the DestinyActivityDefinition that was played.
    #[serde(rename = "directorActivityHash")]
    pub director_activity_hash: u32,

    /// The unique identifier for this *specific* match that was played.
    /// This value can be used to get additional data about this activity such as who else was playing via the GetPostGameCarnageReport endpoint.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "instanceId")]
    pub instance_id: i64,

    /// Indicates the most specific game mode of the activity that we could find.
    #[serde(rename = "mode")]
    pub mode: crate::destiny::historical_stats::definitions::DestinyActivityModeType,

    /// The list of all Activity Modes to which this activity applies, including aggregates. This will let you see, for example, whether the activity was both Clash and part of the Trials of the Nine event.
    #[serde(rename = "modes")]
    pub modes: Option<Vec<crate::destiny::historical_stats::definitions::DestinyActivityModeType>>,

    /// Whether or not the match was a private match.
    #[serde(rename = "isPrivate")]
    pub is_private: bool,

    /// The Membership Type indicating the platform on which this match was played.
    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyPostGameCarnageReportEntry {
    /// Standing of the player
    #[serde(rename = "standing")]
    pub standing: i32,

    /// Score of the player if available
    #[serde(rename = "score")]
    pub score: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValue>,

    /// Identity details of the player
    #[serde(rename = "player")]
    pub player: Option<crate::destiny::historical_stats::DestinyPlayer>,

    /// ID of the player's character used in the activity.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    /// Collection of stats for the player in this activity.
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,

    /// Extended data extracted from the activity blob.
    #[serde(rename = "extended")]
    pub extended: Option<crate::destiny::historical_stats::DestinyPostGameCarnageReportExtendedData>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsValue {
    /// Unique ID for this stat
    #[serde(rename = "statId")]
    pub stat_id: Option<String>,

    /// Basic stat value.
    #[serde(rename = "basic")]
    pub basic: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValuePair>,

    /// Per game average for the statistic, if applicable
    #[serde(rename = "pga")]
    pub pga: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValuePair>,

    /// Weighted value of the stat if a weight greater than 1 has been assigned.
    #[serde(rename = "weighted")]
    pub weighted: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValuePair>,

    /// When a stat represents the best, most, longest, fastest or some other personal best, the actual activity ID where that personal best was established is available on this property.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsValuePair {
    /// Raw value of the statistic
    #[serde(rename = "value")]
    pub value: f64,

    /// Localized formated version of the value.
    #[serde(rename = "displayValue")]
    pub display_value: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyPlayer {
    /// Details about the player as they are known in game (platform display name, Destiny emblem)
    #[serde(rename = "destinyUserInfo")]
    pub destiny_user_info: Option<crate::user::UserInfoCard>,

    /// Class of the character if applicable and available.
    #[serde(rename = "characterClass")]
    pub character_class: Option<String>,

    #[serde(rename = "classHash")]
    pub class_hash: u32,

    #[serde(rename = "raceHash")]
    pub race_hash: u32,

    #[serde(rename = "genderHash")]
    pub gender_hash: u32,

    /// Level of the character if available. Zero if it is not available.
    #[serde(rename = "characterLevel")]
    pub character_level: i32,

    /// Light Level of the character if available. Zero if it is not available.
    #[serde(rename = "lightLevel")]
    pub light_level: i32,

    /// Details about the player as they are known on BungieNet. This will be undefined if the player has marked their credential private, or does not have a BungieNet account.
    #[serde(rename = "bungieNetUserInfo")]
    pub bungie_net_user_info: Option<crate::user::UserInfoCard>,

    /// Current clan name for the player. This value may be null or an empty string if the user does not have a clan.
    #[serde(rename = "clanName")]
    pub clan_name: Option<String>,

    /// Current clan tag for the player. This value may be null or an empty string if the user does not have a clan.
    #[serde(rename = "clanTag")]
    pub clan_tag: Option<String>,

    /// If we know the emblem's hash, this can be used to look up the player's emblem at the time of a match when receiving PGCR data, or otherwise their currently equipped emblem (if we are able to obtain it).
    #[serde(rename = "emblemHash")]
    pub emblem_hash: u32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyPostGameCarnageReportExtendedData {
    /// List of weapons and their perspective values.
    #[serde(rename = "weapons")]
    pub weapons: Option<Vec<crate::destiny::historical_stats::DestinyHistoricalWeaponStats>>,

    /// Collection of stats for the player in this activity.
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalWeaponStats {
    /// The hash ID of the item definition that describes the weapon.
    #[serde(rename = "referenceId")]
    pub reference_id: u32,

    /// Collection of stats for the period.
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyPostGameCarnageReportTeamEntry {
    /// Integer ID for the team.
    #[serde(rename = "teamId")]
    pub team_id: i32,

    /// Team's standing relative to other teams.
    #[serde(rename = "standing")]
    pub standing: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValue>,

    /// Score earned by the team
    #[serde(rename = "score")]
    pub score: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValue>,

    /// Alpha or Bravo
    #[serde(rename = "teamName")]
    pub team_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyLeaderboard {
    #[serde(rename = "statId")]
    pub stat_id: Option<String>,

    #[serde(rename = "entries")]
    pub entries: Option<Vec<crate::destiny::historical_stats::DestinyLeaderboardEntry>>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyLeaderboardEntry {
    /// Where this player ranks on the leaderboard. A value of 1 is the top rank.
    #[serde(rename = "rank")]
    pub rank: i32,

    /// Identity details of the player
    #[serde(rename = "player")]
    pub player: Option<crate::destiny::historical_stats::DestinyPlayer>,

    /// ID of the player's best character for the reported stat.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    /// Value of the stat for this player
    #[serde(rename = "value")]
    pub value: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValue>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyLeaderboardResults {
    /// Indicate the membership ID of the account that is the focal point of the provided leaderboards.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "focusMembershipId")]
    pub focus_membership_id: Option<i64>,

    /// Indicate the character ID of the character that is the focal point of the provided leaderboards. May be null, in which case any character from the focus membership can appear in the provided leaderboards.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "focusCharacterId")]
    pub focus_character_id: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyClanAggregateStat {
    /// The id of the mode of stats (allPvp, allPvE, etc)
    #[serde(rename = "mode")]
    pub mode: crate::destiny::historical_stats::definitions::DestinyActivityModeType,

    /// The id of the stat
    #[serde(rename = "statId")]
    pub stat_id: Option<String>,

    /// Value of the stat for this player
    #[serde(rename = "value")]
    pub value: Option<crate::destiny::historical_stats::DestinyHistoricalStatsValue>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsByPeriod {
    #[serde(rename = "allTime")]
    pub all_time: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,

    #[serde(rename = "allTimeTier1")]
    pub all_time_tier1: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,

    #[serde(rename = "allTimeTier2")]
    pub all_time_tier2: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,

    #[serde(rename = "allTimeTier3")]
    pub all_time_tier3: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,

    #[serde(rename = "daily")]
    pub daily: Option<Vec<crate::destiny::historical_stats::DestinyHistoricalStatsPeriodGroup>>,

    #[serde(rename = "monthly")]
    pub monthly: Option<Vec<crate::destiny::historical_stats::DestinyHistoricalStatsPeriodGroup>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsPeriodGroup {
    /// Period for the group. If the stat periodType is day, then this will have a specific day. If the type is monthly, then this value will be the first day of the applicable month. This value is not set when the periodType is 'all time'.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "period")]
    pub period: OffsetDateTime,

    /// If the period group is for a specific activity, this property will be set.
    #[serde(rename = "activityDetails")]
    pub activity_details: Option<crate::destiny::historical_stats::DestinyHistoricalStatsActivity>,

    /// Collection of stats for the period.
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsResults {}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsAccountResult {
    #[serde(rename = "mergedDeletedCharacters")]
    pub merged_deleted_characters: Option<crate::destiny::historical_stats::DestinyHistoricalStatsWithMerged>,

    #[serde(rename = "mergedAllCharacters")]
    pub merged_all_characters: Option<crate::destiny::historical_stats::DestinyHistoricalStatsWithMerged>,

    #[serde(rename = "characters")]
    pub characters: Option<Vec<crate::destiny::historical_stats::DestinyHistoricalStatsPerCharacter>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsWithMerged {
    #[serde(rename = "results")]
    pub results: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsByPeriod>>,

    #[serde(rename = "merged")]
    pub merged: Option<crate::destiny::historical_stats::DestinyHistoricalStatsByPeriod>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalStatsPerCharacter {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "deleted")]
    pub deleted: bool,

    #[serde(rename = "results")]
    pub results: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsByPeriod>>,

    #[serde(rename = "merged")]
    pub merged: Option<crate::destiny::historical_stats::DestinyHistoricalStatsByPeriod>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyActivityHistoryResults {
    /// List of activities, the most recent activity first.
    #[serde(rename = "activities")]
    pub activities: Option<Vec<crate::destiny::historical_stats::DestinyHistoricalStatsPeriodGroup>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyHistoricalWeaponStatsData {
    /// List of weapons and their perspective values.
    #[serde(rename = "weapons")]
    pub weapons: Option<Vec<crate::destiny::historical_stats::DestinyHistoricalWeaponStats>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyAggregateActivityResults {
    /// List of all activities the player has participated in.
    #[serde(rename = "activities")]
    pub activities: Option<Vec<crate::destiny::historical_stats::DestinyAggregateActivityStats>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyAggregateActivityStats {
    /// Hash ID that can be looked up in the DestinyActivityTable.
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,

    /// Collection of stats for the player in this activity.
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, crate::destiny::historical_stats::DestinyHistoricalStatsValue>>,
}
