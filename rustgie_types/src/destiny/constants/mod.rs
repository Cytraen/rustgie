use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyEnvironmentLocationMapping {
    /// The location that is revealed on the director by this mapping.
    #[serde(rename = "locationHash")]
    pub location_hash: u32,

    /// A hint that the UI uses to figure out how this location is activated by the player.
    #[serde(rename = "activationSource")]
    pub activation_source: Option<String>,

    /// If this is populated, it is the item that you must possess for this location to be active because of this mapping. (theoretically, a location can have multiple mappings, and some might require an item while others don't)
    #[serde(rename = "itemHash")]
    pub item_hash: Option<u32>,

    /// If this is populated, this is an objective related to the location.
    #[serde(rename = "objectiveHash")]
    pub objective_hash: Option<u32>,

    /// If this is populated, this is the activity you have to be playing in order to see this location appear because of this mapping. (theoretically, a location can have multiple mappings, and some might require you to be in a specific activity when others don't)
    #[serde(rename = "activityHash")]
    pub activity_hash: Option<u32>,
}
