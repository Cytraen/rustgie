use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCollectiblesComponent {
    #[serde(rename = "collectibles")]
    pub collectibles: Option<HashMap<u32, crate::destiny::components::collectibles::DestinyCollectibleComponent>>,

    /// The hash for the root presentation node definition of Collection categories.
    #[serde(rename = "collectionCategoriesRootNodeHash")]
    pub collection_categories_root_node_hash: u32,

    /// The hash for the root presentation node definition of Collection Badges.
    #[serde(rename = "collectionBadgesRootNodeHash")]
    pub collection_badges_root_node_hash: u32,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCollectibleComponent {
    #[serde(rename = "state")]
    pub state: enumflags2::BitFlags<crate::destiny::DestinyCollectibleState>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyProfileCollectiblesComponent {
    /// The list of collectibles determined by the game as having been "recently" acquired.
    #[serde(rename = "recentCollectibleHashes")]
    pub recent_collectible_hashes: Option<Vec<u32>>,

    /// The list of collectibles determined by the game as having been "recently" acquired.
    /// The game client itself actually controls this data, so I personally question whether anyone will get much use out of this: because we can't edit this value through the API. But in case anyone finds it useful, here it is.
    #[serde(rename = "newnessFlaggedCollectibleHashes")]
    pub newness_flagged_collectible_hashes: Option<Vec<u32>>,

    #[serde(rename = "collectibles")]
    pub collectibles: Option<HashMap<u32, crate::destiny::components::collectibles::DestinyCollectibleComponent>>,

    /// The hash for the root presentation node definition of Collection categories.
    #[serde(rename = "collectionCategoriesRootNodeHash")]
    pub collection_categories_root_node_hash: u32,

    /// The hash for the root presentation node definition of Collection Badges.
    #[serde(rename = "collectionBadgesRootNodeHash")]
    pub collection_badges_root_node_hash: u32,
}
