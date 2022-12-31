#![allow(clippy::too_many_arguments)]

use anyhow::{Result, Context};
use reqwest::Url;
use std::collections::HashMap;

impl crate::RustgieClient {
    pub async fn get_available_locales(&self, access_token: Option<&str>) -> Result<HashMap<String, String>> {
        self.bungie_api_get::<HashMap<String, String>>(
            Url::parse("https://www.bungie.net/Platform/GetAvailableLocales/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn get_common_settings(&self, access_token: Option<&str>) -> Result<rustgie_types::common::models::CoreSettingsConfiguration> {
        self.bungie_api_get::<rustgie_types::common::models::CoreSettingsConfiguration>(
            Url::parse("https://www.bungie.net/Platform/Settings/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn get_global_alerts(&self, includestreaming: Option<bool>, access_token: Option<&str>) -> Result<Vec<rustgie_types::GlobalAlert>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match includestreaming {
            None => {}
            Some(val) => { query_params.push(("includestreaming", val.to_string())); }
        }
        self.bungie_api_get::<Vec<rustgie_types::GlobalAlert>>(
            Url::parse_with_params("https://www.bungie.net/Platform/GlobalAlerts/", query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn get_user_system_overrides(&self, access_token: Option<&str>) -> Result<HashMap<String, rustgie_types::common::models::CoreSystem>> {
        self.bungie_api_get::<HashMap<String, rustgie_types::common::models::CoreSystem>>(
            Url::parse("https://www.bungie.net/Platform/UserSystemOverrides/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn app_get_application_api_usage(&self, application_id: i32, end: Option<time::OffsetDateTime>, start: Option<time::OffsetDateTime>, access_token: Option<&str>) -> Result<rustgie_types::applications::ApiUsage> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match end {
            None => {}
            Some(val) => { query_params.push(("end", val.to_string())); }
        }
        match start {
            None => {}
            Some(val) => { query_params.push(("start", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::applications::ApiUsage>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/App/ApiUsage/{application_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn app_get_bungie_applications(&self, access_token: Option<&str>) -> Result<Vec<rustgie_types::applications::Application>> {
        self.bungie_api_get::<Vec<rustgie_types::applications::Application>>(
            Url::parse("https://www.bungie.net/Platform/App/FirstParty/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn community_content_get_community_content(&self, media_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, sort: rustgie_types::forum::CommunityContentSortMode, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/CommunityContent/Get/{sort}/{media_filter}/{page}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn content_get_content_by_id(&self, id: i64, locale: &str, head: Option<bool>, access_token: Option<&str>) -> Result<rustgie_types::content::ContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match head {
            None => {}
            Some(val) => { query_params.push(("head", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::content::ContentItemPublicContract>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Content/GetContentById/{id}/{locale}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn content_get_content_by_tag_and_type(&self, locale: &str, tag: &str, r#type: &str, head: Option<bool>, access_token: Option<&str>) -> Result<rustgie_types::content::ContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match head {
            None => {}
            Some(val) => { query_params.push(("head", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::content::ContentItemPublicContract>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Content/GetContentByTagAndType/{tag}/{type}/{locale}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn content_get_content_type(&self, r#type: &str, access_token: Option<&str>) -> Result<rustgie_types::content::models::ContentTypeDescription> {
        self.bungie_api_get::<rustgie_types::content::models::ContentTypeDescription>(
            Url::parse(&format!("https://www.bungie.net/Platform/Content/GetContentType/{type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn content_rss_news_articles(&self, page_token: &str, categoryfilter: Option<&str>, includebody: Option<bool>, access_token: Option<&str>) -> Result<rustgie_types::content::NewsArticleRssResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match categoryfilter {
            None => {}
            Some(val) => { query_params.push(("categoryfilter", val.to_string())); }
        }
        match includebody {
            None => {}
            Some(val) => { query_params.push(("includebody", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::content::NewsArticleRssResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Content/Rss/NewsArticles/{page_token}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn content_search_content_by_tag_and_type(&self, locale: &str, tag: &str, r#type: &str, currentpage: Option<i32>, head: Option<bool>, itemsperpage: Option<i32>, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match currentpage {
            None => {}
            Some(val) => { query_params.push(("currentpage", val.to_string())); }
        }
        match head {
            None => {}
            Some(val) => { query_params.push(("head", val.to_string())); }
        }
        match itemsperpage {
            None => {}
            Some(val) => { query_params.push(("itemsperpage", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::SearchResultOfContentItemPublicContract>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Content/SearchContentByTagAndType/{tag}/{type}/{locale}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn content_search_content_with_text(&self, locale: &str, ctype: Option<&str>, currentpage: Option<i32>, head: Option<bool>, searchtext: Option<&str>, source: Option<&str>, tag: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match ctype {
            None => {}
            Some(val) => { query_params.push(("ctype", val.to_string())); }
        }
        match currentpage {
            None => {}
            Some(val) => { query_params.push(("currentpage", val.to_string())); }
        }
        match head {
            None => {}
            Some(val) => { query_params.push(("head", val.to_string())); }
        }
        match searchtext {
            None => {}
            Some(val) => { query_params.push(("searchtext", val.to_string())); }
        }
        match source {
            None => {}
            Some(val) => { query_params.push(("source", val.to_string())); }
        }
        match tag {
            None => {}
            Some(val) => { query_params.push(("tag", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::SearchResultOfContentItemPublicContract>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Content/Search/{locale}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn content_search_help_articles(&self, searchtext: &str, size: &str, access_token: Option<&str>) -> Result<rustgie_types::destiny::definitions::DestinyDefinition> {
        self.bungie_api_get::<rustgie_types::destiny::definitions::DestinyDefinition>(
            Url::parse(&format!("https://www.bungie.net/Platform/Content/SearchHelpArticles/{searchtext}/{size}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_awa_get_action_token(&self, correlation_id: &str, access_token: Option<&str>) -> Result<rustgie_types::destiny::advanced::AwaAuthorizationResult> {
        self.bungie_api_get::<rustgie_types::destiny::advanced::AwaAuthorizationResult>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/Awa/GetActionToken/{correlation_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_awa_initialize_request(&self, request_body: rustgie_types::destiny::advanced::AwaPermissionRequested, access_token: Option<&str>) -> Result<rustgie_types::destiny::advanced::AwaInitializeResponse> {
        self.bungie_api_post_with_body::<rustgie_types::destiny::advanced::AwaInitializeResponse, rustgie_types::destiny::advanced::AwaPermissionRequested>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/Initialize/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_awa_provide_authorization_result(&self, request_body: rustgie_types::destiny::advanced::AwaUserResponse, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::destiny::advanced::AwaUserResponse>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/AwaProvideAuthorizationResult/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_equip_item(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemActionRequest, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemActionRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItem/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_equip_items(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemSetActionRequest, access_token: Option<&str>) -> Result<rustgie_types::destiny::DestinyEquipItemResults> {
        self.bungie_api_post_with_body::<rustgie_types::destiny::DestinyEquipItemResults, rustgie_types::destiny::requests::actions::DestinyItemSetActionRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItems/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_get_activity_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, count: Option<i32>, mode: Option<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>, page: Option<i32>, access_token: Option<&str>) -> Result<rustgie_types::destiny::historical_stats::DestinyActivityHistoryResults> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match count {
            None => {}
            Some(val) => { query_params.push(("count", val.to_string())); }
        }
        match mode {
            None => {}
            Some(val) => { query_params.push(("mode", val.to_string())); }
        }
        match page {
            None => {}
            Some(val) => { query_params.push(("page", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyActivityHistoryResults>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/Activities/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyCharacterResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyCharacterResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_clan_aggregate_stats(&self, group_id: i64, modes: Option<&str>, access_token: Option<&str>) -> Result<Vec<rustgie_types::destiny::historical_stats::DestinyClanAggregateStat>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match modes {
            None => {}
            Some(val) => { query_params.push(("modes", val.to_string())); }
        }
        self.bungie_api_get::<Vec<rustgie_types::destiny::historical_stats::DestinyClanAggregateStat>>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/Stats/AggregateClanStats/{group_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_clan_banner_source(&self, access_token: Option<&str>) -> Result<rustgie_types::config::clan_banner::ClanBannerSource> {
        self.bungie_api_get::<rustgie_types::config::clan_banner::ClanBannerSource>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Clan/ClanBannerDictionary/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_clan_leaderboards(&self, group_id: i64, maxtop: Option<i32>, modes: Option<&str>, statid: Option<&str>, access_token: Option<&str>) -> Result<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match maxtop {
            None => {}
            Some(val) => { query_params.push(("maxtop", val.to_string())); }
        }
        match modes {
            None => {}
            Some(val) => { query_params.push(("modes", val.to_string())); }
        }
        match statid {
            None => {}
            Some(val) => { query_params.push(("statid", val.to_string())); }
        }
        self.bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/Clans/{group_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_clan_weekly_reward_state(&self, group_id: i64, access_token: Option<&str>) -> Result<rustgie_types::destiny::milestones::DestinyMilestone> {
        self.bungie_api_get::<rustgie_types::destiny::milestones::DestinyMilestone>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/Clan/{group_id}/WeeklyRewardState/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_collectible_node_details(&self, character_id: i64, collectible_presentation_node_hash: u32, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyCollectibleNodeDetailResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyCollectibleNodeDetailResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Collectibles/{collectible_presentation_node_hash}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_destiny_aggregate_activity_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::destiny::historical_stats::DestinyAggregateActivityResults> {
        self.bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyAggregateActivityResults>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/AggregateActivityStats/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_destiny_entity_definition(&self, entity_type: &str, hash_identifier: u32, access_token: Option<&str>) -> Result<rustgie_types::destiny::definitions::DestinyDefinition> {
        self.bungie_api_get::<rustgie_types::destiny::definitions::DestinyDefinition>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/Manifest/{entity_type}/{hash_identifier}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_destiny_manifest(&self, access_token: Option<&str>) -> Result<rustgie_types::destiny::config::DestinyManifest> {
        self.bungie_api_get::<rustgie_types::destiny::config::DestinyManifest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Manifest/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_historical_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, dayend: Option<time::OffsetDateTime>, daystart: Option<time::OffsetDateTime>, groups: Option<Vec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, modes: Option<Vec<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>>, period_type: Option<rustgie_types::destiny::historical_stats::definitions::PeriodType>, access_token: Option<&str>) -> Result<HashMap<String, rustgie_types::destiny::historical_stats::DestinyHistoricalStatsByPeriod>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match dayend {
            None => {}
            Some(val) => { query_params.push(("dayend", val.to_string())); }
        }
        match daystart {
            None => {}
            Some(val) => { query_params.push(("daystart", val.to_string())); }
        }
        match groups {
            None => {}
            Some(val) => { query_params.push(("groups", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match modes {
            None => {}
            Some(val) => { query_params.push(("modes", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match period_type {
            None => {}
            Some(val) => { query_params.push(("periodType", val.to_string())); }
        }
        self.bungie_api_get::<HashMap<String, rustgie_types::destiny::historical_stats::DestinyHistoricalStatsByPeriod>>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_historical_stats_definition(&self, access_token: Option<&str>) -> Result<HashMap<String, rustgie_types::destiny::historical_stats::definitions::DestinyHistoricalStatsDefinition>> {
        self.bungie_api_get::<HashMap<String, rustgie_types::destiny::historical_stats::definitions::DestinyHistoricalStatsDefinition>>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Stats/Definition/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_historical_stats_for_account(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, groups: Option<Vec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, access_token: Option<&str>) -> Result<rustgie_types::destiny::historical_stats::DestinyHistoricalStatsAccountResult> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match groups {
            None => {}
            Some(val) => { query_params.push(("groups", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        self.bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyHistoricalStatsAccountResult>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Stats/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_item(&self, destiny_membership_id: i64, item_instance_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyItemResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyItemResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Item/{item_instance_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_leaderboards(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<&str>, statid: Option<&str>, access_token: Option<&str>) -> Result<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match maxtop {
            None => {}
            Some(val) => { query_params.push(("maxtop", val.to_string())); }
        }
        match modes {
            None => {}
            Some(val) => { query_params.push(("modes", val.to_string())); }
        }
        match statid {
            None => {}
            Some(val) => { query_params.push(("statid", val.to_string())); }
        }
        self.bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Stats/Leaderboards/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_leaderboards_for_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<&str>, statid: Option<&str>, access_token: Option<&str>) -> Result<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match maxtop {
            None => {}
            Some(val) => { query_params.push(("maxtop", val.to_string())); }
        }
        match modes {
            None => {}
            Some(val) => { query_params.push(("modes", val.to_string())); }
        }
        match statid {
            None => {}
            Some(val) => { query_params.push(("statid", val.to_string())); }
        }
        self.bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/{membership_type}/{destiny_membership_id}/{character_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_linked_profiles(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, get_all_memberships: Option<bool>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyLinkedProfilesResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match get_all_memberships {
            None => {}
            Some(val) => { query_params.push(("getAllMemberships", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyLinkedProfilesResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{membership_id}/LinkedProfiles/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_post_game_carnage_report(&self, activity_id: i64, access_token: Option<&str>) -> Result<rustgie_types::destiny::historical_stats::DestinyPostGameCarnageReportData> {
        self.bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyPostGameCarnageReportData>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activity_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_profile(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyProfileResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyProfileResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_public_milestone_content(&self, milestone_hash: u32, access_token: Option<&str>) -> Result<rustgie_types::destiny::milestones::DestinyMilestoneContent> {
        self.bungie_api_get::<rustgie_types::destiny::milestones::DestinyMilestoneContent>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/Milestones/{milestone_hash}/Content/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_public_milestones(&self, access_token: Option<&str>) -> Result<HashMap<u32, rustgie_types::destiny::milestones::DestinyPublicMilestone>> {
        self.bungie_api_get::<HashMap<u32, rustgie_types::destiny::milestones::DestinyPublicMilestone>>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Milestones/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_public_vendors(&self, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyPublicVendorsResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyPublicVendorsResponse>(
            Url::parse_with_params("https://www.bungie.net/Platform/Destiny2/Vendors/", query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_unique_weapon_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::destiny::historical_stats::DestinyHistoricalWeaponStatsData> {
        self.bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyHistoricalWeaponStatsData>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/UniqueWeapons/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_vendor(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, vendor_hash: u32, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyVendorResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyVendorResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Vendors/{vendor_hash}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_get_vendors(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, filter: Option<rustgie_types::destiny::DestinyVendorFilter>, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyVendorsResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match filter {
            None => {}
            Some(val) => { query_params.push(("filter", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::destiny::responses::DestinyVendorsResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Vendors/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_insert_socket_plug(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsActionRequest, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyItemChangeResponse> {
        self.bungie_api_post_with_body::<rustgie_types::destiny::responses::DestinyItemChangeResponse, rustgie_types::destiny::requests::actions::DestinyInsertPlugsActionRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlug/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_insert_socket_plug_free(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsFreeActionRequest, access_token: Option<&str>) -> Result<rustgie_types::destiny::responses::DestinyItemChangeResponse> {
        self.bungie_api_post_with_body::<rustgie_types::destiny::responses::DestinyItemChangeResponse, rustgie_types::destiny::requests::actions::DestinyInsertPlugsFreeActionRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlugFree/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_pull_from_postmaster(&self, request_body: rustgie_types::destiny::requests::actions::DestinyPostmasterTransferRequest, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyPostmasterTransferRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/PullFromPostmaster/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_report_offensive_post_game_carnage_report_player(&self, activity_id: i64, request_body: rustgie_types::destiny::reporting::requests::DestinyReportOffensePgcrRequest, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::destiny::reporting::requests::DestinyReportOffensePgcrRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activity_id}/Report/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_search_destiny_entities(&self, search_term: &str, r#type: &str, page: Option<i32>, access_token: Option<&str>) -> Result<rustgie_types::destiny::definitions::DestinyEntitySearchResult> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match page {
            None => {}
            Some(val) => { query_params.push(("page", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::destiny::definitions::DestinyEntitySearchResult>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Destiny2/Armory/Search/{type}/{search_term}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn destiny2_search_destiny_player_by_bungie_name(&self, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::user::ExactSearchRequest, access_token: Option<&str>) -> Result<Vec<rustgie_types::user::UserInfoCard>> {
        self.bungie_api_post_with_body::<Vec<rustgie_types::user::UserInfoCard>, rustgie_types::user::ExactSearchRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayerByBungieName/{membership_type}/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_set_item_lock_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemStateRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetLockState/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_set_quest_tracked_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemStateRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetTrackedState/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn destiny2_transfer_item(&self, request_body: rustgie_types::destiny::requests::DestinyItemTransferRequest, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::DestinyItemTransferRequest>(
            Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/TransferItem/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn fireteam_get_active_private_clan_fireteam_count(&self, group_id: i64, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_get::<i32>(
            Url::parse(&format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/ActiveCount/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn fireteam_get_available_clan_fireteams(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, group_id: i64, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, public_only: rustgie_types::fireteam::FireteamPublicSearchOption, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfFireteamSummary> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match lang_filter {
            None => {}
            Some(val) => { query_params.push(("langFilter", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::SearchResultOfFireteamSummary>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/Available/{platform}/{activity_type}/{date_range}/{slot_filter}/{public_only}/{page}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn fireteam_get_clan_fireteam(&self, fireteam_id: i64, group_id: i64, access_token: Option<&str>) -> Result<rustgie_types::fireteam::FireteamResponse> {
        self.bungie_api_get::<rustgie_types::fireteam::FireteamResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/Summary/{fireteam_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn fireteam_get_my_clan_fireteams(&self, group_id: i64, include_closed: bool, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, group_filter: Option<bool>, lang_filter: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfFireteamResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match group_filter {
            None => {}
            Some(val) => { query_params.push(("groupFilter", val.to_string())); }
        }
        match lang_filter {
            None => {}
            Some(val) => { query_params.push(("langFilter", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::SearchResultOfFireteamResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/My/{platform}/{include_closed}/{page}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn fireteam_search_public_available_clan_fireteams(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfFireteamSummary> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match lang_filter {
            None => {}
            Some(val) => { query_params.push(("langFilter", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::SearchResultOfFireteamSummary>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Fireteam/Search/Available/{platform}/{activity_type}/{date_range}/{slot_filter}/{page}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_core_topics_paged(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match locales {
            None => {}
            Some(val) => { query_params.push(("locales", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Forum/GetCoreTopicsPaged/{page}/{sort}/{quick_date}/{category_filter}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_forum_tag_suggestions(&self, partialtag: Option<&str>, access_token: Option<&str>) -> Result<Vec<rustgie_types::tags::models::contracts::TagResponse>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match partialtag {
            None => {}
            Some(val) => { query_params.push(("partialtag", val.to_string())); }
        }
        self.bungie_api_get::<Vec<rustgie_types::tags::models::contracts::TagResponse>>(
            Url::parse_with_params("https://www.bungie.net/Platform/Forum/GetForumTagSuggestions/", query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_poll(&self, topic_id: i64, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/Forum/Poll/{topic_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_post_and_parent(&self, child_post_id: i64, showbanned: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Forum/GetPostAndParent/{child_post_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_post_and_parent_awaiting_approval(&self, child_post_id: i64, showbanned: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Forum/GetPostAndParentAwaitingApproval/{child_post_id}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_posts_threaded_paged(&self, get_parent_post: bool, page: i32, page_size: i32, parent_post_id: i64, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPaged/{parent_post_id}/{page}/{page_size}/{reply_size}/{get_parent_post}/{root_thread_mode}/{sort_mode}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_posts_threaded_paged_from_child(&self, child_post_id: i64, page: i32, page_size: i32, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPagedFromChild/{child_post_id}/{page}/{page_size}/{reply_size}/{root_thread_mode}/{sort_mode}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_recruitment_thread_summaries(&self, request_body: Vec<i64>, access_token: Option<&str>) -> Result<Vec<rustgie_types::forum::ForumRecruitmentDetail>> {
        self.bungie_api_post_with_body::<Vec<rustgie_types::forum::ForumRecruitmentDetail>, Vec<i64>>(
            Url::parse("https://www.bungie.net/Platform/Forum/Recruit/Summaries/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn forum_get_topic_for_content(&self, content_id: i64, access_token: Option<&str>) -> Result<i64> {
        self.bungie_api_get::<i64>(
            Url::parse(&format!("https://www.bungie.net/Platform/Forum/GetTopicForContent/{content_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn forum_get_topics_paged(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, group: i64, page: i32, page_size: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<&str>, tagstring: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match locales {
            None => {}
            Some(val) => { query_params.push(("locales", val.to_string())); }
        }
        match tagstring {
            None => {}
            Some(val) => { query_params.push(("tagstring", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::forum::PostSearchResponse>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/Forum/GetTopicsPaged/{page}/{page_size}/{group}/{sort}/{quick_date}/{category_filter}/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_abdicate_foundership(&self, founder_id_new: i64, group_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Admin/AbdicateFoundership/{membership_type}/{founder_id_new}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_add_optional_conversation(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationAddRequest, access_token: Option<&str>) -> Result<i64> {
        self.bungie_api_post_with_body::<i64, rustgie_types::groups_v2::GroupOptionalConversationAddRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/Add/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_approve_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> Result<Vec<rustgie_types::entities::EntityActionResult>> {
        self.bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/ApproveAll/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_approve_pending(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post_with_body::<bool, rustgie_types::groups_v2::GroupApplicationRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/Approve/{membership_type}/{membership_id}/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_approve_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<&str>) -> Result<Vec<rustgie_types::entities::EntityActionResult>> {
        self.bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationListRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/ApproveList/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_ban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupBanRequest, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupBanRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Ban/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_deny_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> Result<Vec<rustgie_types::entities::EntityActionResult>> {
        self.bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/DenyAll/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_deny_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<&str>) -> Result<Vec<rustgie_types::entities::EntityActionResult>> {
        self.bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationListRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/DenyList/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_edit_clan_banner(&self, group_id: i64, request_body: rustgie_types::groups_v2::ClanBanner, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::groups_v2::ClanBanner>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/EditClanBanner/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_edit_founder_options(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionsEditAction, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupOptionsEditAction>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/EditFounderOptions/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_edit_group(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupEditAction, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupEditAction>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Edit/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_edit_group_membership(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, member_type: rustgie_types::groups_v2::RuntimeGroupMemberType, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post::<i32>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/SetMembershipType/{member_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_edit_optional_conversation(&self, conversation_id: i64, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationEditRequest, access_token: Option<&str>) -> Result<i64> {
        self.bungie_api_post_with_body::<i64, rustgie_types::groups_v2::GroupOptionalConversationEditRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/Edit/{conversation_id}/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_get_admins_and_founder_of_group(&self, _currentpage: i32, group_id: i64, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfGroupMember> {
        self.bungie_api_get::<rustgie_types::SearchResultOfGroupMember>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/AdminsAndFounder/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_available_avatars(&self, access_token: Option<&str>) -> Result<HashMap<i32, String>> {
        self.bungie_api_get::<HashMap<i32, String>>(
            Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableAvatars/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_available_themes(&self, access_token: Option<&str>) -> Result<Vec<rustgie_types::config::GroupTheme>> {
        self.bungie_api_get::<Vec<rustgie_types::config::GroupTheme>>(
            Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableThemes/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_banned_members_of_group(&self, _currentpage: i32, group_id: i64, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfGroupBan> {
        self.bungie_api_get::<rustgie_types::SearchResultOfGroupBan>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Banned/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_group(&self, group_id: i64, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupResponse> {
        self.bungie_api_get::<rustgie_types::groups_v2::GroupResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_group_by_name(&self, group_name: &str, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupResponse> {
        self.bungie_api_get::<rustgie_types::groups_v2::GroupResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/Name/{group_name}/{group_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_group_by_name_v2(&self, request_body: rustgie_types::groups_v2::GroupNameSearchRequest, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupResponse> {
        self.bungie_api_post_with_body::<rustgie_types::groups_v2::GroupResponse, rustgie_types::groups_v2::GroupNameSearchRequest>(
            Url::parse("https://www.bungie.net/Platform/GroupV2/NameV2/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_get_group_optional_conversations(&self, group_id: i64, access_token: Option<&str>) -> Result<Vec<rustgie_types::groups_v2::GroupOptionalConversation>> {
        self.bungie_api_get::<Vec<rustgie_types::groups_v2::GroupOptionalConversation>>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupsForMemberFilter, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GetGroupsForMemberResponse> {
        self.bungie_api_get::<rustgie_types::groups_v2::GetGroupsForMemberResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/User/{membership_type}/{membership_id}/{filter}/{group_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_invited_individuals(&self, _currentpage: i32, group_id: i64, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfGroupMemberApplication> {
        self.bungie_api_get::<rustgie_types::SearchResultOfGroupMemberApplication>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/InvitedIndividuals/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_members_of_group(&self, _currentpage: i32, group_id: i64, member_type: Option<rustgie_types::groups_v2::RuntimeGroupMemberType>, name_search: Option<&str>, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfGroupMember> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match member_type {
            None => {}
            Some(val) => { query_params.push(("memberType", val.to_string())); }
        }
        match name_search {
            None => {}
            Some(val) => { query_params.push(("nameSearch", val.to_string())); }
        }
        self.bungie_api_get::<rustgie_types::SearchResultOfGroupMember>(
            Url::parse_with_params(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/"), query_params).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_pending_memberships(&self, _currentpage: i32, group_id: i64, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfGroupMemberApplication> {
        self.bungie_api_get::<rustgie_types::SearchResultOfGroupMemberApplication>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/Pending/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_potential_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupPotentialMemberStatus, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupPotentialMembershipSearchResponse> {
        self.bungie_api_get::<rustgie_types::groups_v2::GroupPotentialMembershipSearchResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/User/Potential/{membership_type}/{membership_id}/{filter}/{group_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_recommended_groups(&self, create_date_range: rustgie_types::groups_v2::GroupDateRange, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<&str>) -> Result<Vec<rustgie_types::groups_v2::GroupV2Card>> {
        self.bungie_api_post::<Vec<rustgie_types::groups_v2::GroupV2Card>>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/Recommended/{group_type}/{create_date_range}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_get_user_clan_invite_setting(&self, m_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_get::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/GetUserClanInviteSetting/{m_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_group_search(&self, request_body: rustgie_types::groups_v2::GroupQuery, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupSearchResponse> {
        self.bungie_api_post_with_body::<rustgie_types::groups_v2::GroupSearchResponse, rustgie_types::groups_v2::GroupQuery>(
            Url::parse("https://www.bungie.net/Platform/GroupV2/Search/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_individual_group_invite(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupApplicationResponse> {
        self.bungie_api_post_with_body::<rustgie_types::groups_v2::GroupApplicationResponse, rustgie_types::groups_v2::GroupApplicationRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/IndividualInvite/{membership_type}/{membership_id}/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn group_v2_individual_group_invite_cancel(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupApplicationResponse> {
        self.bungie_api_post::<rustgie_types::groups_v2::GroupApplicationResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/IndividualInviteCancel/{membership_type}/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_kick_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupMemberLeaveResult> {
        self.bungie_api_post::<rustgie_types::groups_v2::GroupMemberLeaveResult>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Kick/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_recover_group_for_founder(&self, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::groups_v2::GroupMembershipSearchResponse> {
        self.bungie_api_get::<rustgie_types::groups_v2::GroupMembershipSearchResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/Recover/{membership_type}/{membership_id}/{group_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn group_v2_unban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<i32> {
        self.bungie_api_post::<i32>(
            Url::parse(&format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Unban/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_accept_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/Social/Friends/Requests/Accept/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_decline_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/Social/Friends/Requests/Decline/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_get_friend_list(&self, access_token: Option<&str>) -> Result<rustgie_types::social::friends::BungieFriendListResponse> {
        self.bungie_api_get::<rustgie_types::social::friends::BungieFriendListResponse>(
            Url::parse("https://www.bungie.net/Platform/Social/Friends/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_get_friend_request_list(&self, access_token: Option<&str>) -> Result<rustgie_types::social::friends::BungieFriendRequestListResponse> {
        self.bungie_api_get::<rustgie_types::social::friends::BungieFriendRequestListResponse>(
            Url::parse("https://www.bungie.net/Platform/Social/Friends/Requests/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_get_platform_friend_list(&self, friend_platform: rustgie_types::social::friends::PlatformFriendType, page: &str, access_token: Option<&str>) -> Result<rustgie_types::social::friends::PlatformFriendResponse> {
        self.bungie_api_get::<rustgie_types::social::friends::PlatformFriendResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/Social/PlatformFriends/{friend_platform}/{page}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_issue_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/Social/Friends/Add/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_remove_friend(&self, membership_id: &str, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/Social/Friends/Remove/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn social_remove_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/Social/Friends/Requests/Remove/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn tokens_apply_missing_partner_offers_without_claim(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse(&format!("https://www.bungie.net/Platform/Tokens/Partner/ApplyMissingOffers/{partner_application_id}/{target_bnet_membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn tokens_claim_partner_offer(&self, request_body: rustgie_types::tokens::PartnerOfferClaimRequest, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post_with_body::<bool, rustgie_types::tokens::PartnerOfferClaimRequest>(
            Url::parse("https://www.bungie.net/Platform/Tokens/Partner/ClaimOffer/").with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn tokens_force_drops_repair(&self, access_token: Option<&str>) -> Result<bool> {
        self.bungie_api_post::<bool>(
            Url::parse("https://www.bungie.net/Platform/Tokens/Partner/ForceDropsRepair/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn tokens_get_bungie_rewards_for_platform_user(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        self.bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(
            Url::parse(&format!("https://www.bungie.net/Platform/Tokens/Rewards/GetRewardsForPlatformUser/{membership_id}/{membership_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn tokens_get_bungie_rewards_for_user(&self, membership_id: i64, access_token: Option<&str>) -> Result<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        self.bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(
            Url::parse(&format!("https://www.bungie.net/Platform/Tokens/Rewards/GetRewardsForUser/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn tokens_get_bungie_rewards_list(&self, access_token: Option<&str>) -> Result<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        self.bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(
            Url::parse("https://www.bungie.net/Platform/Tokens/Rewards/BungieRewards/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn tokens_get_partner_offer_sku_history(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<&str>) -> Result<Vec<rustgie_types::tokens::PartnerOfferSkuHistoryResponse>> {
        self.bungie_api_get::<Vec<rustgie_types::tokens::PartnerOfferSkuHistoryResponse>>(
            Url::parse(&format!("https://www.bungie.net/Platform/Tokens/Partner/History/{partner_application_id}/{target_bnet_membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn tokens_get_partner_reward_history(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<&str>) -> Result<rustgie_types::tokens::PartnerRewardHistoryResponse> {
        self.bungie_api_get::<rustgie_types::tokens::PartnerRewardHistoryResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/Tokens/Partner/History/{target_bnet_membership_id}/Application/{partner_application_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn trending_get_trending_categories(&self, access_token: Option<&str>) -> Result<rustgie_types::trending::TrendingCategories> {
        self.bungie_api_get::<rustgie_types::trending::TrendingCategories>(
            Url::parse("https://www.bungie.net/Platform/Trending/Categories/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn trending_get_trending_category(&self, category_id: &str, page_number: i32, access_token: Option<&str>) -> Result<rustgie_types::SearchResultOfTrendingEntry> {
        self.bungie_api_get::<rustgie_types::SearchResultOfTrendingEntry>(
            Url::parse(&format!("https://www.bungie.net/Platform/Trending/Categories/{category_id}/{page_number}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn trending_get_trending_entry_detail(&self, identifier: &str, trending_entry_type: rustgie_types::trending::TrendingEntryType, access_token: Option<&str>) -> Result<rustgie_types::trending::TrendingDetail> {
        self.bungie_api_get::<rustgie_types::trending::TrendingDetail>(
            Url::parse(&format!("https://www.bungie.net/Platform/Trending/Details/{trending_entry_type}/{identifier}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_get_available_themes(&self, access_token: Option<&str>) -> Result<Vec<rustgie_types::config::UserTheme>> {
        self.bungie_api_get::<Vec<rustgie_types::config::UserTheme>>(
            Url::parse("https://www.bungie.net/Platform/User/GetAvailableThemes/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_get_bungie_net_user_by_id(&self, id: i64, access_token: Option<&str>) -> Result<rustgie_types::user::GeneralUser> {
        self.bungie_api_get::<rustgie_types::user::GeneralUser>(
            Url::parse(&format!("https://www.bungie.net/Platform/User/GetBungieNetUserById/{id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_get_credential_types_for_target_account(&self, membership_id: i64, access_token: Option<&str>) -> Result<Vec<rustgie_types::user::models::GetCredentialTypesForAccountResponse>> {
        self.bungie_api_get::<Vec<rustgie_types::user::models::GetCredentialTypesForAccountResponse>>(
            Url::parse(&format!("https://www.bungie.net/Platform/User/GetCredentialTypesForTargetAccount/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_get_membership_data_by_id(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> Result<rustgie_types::user::UserMembershipData> {
        self.bungie_api_get::<rustgie_types::user::UserMembershipData>(
            Url::parse(&format!("https://www.bungie.net/Platform/User/GetMembershipsById/{membership_id}/{membership_type}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_get_membership_data_for_current_user(&self, access_token: Option<&str>) -> Result<rustgie_types::user::UserMembershipData> {
        self.bungie_api_get::<rustgie_types::user::UserMembershipData>(
            Url::parse("https://www.bungie.net/Platform/User/GetMembershipsForCurrentUser/").with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_get_membership_from_hard_linked_credential(&self, credential: &str, cr_type: rustgie_types::BungieCredentialType, access_token: Option<&str>) -> Result<rustgie_types::user::HardLinkedUserMembership> {
        self.bungie_api_get::<rustgie_types::user::HardLinkedUserMembership>(
            Url::parse(&format!("https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/{cr_type}/{credential}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_get_sanitized_platform_display_names(&self, membership_id: i64, access_token: Option<&str>) -> Result<HashMap<u8, String>> {
        self.bungie_api_get::<HashMap<u8, String>>(
            Url::parse(&format!("https://www.bungie.net/Platform/User/GetSanitizedPlatformDisplayNames/{membership_id}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }

    pub async fn user_search_by_global_name_post(&self, page: i32, request_body: rustgie_types::user::UserSearchPrefixRequest, access_token: Option<&str>) -> Result<rustgie_types::user::UserSearchResponse> {
        self.bungie_api_post_with_body::<rustgie_types::user::UserSearchResponse, rustgie_types::user::UserSearchPrefixRequest>(
            Url::parse(&format!("https://www.bungie.net/Platform/User/Search/GlobalName/{page}/")).with_context(|| "Error parsing URL")?,
            request_body, access_token
        ).await
    }

    pub async fn user_search_by_global_name_prefix(&self, display_name_prefix: &str, page: i32, access_token: Option<&str>) -> Result<rustgie_types::user::UserSearchResponse> {
        self.bungie_api_get::<rustgie_types::user::UserSearchResponse>(
            Url::parse(&format!("https://www.bungie.net/Platform/User/Search/Prefix/{display_name_prefix}/{page}/")).with_context(|| "Error parsing URL")?,
            access_token
        ).await
    }
}
