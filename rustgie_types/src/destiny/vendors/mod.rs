use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

/// If a character purchased an item that is refundable, a Vendor Receipt will be created on the user's Destiny Profile. These expire after a configurable period of time, but until then can be used to get refunds on items. BNet does not provide the ability to refund a purchase *yet*, but you know.
#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyVendorReceipt {
    /// The amount paid for the item, in terms of items that were consumed in the purchase and their quantity.
    #[serde(rename = "currencyPaid")]
    pub currency_paid: Option<Vec<crate::destiny::DestinyItemQuantity>>,

    /// The item that was received, and its quantity.
    #[serde(rename = "itemReceived")]
    pub item_received: Option<crate::destiny::DestinyItemQuantity>,

    /// The unlock flag used to determine whether you still have the purchased item.
    #[serde(rename = "licenseUnlockHash")]
    pub license_unlock_hash: u32,

    /// The ID of the character who made the purchase.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "purchasedByCharacterId")]
    pub purchased_by_character_id: i64,

    /// Whether you can get a refund, and what happens in order for the refund to be received. See the DestinyVendorItemRefundPolicy enum for details.
    #[serde(rename = "refundPolicy")]
    pub refund_policy: crate::destiny::DestinyVendorItemRefundPolicy,

    /// The identifier of this receipt.
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: i32,

    /// The seconds since epoch at which this receipt is rendered invalid.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "timeToExpiration")]
    pub time_to_expiration: i64,

    /// The date at which this receipt is rendered invalid.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "expiresOn")]
    pub expires_on: OffsetDateTime,
}
