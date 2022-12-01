use std::collections::HashMap;
use std::fmt::Display;
use reqwest::{Response, Url};
use rustgie_types::api_response_::BungieApiResponse;

type RustgieResult<T> = Result<T, String>;

#[must_use]
pub struct RustgieClientBuilder {
    config: ClientBuilderConfig,
}

impl RustgieClientBuilder {
    pub fn new() -> RustgieClientBuilder {
        RustgieClientBuilder { config: ClientBuilderConfig::new() }
    }

    pub fn with_api_key(mut self, api_key: &str) -> RustgieClientBuilder {
        self.config.api_key = Option::from(api_key.to_string());
        self
    }

    pub fn with_user_agent(mut self, user_agent: &str) -> RustgieClientBuilder {
        const RUSTGIE_VERSION: &str = env!("CARGO_PKG_VERSION");
        self.config.user_agent = Option::from(format!("{user_agent} rustgie/{RUSTGIE_VERSION} (+github.com/ashakoor/rustgie)"));
        self
    }

    pub fn with_oauth_client_id(mut self, client_id: u32) -> RustgieClientBuilder {
        self.config.oauth_client_id = Option::from(client_id.to_string());
        self
    }

    pub fn with_oauth_client_secret(mut self, client_secret: &str) -> RustgieClientBuilder {
        self.config.oauth_client_secret = Option::from(client_secret.to_string());
        self
    }

    pub fn build(self) -> RustgieResult<RustgieClient> {
        let mut header_map = reqwest::header::HeaderMap::new();

        match self.config.api_key {
            None => { return Err("An API key is required.".to_string()) }
            Some(key) => { header_map.insert("X-API-Key", reqwest::header::HeaderValue::try_from(key).unwrap()); }
        }

        match self.config.user_agent {
            None => {}
            Some(ua) => { header_map.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::try_from(ua).unwrap()); }
        }

        Ok(RustgieClient::new(header_map, self.config.oauth_client_id, self.config.oauth_client_secret))
    }
}

impl Default for RustgieClientBuilder {
    fn default() -> Self { Self::new() }
}

struct ClientBuilderConfig {
    api_key: Option<String>,
    user_agent: Option<String>,
    oauth_client_id: Option<String>,
    oauth_client_secret: Option<String>,
}

impl ClientBuilderConfig {
    fn new () -> ClientBuilderConfig {
        ClientBuilderConfig {
            api_key: None,
            user_agent: None,
            oauth_client_id: None,
            oauth_client_secret: None
        }
    }
}

#[must_use]
pub struct RustgieClient {
    client: reqwest::Client,
    oauth_client_id: Option<String>,
    oauth_client_secret: Option<String>,
}

impl RustgieClient {
    fn new(default_headers: reqwest::header::HeaderMap, client_id: Option<String>, client_secret: Option<String>) -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
                .brotli(true)
                .gzip(true)
                .deflate(true)
                .https_only(true)
                .cookie_store(true)
                .redirect(reqwest::redirect::Policy::none())
                .default_headers(default_headers)
                .build()
                .unwrap(),
            oauth_client_id: client_id,
            oauth_client_secret: client_secret
        }
    }

    async fn bungie_api_get<T: serde::de::DeserializeOwned>(&self, url: Url, access_token: Option<&str>) -> RustgieResult<T> {
        let request = self.client.get(url);

        match access_token {
            None => { self.process_api_response::<T>(request).await }
            Some(at) => { self.process_api_response::<T>(request.bearer_auth(at.to_string())).await }
        }
    }

    async fn bungie_api_post<T: serde::de::DeserializeOwned>(&self, url: Url, access_token: Option<&str>) -> RustgieResult<T> {
        let request = self.client.post(url);

        match access_token {
            None => { self.process_api_response::<T>(request).await }
            Some(at) => { self.process_api_response::<T>(request.bearer_auth(at.to_string())).await }
        }
    }

    async fn bungie_api_post_with_body<T: serde::de::DeserializeOwned, U: serde::Serialize>(&self, url: Url, request_body: U, access_token: Option<&str>) -> RustgieResult<T> {
        let request = self.client.post(url).json(&request_body);

        match access_token {
            None => { self.process_api_response::<T>(request).await }
            Some(at) => { self.process_api_response::<T>(request.bearer_auth(at.to_string())).await }
        }
    }

    async fn process_api_response<T: serde::de::DeserializeOwned>(&self, request: reqwest::RequestBuilder) -> RustgieResult<T> {
        let http_response: Response;

        match request.send().await {
            Ok(resp) => { http_response = resp; }
            Err(_) => { return Err("There was an error connecting to the Bungie API".to_string()) }
        }

        let headers = http_response.headers();

        if headers.contains_key("Content-Type") && !headers["Content-Type"].to_str().unwrap().starts_with("application/json") {
            return Err("'Content-Type' of response was not 'application/json'".to_string())
        }

        let deserialized_response: BungieApiResponse<T>;

        match http_response.json::<BungieApiResponse<T>>().await {
            Ok(resp) => { deserialized_response = resp }
            Err(_) => { return Err("There was an error deserializing the JSON response".to_string()) }
        };

        match deserialized_response.error_code {
            rustgie_types::exceptions::PlatformErrorCodes::Success => {
                match deserialized_response.response {
                    None => { Err("The Bungie API did not include a response".to_string()) }
                    Some(resp) => { Ok(resp) }
                }
            }
            error_code => {
                    Err(format!("The Bungie API returned a PlatformErrorCode of {error_code}"))
            }
        }
    }

    /////////////////////////////////////// OAUTH FLOW

    pub fn oauth_get_authorization_url_(&self, language_code: &str, state: Option<&str>) -> RustgieResult<String> {
        let mut query_params = Vec::<(&str, &str)>::new();

        match &self.oauth_client_id {
            None => { return Err("OAuth client ID is required".to_string()) }
            Some(client_id) => { query_params.push(("client_id", client_id)); }
        }

        match state {
            None => {}
            Some(state_str) => { query_params.push(("state", state_str)); }
        }

        query_params.push(("response_type", "code"));

        return match Url::parse_with_params(&format!("https://www.bungie.net/{language_code}/OAuth/Authorize/"), query_params) {
            Ok(url) => { Ok(url.to_string()) }
            Err(_) => { Err("".to_string()) }
        }
    }

    async fn process_oauth_response(&self, request: reqwest::RequestBuilder) -> RustgieResult<rustgie_types::api_response_::BungieTokenResponse> {
        let http_response: Response;

        match request.send().await {
            Ok(resp) => { http_response = resp; }
            Err(_) => { return Err("There was an error connecting to the Bungie API".to_string()) }
        }

        let headers = http_response.headers();

        if headers.contains_key("Content-Type") && !headers["Content-Type"].to_str().unwrap().starts_with("application/json") {
            return Err("'Content-Type' of response was not 'application/json'".to_string())
        }

        let deserialized_response: rustgie_types::api_response_::BungieTokenResponse;

        match http_response.json::<rustgie_types::api_response_::BungieTokenResponse>().await {
            Ok(resp) => { deserialized_response = resp }
            Err(_) => { return Err("There was an error deserializing the JSON response".to_string()) }
        };

        match deserialized_response.access_token {
            None => { Err("The Bungie API did not include an access token".to_string()) }
            Some(_) => { Ok(deserialized_response) }
        }
    }

    pub async fn oauth_get_auth_token_(&self, auth_code: &str) -> RustgieResult<rustgie_types::api_response_::BungieTokenResponse> {
        let mut form = HashMap::<&str, String>::new();

        match &self.oauth_client_id {
            None => { return Err("OAuth client ID is required".to_string()) }
            Some(client_id) => { form.insert("client_id", client_id.to_string()); }
        }

        match &self.oauth_client_secret {
            None => {}
            Some(client_secret) => { form.insert("client_secret", client_secret.to_string()); }
        }

        form.insert("grant_type", "authorization_code".to_string());
        form.insert("code", auth_code.to_string());

        self.process_oauth_response(self.client.post("https://www.bungie.net/Platform/App/OAuth/Token/").form(&form)).await
    }

    pub async fn oauth_refresh_auth_token_(&self, refresh_token: &str) -> RustgieResult<rustgie_types::api_response_::BungieTokenResponse> {
        let mut form = HashMap::<&str, String>::new();

        match &self.oauth_client_id {
            None => { return Err("OAuth client ID is required".to_string()) }
            Some(client_id) => { form.insert("client_id", client_id.to_string()); }
        }

        match &self.oauth_client_secret {
            None => { return Err("OAuth client secret is required".to_string()) }
            Some(client_secret) => { form.insert("client_secret", client_secret.to_string()); }
        }

        form.insert("grant_type", "refresh_token".to_string());
        form.insert("refresh_token", refresh_token.to_string());

        self.process_oauth_response(self.client.post("https://www.bungie.net/Platform/App/OAuth/Token/").form(&form)).await
    }

    /////////////////////////////////////// AUTO GENERATED CONTENT BELOW

    pub async fn get_available_locales(&self, access_token: Option<&str>) -> RustgieResult<HashMap<String, String>> {
        match Url::parse("https://www.bungie.net/Platform/GetAvailableLocales/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, String>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn get_common_settings(&self, access_token: Option<&str>) -> RustgieResult<rustgie_types::common::models::CoreSettingsConfiguration> {
        match Url::parse("https://www.bungie.net/Platform/Settings/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::common::models::CoreSettingsConfiguration>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn get_global_alerts(&self, includestreaming: Option<bool>, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::GlobalAlert>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match includestreaming {
            None => {}
            Some(val) => { query_params.push(("includestreaming", val.to_string())); }
        }
        match Url::parse_with_params("https://www.bungie.net/Platform/GlobalAlerts/", query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::GlobalAlert>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn get_user_system_overrides(&self, access_token: Option<&str>) -> RustgieResult<HashMap<String, rustgie_types::common::models::CoreSystem>> {
        match Url::parse("https://www.bungie.net/Platform/UserSystemOverrides/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::common::models::CoreSystem>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn app_get_application_api_usage(&self, application_id: i32, end: Option<time::OffsetDateTime>, start: Option<time::OffsetDateTime>, access_token: Option<&str>) -> RustgieResult<rustgie_types::applications::ApiUsage> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match end {
            None => {}
            Some(val) => { query_params.push(("end", val.to_string())); }
        }
        match start {
            None => {}
            Some(val) => { query_params.push(("start", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/App/ApiUsage/{application_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::applications::ApiUsage>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn app_get_bungie_applications(&self, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::applications::Application>> {
        match Url::parse("https://www.bungie.net/Platform/App/FirstParty/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::applications::Application>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn community_content_get_community_content(&self, media_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, sort: rustgie_types::forum::CommunityContentSortMode, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/CommunityContent/Get/{sort}/{media_filter}/{page}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_get_content_by_id(&self, id: i64, locale: &str, head: Option<bool>, access_token: Option<&str>) -> RustgieResult<rustgie_types::content::ContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match head {
            None => {}
            Some(val) => { query_params.push(("head", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/GetContentById/{id}/{locale}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::content::ContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_get_content_by_tag_and_type(&self, locale: &str, tag: &str, r#type: &str, head: Option<bool>, access_token: Option<&str>) -> RustgieResult<rustgie_types::content::ContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match head {
            None => {}
            Some(val) => { query_params.push(("head", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/GetContentByTagAndType/{tag}/{type}/{locale}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::content::ContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_get_content_type(&self, r#type: &str, access_token: Option<&str>) -> RustgieResult<rustgie_types::content::models::ContentTypeDescription> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Content/GetContentType/{type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::content::models::ContentTypeDescription>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_rss_news_articles(&self, page_token: &str, access_token: Option<&str>) -> RustgieResult<rustgie_types::content::NewsArticleRssResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Content/Rss/NewsArticles/{page_token}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::content::NewsArticleRssResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_search_content_by_tag_and_type(&self, locale: &str, tag: &str, r#type: &str, currentpage: Option<i32>, head: Option<bool>, itemsperpage: Option<i32>, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfContentItemPublicContract> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/SearchContentByTagAndType/{tag}/{type}/{locale}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_search_content_with_text(&self, locale: &str, ctype: Option<&str>, currentpage: Option<i32>, head: Option<bool>, searchtext: Option<&str>, source: Option<&str>, tag: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfContentItemPublicContract> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/Search/{locale}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_search_help_articles(&self, searchtext: &str, size: &str, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::definitions::DestinyDefinition> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Content/SearchHelpArticles/{searchtext}/{size}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::definitions::DestinyDefinition>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_awa_get_action_token(&self, correlation_id: &str, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::advanced::AwaAuthorizationResult> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Awa/GetActionToken/{correlation_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::advanced::AwaAuthorizationResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_awa_initialize_request(&self, request_body: rustgie_types::destiny::advanced::AwaPermissionRequested, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::advanced::AwaInitializeResponse> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/Initialize/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::advanced::AwaInitializeResponse, rustgie_types::destiny::advanced::AwaPermissionRequested>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_awa_provide_authorization_result(&self, request_body: rustgie_types::destiny::advanced::AwaUserResponse, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/AwaProvideAuthorizationResult/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::advanced::AwaUserResponse>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_equip_item(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemActionRequest, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItem/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_equip_items(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemSetActionRequest, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::DestinyEquipItemResults> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItems/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::DestinyEquipItemResults, rustgie_types::destiny::requests::actions::DestinyItemSetActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_activity_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, count: Option<i32>, mode: Option<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>, page: Option<i32>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyActivityHistoryResults> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/Activities/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyActivityHistoryResults>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyCharacterResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyCharacterResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_aggregate_stats(&self, group_id: i64, modes: Option<&str>, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::destiny::historical_stats::DestinyClanAggregateStat>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match modes {
            None => {}
            Some(val) => { query_params.push(("modes", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/AggregateClanStats/{group_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::destiny::historical_stats::DestinyClanAggregateStat>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_banner_source(&self, access_token: Option<&str>) -> RustgieResult<rustgie_types::config::clan_banner::ClanBannerSource> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Clan/ClanBannerDictionary/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::config::clan_banner::ClanBannerSource>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_leaderboards(&self, group_id: i64, maxtop: Option<i32>, modes: Option<&str>, statid: Option<&str>, access_token: Option<&str>) -> RustgieResult<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/Clans/{group_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_weekly_reward_state(&self, group_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::milestones::DestinyMilestone> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Clan/{group_id}/WeeklyRewardState/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::milestones::DestinyMilestone>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_collectible_node_details(&self, character_id: i64, collectible_presentation_node_hash: u32, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyCollectibleNodeDetailResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Collectibles/{collectible_presentation_node_hash}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyCollectibleNodeDetailResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_destiny_aggregate_activity_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyAggregateActivityResults> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/AggregateActivityStats/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyAggregateActivityResults>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_destiny_entity_definition(&self, entity_type: &str, hash_identifier: u32, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::definitions::DestinyDefinition> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Manifest/{entity_type}/{hash_identifier}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::definitions::DestinyDefinition>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_destiny_manifest(&self, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::config::DestinyManifest> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Manifest/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::config::DestinyManifest>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_historical_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, dayend: Option<time::OffsetDateTime>, daystart: Option<time::OffsetDateTime>, groups: Option<Vec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, modes: Option<Vec<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>>, period_type: Option<rustgie_types::destiny::historical_stats::definitions::PeriodType>, access_token: Option<&str>) -> RustgieResult<HashMap<String, rustgie_types::destiny::historical_stats::DestinyHistoricalStatsByPeriod>> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::destiny::historical_stats::DestinyHistoricalStatsByPeriod>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_historical_stats_definition(&self, access_token: Option<&str>) -> RustgieResult<HashMap<String, rustgie_types::destiny::historical_stats::definitions::DestinyHistoricalStatsDefinition>> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Stats/Definition/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::destiny::historical_stats::definitions::DestinyHistoricalStatsDefinition>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_historical_stats_for_account(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, groups: Option<Vec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyHistoricalStatsAccountResult> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match groups {
            None => {}
            Some(val) => { query_params.push(("groups", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Stats/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyHistoricalStatsAccountResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_item(&self, destiny_membership_id: i64, item_instance_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyItemResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Item/{item_instance_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyItemResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_leaderboards(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<&str>, statid: Option<&str>, access_token: Option<&str>) -> RustgieResult<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Stats/Leaderboards/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_leaderboards_for_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<&str>, statid: Option<&str>, access_token: Option<&str>) -> RustgieResult<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/{membership_type}/{destiny_membership_id}/{character_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_linked_profiles(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, get_all_memberships: Option<bool>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyLinkedProfilesResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match get_all_memberships {
            None => {}
            Some(val) => { query_params.push(("getAllMemberships", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{membership_id}/LinkedProfiles/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyLinkedProfilesResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_post_game_carnage_report(&self, activity_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyPostGameCarnageReportData> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activity_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyPostGameCarnageReportData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_profile(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyProfileResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyProfileResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_public_milestone_content(&self, milestone_hash: u32, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::milestones::DestinyMilestoneContent> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Milestones/{milestone_hash}/Content/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::milestones::DestinyMilestoneContent>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_public_milestones(&self, access_token: Option<&str>) -> RustgieResult<HashMap<u32, rustgie_types::destiny::milestones::DestinyPublicMilestone>> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Milestones/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<u32, rustgie_types::destiny::milestones::DestinyPublicMilestone>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_public_vendors(&self, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyPublicVendorsResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match Url::parse_with_params("https://www.bungie.net/Platform/Destiny2/Vendors/", query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyPublicVendorsResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_unique_weapon_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyHistoricalWeaponStatsData> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Account/{destiny_membership_id}/Character/{character_id}/Stats/UniqueWeapons/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyHistoricalWeaponStatsData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_vendor(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, vendor_hash: u32, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyVendorResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Vendors/{vendor_hash}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyVendorResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_vendors(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<Vec<rustgie_types::destiny::DestinyComponentType>>, filter: Option<rustgie_types::destiny::DestinyVendorFilter>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyVendorsResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match components {
            None => {}
            Some(val) => { query_params.push(("components", val.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(","))); }
        }
        match filter {
            None => {}
            Some(val) => { query_params.push(("filter", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{membership_type}/Profile/{destiny_membership_id}/Character/{character_id}/Vendors/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyVendorsResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_insert_socket_plug(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsActionRequest, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyItemChangeResponse> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlug/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::responses::DestinyItemChangeResponse, rustgie_types::destiny::requests::actions::DestinyInsertPlugsActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_insert_socket_plug_free(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsFreeActionRequest, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::responses::DestinyItemChangeResponse> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlugFree/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::responses::DestinyItemChangeResponse, rustgie_types::destiny::requests::actions::DestinyInsertPlugsFreeActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_pull_from_postmaster(&self, request_body: rustgie_types::destiny::requests::actions::DestinyPostmasterTransferRequest, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/PullFromPostmaster/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyPostmasterTransferRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_report_offensive_post_game_carnage_report_player(&self, activity_id: i64, request_body: rustgie_types::destiny::reporting::requests::DestinyReportOffensePgcrRequest, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activity_id}/Report/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::reporting::requests::DestinyReportOffensePgcrRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_search_destiny_entities(&self, search_term: &str, r#type: &str, page: Option<i32>, access_token: Option<&str>) -> RustgieResult<rustgie_types::destiny::definitions::DestinyEntitySearchResult> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match page {
            None => {}
            Some(val) => { query_params.push(("page", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Armory/Search/{type}/{search_term}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::definitions::DestinyEntitySearchResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_search_destiny_player_by_bungie_name(&self, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::user::ExactSearchRequest, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::user::UserInfoCard>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayerByBungieName/{membership_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::user::UserInfoCard>, rustgie_types::user::ExactSearchRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_set_item_lock_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetLockState/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemStateRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_set_quest_tracked_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetTrackedState/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemStateRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_transfer_item(&self, request_body: rustgie_types::destiny::requests::DestinyItemTransferRequest, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/TransferItem/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::DestinyItemTransferRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_active_private_clan_fireteam_count(&self, group_id: i64, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/ActiveCount/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<i32>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_available_clan_fireteams(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, group_id: i64, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, public_only: rustgie_types::fireteam::FireteamPublicSearchOption, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfFireteamSummary> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match lang_filter {
            None => {}
            Some(val) => { query_params.push(("langFilter", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/Available/{platform}/{activity_type}/{date_range}/{slot_filter}/{public_only}/{page}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfFireteamSummary>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_clan_fireteam(&self, fireteam_id: i64, group_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::fireteam::FireteamResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/Summary/{fireteam_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::fireteam::FireteamResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_my_clan_fireteams(&self, group_id: i64, include_closed: bool, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, group_filter: Option<bool>, lang_filter: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfFireteamResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match group_filter {
            None => {}
            Some(val) => { query_params.push(("groupFilter", val.to_string())); }
        }
        match lang_filter {
            None => {}
            Some(val) => { query_params.push(("langFilter", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{group_id}/My/{platform}/{include_closed}/{page}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfFireteamResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_search_public_available_clan_fireteams(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfFireteamSummary> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match lang_filter {
            None => {}
            Some(val) => { query_params.push(("langFilter", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Search/Available/{platform}/{activity_type}/{date_range}/{slot_filter}/{page}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfFireteamSummary>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_core_topics_paged(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match locales {
            None => {}
            Some(val) => { query_params.push(("locales", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetCoreTopicsPaged/{page}/{sort}/{quick_date}/{category_filter}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_forum_tag_suggestions(&self, partialtag: Option<&str>, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::tags::models::contracts::TagResponse>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match partialtag {
            None => {}
            Some(val) => { query_params.push(("partialtag", val.to_string())); }
        }
        match Url::parse_with_params("https://www.bungie.net/Platform/Forum/GetForumTagSuggestions/", query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::tags::models::contracts::TagResponse>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_poll(&self, topic_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Forum/Poll/{topic_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_post_and_parent(&self, child_post_id: i64, showbanned: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostAndParent/{child_post_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_post_and_parent_awaiting_approval(&self, child_post_id: i64, showbanned: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostAndParentAwaitingApproval/{child_post_id}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_posts_threaded_paged(&self, get_parent_post: bool, page: i32, page_size: i32, parent_post_id: i64, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPaged/{parent_post_id}/{page}/{page_size}/{reply_size}/{get_parent_post}/{root_thread_mode}/{sort_mode}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_posts_threaded_paged_from_child(&self, child_post_id: i64, page: i32, page_size: i32, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match showbanned {
            None => {}
            Some(val) => { query_params.push(("showbanned", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPagedFromChild/{child_post_id}/{page}/{page_size}/{reply_size}/{root_thread_mode}/{sort_mode}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_recruitment_thread_summaries(&self, request_body: Vec<i64>, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::forum::ForumRecruitmentDetail>> {
        match Url::parse("https://www.bungie.net/Platform/Forum/Recruit/Summaries/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::forum::ForumRecruitmentDetail>, Vec<i64>>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_topic_for_content(&self, content_id: i64, access_token: Option<&str>) -> RustgieResult<i64> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Forum/GetTopicForContent/{content_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<i64>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_topics_paged(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, group: i64, page: i32, page_size: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<&str>, tagstring: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match locales {
            None => {}
            Some(val) => { query_params.push(("locales", val.to_string())); }
        }
        match tagstring {
            None => {}
            Some(val) => { query_params.push(("tagstring", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetTopicsPaged/{page}/{page_size}/{group}/{sort}/{quick_date}/{category_filter}/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_abdicate_foundership(&self, founder_id_new: i64, group_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Admin/AbdicateFoundership/{membership_type}/{founder_id_new}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_add_optional_conversation(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationAddRequest, access_token: Option<&str>) -> RustgieResult<i64> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/Add/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i64, rustgie_types::groups_v2::GroupOptionalConversationAddRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_approve_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/ApproveAll/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_approve_pending(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/Approve/{membership_type}/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<bool, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_approve_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/ApproveList/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationListRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_ban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupBanRequest, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Ban/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupBanRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_deny_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/DenyAll/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_deny_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/DenyList/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationListRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_clan_banner(&self, group_id: i64, request_body: rustgie_types::groups_v2::ClanBanner, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/EditClanBanner/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::ClanBanner>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_founder_options(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionsEditAction, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/EditFounderOptions/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupOptionsEditAction>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_group(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupEditAction, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Edit/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupEditAction>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_group_membership(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, member_type: rustgie_types::groups_v2::RuntimeGroupMemberType, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/SetMembershipType/{member_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<i32>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_optional_conversation(&self, conversation_id: i64, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationEditRequest, access_token: Option<&str>) -> RustgieResult<i64> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/Edit/{conversation_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i64, rustgie_types::groups_v2::GroupOptionalConversationEditRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_admins_and_founder_of_group(&self, currentpage: i32, group_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfGroupMember> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/AdminsAndFounder/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMember>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_available_avatars(&self, access_token: Option<&str>) -> RustgieResult<HashMap<i32, String>> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableAvatars/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<i32, String>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_available_themes(&self, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::config::GroupTheme>> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableThemes/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::config::GroupTheme>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_banned_members_of_group(&self, currentpage: i32, group_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfGroupBan> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Banned/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupBan>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group(&self, group_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group_by_name(&self, group_name: &str, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Name/{group_name}/{group_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group_by_name_v2(&self, request_body: rustgie_types::groups_v2::GroupNameSearchRequest, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupResponse> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/NameV2/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::groups_v2::GroupResponse, rustgie_types::groups_v2::GroupNameSearchRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group_optional_conversations(&self, group_id: i64, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::groups_v2::GroupOptionalConversation>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/OptionalConversations/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::groups_v2::GroupOptionalConversation>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupsForMemberFilter, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GetGroupsForMemberResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/User/{membership_type}/{membership_id}/{filter}/{group_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GetGroupsForMemberResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_invited_individuals(&self, currentpage: i32, group_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfGroupMemberApplication> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/InvitedIndividuals/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMemberApplication>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_members_of_group(&self, currentpage: i32, group_id: i64, member_type: Option<rustgie_types::groups_v2::RuntimeGroupMemberType>, name_search: Option<&str>, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfGroupMember> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        match member_type {
            None => {}
            Some(val) => { query_params.push(("memberType", val.to_string())); }
        }
        match name_search {
            None => {}
            Some(val) => { query_params.push(("nameSearch", val.to_string())); }
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/"), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMember>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_pending_memberships(&self, currentpage: i32, group_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfGroupMemberApplication> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/Pending/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMemberApplication>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_potential_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupPotentialMemberStatus, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupPotentialMembershipSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/User/Potential/{membership_type}/{membership_id}/{filter}/{group_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupPotentialMembershipSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_recommended_groups(&self, create_date_range: rustgie_types::groups_v2::GroupDateRange, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::groups_v2::GroupV2Card>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Recommended/{group_type}/{create_date_range}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<Vec<rustgie_types::groups_v2::GroupV2Card>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_user_clan_invite_setting(&self, m_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/GetUserClanInviteSetting/{m_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_group_search(&self, request_body: rustgie_types::groups_v2::GroupQuery, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupSearchResponse> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/Search/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::groups_v2::GroupSearchResponse, rustgie_types::groups_v2::GroupQuery>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_individual_group_invite(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupApplicationResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/IndividualInvite/{membership_type}/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::groups_v2::GroupApplicationResponse, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_individual_group_invite_cancel(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupApplicationResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/IndividualInviteCancel/{membership_type}/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<rustgie_types::groups_v2::GroupApplicationResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_kick_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupMemberLeaveResult> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Kick/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<rustgie_types::groups_v2::GroupMemberLeaveResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_recover_group_for_founder(&self, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::groups_v2::GroupMembershipSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Recover/{membership_type}/{membership_id}/{group_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupMembershipSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_unban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{group_id}/Members/{membership_type}/{membership_id}/Unban/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<i32>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_accept_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Accept/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_decline_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Decline/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_get_friend_list(&self, access_token: Option<&str>) -> RustgieResult<rustgie_types::social::friends::BungieFriendListResponse> {
        match Url::parse("https://www.bungie.net/Platform/Social/Friends/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::social::friends::BungieFriendListResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_get_friend_request_list(&self, access_token: Option<&str>) -> RustgieResult<rustgie_types::social::friends::BungieFriendRequestListResponse> {
        match Url::parse("https://www.bungie.net/Platform/Social/Friends/Requests/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::social::friends::BungieFriendRequestListResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_get_platform_friend_list(&self, friend_platform: rustgie_types::social::friends::PlatformFriendType, page: &str, access_token: Option<&str>) -> RustgieResult<rustgie_types::social::friends::PlatformFriendResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/PlatformFriends/{friend_platform}/{page}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::social::friends::PlatformFriendResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_issue_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Add/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_remove_friend(&self, membership_id: &str, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Remove/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_remove_friend_request(&self, membership_id: &str, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Remove/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_apply_missing_partner_offers_without_claim(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Partner/ApplyMissingOffers/{partner_application_id}/{target_bnet_membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_claim_partner_offer(&self, request_body: rustgie_types::tokens::PartnerOfferClaimRequest, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse("https://www.bungie.net/Platform/Tokens/Partner/ClaimOffer/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<bool, rustgie_types::tokens::PartnerOfferClaimRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_force_drops_repair(&self, access_token: Option<&str>) -> RustgieResult<bool> {
        match Url::parse("https://www.bungie.net/Platform/Tokens/Partner/ForceDropsRepair/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_bungie_rewards_for_platform_user(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Rewards/GetRewardsForPlatformUser/{membership_id}/{membership_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_bungie_rewards_for_user(&self, membership_id: i64, access_token: Option<&str>) -> RustgieResult<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Rewards/GetRewardsForUser/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_bungie_rewards_list(&self, access_token: Option<&str>) -> RustgieResult<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        match Url::parse("https://www.bungie.net/Platform/Tokens/Rewards/BungieRewards/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_partner_offer_sku_history(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::tokens::PartnerOfferSkuHistoryResponse>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Partner/History/{partner_application_id}/{target_bnet_membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::tokens::PartnerOfferSkuHistoryResponse>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_partner_reward_history(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::tokens::PartnerRewardHistoryResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Partner/History/{target_bnet_membership_id}/Application/{partner_application_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::tokens::PartnerRewardHistoryResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn trending_get_trending_categories(&self, access_token: Option<&str>) -> RustgieResult<rustgie_types::trending::TrendingCategories> {
        match Url::parse("https://www.bungie.net/Platform/Trending/Categories/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::trending::TrendingCategories>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn trending_get_trending_category(&self, category_id: &str, page_number: i32, access_token: Option<&str>) -> RustgieResult<rustgie_types::SearchResultOfTrendingEntry> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Trending/Categories/{category_id}/{page_number}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfTrendingEntry>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn trending_get_trending_entry_detail(&self, identifier: &str, trending_entry_type: rustgie_types::trending::TrendingEntryType, access_token: Option<&str>) -> RustgieResult<rustgie_types::trending::TrendingDetail> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Trending/Details/{trending_entry_type}/{identifier}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::trending::TrendingDetail>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_available_themes(&self, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::config::UserTheme>> {
        match Url::parse("https://www.bungie.net/Platform/User/GetAvailableThemes/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::config::UserTheme>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_bungie_net_user_by_id(&self, id: i64, access_token: Option<&str>) -> RustgieResult<rustgie_types::user::GeneralUser> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetBungieNetUserById/{id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::GeneralUser>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_credential_types_for_target_account(&self, membership_id: i64, access_token: Option<&str>) -> RustgieResult<Vec<rustgie_types::user::models::GetCredentialTypesForAccountResponse>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetCredentialTypesForTargetAccount/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::user::models::GetCredentialTypesForAccountResponse>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_membership_data_by_id(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<&str>) -> RustgieResult<rustgie_types::user::UserMembershipData> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetMembershipsById/{membership_id}/{membership_type}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::UserMembershipData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_membership_data_for_current_user(&self, access_token: Option<&str>) -> RustgieResult<rustgie_types::user::UserMembershipData> {
        match Url::parse("https://www.bungie.net/Platform/User/GetMembershipsForCurrentUser/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::UserMembershipData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_membership_from_hard_linked_credential(&self, credential: &str, cr_type: rustgie_types::BungieCredentialType, access_token: Option<&str>) -> RustgieResult<rustgie_types::user::HardLinkedUserMembership> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/{cr_type}/{credential}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::HardLinkedUserMembership>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_sanitized_platform_display_names(&self, membership_id: i64, access_token: Option<&str>) -> RustgieResult<HashMap<u8, String>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetSanitizedPlatformDisplayNames/{membership_id}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<u8, String>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_search_by_global_name_post(&self, page: i32, request_body: rustgie_types::user::UserSearchPrefixRequest, access_token: Option<&str>) -> RustgieResult<rustgie_types::user::UserSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/Search/GlobalName/{page}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::user::UserSearchResponse, rustgie_types::user::UserSearchPrefixRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_search_by_global_name_prefix(&self, display_name_prefix: &str, page: i32, access_token: Option<&str>) -> RustgieResult<rustgie_types::user::UserSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/Search/Prefix/{display_name_prefix}/{page}/")) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::UserSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }
}
