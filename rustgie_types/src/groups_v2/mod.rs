use anyhow::{anyhow, Result};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupUserInfoCard {
    /// This will be the display name the clan server last saw the user as. If the account is an active cross save override, this will be the display name to use. Otherwise, this will match the displayName property.
    #[serde(rename = "LastSeenDisplayName")]
    pub last_seen_display_name: Option<String>,

    /// The platform of the LastSeenDisplayName
    #[serde(rename = "LastSeenDisplayNameType")]
    pub last_seen_display_name_type: crate::BungieMembershipType,

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

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupResponse {
    #[serde(rename = "detail")]
    pub detail: Option<crate::groups_v2::GroupV2>,

    #[serde(rename = "founder")]
    pub founder: Option<crate::groups_v2::GroupMember>,

    #[serde(rename = "alliedIds")]
    pub allied_ids: Option<Vec<i64>>,

    #[serde(rename = "parentGroup")]
    pub parent_group: Option<crate::groups_v2::GroupV2>,

    #[serde(rename = "allianceStatus")]
    pub alliance_status: crate::groups_v2::GroupAllianceStatus,

    #[serde(rename = "groupJoinInviteCount")]
    pub group_join_invite_count: i32,

    /// A convenience property that indicates if every membership you (the current user) have that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save.
    #[serde(rename = "currentUserMembershipsInactiveForDestiny")]
    pub current_user_memberships_inactive_for_destiny: bool,

    /// This property will be populated if the authenticated user is a member of the group. Note that because of account linking, a user can sometimes be part of a clan more than once. As such, this returns the highest member type available.
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "currentUserMemberMap")]
    pub current_user_member_map: Option<HashMap<crate::BungieMembershipType, crate::groups_v2::GroupMember>>,

    /// This property will be populated if the authenticated user is an applicant or has an outstanding invitation to join. Note that because of account linking, a user can sometimes be part of a clan more than once.
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "currentUserPotentialMemberMap")]
    pub current_user_potential_member_map: Option<HashMap<crate::BungieMembershipType, crate::groups_v2::GroupPotentialMember>>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupV2 {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "groupType")]
    pub group_type: crate::groups_v2::GroupType,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "membershipIdCreated")]
    pub membership_id_created: i64,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "creationDate")]
    pub creation_date: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "modificationDate")]
    pub modification_date: OffsetDateTime,

    #[serde(rename = "about")]
    pub about: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "memberCount")]
    pub member_count: i32,

    #[serde(rename = "isPublic")]
    pub is_public: bool,

    #[serde(rename = "isPublicTopicAdminOnly")]
    pub is_public_topic_admin_only: bool,

    #[serde(rename = "motto")]
    pub motto: Option<String>,

    #[serde(rename = "allowChat")]
    pub allow_chat: bool,

    #[serde(rename = "isDefaultPostPublic")]
    pub is_default_post_public: bool,

    #[serde(rename = "chatSecurity")]
    pub chat_security: crate::groups_v2::ChatSecuritySetting,

    #[serde(rename = "locale")]
    pub locale: Option<String>,

    #[serde(rename = "avatarImageIndex")]
    pub avatar_image_index: i32,

    #[serde(rename = "homepage")]
    pub homepage: crate::groups_v2::GroupHomepage,

    #[serde(rename = "membershipOption")]
    pub membership_option: crate::groups_v2::MembershipOption,

    #[serde(rename = "defaultPublicity")]
    pub default_publicity: crate::groups_v2::GroupPostPublicity,

    #[serde(rename = "theme")]
    pub theme: Option<String>,

    #[serde(rename = "bannerPath")]
    pub banner_path: Option<String>,

    #[serde(rename = "avatarPath")]
    pub avatar_path: Option<String>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "conversationId")]
    pub conversation_id: i64,

    #[serde(rename = "enableInvitationMessagingForAdmins")]
    pub enable_invitation_messaging_for_admins: bool,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "banExpireDate")]
    pub ban_expire_date: Option<OffsetDateTime>,

    #[serde(rename = "features")]
    pub features: Option<crate::groups_v2::GroupFeatures>,

    #[serde(rename = "clanInfo")]
    pub clan_info: Option<crate::groups_v2::GroupV2ClanInfoAndInvestment>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupType {
    General = 0,
    Clan = 1,
}

impl Display for GroupType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "General" => Ok(GroupType::General),
            "Clan" => Ok(GroupType::Clan),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupType", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChatSecuritySetting {
    Group = 0,
    Admins = 1,
}

impl Display for ChatSecuritySetting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ChatSecuritySetting {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Group" => Ok(ChatSecuritySetting::Group),
            "Admins" => Ok(ChatSecuritySetting::Admins),
            _ => Err(anyhow!("Could not deserialize string '{}' to ChatSecuritySetting", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupHomepage {
    Wall = 0,
    Forum = 1,
    AllianceForum = 2,
}

impl Display for GroupHomepage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupHomepage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Wall" => Ok(GroupHomepage::Wall),
            "Forum" => Ok(GroupHomepage::Forum),
            "AllianceForum" => Ok(GroupHomepage::AllianceForum),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupHomepage", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MembershipOption {
    Reviewed = 0,
    Open = 1,
    Closed = 2,
}

impl Display for MembershipOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for MembershipOption {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Reviewed" => Ok(MembershipOption::Reviewed),
            "Open" => Ok(MembershipOption::Open),
            "Closed" => Ok(MembershipOption::Closed),
            _ => Err(anyhow!("Could not deserialize string '{}' to MembershipOption", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupPostPublicity {
    Public = 0,
    Alliance = 1,
    Private = 2,
}

impl Display for GroupPostPublicity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupPostPublicity {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Public" => Ok(GroupPostPublicity::Public),
            "Alliance" => Ok(GroupPostPublicity::Alliance),
            "Private" => Ok(GroupPostPublicity::Private),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupPostPublicity", s)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GroupFeatures {
    #[serde(rename = "maximumMembers")]
    pub maximum_members: i32,

    /// Maximum number of groups of this type a typical membership may join. For example, a user may join about 50 General groups with their Bungie.net account. They may join one clan per Destiny membership.
    #[serde(rename = "maximumMembershipsOfGroupType")]
    pub maximum_memberships_of_group_type: i32,

    #[serde(rename = "capabilities")]
    pub capabilities: enumflags2::BitFlags<crate::groups_v2::Capabilities>,

    #[serde(rename = "membershipTypes")]
    pub membership_types: Option<Vec<crate::BungieMembershipType>>,

    /// Minimum Member Level allowed to invite new members to group
    /// Always Allowed: Founder, Acting Founder
    /// True means admins have this power, false means they don't
    /// Default is false for clans, true for groups.
    #[serde(rename = "invitePermissionOverride")]
    pub invite_permission_override: bool,

    /// Minimum Member Level allowed to update group culture
    /// Always Allowed: Founder, Acting Founder
    /// True means admins have this power, false means they don't
    /// Default is false for clans, true for groups.
    #[serde(rename = "updateCulturePermissionOverride")]
    pub update_culture_permission_override: bool,

    /// Minimum Member Level allowed to host guided games
    /// Always Allowed: Founder, Acting Founder, Admin
    /// Allowed Overrides: None, Member, Beginner
    /// Default is Member for clans, None for groups, although this means nothing for groups.
    #[serde(rename = "hostGuidedGamePermissionOverride")]
    pub host_guided_game_permission_override: crate::groups_v2::HostGuidedGamesPermissionLevel,

    /// Minimum Member Level allowed to update banner
    /// Always Allowed: Founder, Acting Founder
    /// True means admins have this power, false means they don't
    /// Default is false for clans, true for groups.
    #[serde(rename = "updateBannerPermissionOverride")]
    pub update_banner_permission_override: bool,

    /// Level to join a member at when accepting an invite, application, or joining an open clan
    /// Default is Beginner.
    #[serde(rename = "joinLevel")]
    pub join_level: crate::groups_v2::RuntimeGroupMemberType,
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Capabilities {
    Leaderboards = 1,
    Callsign = 2,
    OptionalConversations = 4,
    ClanBanner = 8,
    D2InvestmentData = 16,
    Tags = 32,
    Alliances = 64,
}

impl Display for Capabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for Capabilities {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Leaderboards" => Ok(Capabilities::Leaderboards),
            "Callsign" => Ok(Capabilities::Callsign),
            "OptionalConversations" => Ok(Capabilities::OptionalConversations),
            "ClanBanner" => Ok(Capabilities::ClanBanner),
            "D2InvestmentData" => Ok(Capabilities::D2InvestmentData),
            "Tags" => Ok(Capabilities::Tags),
            "Alliances" => Ok(Capabilities::Alliances),
            _ => Err(anyhow!("Could not deserialize string '{}' to Capabilities", s)),
        }
    }
}

/// Used for setting the guided game permission level override (admins and founders can always host guided games).
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HostGuidedGamesPermissionLevel {
    None = 0,
    Beginner = 1,
    Member = 2,
}

impl Display for HostGuidedGamesPermissionLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for HostGuidedGamesPermissionLevel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(HostGuidedGamesPermissionLevel::None),
            "Beginner" => Ok(HostGuidedGamesPermissionLevel::Beginner),
            "Member" => Ok(HostGuidedGamesPermissionLevel::Member),
            _ => Err(anyhow!("Could not deserialize string '{}' to HostGuidedGamesPermissionLevel", s)),
        }
    }
}

/// The member levels used by all V2 Groups API. Individual group types use their own mappings in their native storage (general uses BnetDbGroupMemberType and D2 clans use ClanMemberLevel), but they are all translated to this in the runtime api. These runtime values should NEVER be stored anywhere, so the values can be changed as necessary.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RuntimeGroupMemberType {
    None = 0,
    Beginner = 1,
    Member = 2,
    Admin = 3,
    ActingFounder = 4,
    Founder = 5,
}

impl Display for RuntimeGroupMemberType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for RuntimeGroupMemberType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(RuntimeGroupMemberType::None),
            "Beginner" => Ok(RuntimeGroupMemberType::Beginner),
            "Member" => Ok(RuntimeGroupMemberType::Member),
            "Admin" => Ok(RuntimeGroupMemberType::Admin),
            "ActingFounder" => Ok(RuntimeGroupMemberType::ActingFounder),
            "Founder" => Ok(RuntimeGroupMemberType::Founder),
            _ => Err(anyhow!("Could not deserialize string '{}' to RuntimeGroupMemberType", s)),
        }
    }
}

/// This contract contains clan-specific group information. It does not include any investment data.
#[derive(Deserialize, Serialize)]
pub struct GroupV2ClanInfo {
    #[serde(rename = "clanCallsign")]
    pub clan_callsign: Option<String>,

    #[serde(rename = "clanBannerData")]
    pub clan_banner_data: Option<crate::groups_v2::ClanBanner>,
}

#[derive(Deserialize, Serialize)]
pub struct ClanBanner {
    #[serde(rename = "decalId")]
    pub decal_id: u32,

    #[serde(rename = "decalColorId")]
    pub decal_color_id: u32,

    #[serde(rename = "decalBackgroundColorId")]
    pub decal_background_color_id: u32,

    #[serde(rename = "gonfalonId")]
    pub gonfalon_id: u32,

    #[serde(rename = "gonfalonColorId")]
    pub gonfalon_color_id: u32,

    #[serde(rename = "gonfalonDetailId")]
    pub gonfalon_detail_id: u32,

    #[serde(rename = "gonfalonDetailColorId")]
    pub gonfalon_detail_color_id: u32,
}

/// The same as GroupV2ClanInfo, but includes any investment data.
#[derive(Deserialize, Serialize)]
pub struct GroupV2ClanInfoAndInvestment {
    #[serde(rename = "d2ClanProgressions")]
    pub d2_clan_progressions: Option<HashMap<u32, crate::destiny::DestinyProgression>>,

    #[serde(rename = "clanCallsign")]
    pub clan_callsign: Option<String>,

    #[serde(rename = "clanBannerData")]
    pub clan_banner_data: Option<crate::groups_v2::ClanBanner>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupUserBase {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "destinyUserInfo")]
    pub destiny_user_info: Option<crate::groups_v2::GroupUserInfoCard>,

    #[serde(rename = "bungieNetUserInfo")]
    pub bungie_net_user_info: Option<crate::user::UserInfoCard>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "joinDate")]
    pub join_date: OffsetDateTime,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupMember {
    #[serde(rename = "memberType")]
    pub member_type: crate::groups_v2::RuntimeGroupMemberType,

    #[serde(rename = "isOnline")]
    pub is_online: bool,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "lastOnlineStatusChange")]
    pub last_online_status_change: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "destinyUserInfo")]
    pub destiny_user_info: Option<crate::groups_v2::GroupUserInfoCard>,

    #[serde(rename = "bungieNetUserInfo")]
    pub bungie_net_user_info: Option<crate::user::UserInfoCard>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "joinDate")]
    pub join_date: OffsetDateTime,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupAllianceStatus {
    Unallied = 0,
    Parent = 1,
    Child = 2,
}

impl Display for GroupAllianceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupAllianceStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Unallied" => Ok(GroupAllianceStatus::Unallied),
            "Parent" => Ok(GroupAllianceStatus::Parent),
            "Child" => Ok(GroupAllianceStatus::Child),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupAllianceStatus", s)),
        }
    }
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupPotentialMember {
    #[serde(rename = "potentialStatus")]
    pub potential_status: crate::groups_v2::GroupPotentialMemberStatus,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "destinyUserInfo")]
    pub destiny_user_info: Option<crate::groups_v2::GroupUserInfoCard>,

    #[serde(rename = "bungieNetUserInfo")]
    pub bungie_net_user_info: Option<crate::user::UserInfoCard>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "joinDate")]
    pub join_date: OffsetDateTime,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupPotentialMemberStatus {
    None = 0,
    Applicant = 1,
    Invitee = 2,
}

impl Display for GroupPotentialMemberStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupPotentialMemberStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(GroupPotentialMemberStatus::None),
            "Applicant" => Ok(GroupPotentialMemberStatus::Applicant),
            "Invitee" => Ok(GroupPotentialMemberStatus::Invitee),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupPotentialMemberStatus", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupDateRange {
    All = 0,
    PastDay = 1,
    PastWeek = 2,
    PastMonth = 3,
    PastYear = 4,
}

impl Display for GroupDateRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupDateRange {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "All" => Ok(GroupDateRange::All),
            "PastDay" => Ok(GroupDateRange::PastDay),
            "PastWeek" => Ok(GroupDateRange::PastWeek),
            "PastMonth" => Ok(GroupDateRange::PastMonth),
            "PastYear" => Ok(GroupDateRange::PastYear),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupDateRange", s)),
        }
    }
}

/// A small infocard of group information, usually used for when a list of groups are returned
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupV2Card {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "groupType")]
    pub group_type: crate::groups_v2::GroupType,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "creationDate")]
    pub creation_date: OffsetDateTime,

    #[serde(rename = "about")]
    pub about: Option<String>,

    #[serde(rename = "motto")]
    pub motto: Option<String>,

    #[serde(rename = "memberCount")]
    pub member_count: i32,

    #[serde(rename = "locale")]
    pub locale: Option<String>,

    #[serde(rename = "membershipOption")]
    pub membership_option: crate::groups_v2::MembershipOption,

    #[serde(rename = "capabilities")]
    pub capabilities: enumflags2::BitFlags<crate::groups_v2::Capabilities>,

    #[serde(rename = "clanInfo")]
    pub clan_info: Option<crate::groups_v2::GroupV2ClanInfo>,

    #[serde(rename = "avatarPath")]
    pub avatar_path: Option<String>,

    #[serde(rename = "theme")]
    pub theme: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupSearchResponse {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::groups_v2::GroupV2Card>>,

    #[serde(rename = "totalResults")]
    pub total_results: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "query")]
    pub query: Option<crate::queries::PagedQuery>,

    #[serde(rename = "replacementContinuationToken")]
    pub replacement_continuation_token: Option<String>,

    /// If useTotalResults is true, then totalResults represents an accurate count.
    /// If False, it does not, and may be estimated/only the size of the current page.
    /// Either way, you should probably always only trust hasMore.
    /// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    #[serde(rename = "useTotalResults")]
    pub use_total_results: bool,
}

/// NOTE: GroupQuery, as of Destiny 2, has essentially two totally different and incompatible "modes".
/// If you are querying for a group, you can pass any of the properties below.
/// If you are querying for a Clan, you MUST NOT pass any of the following properties (they must be null or undefined in your request, not just empty string/default values):
/// - groupMemberCountFilter - localeFilter - tagText
/// If you pass these, you will get a useless InvalidParameters error.
#[derive(Deserialize, Serialize)]
pub struct GroupQuery {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "groupType")]
    pub group_type: crate::groups_v2::GroupType,

    #[serde(rename = "creationDate")]
    pub creation_date: crate::groups_v2::GroupDateRange,

    #[serde(rename = "sortBy")]
    pub sort_by: crate::groups_v2::GroupSortBy,

    #[serde(rename = "groupMemberCountFilter")]
    pub group_member_count_filter: Option<i32>,

    #[serde(rename = "localeFilter")]
    pub locale_filter: Option<String>,

    #[serde(rename = "tagText")]
    pub tag_text: Option<String>,

    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i32,

    #[serde(rename = "currentPage")]
    pub current_page: i32,

    #[serde(rename = "requestContinuationToken")]
    pub request_continuation_token: Option<String>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupSortBy {
    Name = 0,
    Date = 1,
    Popularity = 2,
    Id = 3,
}

impl Display for GroupSortBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupSortBy {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Name" => Ok(GroupSortBy::Name),
            "Date" => Ok(GroupSortBy::Date),
            "Popularity" => Ok(GroupSortBy::Popularity),
            "Id" => Ok(GroupSortBy::Id),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupSortBy", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupMemberCountFilter {
    All = 0,
    OneToTen = 1,
    ElevenToOneHundred = 2,
    GreaterThanOneHundred = 3,
}

impl Display for GroupMemberCountFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupMemberCountFilter {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "All" => Ok(GroupMemberCountFilter::All),
            "OneToTen" => Ok(GroupMemberCountFilter::OneToTen),
            "ElevenToOneHundred" => Ok(GroupMemberCountFilter::ElevenToOneHundred),
            "GreaterThanOneHundred" => Ok(GroupMemberCountFilter::GreaterThanOneHundred),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupMemberCountFilter", s)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GroupNameSearchRequest {
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,

    #[serde(rename = "groupType")]
    pub group_type: crate::groups_v2::GroupType,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupOptionalConversation {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "conversationId")]
    pub conversation_id: i64,

    #[serde(rename = "chatEnabled")]
    pub chat_enabled: bool,

    #[serde(rename = "chatName")]
    pub chat_name: Option<String>,

    #[serde(rename = "chatSecurity")]
    pub chat_security: crate::groups_v2::ChatSecuritySetting,
}

#[derive(Deserialize, Serialize)]
pub struct GroupEditAction {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "about")]
    pub about: Option<String>,

    #[serde(rename = "motto")]
    pub motto: Option<String>,

    #[serde(rename = "theme")]
    pub theme: Option<String>,

    #[serde(rename = "avatarImageIndex")]
    pub avatar_image_index: Option<i32>,

    #[serde(rename = "tags")]
    pub tags: Option<String>,

    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,

    #[serde(rename = "membershipOption")]
    pub membership_option: Option<i32>,

    #[serde(rename = "isPublicTopicAdminOnly")]
    pub is_public_topic_admin_only: Option<bool>,

    #[serde(rename = "allowChat")]
    pub allow_chat: Option<bool>,

    #[serde(rename = "chatSecurity")]
    pub chat_security: Option<i32>,

    #[serde(rename = "callsign")]
    pub callsign: Option<String>,

    #[serde(rename = "locale")]
    pub locale: Option<String>,

    #[serde(rename = "homepage")]
    pub homepage: Option<i32>,

    #[serde(rename = "enableInvitationMessagingForAdmins")]
    pub enable_invitation_messaging_for_admins: Option<bool>,

    #[serde(rename = "defaultPublicity")]
    pub default_publicity: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupOptionsEditAction {
    /// Minimum Member Level allowed to invite new members to group
    /// Always Allowed: Founder, Acting Founder
    /// True means admins have this power, false means they don't
    /// Default is false for clans, true for groups.
    #[serde(rename = "InvitePermissionOverride")]
    pub invite_permission_override: Option<bool>,

    /// Minimum Member Level allowed to update group culture
    /// Always Allowed: Founder, Acting Founder
    /// True means admins have this power, false means they don't
    /// Default is false for clans, true for groups.
    #[serde(rename = "UpdateCulturePermissionOverride")]
    pub update_culture_permission_override: Option<bool>,

    /// Minimum Member Level allowed to host guided games
    /// Always Allowed: Founder, Acting Founder, Admin
    /// Allowed Overrides: None, Member, Beginner
    /// Default is Member for clans, None for groups, although this means nothing for groups.
    #[serde(rename = "HostGuidedGamePermissionOverride")]
    pub host_guided_game_permission_override: Option<i32>,

    /// Minimum Member Level allowed to update banner
    /// Always Allowed: Founder, Acting Founder
    /// True means admins have this power, false means they don't
    /// Default is false for clans, true for groups.
    #[serde(rename = "UpdateBannerPermissionOverride")]
    pub update_banner_permission_override: Option<bool>,

    /// Level to join a member at when accepting an invite, application, or joining an open clan
    /// Default is Beginner.
    #[serde(rename = "JoinLevel")]
    pub join_level: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupOptionalConversationAddRequest {
    #[serde(rename = "chatName")]
    pub chat_name: Option<String>,

    #[serde(rename = "chatSecurity")]
    pub chat_security: crate::groups_v2::ChatSecuritySetting,
}

#[derive(Deserialize, Serialize)]
pub struct GroupOptionalConversationEditRequest {
    #[serde(rename = "chatEnabled")]
    pub chat_enabled: Option<bool>,

    #[serde(rename = "chatName")]
    pub chat_name: Option<String>,

    #[serde(rename = "chatSecurity")]
    pub chat_security: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupMemberLeaveResult {
    #[serde(rename = "group")]
    pub group: Option<crate::groups_v2::GroupV2>,

    #[serde(rename = "groupDeleted")]
    pub group_deleted: bool,
}

#[derive(Deserialize, Serialize)]
pub struct GroupBanRequest {
    #[serde(rename = "comment")]
    pub comment: Option<String>,

    #[serde(rename = "length")]
    pub length: crate::ignores::IgnoreLength,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupBan {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: Option<crate::user::UserInfoCard>,

    #[serde(rename = "createdBy")]
    pub created_by: Option<crate::user::UserInfoCard>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "dateBanned")]
    pub date_banned: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "dateExpires")]
    pub date_expires: OffsetDateTime,

    #[serde(rename = "comment")]
    pub comment: Option<String>,

    #[serde(rename = "bungieNetUserInfo")]
    pub bungie_net_user_info: Option<crate::user::UserInfoCard>,

    #[serde(rename = "destinyUserInfo")]
    pub destiny_user_info: Option<crate::groups_v2::GroupUserInfoCard>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GroupMemberApplication {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "creationDate")]
    pub creation_date: OffsetDateTime,

    #[serde(rename = "resolveState")]
    pub resolve_state: crate::groups_v2::GroupApplicationResolveState,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "resolveDate")]
    pub resolve_date: Option<OffsetDateTime>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "resolvedByMembershipId")]
    pub resolved_by_membership_id: Option<i64>,

    #[serde(rename = "requestMessage")]
    pub request_message: Option<String>,

    #[serde(rename = "resolveMessage")]
    pub resolve_message: Option<String>,

    #[serde(rename = "destinyUserInfo")]
    pub destiny_user_info: Option<crate::groups_v2::GroupUserInfoCard>,

    #[serde(rename = "bungieNetUserInfo")]
    pub bungie_net_user_info: Option<crate::user::UserInfoCard>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupApplicationResolveState {
    Unresolved = 0,
    Accepted = 1,
    Denied = 2,
    Rescinded = 3,
}

impl Display for GroupApplicationResolveState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupApplicationResolveState {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Unresolved" => Ok(GroupApplicationResolveState::Unresolved),
            "Accepted" => Ok(GroupApplicationResolveState::Accepted),
            "Denied" => Ok(GroupApplicationResolveState::Denied),
            "Rescinded" => Ok(GroupApplicationResolveState::Rescinded),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupApplicationResolveState", s)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GroupApplicationRequest {
    #[serde(rename = "message")]
    pub message: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupApplicationListRequest {
    #[serde(rename = "memberships")]
    pub memberships: Option<Vec<crate::user::UserMembership>>,

    #[serde(rename = "message")]
    pub message: Option<String>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupsForMemberFilter {
    All = 0,
    Founded = 1,
    NonFounded = 2,
}

impl Display for GroupsForMemberFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GroupsForMemberFilter {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "All" => Ok(GroupsForMemberFilter::All),
            "Founded" => Ok(GroupsForMemberFilter::Founded),
            "NonFounded" => Ok(GroupsForMemberFilter::NonFounded),
            _ => Err(anyhow!("Could not deserialize string '{}' to GroupsForMemberFilter", s)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GroupMembershipBase {
    #[serde(rename = "group")]
    pub group: Option<crate::groups_v2::GroupV2>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupMembership {
    #[serde(rename = "member")]
    pub member: Option<crate::groups_v2::GroupMember>,

    #[serde(rename = "group")]
    pub group: Option<crate::groups_v2::GroupV2>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupMembershipSearchResponse {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::groups_v2::GroupMembership>>,

    #[serde(rename = "totalResults")]
    pub total_results: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "query")]
    pub query: Option<crate::queries::PagedQuery>,

    #[serde(rename = "replacementContinuationToken")]
    pub replacement_continuation_token: Option<String>,

    /// If useTotalResults is true, then totalResults represents an accurate count.
    /// If False, it does not, and may be estimated/only the size of the current page.
    /// Either way, you should probably always only trust hasMore.
    /// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    #[serde(rename = "useTotalResults")]
    pub use_total_results: bool,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct GetGroupsForMemberResponse {
    /// A convenience property that indicates if every membership this user has that is a part of this group are part of an account that is considered inactive - for example, overridden accounts in Cross Save.
    /// The key is the Group ID for the group being checked, and the value is true if the users' memberships for that group are all inactive.
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "areAllMembershipsInactive")]
    pub are_all_memberships_inactive: Option<HashMap<i64, bool>>,

    #[serde(rename = "results")]
    pub results: Option<Vec<crate::groups_v2::GroupMembership>>,

    #[serde(rename = "totalResults")]
    pub total_results: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "query")]
    pub query: Option<crate::queries::PagedQuery>,

    #[serde(rename = "replacementContinuationToken")]
    pub replacement_continuation_token: Option<String>,

    /// If useTotalResults is true, then totalResults represents an accurate count.
    /// If False, it does not, and may be estimated/only the size of the current page.
    /// Either way, you should probably always only trust hasMore.
    /// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    #[serde(rename = "useTotalResults")]
    pub use_total_results: bool,
}

#[derive(Deserialize, Serialize)]
pub struct GroupPotentialMembership {
    #[serde(rename = "member")]
    pub member: Option<crate::groups_v2::GroupPotentialMember>,

    #[serde(rename = "group")]
    pub group: Option<crate::groups_v2::GroupV2>,
}

#[derive(Deserialize, Serialize)]
pub struct GroupPotentialMembershipSearchResponse {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::groups_v2::GroupPotentialMembership>>,

    #[serde(rename = "totalResults")]
    pub total_results: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "query")]
    pub query: Option<crate::queries::PagedQuery>,

    #[serde(rename = "replacementContinuationToken")]
    pub replacement_continuation_token: Option<String>,

    /// If useTotalResults is true, then totalResults represents an accurate count.
    /// If False, it does not, and may be estimated/only the size of the current page.
    /// Either way, you should probably always only trust hasMore.
    /// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    #[serde(rename = "useTotalResults")]
    pub use_total_results: bool,
}

#[derive(Deserialize, Serialize)]
pub struct GroupApplicationResponse {
    #[serde(rename = "resolution")]
    pub resolution: crate::groups_v2::GroupApplicationResolveState,
}
