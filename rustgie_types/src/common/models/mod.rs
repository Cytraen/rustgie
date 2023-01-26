use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CoreSettingsConfiguration {
    #[serde(rename = "environment")]
    pub environment: Option<String>,

    #[serde(rename = "systems")]
    pub systems: Option<HashMap<String, crate::common::models::CoreSystem>>,

    #[serde(rename = "ignoreReasons")]
    pub ignore_reasons: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "forumCategories")]
    pub forum_categories: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "groupAvatars")]
    pub group_avatars: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "defaultGroupTheme")]
    pub default_group_theme: Option<crate::common::models::CoreSetting>,

    #[serde(rename = "destinyMembershipTypes")]
    pub destiny_membership_types: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "recruitmentPlatformTags")]
    pub recruitment_platform_tags: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "recruitmentMiscTags")]
    pub recruitment_misc_tags: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "recruitmentActivities")]
    pub recruitment_activities: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "userContentLocales")]
    pub user_content_locales: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "systemContentLocales")]
    pub system_content_locales: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "clanBannerDecals")]
    pub clan_banner_decals: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "clanBannerDecalColors")]
    pub clan_banner_decal_colors: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "clanBannerGonfalons")]
    pub clan_banner_gonfalons: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "clanBannerGonfalonColors")]
    pub clan_banner_gonfalon_colors: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "clanBannerGonfalonDetails")]
    pub clan_banner_gonfalon_details: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "clanBannerGonfalonDetailColors")]
    pub clan_banner_gonfalon_detail_colors: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "clanBannerStandards")]
    pub clan_banner_standards: Option<Vec<crate::common::models::CoreSetting>>,

    #[serde(rename = "destiny2CoreSettings")]
    pub destiny2_core_settings: Option<crate::common::models::Destiny2CoreSettings>,

    #[serde(rename = "emailSettings")]
    pub email_settings: Option<crate::user::EmailSettings>,

    #[serde(rename = "fireteamActivities")]
    pub fireteam_activities: Option<Vec<crate::common::models::CoreSetting>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CoreSystem {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "parameters")]
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CoreSetting {
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    #[serde(rename = "isDefault")]
    pub is_default: bool,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "summary")]
    pub summary: Option<String>,

    #[serde(rename = "imagePath")]
    pub image_path: Option<String>,

    #[serde(rename = "childSettings")]
    pub child_settings: Option<Vec<crate::common::models::CoreSetting>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Destiny2CoreSettings {
    #[serde(rename = "collectionRootNode")]
    pub collection_root_node: u32,

    #[serde(rename = "badgesRootNode")]
    pub badges_root_node: u32,

    #[serde(rename = "recordsRootNode")]
    pub records_root_node: u32,

    #[serde(rename = "medalsRootNode")]
    pub medals_root_node: u32,

    #[serde(rename = "metricsRootNode")]
    pub metrics_root_node: u32,

    #[serde(rename = "activeTriumphsRootNodeHash")]
    pub active_triumphs_root_node_hash: u32,

    #[serde(rename = "activeSealsRootNodeHash")]
    pub active_seals_root_node_hash: u32,

    #[serde(rename = "legacyTriumphsRootNodeHash")]
    pub legacy_triumphs_root_node_hash: u32,

    #[serde(rename = "legacySealsRootNodeHash")]
    pub legacy_seals_root_node_hash: u32,

    #[serde(rename = "medalsRootNodeHash")]
    pub medals_root_node_hash: u32,

    #[serde(rename = "exoticCatalystsRootNodeHash")]
    pub exotic_catalysts_root_node_hash: u32,

    #[serde(rename = "loreRootNodeHash")]
    pub lore_root_node_hash: u32,

    #[serde(rename = "craftingRootNodeHash")]
    pub crafting_root_node_hash: u32,

    #[serde(rename = "currentRankProgressionHashes")]
    pub current_rank_progression_hashes: Option<Vec<u32>>,

    #[serde(rename = "insertPlugFreeProtectedPlugItemHashes")]
    pub insert_plug_free_protected_plug_item_hashes: Option<Vec<u32>>,

    #[serde(rename = "insertPlugFreeBlockedSocketTypeHashes")]
    pub insert_plug_free_blocked_socket_type_hashes: Option<Vec<u32>>,

    #[serde(rename = "undiscoveredCollectibleImage")]
    pub undiscovered_collectible_image: Option<String>,

    #[serde(rename = "ammoTypeHeavyIcon")]
    pub ammo_type_heavy_icon: Option<String>,

    #[serde(rename = "ammoTypeSpecialIcon")]
    pub ammo_type_special_icon: Option<String>,

    #[serde(rename = "ammoTypePrimaryIcon")]
    pub ammo_type_primary_icon: Option<String>,

    #[serde(rename = "currentSeasonalArtifactHash")]
    pub current_seasonal_artifact_hash: u32,

    #[serde(rename = "currentSeasonHash")]
    pub current_season_hash: Option<u32>,

    #[serde(rename = "seasonalChallengesPresentationNodeHash")]
    pub seasonal_challenges_presentation_node_hash: Option<u32>,

    #[serde(rename = "futureSeasonHashes")]
    pub future_season_hashes: Option<Vec<u32>>,

    #[serde(rename = "pastSeasonHashes")]
    pub past_season_hashes: Option<Vec<u32>>,
}
