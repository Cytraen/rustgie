use anyhow::{anyhow, Result};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;

#[bitflags]
#[repr(u64)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationScopes {
    /// Read basic user profile information such as the user's handle, avatar icon, etc.
    ReadBasicUserProfile = 1,
    /// Read Group/Clan Forums, Wall, and Members for groups and clans that the user has joined.
    ReadGroups = 2,
    /// Write Group/Clan Forums, Wall, and Members for groups and clans that the user has joined.
    WriteGroups = 4,
    /// Administer Group/Clan Forums, Wall, and Members for groups and clans that the user is a founder or an administrator.
    AdminGroups = 8,
    /// Create new groups, clans, and forum posts, along with other actions that are reserved for Bungie.net elevated scope: not meant to be used by third party applications.
    BnetWrite = 16,
    /// Move or equip Destiny items
    MoveEquipDestinyItems = 32,
    /// Read Destiny 1 Inventory and Vault contents. For Destiny 2, this scope is needed to read anything regarded as private. This is the only scope a Destiny 2 app needs for read operations against Destiny 2 data such as inventory, vault, currency, vendors, milestones, progression, etc.
    ReadDestinyInventoryAndVault = 64,
    /// Read user data such as who they are web notifications, clan/group memberships, recent activity, muted users.
    ReadUserData = 128,
    /// Edit user data such as preferred language, status, motto, avatar selection and theme.
    EditUserData = 256,
    /// Access vendor and advisor data specific to a user. OBSOLETE. This scope is only used on the Destiny 1 API.
    ReadDestinyVendorsAndAdvisors = 512,
    /// Read offer history and claim and apply tokens for the user.
    ReadAndApplyTokens = 1024,
    /// Can perform actions that will result in a prompt to the user via the Destiny app.
    AdvancedWriteActions = 2048,
    /// Can use the partner offer api to claim rewards defined for a partner
    PartnerOfferGrant = 4096,
    /// Allows an app to query sensitive information like unlock flags and values not available through normal methods.
    DestinyUnlockValueQuery = 8192,
    /// Allows an app to query sensitive user PII, most notably email information.
    UserPiiRead = 16384,
}

impl Display for ApplicationScopes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u64)
    }
}

impl FromStr for ApplicationScopes {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "ReadBasicUserProfile" => Ok(ApplicationScopes::ReadBasicUserProfile),
            "ReadGroups" => Ok(ApplicationScopes::ReadGroups),
            "WriteGroups" => Ok(ApplicationScopes::WriteGroups),
            "AdminGroups" => Ok(ApplicationScopes::AdminGroups),
            "BnetWrite" => Ok(ApplicationScopes::BnetWrite),
            "MoveEquipDestinyItems" => Ok(ApplicationScopes::MoveEquipDestinyItems),
            "ReadDestinyInventoryAndVault" => Ok(ApplicationScopes::ReadDestinyInventoryAndVault),
            "ReadUserData" => Ok(ApplicationScopes::ReadUserData),
            "EditUserData" => Ok(ApplicationScopes::EditUserData),
            "ReadDestinyVendorsAndAdvisors" => Ok(ApplicationScopes::ReadDestinyVendorsAndAdvisors),
            "ReadAndApplyTokens" => Ok(ApplicationScopes::ReadAndApplyTokens),
            "AdvancedWriteActions" => Ok(ApplicationScopes::AdvancedWriteActions),
            "PartnerOfferGrant" => Ok(ApplicationScopes::PartnerOfferGrant),
            "DestinyUnlockValueQuery" => Ok(ApplicationScopes::DestinyUnlockValueQuery),
            "UserPiiRead" => Ok(ApplicationScopes::UserPiiRead),
            _ => Err(anyhow!("Could not deserialize string '{}' to ApplicationScopes", s)),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct ApiUsage {
    /// Counts for on API calls made for the time range.
    #[serde(rename = "apiCalls")]
    pub api_calls: Option<Vec<crate::applications::Series>>,

    /// Instances of blocked requests or requests that crossed the warn threshold during the time range.
    #[serde(rename = "throttledRequests")]
    pub throttled_requests: Option<Vec<crate::applications::Series>>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Series {
    /// Collection of samples with time and value.
    #[serde(rename = "datapoints")]
    pub datapoints: Option<Vec<crate::applications::Datapoint>>,

    /// Target to which to datapoints apply.
    #[serde(rename = "target")]
    pub target: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Datapoint {
    /// Timestamp for the related count.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "time")]
    pub time: OffsetDateTime,

    /// Count associated with timestamp
    #[serde(rename = "count")]
    pub count: Option<f64>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct Application {
    /// Unique ID assigned to the application
    #[serde(rename = "applicationId")]
    pub application_id: i32,

    /// Name of the application
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL used to pass the user's authorization code to the application
    #[serde(rename = "redirectUrl")]
    pub redirect_url: Option<String>,

    /// Link to website for the application where a user can learn more about the app.
    #[serde(rename = "link")]
    pub link: Option<String>,

    /// Permissions the application needs to work
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "scope")]
    pub scope: i64,

    /// Value of the Origin header sent in requests generated by this application.
    #[serde(rename = "origin")]
    pub origin: Option<String>,

    /// Current status of the application.
    #[serde(rename = "status")]
    pub status: crate::applications::ApplicationStatus,

    /// Date the application was first added to our database.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "creationDate")]
    pub creation_date: OffsetDateTime,

    /// Date the application status last changed.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "statusChanged")]
    pub status_changed: OffsetDateTime,

    /// Date the first time the application status entered the 'Public' status.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "firstPublished")]
    pub first_published: OffsetDateTime,

    /// List of team members who manage this application on Bungie.net. Will always consist of at least the application owner.
    #[serde(rename = "team")]
    pub team: Option<Vec<crate::applications::ApplicationDeveloper>>,

    /// An optional override for the Authorize view name.
    #[serde(rename = "overrideAuthorizeViewName")]
    pub override_authorize_view_name: Option<String>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationStatus {
    /// No value assigned
    None = 0,
    /// Application exists and works but will not appear in any public catalog. New applications start in this state, test applications will remain in this state.
    Private = 1,
    /// Active applications that can appear in an catalog.
    Public = 2,
    /// Application disabled by the owner. All authorizations will be treated as terminated while in this state. Owner can move back to private or public state.
    Disabled = 3,
    /// Application has been blocked by Bungie. It cannot be transitioned out of this state by the owner. Authorizations are terminated when an application is in this state.
    Blocked = 4,
}

impl Display for ApplicationStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ApplicationStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(ApplicationStatus::None),
            "Private" => Ok(ApplicationStatus::Private),
            "Public" => Ok(ApplicationStatus::Public),
            "Disabled" => Ok(ApplicationStatus::Disabled),
            "Blocked" => Ok(ApplicationStatus::Blocked),
            _ => Err(anyhow!("Could not deserialize string '{}' to ApplicationStatus", s)),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct ApplicationDeveloper {
    #[serde(rename = "role")]
    pub role: crate::applications::DeveloperRole,

    #[serde(rename = "apiEulaVersion")]
    pub api_eula_version: i32,

    #[serde(rename = "user")]
    pub user: Option<crate::user::UserInfoCard>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeveloperRole {
    None = 0,
    Owner = 1,
    TeamMember = 2,
}

impl Display for DeveloperRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for DeveloperRole {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(DeveloperRole::None),
            "Owner" => Ok(DeveloperRole::Owner),
            "TeamMember" => Ok(DeveloperRole::TeamMember),
            _ => Err(anyhow!("Could not deserialize string '{}' to DeveloperRole", s)),
        }
    }
}
