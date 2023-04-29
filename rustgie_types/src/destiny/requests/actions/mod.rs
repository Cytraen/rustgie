use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyActionRequest {
    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyCharacterActionRequest {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemActionRequest {
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

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyPostmasterTransferRequest {
    #[serde(rename = "itemReferenceHash")]
    pub item_reference_hash: u32,

    #[serde(rename = "stackSize")]
    pub stack_size: i32,

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

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemSetActionRequest {
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(rename = "itemIds")]
    pub item_ids: Option<Vec<i64>>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutActionRequest {
    /// The index of the loadout for this action request.
    #[serde(rename = "loadoutIndex")]
    pub loadout_index: i32,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutUpdateActionRequest {
    #[serde(rename = "colorHash")]
    pub color_hash: Option<u32>,

    #[serde(rename = "iconHash")]
    pub icon_hash: Option<u32>,

    #[serde(rename = "nameHash")]
    pub name_hash: Option<u32>,

    /// The index of the loadout for this action request.
    #[serde(rename = "loadoutIndex")]
    pub loadout_index: i32,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyItemStateRequest {
    #[serde(rename = "state")]
    pub state: bool,

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

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyInsertPlugsActionRequest {
    /// Action token provided by the AwaGetActionToken API call.
    #[serde(rename = "actionToken")]
    pub action_token: Option<String>,

    /// The instance ID of the item having a plug inserted. Only instanced items can have sockets.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: i64,

    /// The plugs being inserted.
    #[serde(rename = "plug")]
    pub plug: Option<crate::destiny::requests::actions::DestinyInsertPlugsRequestEntry>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

/// Represents all of the data related to a single plug to be inserted.
/// Note that, while you *can* point to a socket that represents infusion, you will receive an error if you attempt to do so. Come on guys, let's play nice.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyInsertPlugsRequestEntry {
    /// The index into the socket array, which identifies the specific socket being operated on. We also need to know the socketArrayType in order to uniquely identify the socket.
    /// Don't point to or try to insert a plug into an infusion socket. It won't work.
    #[serde(rename = "socketIndex")]
    pub socket_index: i32,

    /// This property, combined with the socketIndex, tells us which socket we are referring to (since operations can be performed on both Intrinsic and "default" sockets, and they occupy different arrays in the Inventory Item Definition). I know, I know. Don't give me that look.
    #[serde(rename = "socketArrayType")]
    pub socket_array_type: crate::destiny::requests::actions::DestinySocketArrayType,

    /// Plugs are never instanced (except in infusion). So with the hash alone, we should be able to: 1) Infer whether the player actually needs to have the item, or if it's a reusable plug 2) Perform any operation needed to use the Plug, including removing the plug item and running reward sheets.
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,
}

/// If you look in the DestinyInventoryItemDefinition's "sockets" property, you'll see that there are two types of sockets: intrinsic, and "socketEntry."
/// Unfortunately, because Intrinsic sockets are a whole separate array, it is no longer sufficient to know the index into that array to know which socket we're talking about. You have to know whether it's in the default "socketEntries" or if it's in the "intrinsic" list.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DestinySocketArrayType {
    Default = 0,
    Intrinsic = 1,
}

impl Display for DestinySocketArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for DestinySocketArrayType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Default" => Ok(DestinySocketArrayType::Default),
            "Intrinsic" => Ok(DestinySocketArrayType::Intrinsic),
            _ => Err(anyhow!("Could not deserialize string '{}' to DestinySocketArrayType", s)),
        }
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyInsertPlugsFreeActionRequest {
    /// The plugs being inserted.
    #[serde(rename = "plug")]
    pub plug: Option<crate::destiny::requests::actions::DestinyInsertPlugsRequestEntry>,

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
