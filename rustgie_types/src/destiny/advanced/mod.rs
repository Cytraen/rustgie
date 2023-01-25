use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct AwaInitializeResponse {
    /// ID used to get the token. Present this ID to the user as it will identify this specific request on their device.
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,

    /// True if the PUSH message will only be sent to the device that made this request.
    #[serde(rename = "sentToSelf")]
    pub sent_to_self: bool,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct AwaPermissionRequested {
    /// Type of advanced write action.
    #[serde(rename = "type")]
    pub r#type: crate::destiny::advanced::AwaType,

    /// Item instance ID the action shall be applied to. This is optional for all but a new AwaType values. Rule of thumb is to provide the item instance ID if one is available.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "affectedItemId")]
    pub affected_item_id: Option<i64>,

    /// Destiny membership type of the account to modify.
    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,

    /// Destiny character ID, if applicable, that will be affected by the action.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "characterId")]
    pub character_id: Option<i64>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AwaType {
    None = 0,
    /// Insert plugs into sockets.
    InsertPlugs = 1,
}

impl Display for AwaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for AwaType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(AwaType::None),
            "InsertPlugs" => Ok(AwaType::InsertPlugs),
            _ => Err(anyhow!("Could not deserialize string '{}' to AwaType", s)),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct AwaUserResponse {
    /// Indication of the selection the user has made (Approving or rejecting the action)
    #[serde(rename = "selection")]
    pub selection: crate::destiny::advanced::AwaUserSelection,

    /// Correlation ID of the request
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,

    /// Secret nonce received via the PUSH notification.
    #[serde(rename = "nonce")]
    pub nonce: Option<Vec<u8>>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AwaUserSelection {
    None = 0,
    Rejected = 1,
    Approved = 2,
}

impl Display for AwaUserSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for AwaUserSelection {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(AwaUserSelection::None),
            "Rejected" => Ok(AwaUserSelection::Rejected),
            "Approved" => Ok(AwaUserSelection::Approved),
            _ => Err(anyhow!("Could not deserialize string '{}' to AwaUserSelection", s)),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct AwaAuthorizationResult {
    /// Indication of how the user responded to the request. If the value is "Approved" the actionToken will contain the token that can be presented when performing the advanced write action.
    #[serde(rename = "userSelection")]
    pub user_selection: crate::destiny::advanced::AwaUserSelection,

    #[serde(rename = "responseReason")]
    pub response_reason: crate::destiny::advanced::AwaResponseReason,

    /// Message to the app developer to help understand the response.
    #[serde(rename = "developerNote")]
    pub developer_note: Option<String>,

    /// Credential used to prove the user authorized an advanced write action.
    #[serde(rename = "actionToken")]
    pub action_token: Option<String>,

    /// This token may be used to perform the requested action this number of times, at a maximum. If this value is 0, then there is no limit.
    #[serde(rename = "maximumNumberOfUses")]
    pub maximum_number_of_uses: i32,

    /// Time, UTC, when token expires.
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "validUntil")]
    pub valid_until: Option<OffsetDateTime>,

    /// Advanced Write Action Type from the permission request.
    #[serde(rename = "type")]
    pub r#type: crate::destiny::advanced::AwaType,

    /// MembershipType from the permission request.
    #[serde(rename = "membershipType")]
    pub membership_type: crate::BungieMembershipType,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AwaResponseReason {
    None = 0,
    /// User provided an answer
    Answered = 1,
    /// The HTTP request timed out, a new request may be made and an answer may still be provided.
    TimedOut = 2,
    /// This request was replaced by another request.
    Replaced = 3,
}

impl Display for AwaResponseReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for AwaResponseReason {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(AwaResponseReason::None),
            "Answered" => Ok(AwaResponseReason::Answered),
            "TimedOut" => Ok(AwaResponseReason::TimedOut),
            "Replaced" => Ok(AwaResponseReason::Replaced),
            _ => Err(anyhow!("Could not deserialize string '{}' to AwaResponseReason", s)),
        }
    }
}
