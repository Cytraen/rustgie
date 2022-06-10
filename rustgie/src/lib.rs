use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct QueryParamVec<T: Display>(Vec<T>);

impl<T: Display> Display for QueryParamVec<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut comma_separated = String::new();

        for thing in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&thing.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", comma_separated)
    }
}

pub struct RustgieClientBuilder {
    config: ClientBuilderConfig,
}

impl RustgieClientBuilder {
    pub fn new() -> RustgieClientBuilder {
        RustgieClientBuilder { config: ClientBuilderConfig::new() }
    }

    pub fn with_api_key(mut self, api_key: String) -> RustgieClientBuilder {
        self.config.api_key = Option::from(api_key);
        self
    }

    pub fn with_user_agent(mut self, user_agent: String) -> RustgieClientBuilder {
        self.config.user_agent = Option::from(user_agent);
        self
    }

    pub fn build(self) -> RustgieClient {
        let mut header_map = reqwest::header::HeaderMap::new();
        header_map.insert("X-API-Key", reqwest::header::HeaderValue::from_str(self.config.api_key.as_ref().unwrap()).unwrap());
        if self.config.user_agent.is_some() {
            header_map.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_str(self.config.user_agent.as_ref().unwrap()).unwrap());
        }
        RustgieClient::new(header_map)
    }
}

impl Default for RustgieClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

struct ClientBuilderConfig {
    api_key: std::option::Option<String>,
    user_agent: std::option::Option<String>,
}

impl ClientBuilderConfig {
    fn new () -> ClientBuilderConfig {
        ClientBuilderConfig {
            api_key: None,
            user_agent: None
        }
    }
}

pub struct RustgieClient {
    client: reqwest::Client,
}

impl RustgieClient {
    fn new(default_headers: reqwest::header::HeaderMap) -> Self {
        Self { client: reqwest::ClientBuilder::new()
            .deflate(true)
            .gzip(true)
            .brotli(true)
            .https_only(true)
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::none())
            .default_headers(default_headers)
            .build()
            .unwrap()
        }
    }

    async fn bungie_api_get<T: serde::de::DeserializeOwned>(&self, url: reqwest::Url, access_token: Option<String>) -> T {
        if access_token.is_some() {
            self.client.get(url)
                .bearer_auth(access_token.unwrap())
                .send()
                .await
                .unwrap()
                .json::<rustgie_types::api_response_::BungieApiResponse<T>>()
                .await
                .unwrap()
                .response
                .unwrap()
        }
        else {
            self.client.get(url)
                .send()
                .await
                .unwrap()
                .json::<rustgie_types::api_response_::BungieApiResponse<T>>()
                .await
                .unwrap()
                .response
                .unwrap()
        }
    }

    async fn bungie_api_post<T: serde::de::DeserializeOwned>(&self, url: reqwest::Url, access_token: Option<String>) -> T {
        if access_token.is_some() {
            self.client.post(url)
                .bearer_auth(access_token.unwrap())
                .send()
                .await
                .unwrap()
                .json::<rustgie_types::api_response_::BungieApiResponse<T>>()
                .await
                .unwrap()
                .response
                .unwrap()
        }
        else {
            self.client.post(url)
                .send()
                .await
                .unwrap()
                .json::<rustgie_types::api_response_::BungieApiResponse<T>>()
                .await
                .unwrap()
                .response
                .unwrap()
        }
    }

    async fn bungie_api_post_with_body<T: serde::de::DeserializeOwned, U: serde::Serialize>(&self, url: reqwest::Url, request_body: U, access_token: Option<String>) -> T {
        if access_token.is_some() {
            self.client.post(url)
                .bearer_auth(access_token.unwrap())
                .json(&request_body)
                .send()
                .await
                .unwrap()
                .json::<rustgie_types::api_response_::BungieApiResponse<T>>()
                .await
                .unwrap()
                .response
                .unwrap()
        }
        else {
            self.client.post(url)
                .json(&request_body)
                .send()
                .await
                .unwrap()
                .json::<rustgie_types::api_response_::BungieApiResponse<T>>()
                .await
                .unwrap()
                .response
                .unwrap()
        }
    }

    /////////////////////////////////////// AUTO GENERATED CONTENT BELOW

    pub async fn get_available_locales(&self, access_token: Option<String>) -> HashMap<String, String> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/GetAvailableLocales/").unwrap(), access_token).await
    }

    pub async fn get_common_settings(&self, access_token: Option<String>) -> rustgie_types::common::models::CoreSettingsConfiguration {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Settings/").unwrap(), access_token).await
    }

    pub async fn get_global_alerts(&self, includestreaming: Option<bool>, access_token: Option<String>) -> Vec<rustgie_types::GlobalAlert> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if includestreaming.is_some(){
            query_params.push(("includestreaming", includestreaming.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params("https://www.bungie.net/Platform/GlobalAlerts/", query_params).unwrap(), access_token).await
    }

    pub async fn get_user_system_overrides(&self, access_token: Option<String>) -> HashMap<String, rustgie_types::common::models::CoreSystem> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/UserSystemOverrides/").unwrap(), access_token).await
    }

    pub async fn app_get_application_api_usage(&self, application_id: i32, end: Option<time::OffsetDateTime>, start: Option<time::OffsetDateTime>, access_token: Option<String>) -> rustgie_types::applications::ApiUsage {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if end.is_some(){
            query_params.push(("end", end.unwrap().to_string()));
        }
        if start.is_some(){
            query_params.push(("start", start.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/App/ApiUsage/{application_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn app_get_bungie_applications(&self, access_token: Option<String>) -> Vec<rustgie_types::applications::Application> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/App/FirstParty/").unwrap(), access_token).await
    }

    pub async fn community_content_get_community_content(&self, media_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, sort: rustgie_types::forum::CommunityContentSortMode, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/CommunityContent/Get/{sort}/{media_filter}/{page}/")).unwrap(), access_token).await
    }

    pub async fn content_get_content_by_id(&self, id: i64, locale: String, head: Option<bool>, access_token: Option<String>) -> rustgie_types::content::ContentItemPublicContract {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if head.is_some(){
            query_params.push(("head", head.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/GetContentById/{id}/{locale}/"), query_params).unwrap(), access_token).await
    }

    pub async fn content_get_content_by_tag_and_type(&self, locale: String, tag: String, r#type: String, head: Option<bool>, access_token: Option<String>) -> rustgie_types::content::ContentItemPublicContract {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if head.is_some(){
            query_params.push(("head", head.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/GetContentByTagAndType/{tag}/{type}/{locale}/"), query_params).unwrap(), access_token).await
    }

    pub async fn content_get_content_type(&self, r#type: String, access_token: Option<String>) -> rustgie_types::content::models::ContentTypeDescription {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Content/GetContentType/{type}/")).unwrap(), access_token).await
    }

    pub async fn content_search_content_by_tag_and_type(&self, locale: String, tag: String, r#type: String, currentpage: Option<i32>, head: Option<bool>, itemsperpage: Option<i32>, access_token: Option<String>) -> rustgie_types::SearchResultOfContentItemPublicContract {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if currentpage.is_some(){
            query_params.push(("currentpage", currentpage.unwrap().to_string()));
        }
        if head.is_some(){
            query_params.push(("head", head.unwrap().to_string()));
        }
        if itemsperpage.is_some(){
            query_params.push(("itemsperpage", itemsperpage.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/SearchContentByTagAndType/{tag}/{type}/{locale}/"), query_params).unwrap(), access_token).await
    }

    pub async fn content_search_content_with_text(&self, locale: String, ctype: Option<String>, currentpage: Option<i32>, head: Option<bool>, searchtext: Option<String>, source: Option<String>, tag: Option<String>, access_token: Option<String>) -> rustgie_types::SearchResultOfContentItemPublicContract {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if ctype.is_some(){
            query_params.push(("ctype", ctype.unwrap()));
        }
        if currentpage.is_some(){
            query_params.push(("currentpage", currentpage.unwrap().to_string()));
        }
        if head.is_some(){
            query_params.push(("head", head.unwrap().to_string()));
        }
        if searchtext.is_some(){
            query_params.push(("searchtext", searchtext.unwrap()));
        }
        if source.is_some(){
            query_params.push(("source", source.unwrap()));
        }
        if tag.is_some(){
            query_params.push(("tag", tag.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/Search/{locale}/"), query_params).unwrap(), access_token).await
    }

    pub async fn content_search_help_articles(&self, searchtext: String, size: String, access_token: Option<String>) -> rustgie_types::destiny::definitions::DestinyDefinition {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Content/SearchHelpArticles/{searchtext}/{size}/")).unwrap(), access_token).await
    }

    pub async fn destiny2_awa_get_action_token(&self, correlation_id: String, access_token: Option<String>) -> rustgie_types::destiny::advanced::AwaAuthorizationResult {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Awa/GetActionToken/{correlation_id}/")).unwrap(), access_token).await
    }

    pub async fn destiny2_awa_initialize_request(&self, request_body: rustgie_types::destiny::advanced::AwaPermissionRequested, access_token: Option<String>) -> rustgie_types::destiny::advanced::AwaInitializeResponse {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/Initialize/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_awa_provide_authorization_result(&self, request_body: rustgie_types::destiny::advanced::AwaUserResponse, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/AwaProvideAuthorizationResult/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_equip_item(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemActionRequest, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItem/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_equip_items(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemSetActionRequest, access_token: Option<String>) -> rustgie_types::destiny::DestinyEquipItemResults {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItems/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_get_activity_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, count: Option<i32>, mode: Option<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>, page: Option<i32>, access_token: Option<String>) -> rustgie_types::destiny::historical_stats::DestinyActivityHistoryResults {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if count.is_some(){
            query_params.push(("count", count.unwrap().to_string()));
        }
        if mode.is_some(){
            query_params.push(("mode", mode.unwrap().to_string()));
        }
        if page.is_some(){
            query_params.push(("page", page.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/Activities/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyCharacterResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_clan_aggregate_stats(&self, group_id: i64, modes: Option<String>, access_token: Option<String>) -> Vec<rustgie_types::destiny::historical_stats::DestinyClanAggregateStat> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/AggregateClanStats/{group_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_clan_banner_source(&self, access_token: Option<String>) -> rustgie_types::config::clan_banner::ClanBannerSource {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Clan/ClanBannerDictionary/").unwrap(), access_token).await
    }

    pub async fn destiny2_get_clan_leaderboards(&self, group_id: i64, maxtop: Option<i32>, modes: Option<String>, statid: Option<String>, access_token: Option<String>) -> HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if maxtop.is_some(){
            query_params.push(("maxtop", maxtop.unwrap().to_string()));
        }
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap()));
        }
        if statid.is_some(){
            query_params.push(("statid", statid.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/Clans/{group_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_clan_weekly_reward_state(&self, group_id: i64, access_token: Option<String>) -> rustgie_types::destiny::milestones::DestinyMilestone {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Clan/{group_id}/WeeklyRewardState/")).unwrap(), access_token).await
    }

    pub async fn destiny2_get_collectible_node_details(&self, character_id: i64, collectible_presentation_node_hash: u32, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyCollectibleNodeDetailResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Collectibles/{collectible_presentation_node_hash}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_destiny_aggregate_activity_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::destiny::historical_stats::DestinyAggregateActivityResults {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/AggregateActivityStats/")).unwrap(), access_token).await
    }

    pub async fn destiny2_get_destiny_entity_definition(&self, entity_type: String, hash_identifier: u32, access_token: Option<String>) -> rustgie_types::destiny::definitions::DestinyDefinition {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Manifest/{entity_type}/{hash_identifier}/")).unwrap(), access_token).await
    }

    pub async fn destiny2_get_destiny_manifest(&self, access_token: Option<String>) -> rustgie_types::destiny::config::DestinyManifest {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Manifest/").unwrap(), access_token).await
    }

    pub async fn destiny2_get_historical_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, dayend: Option<time::OffsetDateTime>, daystart: Option<time::OffsetDateTime>, groups: Option<QueryParamVec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, modes: Option<QueryParamVec<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>>, period_type: Option<rustgie_types::destiny::historical_stats::definitions::PeriodType>, access_token: Option<String>) -> HashMap<String, rustgie_types::destiny::historical_stats::DestinyHistoricalStatsByPeriod> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if dayend.is_some(){
            query_params.push(("dayend", dayend.unwrap().to_string()));
        }
        if daystart.is_some(){
            query_params.push(("daystart", daystart.unwrap().to_string()));
        }
        if groups.is_some(){
            query_params.push(("groups", groups.unwrap().to_string()));
        }
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap().to_string()));
        }
        if period_type.is_some(){
            query_params.push(("periodType", period_type.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_historical_stats_definition(&self, access_token: Option<String>) -> HashMap<String, rustgie_types::destiny::historical_stats::definitions::DestinyHistoricalStatsDefinition> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Stats/Definition/").unwrap(), access_token).await
    }

    pub async fn destiny2_get_historical_stats_for_account(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, groups: Option<QueryParamVec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, access_token: Option<String>) -> rustgie_types::destiny::historical_stats::DestinyHistoricalStatsAccountResult {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if groups.is_some(){
            query_params.push(("groups", groups.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Stats/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_item(&self, destiny_membership_id: i64, item_instance_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyItemResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Item/{item_instance_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_leaderboards(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<String>, statid: Option<String>, access_token: Option<String>) -> HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if maxtop.is_some(){
            query_params.push(("maxtop", maxtop.unwrap().to_string()));
        }
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap()));
        }
        if statid.is_some(){
            query_params.push(("statid", statid.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Stats/Leaderboards/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_leaderboards_for_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<String>, statid: Option<String>, access_token: Option<String>) -> HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if maxtop.is_some(){
            query_params.push(("maxtop", maxtop.unwrap().to_string()));
        }
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap()));
        }
        if statid.is_some(){
            query_params.push(("statid", statid.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/{membership_type}/{destiny_membership_id}/{character_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_linked_profiles(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, get_all_memberships: Option<bool>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyLinkedProfilesResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if get_all_memberships.is_some(){
            query_params.push(("getAllMemberships", get_all_memberships.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{membership_id}/LinkedProfiles/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_post_game_carnage_report(&self, activity_id: i64, access_token: Option<String>) -> rustgie_types::destiny::historical_stats::DestinyPostGameCarnageReportData {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activity_id}/")).unwrap(), access_token).await
    }

    pub async fn destiny2_get_profile(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyProfileResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_public_milestone_content(&self, milestone_hash: u32, access_token: Option<String>) -> rustgie_types::destiny::milestones::DestinyMilestoneContent {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Milestones/{milestone_hash}/Content/")).unwrap(), access_token).await
    }

    pub async fn destiny2_get_public_milestones(&self, access_token: Option<String>) -> HashMap<u32, rustgie_types::destiny::milestones::DestinyPublicMilestone> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Milestones/").unwrap(), access_token).await
    }

    pub async fn destiny2_get_public_vendors(&self, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyPublicVendorsResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params("https://www.bungie.net/Platform/Destiny2/Vendors/", query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_unique_weapon_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::destiny::historical_stats::DestinyHistoricalWeaponStatsData {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/UniqueWeapons/")).unwrap(), access_token).await
    }

    pub async fn destiny2_get_vendor(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, vendor_hash: u32, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyVendorResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Vendors/{vendor_hash}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_get_vendors(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, filter: Option<rustgie_types::destiny::DestinyVendorFilter>, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyVendorsResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        if filter.is_some(){
            query_params.push(("filter", filter.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Vendors/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_insert_socket_plug(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsActionRequest, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyItemChangeResponse {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlug/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_insert_socket_plug_free(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsFreeActionRequest, access_token: Option<String>) -> rustgie_types::destiny::responses::DestinyItemChangeResponse {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlugFree/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_pull_from_postmaster(&self, request_body: rustgie_types::destiny::requests::actions::DestinyPostmasterTransferRequest, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/PullFromPostmaster/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_report_offensive_post_game_carnage_report_player(&self, activity_id: i64, request_body: rustgie_types::destiny::reporting::requests::DestinyReportOffensePgcrRequest, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activity_id}/Report/")).unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_search_destiny_entities(&self, search_term: String, r#type: String, page: Option<i32>, access_token: Option<String>) -> rustgie_types::destiny::definitions::DestinyEntitySearchResult {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if page.is_some(){
            query_params.push(("page", page.unwrap().to_string()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Armory/Search/{type}/{search_term}/"), query_params).unwrap(), access_token).await
    }

    pub async fn destiny2_search_destiny_player_by_bungie_name(&self, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::user::ExactSearchRequest, access_token: Option<String>) -> Vec<rustgie_types::user::UserInfoCard> {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayerByBungieName/{membership_type}/")).unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_set_item_lock_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetLockState/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_set_quest_tracked_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetTrackedState/").unwrap(), request_body, access_token).await
    }

    pub async fn destiny2_transfer_item(&self, request_body: rustgie_types::destiny::requests::DestinyItemTransferRequest, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/TransferItem/").unwrap(), request_body, access_token).await
    }

    pub async fn fireteam_get_active_private_clan_fireteam_count(&self, group_id: i64, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/ActiveCount/")).unwrap(), access_token).await
    }

    pub async fn fireteam_get_available_clan_fireteams(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, group_id: i64, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, public_only: rustgie_types::fireteam::FireteamPublicSearchOption, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<String>, access_token: Option<String>) -> rustgie_types::SearchResultOfFireteamSummary {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if lang_filter.is_some(){
            query_params.push(("langFilter", lang_filter.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/Available/{platform}/{activity_type}/{date_range}/{slot_filter}/{public_only}/{page}/"), query_params).unwrap(), access_token).await
    }

    pub async fn fireteam_get_clan_fireteam(&self, fireteam_id: i64, group_id: i64, access_token: Option<String>) -> rustgie_types::fireteam::FireteamResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/Summary/{fireteam_id}/")).unwrap(), access_token).await
    }

    pub async fn fireteam_get_my_clan_fireteams(&self, group_id: i64, include_closed: bool, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, group_filter: Option<bool>, lang_filter: Option<String>, access_token: Option<String>) -> rustgie_types::SearchResultOfFireteamResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if group_filter.is_some(){
            query_params.push(("groupFilter", group_filter.unwrap().to_string()));
        }
        if lang_filter.is_some(){
            query_params.push(("langFilter", lang_filter.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/My/{platform}/{include_closed}/{page}/"), query_params).unwrap(), access_token).await
    }

    pub async fn fireteam_search_public_available_clan_fireteams(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<String>, access_token: Option<String>) -> rustgie_types::SearchResultOfFireteamSummary {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if lang_filter.is_some(){
            query_params.push(("langFilter", lang_filter.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Search/Available/{platform}/{activity_type}/{date_range}/{slot_filter}/{page}/"), query_params).unwrap(), access_token).await
    }

    pub async fn forum_get_core_topics_paged(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<String>, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if locales.is_some(){
            query_params.push(("locales", locales.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetCoreTopicsPaged/{page}/{sort}/{quick_date}/{category_filter}/"), query_params).unwrap(), access_token).await
    }

    pub async fn forum_get_forum_tag_suggestions(&self, partialtag: Option<String>, access_token: Option<String>) -> Vec<rustgie_types::tags::models::contracts::TagResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if partialtag.is_some(){
            query_params.push(("partialtag", partialtag.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params("https://www.bungie.net/Platform/Forum/GetForumTagSuggestions/", query_params).unwrap(), access_token).await
    }

    pub async fn forum_get_poll(&self, topic_id: i64, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Forum/Poll/{topic_id}/")).unwrap(), access_token).await
    }

    pub async fn forum_get_post_and_parent(&self, child_post_id: i64, showbanned: Option<String>, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostAndParent/{child_post_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn forum_get_post_and_parent_awaiting_approval(&self, child_post_id: i64, showbanned: Option<String>, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostAndParentAwaitingApproval/{child_post_id}/"), query_params).unwrap(), access_token).await
    }

    pub async fn forum_get_posts_threaded_paged(&self, get_parent_post: bool, page: i32, page_size: i32, parent_post_id: i64, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<String>, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPaged/{parent_post_id}/{page}/{page_size}/{reply_size}/{get_parent_post}/{root_thread_mode}/{sort_mode}/"), query_params).unwrap(), access_token).await
    }

    pub async fn forum_get_posts_threaded_paged_from_child(&self, child_post_id: i64, page: i32, page_size: i32, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<String>, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPagedFromChild/{child_post_id}/{page}/{page_size}/{reply_size}/{root_thread_mode}/{sort_mode}/"), query_params).unwrap(), access_token).await
    }

    pub async fn forum_get_recruitment_thread_summaries(&self, request_body: Vec<i64>, access_token: Option<String>) -> Vec<rustgie_types::forum::ForumRecruitmentDetail> {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Forum/Recruit/Summaries/").unwrap(), request_body, access_token).await
    }

    pub async fn forum_get_topic_for_content(&self, content_id: i64, access_token: Option<String>) -> i64 {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Forum/GetTopicForContent/{content_id}/")).unwrap(), access_token).await
    }

    pub async fn forum_get_topics_paged(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, group: i64, page: i32, page_size: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<String>, tagstring: Option<String>, access_token: Option<String>) -> rustgie_types::forum::PostSearchResponse {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if locales.is_some(){
            query_params.push(("locales", locales.unwrap()));
        }
        if tagstring.is_some(){
            query_params.push(("tagstring", tagstring.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetTopicsPaged/{page}/{page_size}/{group}/{sort}/{quick_date}/{category_filter}/"), query_params).unwrap(), access_token).await
    }

    pub async fn group_v2_abdicate_foundership(&self, founder_id_new: i64, group_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Admin/AbdicateFoundership/{membership_type}/{founder_id_new}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_add_optional_conversation(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationAddRequest, access_token: Option<String>) -> i64 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/Add/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_approve_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> Vec<rustgie_types::entities::EntityActionResult> {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/ApproveAll/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_approve_pending(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/Approve/{membership_type}/{membership_id}/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_approve_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<String>) -> Vec<rustgie_types::entities::EntityActionResult> {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/ApproveList/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_ban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupBanRequest, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Ban/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_deny_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> Vec<rustgie_types::entities::EntityActionResult> {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/DenyAll/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_deny_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<String>) -> Vec<rustgie_types::entities::EntityActionResult> {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/DenyList/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_edit_clan_banner(&self, group_id: i64, request_body: rustgie_types::groups_v2::ClanBanner, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/EditClanBanner/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_edit_founder_options(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionsEditAction, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/EditFounderOptions/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_edit_group(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupEditAction, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Edit/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_edit_group_membership(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, member_type: rustgie_types::groups_v2::RuntimeGroupMemberType, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/SetMembershipType/{member_type}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_edit_optional_conversation(&self, conversation_id: i64, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationEditRequest, access_token: Option<String>) -> i64 {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/Edit/{conversation_id}/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_get_admins_and_founder_of_group(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> rustgie_types::SearchResultOfGroupMember {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/AdminsAndFounder/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_available_avatars(&self, access_token: Option<String>) -> HashMap<i32, String> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableAvatars/").unwrap(), access_token).await
    }

    pub async fn group_v2_get_available_themes(&self, access_token: Option<String>) -> Vec<rustgie_types::config::GroupTheme> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableThemes/").unwrap(), access_token).await
    }

    pub async fn group_v2_get_banned_members_of_group(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> rustgie_types::SearchResultOfGroupBan {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Banned/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_group(&self, group_id: i64, access_token: Option<String>) -> rustgie_types::groups_v2::GroupResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_group_by_name(&self, group_name: String, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<String>) -> rustgie_types::groups_v2::GroupResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Name/{group_name}/{group_type}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_group_by_name_v2(&self, request_body: rustgie_types::groups_v2::GroupNameSearchRequest, access_token: Option<String>) -> rustgie_types::groups_v2::GroupResponse {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/GroupV2/NameV2/").unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_get_group_optional_conversations(&self, group_id: i64, access_token: Option<String>) -> Vec<rustgie_types::groups_v2::GroupOptionalConversation> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupsForMemberFilter, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::groups_v2::GetGroupsForMemberResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/User/{membership_type}/{membership_id}/{filter}/{group_type}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_invited_individuals(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> rustgie_types::SearchResultOfGroupMemberApplication {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/InvitedIndividuals/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_members_of_group(&self, currentpage: i32, group_id: i64, member_type: Option<rustgie_types::groups_v2::RuntimeGroupMemberType>, name_search: Option<String>, access_token: Option<String>) -> rustgie_types::SearchResultOfGroupMember {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if member_type.is_some(){
            query_params.push(("memberType", member_type.unwrap().to_string()));
        }
        if name_search.is_some(){
            query_params.push(("nameSearch", name_search.unwrap()));
        }
        RustgieClient::bungie_api_get(self, reqwest::Url::parse_with_params(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/"), query_params).unwrap(), access_token).await
    }

    pub async fn group_v2_get_pending_memberships(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> rustgie_types::SearchResultOfGroupMemberApplication {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/Pending/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_potential_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupPotentialMemberStatus, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::groups_v2::GroupPotentialMembershipSearchResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/User/Potential/{membership_type}/{membership_id}/{filter}/{group_type}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_recommended_groups(&self, create_date_range: rustgie_types::groups_v2::GroupDateRange, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<String>) -> Vec<rustgie_types::groups_v2::GroupV2Card> {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Recommended/{group_type}/{create_date_range}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_get_user_clan_invite_setting(&self, m_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/GetUserClanInviteSetting/{m_type}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_group_search(&self, request_body: rustgie_types::groups_v2::GroupQuery, access_token: Option<String>) -> rustgie_types::groups_v2::GroupSearchResponse {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/GroupV2/Search/").unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_individual_group_invite(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> rustgie_types::groups_v2::GroupApplicationResponse {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/IndividualInvite/{membership_type}/{membership_id}/")).unwrap(), request_body, access_token).await
    }

    pub async fn group_v2_individual_group_invite_cancel(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::groups_v2::GroupApplicationResponse {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/IndividualInviteCancel/{membership_type}/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_kick_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::groups_v2::GroupMemberLeaveResult {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Kick/")).unwrap(), access_token).await
    }

    pub async fn group_v2_recover_group_for_founder(&self, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::groups_v2::GroupMembershipSearchResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Recover/{membership_type}/{membership_id}/{group_type}/")).unwrap(), access_token).await
    }

    pub async fn group_v2_unban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> i32 {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Unban/")).unwrap(), access_token).await
    }

    pub async fn social_accept_friend_request(&self, membership_id: String, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Accept/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn social_decline_friend_request(&self, membership_id: String, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Decline/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn social_get_friend_list(&self, access_token: Option<String>) -> rustgie_types::social::friends::BungieFriendListResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Social/Friends/").unwrap(), access_token).await
    }

    pub async fn social_get_friend_request_list(&self, access_token: Option<String>) -> rustgie_types::social::friends::BungieFriendRequestListResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Social/Friends/Requests/").unwrap(), access_token).await
    }

    pub async fn social_get_platform_friend_list(&self, friend_platform: rustgie_types::social::friends::PlatformFriendType, page: String, access_token: Option<String>) -> rustgie_types::social::friends::PlatformFriendResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Social/PlatformFriends/{friend_platform}/{page}/")).unwrap(), access_token).await
    }

    pub async fn social_issue_friend_request(&self, membership_id: String, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Add/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn social_remove_friend(&self, membership_id: String, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Remove/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn social_remove_friend_request(&self, membership_id: String, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Remove/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn tokens_apply_missing_partner_offers_without_claim(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Partner/ApplyMissingOffers/{partner_application_id}/{target_bnet_membership_id}/")).unwrap(), access_token).await
    }

    pub async fn tokens_claim_partner_offer(&self, request_body: rustgie_types::tokens::PartnerOfferClaimRequest, access_token: Option<String>) -> bool {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse("https://www.bungie.net/Platform/Tokens/Partner/ClaimOffer/").unwrap(), request_body, access_token).await
    }

    pub async fn tokens_get_bungie_rewards_for_user(&self, membership_id: i64, access_token: Option<String>) -> HashMap<String, rustgie_types::tokens::BungieRewardDisplay> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Rewards/GetRewardsForUser/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn tokens_get_bungie_rewards_list(&self, access_token: Option<String>) -> HashMap<String, rustgie_types::tokens::BungieRewardDisplay> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Tokens/Rewards/BungieRewards/").unwrap(), access_token).await
    }

    pub async fn tokens_get_partner_offer_sku_history(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<String>) -> Vec<rustgie_types::tokens::PartnerOfferSkuHistoryResponse> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Partner/History/{partner_application_id}/{target_bnet_membership_id}/")).unwrap(), access_token).await
    }

    pub async fn trending_get_trending_categories(&self, access_token: Option<String>) -> rustgie_types::trending::TrendingCategories {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/Trending/Categories/").unwrap(), access_token).await
    }

    pub async fn trending_get_trending_category(&self, category_id: String, page_number: i32, access_token: Option<String>) -> rustgie_types::SearchResultOfTrendingEntry {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Trending/Categories/{category_id}/{page_number}/")).unwrap(), access_token).await
    }

    pub async fn trending_get_trending_entry_detail(&self, identifier: String, trending_entry_type: rustgie_types::trending::TrendingEntryType, access_token: Option<String>) -> rustgie_types::trending::TrendingDetail {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/Trending/Details/{trending_entry_type}/{identifier}/")).unwrap(), access_token).await
    }

    pub async fn user_get_available_themes(&self, access_token: Option<String>) -> Vec<rustgie_types::config::UserTheme> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/User/GetAvailableThemes/").unwrap(), access_token).await
    }

    pub async fn user_get_bungie_net_user_by_id(&self, id: i64, access_token: Option<String>) -> rustgie_types::user::GeneralUser {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/User/GetBungieNetUserById/{id}/")).unwrap(), access_token).await
    }

    pub async fn user_get_credential_types_for_target_account(&self, membership_id: i64, access_token: Option<String>) -> Vec<rustgie_types::user::models::GetCredentialTypesForAccountResponse> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/User/GetCredentialTypesForTargetAccount/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn user_get_membership_data_by_id(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> rustgie_types::user::UserMembershipData {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/User/GetMembershipsById/{membership_id}/{membership_type}/")).unwrap(), access_token).await
    }

    pub async fn user_get_membership_data_for_current_user(&self, access_token: Option<String>) -> rustgie_types::user::UserMembershipData {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse("https://www.bungie.net/Platform/User/GetMembershipsForCurrentUser/").unwrap(), access_token).await
    }

    pub async fn user_get_membership_from_hard_linked_credential(&self, credential: String, cr_type: rustgie_types::BungieCredentialType, access_token: Option<String>) -> rustgie_types::user::HardLinkedUserMembership {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/{cr_type}/{credential}/")).unwrap(), access_token).await
    }

    pub async fn user_get_sanitized_platform_display_names(&self, membership_id: i64, access_token: Option<String>) -> HashMap<u8, String> {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/User/GetSanitizedPlatformDisplayNames/{membership_id}/")).unwrap(), access_token).await
    }

    pub async fn user_search_by_global_name_post(&self, page: i32, request_body: rustgie_types::user::UserSearchPrefixRequest, access_token: Option<String>) -> rustgie_types::user::UserSearchResponse {
        RustgieClient::bungie_api_post_with_body(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/User/Search/GlobalName/{page}/")).unwrap(), request_body, access_token).await
    }

    pub async fn user_search_by_global_name_prefix(&self, display_name_prefix: String, page: i32, access_token: Option<String>) -> rustgie_types::user::UserSearchResponse {
        RustgieClient::bungie_api_get(self, reqwest::Url::parse(&*format!("https://www.bungie.net/Platform/User/Search/Prefix/{display_name_prefix}/{page}/")).unwrap(), access_token).await
    }
}
