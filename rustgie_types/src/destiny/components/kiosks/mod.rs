﻿use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Kiosk is a Vendor (DestinyVendorDefinition) that sells items based on whether you have already acquired that item before.
/// This component returns information about what Kiosk items are available to you on a *Profile* level. It is theoretically possible for Kiosks to have items gated by specific Character as well. If you ever have those, you will find them on the individual character's DestinyCharacterKiosksComponent.
/// Note that, because this component returns vendorItemIndexes (that is to say, indexes into the Kiosk Vendor's itemList property), these results are necessarily content version dependent. Make sure that you have the latest version of the content manifest databases before using this data.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyKiosksComponent {
    /// A dictionary keyed by the Kiosk Vendor's hash identifier (use it to look up the DestinyVendorDefinition for the relevant kiosk vendor), and whose value is a list of all the items that the user can "see" in the Kiosk, and any other interesting metadata.
    #[serde(rename = "kioskItems")]
    pub kiosk_items: Option<HashMap<u32, Vec<crate::destiny::components::kiosks::DestinyKioskItem>>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyKioskItem {
    /// The index of the item in the related DestinyVendorDefintion's itemList property, representing the sale.
    #[serde(rename = "index")]
    pub index: i32,

    /// If true, the user can not only see the item, but they can acquire it. It is possible that a user can see a kiosk item and not be able to acquire it.
    #[serde(rename = "canAcquire")]
    pub can_acquire: bool,

    /// Indexes into failureStrings for the Vendor, indicating the reasons why it failed if any.
    #[serde(rename = "failureIndexes")]
    pub failure_indexes: Option<Vec<i32>>,

    /// I may regret naming it this way - but this represents when an item has an objective that doesn't serve a beneficial purpose, but rather is used for "flavor" or additional information. For instance, when Emblems track specific stats, those stats are represented as Objectives on the item.
    #[serde(rename = "flavorObjective")]
    pub flavor_objective: Option<crate::destiny::quests::DestinyObjectiveProgress>,
}
