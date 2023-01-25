#![forbid(unsafe_code)]

pub mod api_response_;
pub mod applications;
pub mod common;
pub mod components;
pub mod config;
pub mod content;
pub mod dates;
pub mod destiny;
pub mod entities;
pub mod exceptions;
pub mod fireteam;
pub mod forum;
pub mod forums;
pub mod groups_v2;
pub mod ignores;
pub mod interpolation;
pub mod links;
pub mod queries;
pub mod social;
pub mod streaming;
pub mod tags;
pub mod tokens;
pub mod trending;
pub mod user;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;

/// The types of membership the Accounts system supports. This is the external facing enum used in place of the internal-only Bungie.SharedDefinitions.MembershipType.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BungieMembershipType {
    None = 0,
    TigerXbox = 1,
    TigerPsn = 2,
    TigerSteam = 3,
    TigerBlizzard = 4,
    TigerStadia = 5,
    TigerEgs = 6,
    TigerDemon = 10,
    BungieNext = 254,
    /// "All" is only valid for searching capabilities: you need to pass the actual matching BungieMembershipType for any query where you pass a known membershipId.
    All = -1,
}

impl Display for BungieMembershipType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for BungieMembershipType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(BungieMembershipType::None),
            "TigerXbox" => Ok(BungieMembershipType::TigerXbox),
            "TigerPsn" => Ok(BungieMembershipType::TigerPsn),
            "TigerSteam" => Ok(BungieMembershipType::TigerSteam),
            "TigerBlizzard" => Ok(BungieMembershipType::TigerBlizzard),
            "TigerStadia" => Ok(BungieMembershipType::TigerStadia),
            "TigerEgs" => Ok(BungieMembershipType::TigerEgs),
            "TigerDemon" => Ok(BungieMembershipType::TigerDemon),
            "BungieNext" => Ok(BungieMembershipType::BungieNext),
            "All" => Ok(BungieMembershipType::All),
            _ => Err(anyhow!("Could not deserialize string '{}' to BungieMembershipType", s)),
        }
    }
}

/// The types of credentials the Accounts system supports. This is the external facing enum used in place of the internal-only Bungie.SharedDefinitions.CredentialType.
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BungieCredentialType {
    None = 0,
    Xuid = 1,
    Psnid = 2,
    Wlid = 3,
    Fake = 4,
    Facebook = 5,
    Google = 8,
    Windows = 9,
    DemonId = 10,
    SteamId = 12,
    BattleNetId = 14,
    StadiaId = 16,
    TwitchId = 18,
    EgsId = 20,
}

impl Display for BungieCredentialType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for BungieCredentialType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(BungieCredentialType::None),
            "Xuid" => Ok(BungieCredentialType::Xuid),
            "Psnid" => Ok(BungieCredentialType::Psnid),
            "Wlid" => Ok(BungieCredentialType::Wlid),
            "Fake" => Ok(BungieCredentialType::Fake),
            "Facebook" => Ok(BungieCredentialType::Facebook),
            "Google" => Ok(BungieCredentialType::Google),
            "Windows" => Ok(BungieCredentialType::Windows),
            "DemonId" => Ok(BungieCredentialType::DemonId),
            "SteamId" => Ok(BungieCredentialType::SteamId),
            "BattleNetId" => Ok(BungieCredentialType::BattleNetId),
            "StadiaId" => Ok(BungieCredentialType::StadiaId),
            "TwitchId" => Ok(BungieCredentialType::TwitchId),
            "EgsId" => Ok(BungieCredentialType::EgsId),
            _ => Err(anyhow!("Could not deserialize string '{}' to BungieCredentialType", s)),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfContentItemPublicContract {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::content::ContentItemPublicContract>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfPostResponse {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::forum::PostResponse>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfGroupV2Card {
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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfGroupMember {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::groups_v2::GroupMember>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfGroupBan {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::groups_v2::GroupBan>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfGroupMemberApplication {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::groups_v2::GroupMemberApplication>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfGroupMembership {
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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfGroupPotentialMembership {
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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyVendorReceiptsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::profiles::DestinyVendorReceiptsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyInventoryComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::inventory::DestinyInventoryComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyProfileComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::profiles::DestinyProfileComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyPlatformSilverComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::inventory::DestinyPlatformSilverComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyKiosksComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::kiosks::DestinyKiosksComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyPlugSetsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::plug_sets::DestinyPlugSetsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyProfileProgressionComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::profiles::DestinyProfileProgressionComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyPresentationNodesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::presentation::DestinyPresentationNodesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyProfileRecordsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::records::DestinyProfileRecordsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyProfileCollectiblesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::collectibles::DestinyProfileCollectiblesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyProfileTransitoryComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::profiles::DestinyProfileTransitoryComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyMetricsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::metrics::DestinyMetricsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyStringVariablesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::string_variables::DestinyStringVariablesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::characters::DestinyCharacterComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyInventoryComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::inventory::DestinyInventoryComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterProgressionComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::characters::DestinyCharacterProgressionComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterRenderComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::characters::DestinyCharacterRenderComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::characters::DestinyCharacterActivitiesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyKiosksComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::kiosks::DestinyKiosksComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyPlugSetsComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::plug_sets::DestinyPlugSetsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyBaseItemComponentSetOfuint32 {
    #[serde(rename = "objectives")]
    pub objectives: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent>,

    #[serde(rename = "perks")]
    pub perks: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemObjectivesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemPerksComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyPresentationNodesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::presentation::DestinyPresentationNodesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterRecordsComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::records::DestinyCharacterRecordsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCollectiblesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::collectibles::DestinyCollectiblesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyStringVariablesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::string_variables::DestinyStringVariablesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCraftablesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::craftables::DestinyCraftablesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyBaseItemComponentSetOfint64 {
    #[serde(rename = "objectives")]
    pub objectives: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent>,

    #[serde(rename = "perks")]
    pub perks: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemPerksComponent>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::items::DestinyItemObjectivesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemPerksComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::items::DestinyItemPerksComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyItemComponentSetOfint64 {
    #[serde(rename = "instances")]
    pub instances: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemInstanceComponent>,

    #[serde(rename = "renderData")]
    pub render_data: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemRenderComponent>,

    #[serde(rename = "stats")]
    pub stats: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemStatsComponent>,

    #[serde(rename = "sockets")]
    pub sockets: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent>,

    #[serde(rename = "reusablePlugs")]
    pub reusable_plugs: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemReusablePlugsComponent>,

    #[serde(rename = "plugObjectives")]
    pub plug_objectives: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemPlugObjectivesComponent>,

    #[serde(rename = "talentGrids")]
    pub talent_grids: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemTalentGridComponent>,

    #[serde(rename = "plugStates")]
    pub plug_states: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent>,

    #[serde(rename = "objectives")]
    pub objectives: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemObjectivesComponent>,

    #[serde(rename = "perks")]
    pub perks: Option<crate::DictionaryComponentResponseOfint64AndDestinyItemPerksComponent>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemInstanceComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::items::DestinyItemInstanceComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemRenderComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::items::DestinyItemRenderComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemStatsComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::items::DestinyItemStatsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemSocketsComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::items::DestinyItemSocketsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemReusablePlugsComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::items::DestinyItemReusablePlugsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemPlugObjectivesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::items::DestinyItemPlugObjectivesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyItemTalentGridComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::entities::items::DestinyItemTalentGridComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::components::items::DestinyItemPlugComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint64AndDestinyCurrenciesComponent {
    #[serde_as(as = "Option<HashMap<DisplayFromStr, _>>")]
    #[serde(rename = "data")]
    pub data: Option<HashMap<i64, crate::destiny::components::inventory::DestinyCurrenciesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyCharacterComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::characters::DestinyCharacterComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyCharacterProgressionComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::characters::DestinyCharacterProgressionComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyCharacterRenderComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::characters::DestinyCharacterRenderComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyCharacterActivitiesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::characters::DestinyCharacterActivitiesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyCharacterRecordsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::records::DestinyCharacterRecordsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyCollectiblesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::collectibles::DestinyCollectiblesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyCurrenciesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::inventory::DestinyCurrenciesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemInstanceComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemInstanceComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemObjectivesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemObjectivesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemPerksComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemPerksComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemRenderComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemRenderComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemStatsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemStatsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemTalentGridComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemTalentGridComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemSocketsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::items::DestinyItemSocketsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemReusablePlugsComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::items::DestinyItemReusablePlugsComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyItemPlugObjectivesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::items::DestinyItemPlugObjectivesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyVendorGroupComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::components::vendors::DestinyVendorGroupComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyVendorComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::vendors::DestinyVendorComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyVendorCategoriesComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::vendors::DestinyVendorCategoriesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyVendorSaleItemSetComponentOfDestinyVendorSaleItemComponent {
    #[serde(rename = "saleItems")]
    pub sale_items: Option<HashMap<i32, crate::destiny::entities::vendors::DestinyVendorSaleItemComponent>>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndPersonalDestinyVendorSaleItemSetComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::responses::PersonalDestinyVendorSaleItemSetComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyBaseItemComponentSetOfint32 {
    #[serde(rename = "objectives")]
    pub objectives: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent>,

    #[serde(rename = "perks")]
    pub perks: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::items::DestinyItemObjectivesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemPerksComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::items::DestinyItemPerksComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyItemComponentSetOfint32 {
    #[serde(rename = "instances")]
    pub instances: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent>,

    #[serde(rename = "renderData")]
    pub render_data: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemRenderComponent>,

    #[serde(rename = "stats")]
    pub stats: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemStatsComponent>,

    #[serde(rename = "sockets")]
    pub sockets: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent>,

    #[serde(rename = "reusablePlugs")]
    pub reusable_plugs: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemReusablePlugsComponent>,

    #[serde(rename = "plugObjectives")]
    pub plug_objectives: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemPlugObjectivesComponent>,

    #[serde(rename = "talentGrids")]
    pub talent_grids: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent>,

    #[serde(rename = "plugStates")]
    pub plug_states: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent>,

    #[serde(rename = "objectives")]
    pub objectives: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent>,

    #[serde(rename = "perks")]
    pub perks: Option<crate::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::items::DestinyItemInstanceComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemRenderComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::items::DestinyItemRenderComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemStatsComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::items::DestinyItemStatsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::items::DestinyItemSocketsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemReusablePlugsComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::components::items::DestinyItemReusablePlugsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemPlugObjectivesComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::components::items::DestinyItemPlugObjectivesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::items::DestinyItemTalentGridComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyVendorComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::vendors::DestinyVendorComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SingleComponentResponseOfDestinyVendorCategoriesComponent {
    #[serde(rename = "data")]
    pub data: Option<crate::destiny::entities::vendors::DestinyVendorCategoriesComponent>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfint32AndDestinyVendorSaleItemComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<i32, crate::destiny::entities::vendors::DestinyVendorSaleItemComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyPublicVendorComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::components::vendors::DestinyPublicVendorComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyVendorSaleItemSetComponentOfDestinyPublicVendorSaleItemComponent {
    #[serde(rename = "saleItems")]
    pub sale_items: Option<HashMap<i32, crate::destiny::components::vendors::DestinyPublicVendorSaleItemComponent>>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndPublicDestinyVendorSaleItemSetComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::responses::PublicDestinyVendorSaleItemSetComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyItemComponentSetOfuint32 {
    #[serde(rename = "instances")]
    pub instances: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent>,

    #[serde(rename = "renderData")]
    pub render_data: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent>,

    #[serde(rename = "stats")]
    pub stats: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent>,

    #[serde(rename = "sockets")]
    pub sockets: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent>,

    #[serde(rename = "reusablePlugs")]
    pub reusable_plugs: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent>,

    #[serde(rename = "plugObjectives")]
    pub plug_objectives: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent>,

    #[serde(rename = "talentGrids")]
    pub talent_grids: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent>,

    #[serde(rename = "plugStates")]
    pub plug_states: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent>,

    #[serde(rename = "objectives")]
    pub objectives: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemObjectivesComponent>,

    #[serde(rename = "perks")]
    pub perks: Option<crate::DictionaryComponentResponseOfuint32AndDestinyItemPerksComponent>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemInstanceComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemInstanceComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemRenderComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemRenderComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemStatsComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemStatsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemSocketsComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemSocketsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemReusablePlugsComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::components::items::DestinyItemReusablePlugsComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemPlugObjectivesComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::components::items::DestinyItemPlugObjectivesComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DictionaryComponentResponseOfuint32AndDestinyItemTalentGridComponent {
    #[serde(rename = "data")]
    pub data: Option<HashMap<u32, crate::destiny::entities::items::DestinyItemTalentGridComponent>>,

    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfDestinyEntitySearchResultItem {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::destiny::definitions::DestinyEntitySearchResultItem>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfTrendingEntry {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::trending::TrendingEntry>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfFireteamSummary {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::fireteam::FireteamSummary>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SearchResultOfFireteamResponse {
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::fireteam::FireteamResponse>>,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct GlobalAlert {
    #[serde(rename = "AlertKey")]
    pub alert_key: Option<String>,

    #[serde(rename = "AlertHtml")]
    pub alert_html: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "AlertTimestamp")]
    pub alert_timestamp: OffsetDateTime,

    #[serde(rename = "AlertLink")]
    pub alert_link: Option<String>,

    #[serde(rename = "AlertLevel")]
    pub alert_level: crate::GlobalAlertLevel,

    #[serde(rename = "AlertType")]
    pub alert_type: crate::GlobalAlertType,

    #[serde(rename = "StreamInfo")]
    pub stream_info: Option<crate::StreamInfo>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GlobalAlertLevel {
    Unknown = 0,
    Blue = 1,
    Yellow = 2,
    Red = 3,
}

impl Display for GlobalAlertLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GlobalAlertLevel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Unknown" => Ok(GlobalAlertLevel::Unknown),
            "Blue" => Ok(GlobalAlertLevel::Blue),
            "Yellow" => Ok(GlobalAlertLevel::Yellow),
            "Red" => Ok(GlobalAlertLevel::Red),
            _ => Err(anyhow!("Could not deserialize string '{}' to GlobalAlertLevel", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GlobalAlertType {
    GlobalAlert = 0,
    StreamingAlert = 1,
}

impl Display for GlobalAlertType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for GlobalAlertType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "GlobalAlert" => Ok(GlobalAlertType::GlobalAlert),
            "StreamingAlert" => Ok(GlobalAlertType::StreamingAlert),
            _ => Err(anyhow!("Could not deserialize string '{}' to GlobalAlertType", s)),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct StreamInfo {
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,
}
