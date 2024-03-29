﻿use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use time::OffsetDateTime;

/// The set of progression-related information that applies at a Profile-wide level for your Destiny experience. This differs from the Jimi Hendrix Experience because there's less guitars on fire. Yet. #spoileralert?
/// This will include information such as Checklist info.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProfileProgressionComponent {
    /// The set of checklists that can be examined on a profile-wide basis, keyed by the hash identifier of the Checklist (DestinyChecklistDefinition)
    /// For each checklist returned, its value is itself a Dictionary keyed by the checklist's hash identifier with the value being a boolean indicating if it's been discovered yet.
    #[serde(rename = "checklists")]
    pub checklists: Option<HashMap<u32, HashMap<u32, bool>>>,

    /// Data related to your progress on the current season's artifact that is the same across characters.
    #[serde(rename = "seasonalArtifact")]
    pub seasonal_artifact: Option<crate::destiny::artifacts::DestinyArtifactProfileScoped>,
}

/// This is an experimental set of data that Bungie considers to be "transitory" - information that may be useful for API users, but that is coming from a non-authoritative data source about information that could potentially change at a more frequent pace than Bungie.net will receive updates about it.
/// This information is provided exclusively for convenience should any of it be useful to users: we provide no guarantees to the accuracy or timeliness of data that comes from this source. Know that this data can potentially be out-of-date or even wrong entirely if the user disconnected from the game or suddenly changed their status before we can receive refreshed data.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProfileTransitoryComponent {
    /// If you have any members currently in your party, this is some (very) bare-bones information about those members.
    #[serde(rename = "partyMembers")]
    pub party_members: Option<Vec<crate::destiny::components::profiles::DestinyProfileTransitoryPartyMember>>,

    /// If you are in an activity, this is some transitory info about the activity currently being played.
    #[serde(rename = "currentActivity")]
    pub current_activity: Option<crate::destiny::components::profiles::DestinyProfileTransitoryCurrentActivity>,

    /// Information about whether and what might prevent you from joining this person on a fireteam.
    #[serde(rename = "joinability")]
    pub joinability: Option<crate::destiny::components::profiles::DestinyProfileTransitoryJoinability>,

    /// Information about tracked entities.
    #[serde(rename = "tracking")]
    pub tracking: Option<Vec<crate::destiny::components::profiles::DestinyProfileTransitoryTrackingEntry>>,

    /// The hash identifier for the DestinyDestinationDefinition of the last location you were orbiting when in orbit.
    #[serde(rename = "lastOrbitedDestinationHash")]
    pub last_orbited_destination_hash: Option<u32>,
}

/// This is some bare minimum information about a party member in a Fireteam. Unfortunately, without great computational expense on our side we can only get at the data contained here. I'd like to give you a character ID for example, but we don't have it. But we do have these three pieces of information. May they help you on your quest to show meaningful data about current Fireteams.
/// Notably, we don't and can't feasibly return info on characters. If you can, try to use just the data below for your UI and purposes. Only hit us with further queries if you absolutely must know the character ID of the currently playing character. Pretty please with sugar on top.
#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProfileTransitoryPartyMember {
    /// The Membership ID that matches the party member.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "membershipId")]
    pub membership_id: i64,

    /// The identifier for the DestinyInventoryItemDefinition of the player's emblem.
    #[serde(rename = "emblemHash")]
    pub emblem_hash: u32,

    /// The player's last known display name.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// A Flags Enumeration value indicating the states that the player is in relevant to being on a fireteam.
    #[serde(rename = "status")]
    pub status: enumflags2::BitFlags<crate::destiny::DestinyPartyMemberStates>,
}

/// If you are playing in an activity, this is some information about it.
/// Note that we cannot guarantee any of this resembles what ends up in the PGCR in any way. They are sourced by two entirely separate systems with their own logic, and the one we source this data from should be considered non-authoritative in comparison.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProfileTransitoryCurrentActivity {
    /// When the activity started.
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "startTime")]
    pub start_time: Option<OffsetDateTime>,

    /// If you're still in it but it "ended" (like when folks are dancing around the loot after they beat a boss), this is when the activity ended.
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "endTime")]
    pub end_time: Option<OffsetDateTime>,

    /// This is what our non-authoritative source thought the score was.
    #[serde(rename = "score")]
    pub score: f32,

    /// If you have human opponents, this is the highest opposing team's score.
    #[serde(rename = "highestOpposingFactionScore")]
    pub highest_opposing_faction_score: f32,

    /// This is how many human or poorly crafted aimbot opponents you have.
    #[serde(rename = "numberOfOpponents")]
    pub number_of_opponents: i32,

    /// This is how many human or poorly crafted aimbots are on your team.
    #[serde(rename = "numberOfPlayers")]
    pub number_of_players: i32,
}

/// Some basic information about whether you can be joined, how many slots are left etc. Note that this can change quickly, so it may not actually be useful. But perhaps it will be in some use cases?
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProfileTransitoryJoinability {
    /// The number of slots still available on this person's fireteam.
    #[serde(rename = "openSlots")]
    pub open_slots: i32,

    /// Who the person is currently allowing invites from.
    #[serde(rename = "privacySetting")]
    pub privacy_setting: crate::destiny::DestinyGamePrivacySetting,

    /// Reasons why a person can't join this person's fireteam.
    #[serde(rename = "closedReasons")]
    pub closed_reasons: enumflags2::BitFlags<crate::destiny::DestinyJoinClosedReasons>,
}

/// This represents a single "thing" being tracked by the player.
/// This can point to many types of entities, but only a subset of them will actually have a valid hash identifier for whatever it is being pointed to.
/// It's up to you to interpret what it means when various combinations of these entries have values being tracked.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProfileTransitoryTrackingEntry {
    /// OPTIONAL - If this is tracking a DestinyLocationDefinition, this is the identifier for that location.
    #[serde(rename = "locationHash")]
    pub location_hash: Option<u32>,

    /// OPTIONAL - If this is tracking the status of a DestinyInventoryItemDefinition, this is the identifier for that item.
    #[serde(rename = "itemHash")]
    pub item_hash: Option<u32>,

    /// OPTIONAL - If this is tracking the status of a DestinyObjectiveDefinition, this is the identifier for that objective.
    #[serde(rename = "objectiveHash")]
    pub objective_hash: Option<u32>,

    /// OPTIONAL - If this is tracking the status of a DestinyActivityDefinition, this is the identifier for that activity.
    #[serde(rename = "activityHash")]
    pub activity_hash: Option<u32>,

    /// OPTIONAL - If this is tracking the status of a quest, this is the identifier for the DestinyInventoryItemDefinition that containst that questline data.
    #[serde(rename = "questlineItemHash")]
    pub questline_item_hash: Option<u32>,

    /// OPTIONAL - I've got to level with you, I don't really know what this is. Is it when you started tracking it? Is it only populated for tracked items that have time limits?
    /// I don't know, but we can get at it - when I get time to actually test what it is, I'll update this. In the meantime, bask in the mysterious data.
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "trackedDate")]
    pub tracked_date: Option<OffsetDateTime>,
}
