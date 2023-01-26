use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// For now, this isn't used for much: it's a record of the recent refundable purchases that the user has made. In the future, it could be used for providing refunds/buyback via the API. Wouldn't that be fun?
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyVendorReceiptsComponent {
    /// The receipts for refundable purchases made at a vendor.
    #[serde(rename = "receipts")]
    pub receipts: Option<Vec<crate::destiny::vendors::DestinyVendorReceipt>>,
}

/// The most essential summary information about a Profile (in Destiny 1, we called these "Accounts").
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyProfileComponent {
    /// If you need to render the Profile (their platform name, icon, etc...) somewhere, this property contains that information.
    #[serde(rename = "userInfo")]
    pub user_info: Option<crate::user::UserInfoCard>,

    /// The last time the user played with any character on this Profile.
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "dateLastPlayed")]
    pub date_last_played: OffsetDateTime,

    /// If you want to know what expansions they own, this will contain that data.
    /// IMPORTANT: This field may not return the data you're interested in for Cross-Saved users. It returns the last ownership data we saw for this account - which is to say, what they've purchased on the platform on which they last played, which now could be a different platform.
    /// If you don't care about per-platform ownership and only care about whatever platform it seems they are playing on most recently, then this should be "good enough." Otherwise, this should be considered deprecated. We do not have a good alternative to provide at this time with platform specific ownership data for DLC.
    #[serde(rename = "versionsOwned")]
    pub versions_owned: enumflags2::BitFlags<crate::destiny::DestinyGameVersions>,

    /// A list of the character IDs, for further querying on your part.
    #[serde(rename = "characterIds")]
    pub character_ids: Option<Vec<i64>>,

    /// A list of seasons that this profile owns. Unlike versionsOwned, these stay with the profile across Platforms, and thus will be valid.
    /// It turns out that Stadia Pro subscriptions will give access to seasons but only while playing on Stadia and with an active subscription. So some users (users who have Stadia Pro but choose to play on some other platform) won't see these as available: it will be whatever seasons are available for the platform on which they last played.
    #[serde(rename = "seasonHashes")]
    pub season_hashes: Option<Vec<u32>>,

    /// A list of hashes for event cards that a profile owns. Unlike most values in versionsOwned, these stay with the profile across all platforms.
    #[serde(rename = "eventCardHashesOwned")]
    pub event_card_hashes_owned: Option<Vec<u32>>,

    /// If populated, this is a reference to the season that is currently active.
    #[serde(rename = "currentSeasonHash")]
    pub current_season_hash: Option<u32>,

    /// If populated, this is the reward power cap for the current season.
    #[serde(rename = "currentSeasonRewardPowerCap")]
    pub current_season_reward_power_cap: Option<i32>,

    /// If populated, this is a reference to the event card that is currently active.
    #[serde(rename = "activeEventCardHash")]
    pub active_event_card_hash: Option<u32>,
}
