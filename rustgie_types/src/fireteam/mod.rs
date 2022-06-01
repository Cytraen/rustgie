use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FireteamDateRange {
    All = 0,
    Now = 1,
    TwentyFourHours = 2,
    FortyEightHours = 3,
    ThisWeek = 4,
}

impl Display for FireteamDateRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u8)
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FireteamPlatform {
    Any = 0,
    Playstation4 = 1,
    XboxOne = 2,
    Blizzard = 3,
    Steam = 4,
    Stadia = 5,
}

impl Display for FireteamPlatform {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u8)
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FireteamPublicSearchOption {
    PublicAndPrivate = 0,
    PublicOnly = 1,
    PrivateOnly = 2,
}

impl Display for FireteamPublicSearchOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u8)
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FireteamSlotSearch {
    NoSlotRestriction = 0,
    HasOpenPlayerSlots = 1,
    HasOpenPlayerOrAltSlots = 2,
}

impl Display for FireteamSlotSearch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u8)
    }
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct FireteamSummary {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "fireteamId")]
    pub fireteam_id: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "platform")]
    pub platform: crate::fireteam::FireteamPlatform,

    #[serde(rename = "activityType")]
    pub activity_type: i32,

    #[serde(rename = "isImmediate")]
    pub is_immediate: bool,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "scheduledTime")]
    pub scheduled_time: Option<OffsetDateTime>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "ownerMembershipId")]
    pub owner_membership_id: i64,

    #[serde(rename = "playerSlotCount")]
    pub player_slot_count: i32,

    #[serde(rename = "alternateSlotCount")]
    pub alternate_slot_count: Option<i32>,

    #[serde(rename = "availablePlayerSlotCount")]
    pub available_player_slot_count: i32,

    #[serde(rename = "availableAlternateSlotCount")]
    pub available_alternate_slot_count: i32,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "dateCreated")]
    pub date_created: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "dateModified")]
    pub date_modified: Option<OffsetDateTime>,

    #[serde(rename = "isPublic")]
    pub is_public: bool,

    #[serde(rename = "locale")]
    pub locale: Option<String>,

    #[serde(rename = "isValid")]
    pub is_valid: bool,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "datePlayerModified")]
    pub date_player_modified: OffsetDateTime,

    #[serde(rename = "titleBeforeModeration")]
    pub title_before_moderation: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FireteamResponse {
    #[serde(rename = "Summary")]
    pub summary: Option<crate::fireteam::FireteamSummary>,

    #[serde(rename = "Members")]
    pub members: Option<Vec<crate::fireteam::FireteamMember>>,

    #[serde(rename = "Alternates")]
    pub alternates: Option<Vec<crate::fireteam::FireteamMember>>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct FireteamMember {
    #[serde(rename = "destinyUserInfo")]
    pub destiny_user_info: Option<crate::fireteam::FireteamUserInfoCard>,

    #[serde(rename = "bungieNetUserInfo")]
    pub bungie_net_user_info: Option<crate::user::UserInfoCard>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "dateJoined")]
    pub date_joined: OffsetDateTime,

    #[serde(rename = "hasMicrophone")]
    pub has_microphone: bool,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "lastPlatformInviteAttemptDate")]
    pub last_platform_invite_attempt_date: OffsetDateTime,

    #[serde(rename = "lastPlatformInviteAttemptResult")]
    pub last_platform_invite_attempt_result: crate::fireteam::FireteamPlatformInviteResult,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct FireteamUserInfoCard {
    #[serde(rename = "FireteamDisplayName")]
    pub fireteam_display_name: Option<String>,

    #[serde(rename = "FireteamMembershipType")]
    pub fireteam_membership_type: crate::BungieMembershipType,

    /// A platform specific additional display name - ex: psn Real Name, bnet Unique Name, etc.
    #[serde(rename = "supplementalDisplayName")]
    pub supplemental_display_name: Option<String>,

    /// URL the Icon if available.
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,

    /// If there is a cross save override in effect, this value will tell you the type that is overridding this one.
    #[serde(rename = "crossSaveOverride")]
    pub cross_save_override: crate::BungieMembershipType,

    /// The list of Membership Types indicating the platforms on which this Membership can be used.
    /// Not in Cross Save = its original membership type. Cross Save Primary = Any membership types it is overridding, and its original membership type Cross Save Overridden = Empty list
    #[serde(rename = "applicableMembershipTypes")]
    pub applicable_membership_types: Option<Vec<crate::BungieMembershipType>>,

    /// If True, this is a public user membership.
    #[serde(rename = "isPublic")]
    pub is_public: bool,

    /// Type of the membership. Not necessarily the native type.
    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,

    /// Membership ID as they user is known in the Accounts service
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "membershipId")]
    pub membership_id: i64,

    /// Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// The bungie global display name, if set.
    #[serde(rename = "bungieGlobalDisplayName")]
    pub bungie_global_display_name: Option<String>,

    /// The bungie global display name code, if set.
    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub bungie_global_display_name_code: Option<i16>,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FireteamPlatformInviteResult {
    None = 0,
    Success = 1,
    AlreadyInFireteam = 2,
    Throttled = 3,
    ServiceError = 4,
}

impl Display for FireteamPlatformInviteResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u8)
    }
}
