pub mod models;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

/// Very basic info about a user as returned by the Account server.
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct UserMembership {
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

/// Very basic info about a user as returned by the Account server, but including CrossSave information. Do NOT use as a request contract.
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct CrossSaveUserMembership {
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

/// This contract supplies basic information commonly used to display a minimal amount of information about a user. Take care to not add more properties here unless the property applies in all (or at least the majority) of the situations where UserInfoCard is used. Avoid adding game specific or platform specific details here. In cases where UserInfoCard is a subset of the data needed in a contract, use UserInfoCard as a property of other contracts.
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct UserInfoCard {
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
pub struct GeneralUser {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "membershipId")]
    pub membership_id: i64,

    #[serde(rename = "uniqueName")]
    pub unique_name: Option<String>,

    #[serde(rename = "normalizedName")]
    pub normalized_name: Option<String>,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "profilePicture")]
    pub profile_picture: i32,

    #[serde(rename = "profileTheme")]
    pub profile_theme: i32,

    #[serde(rename = "userTitle")]
    pub user_title: i32,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "successMessageFlags")]
    pub success_message_flags: i64,

    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,

    #[serde(rename = "about")]
    pub about: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "firstAccess")]
    pub first_access: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "lastUpdate")]
    pub last_update: Option<OffsetDateTime>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "legacyPortalUID")]
    pub legacy_portal_u_i_d: Option<i64>,

    #[serde(rename = "context")]
    pub context: Option<crate::user::UserToUserContext>,

    #[serde(rename = "psnDisplayName")]
    pub psn_display_name: Option<String>,

    #[serde(rename = "xboxDisplayName")]
    pub xbox_display_name: Option<String>,

    #[serde(rename = "fbDisplayName")]
    pub fb_display_name: Option<String>,

    #[serde(rename = "showActivity")]
    pub show_activity: Option<bool>,

    #[serde(rename = "locale")]
    pub locale: Option<String>,

    #[serde(rename = "localeInheritDefault")]
    pub locale_inherit_default: bool,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "lastBanReportId")]
    pub last_ban_report_id: Option<i64>,

    #[serde(rename = "showGroupMessaging")]
    pub show_group_messaging: bool,

    #[serde(rename = "profilePicturePath")]
    pub profile_picture_path: Option<String>,

    #[serde(rename = "profilePictureWidePath")]
    pub profile_picture_wide_path: Option<String>,

    #[serde(rename = "profileThemeName")]
    pub profile_theme_name: Option<String>,

    #[serde(rename = "userTitleDisplay")]
    pub user_title_display: Option<String>,

    #[serde(rename = "statusText")]
    pub status_text: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "statusDate")]
    pub status_date: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "profileBanExpire")]
    pub profile_ban_expire: Option<OffsetDateTime>,

    #[serde(rename = "blizzardDisplayName")]
    pub blizzard_display_name: Option<String>,

    #[serde(rename = "steamDisplayName")]
    pub steam_display_name: Option<String>,

    #[serde(rename = "stadiaDisplayName")]
    pub stadia_display_name: Option<String>,

    #[serde(rename = "twitchDisplayName")]
    pub twitch_display_name: Option<String>,

    #[serde(rename = "cachedBungieGlobalDisplayName")]
    pub cached_bungie_global_display_name: Option<String>,

    #[serde(rename = "cachedBungieGlobalDisplayNameCode")]
    pub cached_bungie_global_display_name_code: Option<i16>,
}

#[derive(Deserialize, Serialize)]
pub struct UserToUserContext {
    #[serde(rename = "isFollowing")]
    pub is_following: bool,

    #[serde(rename = "ignoreStatus")]
    pub ignore_status: Option<crate::ignores::IgnoreResponse>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "globalIgnoreEndDate")]
    pub global_ignore_end_date: Option<OffsetDateTime>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct UserMembershipData {
    /// this allows you to see destiny memberships that are visible and linked to this account (regardless of whether or not they have characters on the world server)
    #[serde(rename = "destinyMemberships")]
    pub destiny_memberships: Option<Vec<crate::groups_v2::GroupUserInfoCard>>,

    /// If this property is populated, it will have the membership ID of the account considered to be "primary" in this user's cross save relationship.
    /// If null, this user has no cross save relationship, nor primary account.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "primaryMembershipId")]
    pub primary_membership_id: Option<i64>,

    #[serde(rename = "bungieNetUser")]
    pub bungie_net_user: Option<crate::user::GeneralUser>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct HardLinkedUserMembership {
    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "membershipId")]
    pub membership_id: i64,

    #[serde(rename = "CrossSaveOverriddenType")]
    pub cross_save_overridden_type: crate::BungieMembershipType,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "CrossSaveOverriddenMembershipId")]
    pub cross_save_overridden_membership_id: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct UserSearchResponse {
    #[serde(rename = "searchResults")]
    pub search_results: Option<Vec<crate::user::UserSearchResponseDetail>>,

    #[serde(rename = "page")]
    pub page: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct UserSearchResponseDetail {
    #[serde(rename = "bungieGlobalDisplayName")]
    pub bungie_global_display_name: Option<String>,

    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub bungie_global_display_name_code: Option<i16>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "bungieNetMembershipId")]
    pub bungie_net_membership_id: Option<i64>,

    #[serde(rename = "destinyMemberships")]
    pub destiny_memberships: Option<Vec<crate::user::UserInfoCard>>,
}

#[derive(Deserialize, Serialize)]
pub struct UserSearchPrefixRequest {
    #[serde(rename = "displayNamePrefix")]
    pub display_name_prefix: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ExactSearchRequest {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "displayNameCode")]
    pub display_name_code: i16,
}

/// The set of all email subscription/opt-in settings and definitions.
#[derive(Deserialize, Serialize)]
pub struct EmailSettings {
    /// Keyed by the name identifier of the opt-in definition.
    #[serde(rename = "optInDefinitions")]
    pub opt_in_definitions: Option<HashMap<String, crate::user::EmailOptInDefinition>>,

    /// Keyed by the name identifier of the Subscription definition.
    #[serde(rename = "subscriptionDefinitions")]
    pub subscription_definitions: Option<HashMap<String, crate::user::EmailSubscriptionDefinition>>,

    /// Keyed by the name identifier of the View definition.
    #[serde(rename = "views")]
    pub views: Option<HashMap<String, crate::user::EmailViewDefinition>>,
}

/// Defines a single opt-in category: a wide-scoped permission to send emails for the subject related to the opt-in.
#[derive(Deserialize, Serialize)]
pub struct EmailOptInDefinition {
    /// The unique identifier for this opt-in category.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The flag value for this opt-in category. For historical reasons, this is defined as a flags enum.
    #[serde(rename = "value")]
    pub value: crate::user::OptInFlags,

    /// If true, this opt-in setting should be set by default in situations where accounts are created without explicit choices about what they're opting into.
    #[serde(rename = "setByDefault")]
    pub set_by_default: bool,

    /// Information about the dependent subscriptions for this opt-in.
    #[serde(rename = "dependentSubscriptions")]
    pub dependent_subscriptions: Option<Vec<crate::user::EmailSubscriptionDefinition>>,
}

#[bitflags]
#[repr(u64)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptInFlags {
    Newsletter = 1,
    System = 2,
    Marketing = 4,
    UserResearch = 8,
    CustomerService = 16,
    Social = 32,
    PlayTests = 64,
    PlayTestsLocal = 128,
    Careers = 256,
}

impl Display for OptInFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u64)
    }
}

/// Defines a single subscription: permission to send emails for a specific, focused subject (generally timeboxed, such as for a specific release of a product or feature).
#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct EmailSubscriptionDefinition {
    /// The unique identifier for this subscription.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// A dictionary of localized text for the EMail Opt-in setting, keyed by the locale.
    #[serde(rename = "localization")]
    pub localization: Option<HashMap<String, crate::user::EMailSettingSubscriptionLocalization>>,

    /// The bitflag value for this subscription. Should be a unique power of two value.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "value")]
    pub value: i64,
}

/// Localized text relevant to a given EMail setting in a given localization.
#[derive(Deserialize, Serialize)]
pub struct EMailSettingLocalization {
    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,
}

/// Localized text relevant to a given EMail setting in a given localization. Extra settings specifically for subscriptions.
#[derive(Deserialize, Serialize)]
pub struct EMailSettingSubscriptionLocalization {
    #[serde(rename = "unknownUserDescription")]
    pub unknown_user_description: Option<String>,

    #[serde(rename = "registeredUserDescription")]
    pub registered_user_description: Option<String>,

    #[serde(rename = "unregisteredUserDescription")]
    pub unregistered_user_description: Option<String>,

    #[serde(rename = "unknownUserActionText")]
    pub unknown_user_action_text: Option<String>,

    #[serde(rename = "knownUserActionText")]
    pub known_user_action_text: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,
}

/// Represents a data-driven view for Email settings. Web/Mobile UI can use this data to show new EMail settings consistently without further manual work.
#[derive(Deserialize, Serialize)]
pub struct EmailViewDefinition {
    /// The identifier for this view.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The ordered list of settings to show in this view.
    #[serde(rename = "viewSettings")]
    pub view_settings: Option<Vec<crate::user::EmailViewDefinitionSetting>>,
}

#[derive(Deserialize, Serialize)]
pub struct EmailViewDefinitionSetting {
    /// The identifier for this UI Setting, which can be used to relate it to custom strings or other data as desired.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// A dictionary of localized text for the EMail setting, keyed by the locale.
    #[serde(rename = "localization")]
    pub localization: Option<HashMap<String, crate::user::EMailSettingLocalization>>,

    /// If true, this setting should be set by default if the user hasn't chosen whether it's set or cleared yet.
    #[serde(rename = "setByDefault")]
    pub set_by_default: bool,

    /// The OptInFlags value to set or clear if this setting is set or cleared in the UI. It is the aggregate of all underlying opt-in flags related to this setting.
    #[serde(rename = "optInAggregateValue")]
    pub opt_in_aggregate_value: crate::user::OptInFlags,

    /// The subscriptions to show as children of this setting, if any.
    #[serde(rename = "subscriptions")]
    pub subscriptions: Option<Vec<crate::user::EmailSubscriptionDefinition>>,
}
