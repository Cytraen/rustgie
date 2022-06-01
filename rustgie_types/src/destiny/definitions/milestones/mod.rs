use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Milestones are an in-game concept where they're attempting to tell you what you can do next in-game.
/// If that sounds a lot like Advisors in Destiny 1, it is! So we threw out Advisors in the Destiny 2 API and tacked all of the data we would have put on Advisors onto Milestones instead.
/// Each Milestone represents something going on in the game right now:
/// - A "ritual activity" you can perform, like nightfall
/// - A "special event" that may have activities related to it, like Taco Tuesday (there's no Taco Tuesday in Destiny 2)
/// - A checklist you can fulfill, like helping your Clan complete all of its weekly objectives
/// - A tutorial quest you can play through, like the introduction to the Crucible.
/// Most of these milestones appear in game as well. Some of them are BNet only, because we're so extra. You're welcome.
/// There are some important caveats to understand about how we currently render Milestones and their deficiencies. The game currently doesn't have any content that actually tells you oughtright *what* the Milestone is: that is to say, what you'll be doing. The best we get is either a description of the overall Milestone, or of the Quest that the Milestone is having you partake in: which is usually something that assumes you already know what it's talking about, like "Complete 5 Challenges". 5 Challenges for what? What's a challenge? These are not questions that the Milestone data will answer for you unfortunately.
/// This isn't great, and in the future I'd like to add some custom text to give you more contextual information to pass on to your users. But for now, you can do what we do to render what little display info we do have:
/// Start by looking at the currently active quest (ideally, you've fetched DestinyMilestone or DestinyPublicMilestone data from the API, so you know the currently active quest for the Milestone in question). Look up the Quests property in the Milestone Definition, and check if it has display properties. If it does, show that as the description of the Milestone. If it doesn't, fall back on the Milestone's description.
/// This approach will let you avoid, whenever possible, the even less useful (and sometimes nonexistant) milestone-level names and descriptions.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// A hint to the UI to indicate what to show as the display properties for this Milestone when showing "Live" milestone data. Feel free to show more than this if desired: this hint is meant to simplify our own UI, but it may prove useful to you as well.
    #[serde(rename = "displayPreference")]
    pub display_preference: crate::destiny::definitions::milestones::DestinyMilestoneDisplayPreference,

    /// A custom image someone made just for the milestone. Isn't that special?
    #[serde(rename = "image")]
    pub image: Option<String>,

    /// An enumeration listing one of the possible types of milestones. Check out the DestinyMilestoneType enum for more info!
    #[serde(rename = "milestoneType")]
    pub milestone_type: crate::destiny::definitions::milestones::DestinyMilestoneType,

    /// If True, then the Milestone has been integrated with BNet's recruiting feature.
    #[serde(rename = "recruitable")]
    pub recruitable: bool,

    /// If the milestone has a friendly identifier for association with other features - such as Recruiting - that identifier can be found here. This is "friendly" in that it looks better in a URL than whatever the identifier for the Milestone actually is.
    #[serde(rename = "friendlyName")]
    pub friendly_name: Option<String>,

    /// If TRUE, this entry should be returned in the list of milestones for the "Explore Destiny" (i.e. new BNet homepage) features of Bungie.net (as long as the underlying event is active) Note that this is a property specifically used by BNet and the companion app for the "Live Events" feature of the front page/welcome view: it's not a reflection of what you see in-game.
    #[serde(rename = "showInExplorer")]
    pub show_in_explorer: bool,

    /// Determines whether we'll show this Milestone in the user's personal Milestones list.
    #[serde(rename = "showInMilestones")]
    pub show_in_milestones: bool,

    /// If TRUE, "Explore Destiny" (the front page of BNet and the companion app) prioritize using the activity image over any overriding Quest or Milestone image provided. This unfortunate hack is brought to you by Trials of The Nine.
    #[serde(rename = "explorePrioritizesActivityImage")]
    pub explore_prioritizes_activity_image: bool,

    /// A shortcut for clients - and the server - to understand whether we can predict the start and end dates for this event. In practice, there are multiple ways that an event could have predictable date ranges, but not all events will be able to be predicted via any mechanism (for instance, events that are manually triggered on and off)
    #[serde(rename = "hasPredictableDates")]
    pub has_predictable_dates: bool,

    /// The full set of possible Quests that give the overview of the Milestone event/activity in question. Only one of these can be active at a time for a given Conceptual Milestone, but many of them may be "available" for the user to choose from. (for instance, with Milestones you can choose from the three available Quests, but only one can be active at a time) Keyed by the quest item.
    /// As of Forsaken (~September 2018), Quest-style Milestones are being removed for many types of activities. There will likely be further revisions to the Milestone concept in the future.
    #[serde(rename = "quests")]
    pub quests: Option<HashMap<u32, crate::destiny::definitions::milestones::DestinyMilestoneQuestDefinition>>,

    /// If this milestone can provide rewards, this will define the categories into which the individual reward entries are placed.
    /// This is keyed by the Category's hash, which is only guaranteed to be unique within a given Milestone.
    #[serde(rename = "rewards")]
    pub rewards: Option<HashMap<u32, crate::destiny::definitions::milestones::DestinyMilestoneRewardCategoryDefinition>>,

    /// If you're going to show Vendors for the Milestone, you can use this as a localized "header" for the section where you show that vendor data. It'll provide a more context-relevant clue about what the vendor's role is in the Milestone.
    #[serde(rename = "vendorsDisplayTitle")]
    pub vendors_display_title: Option<String>,

    /// Sometimes, milestones will have rewards provided by Vendors. This definition gives the information needed to understand which vendors are relevant, the order in which they should be returned if order matters, and the conditions under which the Vendor is relevant to the user.
    #[serde(rename = "vendors")]
    pub vendors: Option<Vec<crate::destiny::definitions::milestones::DestinyMilestoneVendorDefinition>>,

    /// Sometimes, milestones will have arbitrary values associated with them that are of interest to us or to third party developers. This is the collection of those values' definitions, keyed by the identifier of the value and providing useful definition information such as localizable names and descriptions for the value.
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, crate::destiny::definitions::milestones::DestinyMilestoneValueDefinition>>,

    /// Some milestones are explicit objectives that you can see and interact with in the game. Some milestones are more conceptual, built by BNet to help advise you on activities and events that happen in-game but that aren't explicitly shown in game as Milestones. If this is TRUE, you can see this as a milestone in the game. If this is FALSE, it's an event or activity you can participate in, but you won't see it as a Milestone in the game's UI.
    #[serde(rename = "isInGameMilestone")]
    pub is_in_game_milestone: bool,

    /// A Milestone can now be represented by one or more activities directly (without a backing Quest), and that activity can have many challenges, modifiers, and related to it.
    #[serde(rename = "activities")]
    pub activities: Option<Vec<crate::destiny::definitions::milestones::DestinyMilestoneChallengeActivityDefinition>>,

    #[serde(rename = "defaultOrder")]
    pub default_order: i32,

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

/// A hint for the UI as to what display information ought to be shown. Defaults to showing the static MilestoneDefinition's display properties.
/// If for some reason the indicated property is not populated, fall back to the MilestoneDefinition.displayProperties.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyMilestoneDisplayPreference {
    /// Indicates you should show DestinyMilestoneDefinition.displayProperties for this Milestone.
    MilestoneDefinition = 0,
    /// Indicates you should show the displayProperties for any currently active Quest Steps in DestinyMilestone.availableQuests.
    CurrentQuestSteps = 1,
    /// Indicates you should show the displayProperties for any currently active Activities and their Challenges in DestinyMilestone.activities.
    CurrentActivityChallenges = 2,
}

impl Display for DestinyMilestoneDisplayPreference {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// The type of milestone. Milestones can be Tutorials, one-time/triggered/non-repeating but not necessarily tutorials, or Repeating Milestones.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DestinyMilestoneType {
    Unknown = 0,
    /// One-time milestones that are specifically oriented toward teaching players about new mechanics and gameplay modes.
    Tutorial = 1,
    /// Milestones that, once completed a single time, can never be repeated.
    OneTime = 2,
    /// Milestones that repeat/reset on a weekly basis. They need not all reset on the same day or time, but do need to reset weekly to qualify for this type.
    Weekly = 3,
    /// Milestones that repeat or reset on a daily basis.
    Daily = 4,
    /// Special indicates that the event is not on a daily/weekly cadence, but does occur more than once. For instance, Iron Banner in Destiny 1 or the Dawning were examples of what could be termed "Special" events.
    Special = 5,
}

impl Display for DestinyMilestoneType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

/// Any data we need to figure out whether this Quest Item is the currently active one for the conceptual Milestone. Even just typing this description, I already regret it.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneQuestDefinition {
    /// The item representing this Milestone quest. Use this hash to look up the DestinyInventoryItemDefinition for the quest to find its steps and human readable data.
    #[serde(rename = "questItemHash")]
    pub quest_item_hash: u32,

    /// The individual quests may have different definitions from the overall milestone: if there's a specific active quest, use these displayProperties instead of that of the overall DestinyMilestoneDefinition.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If populated, this image can be shown instead of the generic milestone's image when this quest is live, or it can be used to show a background image for the quest itself that differs from that of the Activity or the Milestone.
    #[serde(rename = "overrideImage")]
    pub override_image: Option<String>,

    /// The rewards you will get for completing this quest, as best as we could extract them from our data. Sometimes, it'll be a decent amount of data. Sometimes, it's going to be sucky. Sorry.
    #[serde(rename = "questRewards")]
    pub quest_rewards: Option<crate::destiny::definitions::milestones::DestinyMilestoneQuestRewardsDefinition>,

    /// The full set of all possible "conceptual activities" that are related to this Milestone. Tiers or alternative modes of play within these conceptual activities will be defined as sub-entities. Keyed by the Conceptual Activity Hash. Use the key to look up DestinyActivityDefinition.
    #[serde(rename = "activities")]
    pub activities: Option<HashMap<u32, crate::destiny::definitions::milestones::DestinyMilestoneActivityDefinition>>,

    /// Sometimes, a Milestone's quest is related to an entire Destination rather than a specific activity. In that situation, this will be the hash of that Destination. Hotspots are currently the only Milestones that expose this data, but that does not preclude this data from being returned for other Milestones in the future.
    #[serde(rename = "destinationHash")]
    pub destination_hash: Option<u32>,
}

/// If rewards are given in a quest - as opposed to overall in the entire Milestone - there's way less to track. We're going to simplify this contract as a result. However, this also gives us the opportunity to potentially put more than just item information into the reward data if we're able to mine it out in the future. Remember this if you come back and ask "why are quest reward items nested inside of their own class?"
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneQuestRewardsDefinition {
    /// The items that represent your reward for completing the quest.
    /// Be warned, these could be "dummy" items: items that are only used to render a good-looking in-game tooltip, but aren't the actual items themselves.
    /// For instance, when experience is given there's often a dummy item representing "experience", with quantity being the amount of experience you got. We don't have a programmatic association between those and whatever Progression is actually getting that experience... yet.
    #[serde(rename = "items")]
    pub items: Option<Vec<crate::destiny::definitions::milestones::DestinyMilestoneQuestRewardItem>>,
}

/// A subclass of DestinyItemQuantity, that provides not just the item and its quantity but also information that BNet can - at some point - use internally to provide more robust runtime information about the item's qualities.
/// If you want it, please ask! We're just out of time to wire it up right now. Or a clever person just may do it with our existing endpoints.
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneQuestRewardItem {
    /// The quest reward item *may* be associated with a vendor. If so, this is that vendor. Use this hash to look up the DestinyVendorDefinition.
    #[serde(rename = "vendorHash")]
    pub vendor_hash: Option<u32>,

    /// The quest reward item *may* be associated with a vendor. If so, this is the index of the item being sold, which we can use at runtime to find instanced item information for the reward item.
    #[serde(rename = "vendorItemIndex")]
    pub vendor_item_index: Option<i32>,

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

/// Milestones can have associated activities which provide additional information about the context, challenges, modifiers, state etc... related to this Milestone.
/// Information we need to be able to return that data is defined here, along with Tier data to establish a relationship between a conceptual Activity and its difficulty levels and variants.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneActivityDefinition {
    /// The "Conceptual" activity hash. Basically, we picked the lowest level activity and are treating it as the canonical definition of the activity for rendering purposes.
    /// If you care about the specific difficulty modes and variations, use the activities under "Variants".
    #[serde(rename = "conceptualActivityHash")]
    pub conceptual_activity_hash: u32,

    /// A milestone-referenced activity can have many variants, such as Tiers or alternative modes of play.
    /// Even if there is only a single variant, the details for these are represented within as a variant definition.
    /// It is assumed that, if this DestinyMilestoneActivityDefinition is active, then all variants should be active.
    /// If a Milestone could ever split the variants' active status conditionally, they should all have their own DestinyMilestoneActivityDefinition instead! The potential duplication will be worth it for the obviousness of processing and use.
    #[serde(rename = "variants")]
    pub variants: Option<HashMap<u32, crate::destiny::definitions::milestones::DestinyMilestoneActivityVariantDefinition>>,
}

/// Represents a variant on an activity for a Milestone: a specific difficulty tier, or a specific activity variant for example.
/// These will often have more specific details, such as an associated Guided Game, progression steps, tier-specific rewards, and custom values.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneActivityVariantDefinition {
    /// The hash to use for looking up the variant Activity's definition (DestinyActivityDefinition), where you can find its distinguishing characteristics such as difficulty level and recommended light level.
    /// Frequently, that will be the only distinguishing characteristics in practice, which is somewhat of a bummer.
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,

    /// If you care to do so, render the variants in the order prescribed by this value.
    /// When you combine live Milestone data with the definition, the order becomes more useful because you'll be cross-referencing between the definition and live data.
    #[serde(rename = "order")]
    pub order: i32,
}

/// The definition of a category of rewards, that contains many individual rewards.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneRewardCategoryDefinition {
    /// Identifies the reward category. Only guaranteed unique within this specific component!
    #[serde(rename = "categoryHash")]
    pub category_hash: u32,

    /// The string identifier for the category, if you want to use it for some end. Guaranteed unique within the specific component.
    #[serde(rename = "categoryIdentifier")]
    pub category_identifier: Option<String>,

    /// Hopefully this is obvious by now.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If this milestone can provide rewards, this will define the sets of rewards that can be earned, the conditions under which they can be acquired, internal data that we'll use at runtime to determine whether you've already earned or redeemed this set of rewards, and the category that this reward should be placed under.
    #[serde(rename = "rewardEntries")]
    pub reward_entries: Option<HashMap<u32, crate::destiny::definitions::milestones::DestinyMilestoneRewardEntryDefinition>>,

    /// If you want to use BNet's recommended order for rendering categories programmatically, use this value and compare it to other categories to determine the order in which they should be rendered. I don't feel great about putting this here, I won't lie.
    #[serde(rename = "order")]
    pub order: i32,
}

/// The definition of a specific reward, which may be contained in a category of rewards and that has optional information about how it is obtained.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneRewardEntryDefinition {
    /// The identifier for this reward entry. Runtime data will refer to reward entries by this hash. Only guaranteed unique within the specific Milestone.
    #[serde(rename = "rewardEntryHash")]
    pub reward_entry_hash: u32,

    /// The string identifier, if you care about it. Only guaranteed unique within the specific Milestone.
    #[serde(rename = "rewardEntryIdentifier")]
    pub reward_entry_identifier: Option<String>,

    /// The items you will get as rewards, and how much of it you'll get.
    #[serde(rename = "items")]
    pub items: Option<Vec<crate::destiny::DestinyItemQuantity>>,

    /// If this reward is redeemed at a Vendor, this is the hash of the Vendor to go to in order to redeem the reward. Use this hash to look up the DestinyVendorDefinition.
    #[serde(rename = "vendorHash")]
    pub vendor_hash: Option<u32>,

    /// For us to bother returning this info, we should be able to return some kind of information about why these rewards are grouped together. This is ideally that information. Look at how confident I am that this will always remain true.
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// If you want to follow BNet's ordering of these rewards, use this number within a given category to order the rewards. Yeah, I know. I feel dirty too.
    #[serde(rename = "order")]
    pub order: i32,
}

/// If the Milestone or a component has vendors whose inventories could/should be displayed that are relevant to it, this will return the vendor in question.
/// It also contains information we need to determine whether that vendor is actually relevant at the moment, given the user's current state.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneVendorDefinition {
    /// The hash of the vendor whose wares should be shown as associated with the Milestone.
    #[serde(rename = "vendorHash")]
    pub vendor_hash: u32,
}

/// The definition for information related to a key/value pair that is relevant for a particular Milestone or component within the Milestone.
/// This lets us more flexibly pass up information that's useful to someone, even if it's not necessarily us.
#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneValueDefinition {
    #[serde(rename = "key")]
    pub key: Option<String>,

    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneChallengeActivityDefinition {
    /// The activity for which this challenge is active.
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,

    #[serde(rename = "challenges")]
    pub challenges: Option<Vec<crate::destiny::definitions::milestones::DestinyMilestoneChallengeDefinition>>,

    /// If the activity and its challenge is visible on any of these nodes, it will be returned.
    #[serde(rename = "activityGraphNodes")]
    pub activity_graph_nodes: Option<Vec<crate::destiny::definitions::milestones::DestinyMilestoneChallengeActivityGraphNodeEntry>>,

    /// Phases related to this activity, if there are any.
    /// These will be listed in the order in which they will appear in the actual activity.
    #[serde(rename = "phases")]
    pub phases: Option<Vec<crate::destiny::definitions::milestones::DestinyMilestoneChallengeActivityPhase>>,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneChallengeDefinition {
    /// The challenge related to this milestone.
    #[serde(rename = "challengeObjectiveHash")]
    pub challenge_objective_hash: u32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneChallengeActivityGraphNodeEntry {
    #[serde(rename = "activityGraphHash")]
    pub activity_graph_hash: u32,

    #[serde(rename = "activityGraphNodeHash")]
    pub activity_graph_node_hash: u32,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyMilestoneChallengeActivityPhase {
    /// The hash identifier of the activity's phase.
    #[serde(rename = "phaseHash")]
    pub phase_hash: u32,
}
