pub mod actions;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyItemTransferRequest {
    #[serde(rename = "itemReferenceHash")]
    pub item_reference_hash: u32,

    #[serde(rename = "stackSize")]
    pub stack_size: i32,

    #[serde(rename = "transferToVault")]
    pub transfer_to_vault: bool,

    /// The instance ID of the item for this action request.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}
