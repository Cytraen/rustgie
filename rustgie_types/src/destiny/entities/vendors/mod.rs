﻿use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// This component contains essential/summary information about the vendor.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorComponent {
    /// If True, you can purchase from the Vendor.
    #[serde(rename = "canPurchase")]
    pub can_purchase: bool,

    /// If the Vendor has a related Reputation, this is the Progression data that represents the character's Reputation level with this Vendor.
    #[serde(rename = "progression")]
    pub progression: Option<crate::destiny::DestinyProgression>,

    /// An index into the vendor definition's "locations" property array, indicating which location they are at currently. If -1, then the vendor has no known location (and you may choose not to show them in your UI as a result. I mean, it's your bag honey)
    #[serde(rename = "vendorLocationIndex")]
    pub vendor_location_index: i32,

    /// If this vendor has a seasonal rank, this will be the calculated value of that rank. How nice is that? I mean, that's pretty sweeet. It's a whole 32 bit integer.
    #[serde(rename = "seasonalRank")]
    pub seasonal_rank: Option<i32>,

    /// The unique identifier for the vendor. Use it to look up their DestinyVendorDefinition.
    #[serde(rename = "vendorHash")]
    pub vendor_hash: u32,

    /// The date when this vendor's inventory will next rotate/refresh.
    /// Note that this is distinct from the date ranges that the vendor is visible/available in-game: this field indicates the specific time when the vendor's available items refresh and rotate, regardless of whether the vendor is actually available at that time. Unfortunately, these two values may be (and are, for the case of important vendors like Xur) different.
    /// Issue https://github.com/Bungie-net/api/issues/353 is tracking a fix to start providing visibility date ranges where possible in addition to this refresh date, so that all important dates for vendors are available for use.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "nextRefreshDate")]
    pub next_refresh_date: OffsetDateTime,

    /// If True, the Vendor is currently accessible.
    /// If False, they may not actually be visible in the world at the moment.
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

/// A vendor can have many categories of items that they sell. This component will return the category information for available items, as well as the index into those items in the user's sale item list.
/// Note that, since both the category and items are indexes, this data is Content Version dependent. Be sure to check that your content is up to date before using this data. This is an unfortunate, but permanent, limitation of Vendor data.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorCategoriesComponent {
    /// The list of categories for items that the vendor sells, in rendering order.
    /// These categories each point to a "display category" in the displayCategories property of the DestinyVendorDefinition, as opposed to the other categories.
    #[serde(rename = "categories")]
    pub categories: Option<Vec<crate::destiny::entities::vendors::DestinyVendorCategory>>,
}

/// Information about the category and items currently sold in that category.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorCategory {
    /// An index into the DestinyVendorDefinition.displayCategories property, so you can grab the display data for this category.
    #[serde(rename = "displayCategoryIndex")]
    pub display_category_index: i32,

    /// An ordered list of indexes into items being sold in this category (DestinyVendorDefinition.itemList) which will contain more information about the items being sold themselves. Can also be used to index into DestinyVendorSaleItemComponent data, if you asked for that data to be returned.
    #[serde(rename = "itemIndexes")]
    pub item_indexes: Option<Vec<i32>>,
}

/// Request this component if you want the details about an item being sold in relation to the character making the request: whether the character can buy it, whether they can afford it, and other data related to purchasing the item.
/// Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's "items" property.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorSaleItemComponent {
    /// A flag indicating whether the requesting character can buy the item, and if not the reasons why the character can't buy it.
    #[serde(rename = "saleStatus")]
    pub sale_status: enumflags2::BitFlags<crate::destiny::VendorItemStatus>,

    /// If you can't buy the item due to a complex character state, these will be hashes for DestinyUnlockDefinitions that you can check to see messages regarding the failure (if the unlocks have human readable information: it is not guaranteed that Unlocks will have human readable strings, and your application will have to handle that)
    /// Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes.
    #[serde(rename = "requiredUnlocks")]
    pub required_unlocks: Option<Vec<u32>>,

    /// If any complex unlock states are checked in determining purchasability, these will be returned here along with the status of the unlock check.
    /// Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes.
    #[serde(rename = "unlockStatuses")]
    pub unlock_statuses: Option<Vec<crate::destiny::DestinyUnlockStatus>>,

    /// Indexes in to the "failureStrings" lookup table in DestinyVendorDefinition for the given Vendor. Gives some more reliable failure information for why you can't purchase an item.
    /// It is preferred to use these over requiredUnlocks and unlockStatuses: the latter are provided mostly in case someone can do something interesting with it that I didn't anticipate.
    #[serde(rename = "failureIndexes")]
    pub failure_indexes: Option<Vec<i32>>,

    /// A flags enumeration value representing the current state of any "state modifiers" on the item being sold. These are meant to correspond with some sort of visual indicator as to the augmentation: for instance, if an item is on sale or if you already own the item in question.
    /// Determining how you want to represent these in your own app (or if you even want to) is an exercise left for the reader.
    #[serde(rename = "augments")]
    pub augments: enumflags2::BitFlags<crate::destiny::DestinyVendorItemState>,

    /// If available, a list that describes which item values (rewards) should be shown (true) or hidden (false).
    #[serde(rename = "itemValueVisibility")]
    pub item_value_visibility: Option<Vec<bool>>,

    /// The index into the DestinyVendorDefinition.itemList property. Note that this means Vendor data *is* Content Version dependent: make sure you have the latest content before you use Vendor data, or these indexes may mismatch.
    /// Most systems avoid this problem, but Vendors is one area where we are unable to reasonably avoid content dependency at the moment.
    #[serde(rename = "vendorItemIndex")]
    pub vendor_item_index: i32,

    /// The hash of the item being sold, as a quick shortcut for looking up the DestinyInventoryItemDefinition of the sale item.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// If populated, this is the hash of the item whose icon (and other secondary styles, but *not* the human readable strings) should override whatever icons/styles are on the item being sold.
    /// If you don't do this, certain items whose styles are being overridden by socketed items - such as the "Recycle Shader" item - would show whatever their default icon/style is, and it wouldn't be pretty or look accurate.
    #[serde(rename = "overrideStyleItemHash")]
    pub override_style_item_hash: Option<u32>,

    /// How much of the item you'll be getting.
    #[serde(rename = "quantity")]
    pub quantity: i32,

    /// A summary of the current costs of the item.
    #[serde(rename = "costs")]
    pub costs: Option<Vec<crate::destiny::DestinyItemQuantity>>,

    /// If this item has its own custom date where it may be removed from the Vendor's rotation, this is that date.
    /// Note that there's not actually any guarantee that it will go away: it could be chosen again and end up still being in the Vendor's sale items! But this is the next date where that test will occur, and is also the date that the game shows for availability on things like Bounties being sold. So it's the best we can give.
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "overrideNextRefreshDate")]
    pub override_next_refresh_date: Option<OffsetDateTime>,

    /// If true, this item can be purchased through the Bungie.net API.
    #[serde(rename = "apiPurchasable")]
    pub api_purchasable: Option<bool>,
}
