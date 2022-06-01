use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Deserialize, Serialize)]
pub struct BungieFriendListResponse {
    #[serde(rename = "friends")]
    pub friends: Option<Vec<crate::social::friends::BungieFriend>>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct BungieFriend {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "lastSeenAsMembershipId")]
    pub last_seen_as_membership_id: i64,

    #[serde(rename = "lastSeenAsBungieMembershipType")]
    pub last_seen_as_bungie_membership_type: crate::BungieMembershipType,

    #[serde(rename = "bungieGlobalDisplayName")]
    pub bungie_global_display_name: Option<String>,

    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub bungie_global_display_name_code: Option<i16>,

    #[serde(rename = "onlineStatus")]
    pub online_status: crate::social::friends::PresenceStatus,

    #[serde(rename = "onlineTitle")]
    pub online_title: crate::social::friends::PresenceOnlineStateFlags,

    #[serde(rename = "relationship")]
    pub relationship: crate::social::friends::FriendRelationshipState,

    #[serde(rename = "bungieNetUser")]
    pub bungie_net_user: Option<crate::user::GeneralUser>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PresenceStatus {
    OfflineOrUnknown = 0,
    Online = 1,
}

impl Display for PresenceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PresenceOnlineStateFlags {
    None = 0,
    Destiny1 = 1,
    Destiny2 = 2,
}

impl Display for PresenceOnlineStateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FriendRelationshipState {
    Unknown = 0,
    Friend = 1,
    IncomingRequest = 2,
    OutgoingRequest = 3,
}

impl Display for FriendRelationshipState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[derive(Deserialize, Serialize)]
pub struct BungieFriendRequestListResponse {
    #[serde(rename = "incomingRequests")]
    pub incoming_requests: Option<Vec<crate::social::friends::BungieFriend>>,

    #[serde(rename = "outgoingRequests")]
    pub outgoing_requests: Option<Vec<crate::social::friends::BungieFriend>>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlatformFriendType {
    Unknown = 0,
    Xbox = 1,
    PSN = 2,
    Steam = 3,
}

impl Display for PlatformFriendType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlatformFriendResponse {
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i32,

    #[serde(rename = "currentPage")]
    pub current_page: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "platformFriends")]
    pub platform_friends: Option<Vec<crate::social::friends::PlatformFriend>>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct PlatformFriend {
    #[serde(rename = "platformDisplayName")]
    pub platform_display_name: Option<String>,

    #[serde(rename = "friendPlatform")]
    pub friend_platform: crate::social::friends::PlatformFriendType,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "destinyMembershipId")]
    pub destiny_membership_id: Option<i64>,

    #[serde(rename = "destinyMembershipType")]
    pub destiny_membership_type: Option<i32>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "bungieNetMembershipId")]
    pub bungie_net_membership_id: Option<i64>,

    #[serde(rename = "bungieGlobalDisplayName")]
    pub bungie_global_display_name: Option<String>,

    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub bungie_global_display_name_code: Option<i16>,
}
