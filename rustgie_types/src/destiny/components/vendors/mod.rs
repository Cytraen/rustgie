use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// This component returns references to all of the Vendors in the response, grouped by categorizations that Bungie has deemed to be interesting, in the order in which both the groups and the vendors within that group should be rendered.
#[derive(Deserialize, Serialize)]
pub struct DestinyVendorGroupComponent {
    /// The ordered list of groups being returned.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::destiny::components::vendors::DestinyVendorGroup>>,
}

/// Represents a specific group of vendors that can be rendered in the recommended order.
/// How do we figure out this order? It's a long story, and will likely get more complicated over time.
#[derive(Deserialize, Serialize)]
pub struct DestinyVendorGroup {
    #[serde(rename = "vendorGroupHash")]
    pub vendor_group_hash: u32,

    /// The ordered list of vendors within a particular group.
    #[serde(rename = "vendorHashes")]
    pub vendor_hashes: Option<Vec<u32>>,
}

/// This component contains essential/summary information about the vendor.
#[derive(Deserialize, Serialize)]
pub struct DestinyVendorBaseComponent {
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

/// The base class for Vendor Sale Item data. Has a bunch of character-agnostic state about the item being sold.
/// Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's "items" property.
#[derive(Deserialize, Serialize)]
pub struct DestinyVendorSaleItemBaseComponent {
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

/// This component contains essential/summary information about the vendor from the perspective of a character-agnostic view.
#[derive(Deserialize, Serialize)]
pub struct DestinyPublicVendorComponent {
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

/// Has character-agnostic information about an item being sold by a vendor.
/// Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's "items" property. For most of these, however, you'll have to ask for it in context of a specific character.
#[derive(Deserialize, Serialize)]
pub struct DestinyPublicVendorSaleItemComponent {
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
