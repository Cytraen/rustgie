use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use time::OffsetDateTime;

/// This component contains base properties of the character. You'll probably want to always request this component, but hey you do you.
#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCharacterComponent {
    /// Every Destiny Profile has a membershipId. This is provided on the character as well for convenience.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "membershipId")]
    pub membership_id: i64,

    /// membershipType tells you the platform on which the character plays. Examine the BungieMembershipType enumeration for possible values.
    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,

    /// The unique identifier for the character.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    /// The last date that the user played Destiny.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "dateLastPlayed")]
    pub date_last_played: OffsetDateTime,

    /// If the user is currently playing, this is how long they've been playing.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "minutesPlayedThisSession")]
    pub minutes_played_this_session: i64,

    /// If this value is 525,600, then they played Destiny for a year. Or they're a very dedicated Rent fan. Note that this includes idle time, not just time spent actually in activities shooting things.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "minutesPlayedTotal")]
    pub minutes_played_total: i64,

    /// The user's calculated "Light Level". Light level is an indicator of your power that mostly matters in the end game, once you've reached the maximum character level: it's a level that's dependent on the average Attack/Defense power of your items.
    #[serde(rename = "light")]
    pub light: i32,

    /// Your character's stats, such as Agility, Resilience, etc... *not* historical stats.
    /// You'll have to call a different endpoint for those.
    #[serde(rename = "stats")]
    pub stats: Option<HashMap<u32, i32>>,

    /// Use this hash to look up the character's DestinyRaceDefinition.
    #[serde(rename = "raceHash")]
    pub race_hash: u32,

    /// Use this hash to look up the character's DestinyGenderDefinition.
    #[serde(rename = "genderHash")]
    pub gender_hash: u32,

    /// Use this hash to look up the character's DestinyClassDefinition.
    #[serde(rename = "classHash")]
    pub class_hash: u32,

    /// Mostly for historical purposes at this point, this is an enumeration for the character's race.
    /// It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove.
    #[serde(rename = "raceType")]
    pub race_type: crate::destiny::DestinyRace,

    /// Mostly for historical purposes at this point, this is an enumeration for the character's class.
    /// It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove.
    #[serde(rename = "classType")]
    pub class_type: crate::destiny::DestinyClass,

    /// Mostly for historical purposes at this point, this is an enumeration for the character's Gender.
    /// It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove. And yeah, it's an enumeration and not a boolean. Fight me.
    #[serde(rename = "genderType")]
    pub gender_type: crate::destiny::DestinyGender,

    /// A shortcut path to the user's currently equipped emblem image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition.
    #[serde(rename = "emblemPath")]
    pub emblem_path: Option<String>,

    /// A shortcut path to the user's currently equipped emblem background image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition.
    #[serde(rename = "emblemBackgroundPath")]
    pub emblem_background_path: Option<String>,

    /// The hash of the currently equipped emblem for the user. Can be used to look up the DestinyInventoryItemDefinition.
    #[serde(rename = "emblemHash")]
    pub emblem_hash: u32,

    /// A shortcut for getting the background color of the user's currently equipped emblem without having to do a DestinyInventoryItemDefinition lookup.
    #[serde(rename = "emblemColor")]
    pub emblem_color: Option<crate::destiny::misc::DestinyColor>,

    /// The progression that indicates your character's level. Not their light level, but their character level: you know, the thing you max out a couple hours in and then ignore for the sake of light level.
    #[serde(rename = "levelProgression")]
    pub level_progression: Option<crate::destiny::DestinyProgression>,

    /// The "base" level of your character, not accounting for any light level.
    #[serde(rename = "baseCharacterLevel")]
    pub base_character_level: i32,

    /// A number between 0 and 100, indicating the whole and fractional % remaining to get to the next character level.
    #[serde(rename = "percentToNextLevel")]
    pub percent_to_next_level: f32,

    /// If this Character has a title assigned to it, this is the identifier of the DestinyRecordDefinition that has that title information.
    #[serde(rename = "titleRecordHash")]
    pub title_record_hash: Option<u32>,
}

/// This component returns anything that could be considered "Progression" on a user: data where the user is gaining levels, reputation, completions, rewards, etc...
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCharacterProgressionComponent {
    /// A Dictionary of all known progressions for the Character, keyed by the Progression's hash.
    /// Not all progressions have user-facing data, but those who do will have that data contained in the DestinyProgressionDefinition.
    #[serde(rename = "progressions")]
    pub progressions: Option<HashMap<u32, crate::destiny::DestinyProgression>>,

    /// A dictionary of all known Factions, keyed by the Faction's hash. It contains data about this character's status with the faction.
    #[serde(rename = "factions")]
    pub factions: Option<HashMap<u32, crate::destiny::progression::DestinyFactionProgression>>,

    /// Milestones are related to the simple progressions shown in the game, but return additional and hopefully helpful information for users about the specifics of the Milestone's status.
    #[serde(rename = "milestones")]
    pub milestones: Option<HashMap<u32, crate::destiny::milestones::DestinyMilestone>>,

    /// If the user has any active quests, the quests' statuses will be returned here.
    /// Note that quests have been largely supplanted by Milestones, but that doesn't mean that they won't make a comeback independent of milestones at some point.
    /// (Fun fact: quests came back as I feared they would, but we never looped back to populate this... I'm going to put that in the backlog.)
    #[serde(rename = "quests")]
    pub quests: Option<Vec<crate::destiny::quests::DestinyQuestStatus>>,

    /// Sometimes, you have items in your inventory that don't have instances, but still have Objective information. This provides you that objective information for uninstanced items.
    /// This dictionary is keyed by the item's hash: which you can use to look up the name and description for the overall task(s) implied by the objective. The value is the list of objectives for this item, and their statuses.
    #[serde(rename = "uninstancedItemObjectives")]
    pub uninstanced_item_objectives: Option<HashMap<u32, Vec<crate::destiny::quests::DestinyObjectiveProgress>>>,

    /// Sometimes, you have items in your inventory that don't have instances, but still have perks (for example: Trials passage cards). This gives you the perk information for uninstanced items.
    /// This dictionary is keyed by item hash, which you can use to look up the corresponding item definition. The value is the list of perks states for the item.
    #[serde(rename = "uninstancedItemPerks")]
    pub uninstanced_item_perks: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemPerksComponent>>,

    /// The set of checklists that can be examined for this specific character, keyed by the hash identifier of the Checklist (DestinyChecklistDefinition)
    /// For each checklist returned, its value is itself a Dictionary keyed by the checklist's hash identifier with the value being a boolean indicating if it's been discovered yet.
    #[serde(rename = "checklists")]
    pub checklists: Option<HashMap<u32, HashMap<u32, bool>>>,

    /// Data related to your progress on the current season's artifact that can vary per character.
    #[serde(rename = "seasonalArtifact")]
    pub seasonal_artifact: Option<crate::destiny::artifacts::DestinyArtifactCharacterScoped>,
}

/// Only really useful if you're attempting to render the character's current appearance in 3D, this returns a bare minimum of information, pre-aggregated, that you'll need to perform that rendering. Note that you need to combine this with other 3D assets and data from our servers.
/// Examine the Javascript returned by https://bungie.net/sharedbundle/spasm to see how we use this data, but be warned: the rabbit hole goes pretty deep.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCharacterRenderComponent {
    /// Custom dyes, calculated by iterating over the character's equipped items. Useful for pre-fetching all of the dye data needed from our server.
    #[serde(rename = "customDyes")]
    pub custom_dyes: Option<Vec<crate::destiny::DyeReference>>,

    /// This is actually something that Spasm.js *doesn't* do right now, and that we don't return assets for yet. This is the data about what character customization options you picked. You can combine this with DestinyCharacterCustomizationOptionDefinition to show some cool info, and hopefully someday to actually render a user's face in 3D. We'll see if we ever end up with time for that.
    #[serde(rename = "customization")]
    pub customization: Option<crate::destiny::character::DestinyCharacterCustomization>,

    /// A minimal view of:
    /// - Equipped items
    /// - The rendering-related custom options on those equipped items
    /// Combined, that should be enough to render all of the items on the equipped character.
    #[serde(rename = "peerView")]
    pub peer_view: Option<crate::destiny::character::DestinyCharacterPeerView>,
}

/// This component holds activity data for a character. It will tell you about the character's current activity status, as well as activities that are available to the user.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCharacterActivitiesComponent {
    /// The last date that the user started playing an activity.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "dateActivityStarted")]
    pub date_activity_started: OffsetDateTime,

    /// The list of activities that the user can play.
    #[serde(rename = "availableActivities")]
    pub available_activities: Option<Vec<crate::destiny::DestinyActivity>>,

    /// If the user is in an activity, this will be the hash of the Activity being played. Note that you must combine this info with currentActivityModeHash to get a real picture of what the user is doing right now. For instance, PVP "Activities" are just maps: it's the ActivityMode that determines what type of PVP game they're playing.
    #[serde(rename = "currentActivityHash")]
    pub current_activity_hash: u32,

    /// If the user is in an activity, this will be the hash of the activity mode being played. Combine with currentActivityHash to give a person a full picture of what they're doing right now.
    #[serde(rename = "currentActivityModeHash")]
    pub current_activity_mode_hash: u32,

    /// And the current activity's most specific mode type, if it can be found.
    #[serde(rename = "currentActivityModeType")]
    pub current_activity_mode_type: Option<i32>,

    /// If the user is in an activity, this will be the hashes of the DestinyActivityModeDefinition being played. Combine with currentActivityHash to give a person a full picture of what they're doing right now.
    #[serde(rename = "currentActivityModeHashes")]
    pub current_activity_mode_hashes: Option<Vec<u32>>,

    /// All Activity Modes that apply to the current activity being played, in enum form.
    #[serde(rename = "currentActivityModeTypes")]
    pub current_activity_mode_types: Option<Vec<crate::destiny::historical_stats::definitions::DestinyActivityModeType>>,

    /// If the user is in a playlist, this is the hash identifier for the playlist that they chose.
    #[serde(rename = "currentPlaylistActivityHash")]
    pub current_playlist_activity_hash: Option<u32>,

    /// This will have the activity hash of the last completed story/campaign mission, in case you care about that.
    #[serde(rename = "lastCompletedStoryHash")]
    pub last_completed_story_hash: u32,
}
