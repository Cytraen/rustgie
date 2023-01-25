use serde::{Deserialize, Serialize};

/// Mostly for historical purposes, we segregate Faction progressions from other progressions. This is just a DestinyProgression with a shortcut for finding the DestinyFactionDefinition of the faction related to the progression.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyFactionProgression {
    /// The hash identifier of the Faction related to this progression. Use it to look up the DestinyFactionDefinition for more rendering info.
    #[serde(rename = "factionHash")]
    pub faction_hash: u32,

    /// The index of the Faction vendor that is currently available. Will be set to -1 if no vendors are available.
    #[serde(rename = "factionVendorIndex")]
    pub faction_vendor_index: i32,

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
