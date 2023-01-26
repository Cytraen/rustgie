use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PartnerOfferClaimRequest {
    #[serde(rename = "PartnerOfferId")]
    pub partner_offer_id: Option<String>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "BungieNetMembershipId")]
    pub bungie_net_membership_id: i64,

    #[serde(rename = "TransactionId")]
    pub transaction_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PartnerOfferSkuHistoryResponse {
    #[serde(rename = "SkuIdentifier")]
    pub sku_identifier: Option<String>,

    #[serde(rename = "LocalizedName")]
    pub localized_name: Option<String>,

    #[serde(rename = "LocalizedDescription")]
    pub localized_description: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "ClaimDate")]
    pub claim_date: OffsetDateTime,

    #[serde(rename = "AllOffersApplied")]
    pub all_offers_applied: bool,

    #[serde(rename = "TransactionId")]
    pub transaction_id: Option<String>,

    #[serde(rename = "SkuOffers")]
    pub sku_offers: Option<Vec<crate::tokens::PartnerOfferHistoryResponse>>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PartnerOfferHistoryResponse {
    #[serde(rename = "PartnerOfferKey")]
    pub partner_offer_key: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "MembershipId")]
    pub membership_id: Option<i64>,

    #[serde(rename = "MembershipType")]
    pub membership_type: Option<i32>,

    #[serde(rename = "LocalizedName")]
    pub localized_name: Option<String>,

    #[serde(rename = "LocalizedDescription")]
    pub localized_description: Option<String>,

    #[serde(rename = "IsConsumable")]
    pub is_consumable: bool,

    #[serde(rename = "QuantityApplied")]
    pub quantity_applied: i32,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "ApplyDate")]
    pub apply_date: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PartnerRewardHistoryResponse {
    #[serde(rename = "PartnerOffers")]
    pub partner_offers: Option<Vec<crate::tokens::PartnerOfferSkuHistoryResponse>>,

    #[serde(rename = "TwitchDrops")]
    pub twitch_drops: Option<Vec<crate::tokens::TwitchDropHistoryResponse>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TwitchDropHistoryResponse {
    #[serde(rename = "Title")]
    pub title: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<OffsetDateTime>,

    #[serde(rename = "ClaimState")]
    pub claim_state: Option<u8>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct BungieRewardDisplay {
    #[serde(rename = "UserRewardAvailabilityModel")]
    pub user_reward_availability_model: Option<crate::tokens::UserRewardAvailabilityModel>,

    #[serde(rename = "ObjectiveDisplayProperties")]
    pub objective_display_properties: Option<crate::tokens::RewardDisplayProperties>,

    #[serde(rename = "RewardDisplayProperties")]
    pub reward_display_properties: Option<crate::tokens::RewardDisplayProperties>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserRewardAvailabilityModel {
    #[serde(rename = "AvailabilityModel")]
    pub availability_model: Option<crate::tokens::RewardAvailabilityModel>,

    #[serde(rename = "IsAvailableForUser")]
    pub is_available_for_user: bool,

    #[serde(rename = "IsUnlockedForUser")]
    pub is_unlocked_for_user: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RewardAvailabilityModel {
    #[serde(rename = "HasExistingCode")]
    pub has_existing_code: bool,

    #[serde(rename = "RecordDefinitions")]
    pub record_definitions: Option<Vec<crate::destiny::definitions::records::DestinyRecordDefinition>>,

    #[serde(rename = "CollectibleDefinitions")]
    pub collectible_definitions: Option<Vec<crate::tokens::CollectibleDefinitions>>,

    #[serde(rename = "IsOffer")]
    pub is_offer: bool,

    #[serde(rename = "HasOffer")]
    pub has_offer: bool,

    #[serde(rename = "OfferApplied")]
    pub offer_applied: bool,

    #[serde(rename = "DecryptedToken")]
    pub decrypted_token: Option<String>,

    #[serde(rename = "IsLoyaltyReward")]
    pub is_loyalty_reward: bool,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "ShopifyEndDate")]
    pub shopify_end_date: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "GameEarnByDate")]
    pub game_earn_by_date: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "RedemptionEndDate")]
    pub redemption_end_date: OffsetDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CollectibleDefinitions {
    #[serde(rename = "CollectibleDefinition")]
    pub collectible_definition: Option<crate::destiny::definitions::collectibles::DestinyCollectibleDefinition>,

    #[serde(rename = "DestinyInventoryItemDefinition")]
    pub destiny_inventory_item_definition: Option<crate::destiny::definitions::DestinyInventoryItemDefinition>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RewardDisplayProperties {
    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "ImagePath")]
    pub image_path: Option<String>,
}
