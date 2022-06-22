use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use reqwest::{Response, Url};
use rustgie_types::api_response_::BungieApiResponse;

type RustgieResult<T> = Result<T, String>;

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

    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> RustgieClientBuilder {
        self.config.api_key = Option::from(api_key.into());
        self
    }

    pub fn with_user_agent<S: Into<String>>(mut self, user_agent: S) -> RustgieClientBuilder {
        const RUSTGIE_VERSION: &str = env!("CARGO_PKG_VERSION");
        self.config.user_agent = Option::from(format!("{} rustgie/{} (+github.com/ashakoor/rustgie)", user_agent.into(), RUSTGIE_VERSION));
        self
    }

    pub fn with_oauth_client_id(mut self, client_id: u32) -> RustgieClientBuilder {
        self.config.oauth_client_id = Option::from(client_id.to_string());
        self
    }

    pub fn with_oauth_client_secret<S: Into<String>>(mut self, client_secret: S) -> RustgieClientBuilder {
        self.config.oauth_client_secret = Option::from(client_secret.into());
        self
    }

    pub fn build(self) -> Result<RustgieClient, String> {
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

    async fn bungie_api_get<T: serde::de::DeserializeOwned>(&self, url: Url, access_token: Option<String>) -> RustgieResult<T> {
        if access_token.is_some() {
            self.process_api_response::<T>(
                self.client
                    .get(url)
                    .bearer_auth(access_token.unwrap())).await
        }
        else {
            self.process_api_response::<T>(
                self.client
                    .get(url)).await
        }
    }

    async fn bungie_api_post<T: serde::de::DeserializeOwned>(&self, url: Url, access_token: Option<String>) -> RustgieResult<T> {
        if access_token.is_some() {
            self.process_api_response::<T>(
                self.client
                    .post(url)
                    .bearer_auth(access_token.unwrap())).await
        }
        else {
            self.process_api_response::<T>(
                self.client
                    .post(url)).await
        }
    }

    async fn bungie_api_post_with_body<T: serde::de::DeserializeOwned, U: serde::Serialize>(&self, url: Url, request_body: U, access_token: Option<String>) -> RustgieResult<T> {
        if access_token.is_some() {
            self.process_api_response::<T>(
                self.client
                    .post(url)
                    .bearer_auth(access_token.unwrap())
                    .json(&request_body)).await
        }
        else {
            self.process_api_response::<T>(
                self.client
                    .post(url)
                    .json(&request_body)).await
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

        if deserialized_response.error_code != rustgie_types::exceptions::PlatformErrorCodes::Success {
            return Err(format!("The Bungie API returned a PlatformErrorCode of {}", deserialized_response.error_code))
        }

        match deserialized_response.response {
            None => { Err("The Bungie API did not include a response".to_string()) }
            Some(resp) => { Ok(resp) }
        }
    }

    /////////////////////////////////////// OAUTH FLOW

    pub fn oauth_get_authorization_url_<S: Into<String>>(&self, language_code: S, state: Option<S>) -> RustgieResult<String> {
        let mut query_params = Vec::<(&str, String)>::new();

        match &self.oauth_client_id {
            None => { return Err("OAuth client ID is required".to_string()) }
            Some(client_id) => {
                query_params.push(("client_id", client_id.to_string()));
            }
        }

        match state {
            None => {}
            Some(state_str) => { query_params.push(("state", state_str.into())); }
        }

        query_params.push(("response_type", "code".to_string()));

        return match Url::parse_with_params(&*format!("https://www.bungie.net/{}/OAuth/Authorize/", language_code.into()), query_params) {
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

    pub async fn oauth_get_auth_token_(&self, auth_code: String) -> RustgieResult<rustgie_types::api_response_::BungieTokenResponse> {
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
        form.insert("code", auth_code);

        self.process_oauth_response(self.client.post("https://www.bungie.net/Platform/App/OAuth/Token/").form(&form)).await
    }

    pub async fn oauth_refresh_auth_token_(&self, refresh_token: String) -> RustgieResult<rustgie_types::api_response_::BungieTokenResponse> {
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
        form.insert("refresh_token", refresh_token);

        self.process_oauth_response(self.client.post("https://www.bungie.net/Platform/App/OAuth/Token/").form(&form)).await
    }

    /////////////////////////////////////// AUTO GENERATED CONTENT BELOW

    pub async fn get_available_locales(&self, access_token: Option<String>) -> RustgieResult<HashMap<String, String>> {
        match Url::parse("https://www.bungie.net/Platform/GetAvailableLocales/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, String>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn get_common_settings(&self, access_token: Option<String>) -> RustgieResult<rustgie_types::common::models::CoreSettingsConfiguration> {
        match Url::parse("https://www.bungie.net/Platform/Settings/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::common::models::CoreSettingsConfiguration>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn get_global_alerts(&self, includestreaming: Option<bool>, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::GlobalAlert>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if includestreaming.is_some(){
            query_params.push(("includestreaming", includestreaming.unwrap().to_string()));
        }
        match Url::parse_with_params("https://www.bungie.net/Platform/GlobalAlerts/", query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::GlobalAlert>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn get_user_system_overrides(&self, access_token: Option<String>) -> RustgieResult<HashMap<String, rustgie_types::common::models::CoreSystem>> {
        match Url::parse("https://www.bungie.net/Platform/UserSystemOverrides/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::common::models::CoreSystem>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn app_get_application_api_usage(&self, application_id: i32, end: Option<time::OffsetDateTime>, start: Option<time::OffsetDateTime>, access_token: Option<String>) -> RustgieResult<rustgie_types::applications::ApiUsage> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if end.is_some(){
            query_params.push(("end", end.unwrap().to_string()));
        }
        if start.is_some(){
            query_params.push(("start", start.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/App/ApiUsage/{}/", application_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::applications::ApiUsage>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn app_get_bungie_applications(&self, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::applications::Application>> {
        match Url::parse("https://www.bungie.net/Platform/App/FirstParty/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::applications::Application>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn community_content_get_community_content(&self, media_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, sort: rustgie_types::forum::CommunityContentSortMode, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/CommunityContent/Get/{}/{}/{}/", sort.to_string(), media_filter.to_string(), page.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_get_content_by_id<S: Display+Into<String>>(&self, id: i64, locale: S, head: Option<bool>, access_token: Option<String>) -> RustgieResult<rustgie_types::content::ContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if head.is_some(){
            query_params.push(("head", head.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/GetContentById/{}/{}/", id.to_string(), locale.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::content::ContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_get_content_by_tag_and_type<S: Display+Into<String>>(&self, locale: S, tag: S, r#type: S, head: Option<bool>, access_token: Option<String>) -> RustgieResult<rustgie_types::content::ContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if head.is_some(){
            query_params.push(("head", head.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/GetContentByTagAndType/{}/{}/{}/", tag.to_string(), r#type.to_string(), locale.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::content::ContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_get_content_type<S: Display+Into<String>>(&self, r#type: S, access_token: Option<String>) -> RustgieResult<rustgie_types::content::models::ContentTypeDescription> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Content/GetContentType/{}/", r#type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::content::models::ContentTypeDescription>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_search_content_by_tag_and_type<S: Display+Into<String>>(&self, locale: S, tag: S, r#type: S, currentpage: Option<i32>, head: Option<bool>, itemsperpage: Option<i32>, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfContentItemPublicContract> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/SearchContentByTagAndType/{}/{}/{}/", tag.to_string(), r#type.to_string(), locale.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_search_content_with_text<S: Display+Into<String>>(&self, locale: S, ctype: Option<S>, currentpage: Option<i32>, head: Option<bool>, searchtext: Option<S>, source: Option<S>, tag: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfContentItemPublicContract> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if ctype.is_some(){
            query_params.push(("ctype", ctype.unwrap().to_string()));
        }
        if currentpage.is_some(){
            query_params.push(("currentpage", currentpage.unwrap().to_string()));
        }
        if head.is_some(){
            query_params.push(("head", head.unwrap().to_string()));
        }
        if searchtext.is_some(){
            query_params.push(("searchtext", searchtext.unwrap().to_string()));
        }
        if source.is_some(){
            query_params.push(("source", source.unwrap().to_string()));
        }
        if tag.is_some(){
            query_params.push(("tag", tag.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Content/Search/{}/", locale.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfContentItemPublicContract>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn content_search_help_articles<S: Display+Into<String>>(&self, searchtext: S, size: S, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::definitions::DestinyDefinition> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Content/SearchHelpArticles/{}/{}/", searchtext.to_string(), size.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::definitions::DestinyDefinition>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_awa_get_action_token<S: Display+Into<String>>(&self, correlation_id: S, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::advanced::AwaAuthorizationResult> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Awa/GetActionToken/{}/", correlation_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::advanced::AwaAuthorizationResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_awa_initialize_request(&self, request_body: rustgie_types::destiny::advanced::AwaPermissionRequested, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::advanced::AwaInitializeResponse> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/Initialize/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::advanced::AwaInitializeResponse, rustgie_types::destiny::advanced::AwaPermissionRequested>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_awa_provide_authorization_result(&self, request_body: rustgie_types::destiny::advanced::AwaUserResponse, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Awa/AwaProvideAuthorizationResult/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::advanced::AwaUserResponse>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_equip_item(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemActionRequest, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItem/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_equip_items(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemSetActionRequest, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::DestinyEquipItemResults> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/EquipItems/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::DestinyEquipItemResults, rustgie_types::destiny::requests::actions::DestinyItemSetActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_activity_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, count: Option<i32>, mode: Option<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>, page: Option<i32>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyActivityHistoryResults> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Account/{}/Character/{}/Stats/Activities/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyActivityHistoryResults>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyCharacterResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Profile/{}/Character/{}/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyCharacterResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_aggregate_stats<S: Display+Into<String>>(&self, group_id: i64, modes: Option<S>, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::destiny::historical_stats::DestinyClanAggregateStat>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/AggregateClanStats/{}/", group_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::destiny::historical_stats::DestinyClanAggregateStat>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_banner_source(&self, access_token: Option<String>) -> RustgieResult<rustgie_types::config::clan_banner::ClanBannerSource> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Clan/ClanBannerDictionary/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::config::clan_banner::ClanBannerSource>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_leaderboards<S: Display+Into<String>>(&self, group_id: i64, maxtop: Option<i32>, modes: Option<S>, statid: Option<S>, access_token: Option<String>) -> RustgieResult<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if maxtop.is_some(){
            query_params.push(("maxtop", maxtop.unwrap().to_string()));
        }
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap().to_string()));
        }
        if statid.is_some(){
            query_params.push(("statid", statid.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/Clans/{}/", group_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_clan_weekly_reward_state(&self, group_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::milestones::DestinyMilestone> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Clan/{}/WeeklyRewardState/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::milestones::DestinyMilestone>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_collectible_node_details(&self, character_id: i64, collectible_presentation_node_hash: u32, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyCollectibleNodeDetailResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Profile/{}/Character/{}/Collectibles/{}/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string(), collectible_presentation_node_hash.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyCollectibleNodeDetailResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_destiny_aggregate_activity_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyAggregateActivityResults> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Account/{}/Character/{}/Stats/AggregateActivityStats/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyAggregateActivityResults>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_destiny_entity_definition<S: Display+Into<String>>(&self, entity_type: S, hash_identifier: u32, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::definitions::DestinyDefinition> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Manifest/{}/{}/", entity_type.to_string(), hash_identifier.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::definitions::DestinyDefinition>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_destiny_manifest(&self, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::config::DestinyManifest> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Manifest/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::config::DestinyManifest>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_historical_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, dayend: Option<time::OffsetDateTime>, daystart: Option<time::OffsetDateTime>, groups: Option<QueryParamVec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, modes: Option<QueryParamVec<rustgie_types::destiny::historical_stats::definitions::DestinyActivityModeType>>, period_type: Option<rustgie_types::destiny::historical_stats::definitions::PeriodType>, access_token: Option<String>) -> RustgieResult<HashMap<String, rustgie_types::destiny::historical_stats::DestinyHistoricalStatsByPeriod>> {
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
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Account/{}/Character/{}/Stats/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::destiny::historical_stats::DestinyHistoricalStatsByPeriod>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_historical_stats_definition(&self, access_token: Option<String>) -> RustgieResult<HashMap<String, rustgie_types::destiny::historical_stats::definitions::DestinyHistoricalStatsDefinition>> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Stats/Definition/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::destiny::historical_stats::definitions::DestinyHistoricalStatsDefinition>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_historical_stats_for_account(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, groups: Option<QueryParamVec<rustgie_types::destiny::historical_stats::definitions::DestinyStatsGroupType>>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyHistoricalStatsAccountResult> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if groups.is_some(){
            query_params.push(("groups", groups.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Account/{}/Stats/", membership_type.to_string(), destiny_membership_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyHistoricalStatsAccountResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_item(&self, destiny_membership_id: i64, item_instance_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyItemResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Profile/{}/Item/{}/", membership_type.to_string(), destiny_membership_id.to_string(), item_instance_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyItemResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_leaderboards<S: Display+Into<String>>(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<S>, statid: Option<S>, access_token: Option<String>) -> RustgieResult<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if maxtop.is_some(){
            query_params.push(("maxtop", maxtop.unwrap().to_string()));
        }
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap().to_string()));
        }
        if statid.is_some(){
            query_params.push(("statid", statid.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Account/{}/Stats/Leaderboards/", membership_type.to_string(), destiny_membership_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_leaderboards_for_character<S: Display+Into<String>>(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, maxtop: Option<i32>, modes: Option<S>, statid: Option<S>, access_token: Option<String>) -> RustgieResult<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if maxtop.is_some(){
            query_params.push(("maxtop", maxtop.unwrap().to_string()));
        }
        if modes.is_some(){
            query_params.push(("modes", modes.unwrap().to_string()));
        }
        if statid.is_some(){
            query_params.push(("statid", statid.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/Leaderboards/{}/{}/{}/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, HashMap<String, rustgie_types::destiny::historical_stats::DestinyLeaderboard>>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_linked_profiles(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, get_all_memberships: Option<bool>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyLinkedProfilesResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if get_all_memberships.is_some(){
            query_params.push(("getAllMemberships", get_all_memberships.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Profile/{}/LinkedProfiles/", membership_type.to_string(), membership_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyLinkedProfilesResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_post_game_carnage_report(&self, activity_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyPostGameCarnageReportData> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{}/", activity_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyPostGameCarnageReportData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_profile(&self, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyProfileResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Profile/{}/", membership_type.to_string(), destiny_membership_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyProfileResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_public_milestone_content(&self, milestone_hash: u32, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::milestones::DestinyMilestoneContent> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Milestones/{}/Content/", milestone_hash.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::milestones::DestinyMilestoneContent>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_public_milestones(&self, access_token: Option<String>) -> RustgieResult<HashMap<u32, rustgie_types::destiny::milestones::DestinyPublicMilestone>> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Milestones/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<u32, rustgie_types::destiny::milestones::DestinyPublicMilestone>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_public_vendors(&self, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyPublicVendorsResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        match Url::parse_with_params("https://www.bungie.net/Platform/Destiny2/Vendors/", query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyPublicVendorsResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_unique_weapon_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::historical_stats::DestinyHistoricalWeaponStatsData> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Account/{}/Character/{}/Stats/UniqueWeapons/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::historical_stats::DestinyHistoricalWeaponStatsData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_vendor(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, vendor_hash: u32, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyVendorResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Profile/{}/Character/{}/Vendors/{}/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string(), vendor_hash.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyVendorResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_get_vendors(&self, character_id: i64, destiny_membership_id: i64, membership_type: rustgie_types::BungieMembershipType, components: Option<QueryParamVec<rustgie_types::destiny::DestinyComponentType>>, filter: Option<rustgie_types::destiny::DestinyVendorFilter>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyVendorsResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if components.is_some(){
            query_params.push(("components", components.unwrap().to_string()));
        }
        if filter.is_some(){
            query_params.push(("filter", filter.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/{}/Profile/{}/Character/{}/Vendors/", membership_type.to_string(), destiny_membership_id.to_string(), character_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::responses::DestinyVendorsResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_insert_socket_plug(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsActionRequest, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyItemChangeResponse> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlug/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::responses::DestinyItemChangeResponse, rustgie_types::destiny::requests::actions::DestinyInsertPlugsActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_insert_socket_plug_free(&self, request_body: rustgie_types::destiny::requests::actions::DestinyInsertPlugsFreeActionRequest, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::responses::DestinyItemChangeResponse> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/InsertSocketPlugFree/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::destiny::responses::DestinyItemChangeResponse, rustgie_types::destiny::requests::actions::DestinyInsertPlugsFreeActionRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_pull_from_postmaster(&self, request_body: rustgie_types::destiny::requests::actions::DestinyPostmasterTransferRequest, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/PullFromPostmaster/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyPostmasterTransferRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_report_offensive_post_game_carnage_report_player(&self, activity_id: i64, request_body: rustgie_types::destiny::reporting::requests::DestinyReportOffensePgcrRequest, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{}/Report/", activity_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::reporting::requests::DestinyReportOffensePgcrRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_search_destiny_entities<S: Display+Into<String>>(&self, search_term: S, r#type: S, page: Option<i32>, access_token: Option<String>) -> RustgieResult<rustgie_types::destiny::definitions::DestinyEntitySearchResult> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if page.is_some(){
            query_params.push(("page", page.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Destiny2/Armory/Search/{}/{}/", r#type.to_string(), search_term.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::destiny::definitions::DestinyEntitySearchResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_search_destiny_player_by_bungie_name(&self, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::user::ExactSearchRequest, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::user::UserInfoCard>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayerByBungieName/{}/", membership_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::user::UserInfoCard>, rustgie_types::user::ExactSearchRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_set_item_lock_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetLockState/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemStateRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_set_quest_tracked_state(&self, request_body: rustgie_types::destiny::requests::actions::DestinyItemStateRequest, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/SetTrackedState/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::actions::DestinyItemStateRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn destiny2_transfer_item(&self, request_body: rustgie_types::destiny::requests::DestinyItemTransferRequest, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse("https://www.bungie.net/Platform/Destiny2/Actions/Items/TransferItem/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::destiny::requests::DestinyItemTransferRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_active_private_clan_fireteam_count(&self, group_id: i64, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{}/ActiveCount/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<i32>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_available_clan_fireteams<S: Display+Into<String>>(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, group_id: i64, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, public_only: rustgie_types::fireteam::FireteamPublicSearchOption, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfFireteamSummary> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if lang_filter.is_some(){
            query_params.push(("langFilter", lang_filter.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{}/Available/{}/{}/{}/{}/{}/{}/", group_id.to_string(), platform.to_string(), activity_type.to_string(), date_range.to_string(), slot_filter.to_string(), public_only.to_string(), page.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfFireteamSummary>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_clan_fireteam(&self, fireteam_id: i64, group_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::fireteam::FireteamResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{}/Summary/{}/", group_id.to_string(), fireteam_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::fireteam::FireteamResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_get_my_clan_fireteams<S: Display+Into<String>>(&self, group_id: i64, include_closed: bool, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, group_filter: Option<bool>, lang_filter: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfFireteamResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if group_filter.is_some(){
            query_params.push(("groupFilter", group_filter.unwrap().to_string()));
        }
        if lang_filter.is_some(){
            query_params.push(("langFilter", lang_filter.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Clan/{}/My/{}/{}/{}/", group_id.to_string(), platform.to_string(), include_closed.to_string(), page.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfFireteamResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn fireteam_search_public_available_clan_fireteams<S: Display+Into<String>>(&self, activity_type: i32, date_range: rustgie_types::fireteam::FireteamDateRange, page: i32, platform: rustgie_types::fireteam::FireteamPlatform, slot_filter: rustgie_types::fireteam::FireteamSlotSearch, lang_filter: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfFireteamSummary> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if lang_filter.is_some(){
            query_params.push(("langFilter", lang_filter.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Fireteam/Search/Available/{}/{}/{}/{}/{}/", platform.to_string(), activity_type.to_string(), date_range.to_string(), slot_filter.to_string(), page.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfFireteamSummary>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_core_topics_paged<S: Display+Into<String>>(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, page: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if locales.is_some(){
            query_params.push(("locales", locales.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetCoreTopicsPaged/{}/{}/{}/{}/", page.to_string(), sort.to_string(), quick_date.to_string(), category_filter.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_forum_tag_suggestions<S: Display+Into<String>>(&self, partialtag: Option<S>, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::tags::models::contracts::TagResponse>> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if partialtag.is_some(){
            query_params.push(("partialtag", partialtag.unwrap().to_string()));
        }
        match Url::parse_with_params("https://www.bungie.net/Platform/Forum/GetForumTagSuggestions/", query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::tags::models::contracts::TagResponse>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_poll(&self, topic_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Forum/Poll/{}/", topic_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_post_and_parent<S: Display+Into<String>>(&self, child_post_id: i64, showbanned: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostAndParent/{}/", child_post_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_post_and_parent_awaiting_approval<S: Display+Into<String>>(&self, child_post_id: i64, showbanned: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostAndParentAwaitingApproval/{}/", child_post_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_posts_threaded_paged<S: Display+Into<String>>(&self, get_parent_post: bool, page: i32, page_size: i32, parent_post_id: i64, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPaged/{}/{}/{}/{}/{}/{}/{}/", parent_post_id.to_string(), page.to_string(), page_size.to_string(), reply_size.to_string(), get_parent_post.to_string(), root_thread_mode.to_string(), sort_mode.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_posts_threaded_paged_from_child<S: Display+Into<String>>(&self, child_post_id: i64, page: i32, page_size: i32, reply_size: i32, root_thread_mode: bool, sort_mode: rustgie_types::forum::ForumPostSortEnum, showbanned: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if showbanned.is_some(){
            query_params.push(("showbanned", showbanned.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetPostsThreadedPagedFromChild/{}/{}/{}/{}/{}/{}/", child_post_id.to_string(), page.to_string(), page_size.to_string(), reply_size.to_string(), root_thread_mode.to_string(), sort_mode.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_recruitment_thread_summaries(&self, request_body: Vec<i64>, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::forum::ForumRecruitmentDetail>> {
        match Url::parse("https://www.bungie.net/Platform/Forum/Recruit/Summaries/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::forum::ForumRecruitmentDetail>, Vec<i64>>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_topic_for_content(&self, content_id: i64, access_token: Option<String>) -> RustgieResult<i64> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Forum/GetTopicForContent/{}/", content_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<i64>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn forum_get_topics_paged<S: Display+Into<String>>(&self, category_filter: rustgie_types::forum::ForumTopicsCategoryFiltersEnum, group: i64, page: i32, page_size: i32, quick_date: rustgie_types::forum::ForumTopicsQuickDateEnum, sort: rustgie_types::forum::ForumTopicsSortEnum, locales: Option<S>, tagstring: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::forum::PostSearchResponse> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if locales.is_some(){
            query_params.push(("locales", locales.unwrap().to_string()));
        }
        if tagstring.is_some(){
            query_params.push(("tagstring", tagstring.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/Forum/GetTopicsPaged/{}/{}/{}/{}/{}/{}/", page.to_string(), page_size.to_string(), group.to_string(), sort.to_string(), quick_date.to_string(), category_filter.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::forum::PostSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_abdicate_foundership(&self, founder_id_new: i64, group_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Admin/AbdicateFoundership/{}/{}/", group_id.to_string(), membership_type.to_string(), founder_id_new.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_add_optional_conversation(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationAddRequest, access_token: Option<String>) -> RustgieResult<i64> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/OptionalConversations/Add/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i64, rustgie_types::groups_v2::GroupOptionalConversationAddRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_approve_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/ApproveAll/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_approve_pending(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/Approve/{}/{}/", group_id.to_string(), membership_type.to_string(), membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<bool, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_approve_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/ApproveList/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationListRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_ban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupBanRequest, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/{}/{}/Ban/", group_id.to_string(), membership_type.to_string(), membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupBanRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_deny_all_pending(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/DenyAll/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_deny_pending_for_list(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupApplicationListRequest, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::entities::EntityActionResult>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/DenyList/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<Vec<rustgie_types::entities::EntityActionResult>, rustgie_types::groups_v2::GroupApplicationListRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_clan_banner(&self, group_id: i64, request_body: rustgie_types::groups_v2::ClanBanner, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/EditClanBanner/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::ClanBanner>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_founder_options(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionsEditAction, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/EditFounderOptions/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupOptionsEditAction>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_group(&self, group_id: i64, request_body: rustgie_types::groups_v2::GroupEditAction, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Edit/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i32, rustgie_types::groups_v2::GroupEditAction>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_group_membership(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, member_type: rustgie_types::groups_v2::RuntimeGroupMemberType, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/{}/{}/SetMembershipType/{}/", group_id.to_string(), membership_type.to_string(), membership_id.to_string(), member_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<i32>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_edit_optional_conversation(&self, conversation_id: i64, group_id: i64, request_body: rustgie_types::groups_v2::GroupOptionalConversationEditRequest, access_token: Option<String>) -> RustgieResult<i64> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/OptionalConversations/Edit/{}/", group_id.to_string(), conversation_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<i64, rustgie_types::groups_v2::GroupOptionalConversationEditRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_admins_and_founder_of_group(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfGroupMember> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/AdminsAndFounder/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMember>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_available_avatars(&self, access_token: Option<String>) -> RustgieResult<HashMap<i32, String>> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableAvatars/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<i32, String>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_available_themes(&self, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::config::GroupTheme>> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/GetAvailableThemes/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::config::GroupTheme>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_banned_members_of_group(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfGroupBan> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Banned/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupBan>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group(&self, group_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group_by_name<S: Display+Into<String>>(&self, group_name: S, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Name/{}/{}/", group_name.to_string(), group_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group_by_name_v2(&self, request_body: rustgie_types::groups_v2::GroupNameSearchRequest, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupResponse> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/NameV2/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::groups_v2::GroupResponse, rustgie_types::groups_v2::GroupNameSearchRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_group_optional_conversations(&self, group_id: i64, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::groups_v2::GroupOptionalConversation>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/OptionalConversations/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::groups_v2::GroupOptionalConversation>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupsForMemberFilter, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GetGroupsForMemberResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/User/{}/{}/{}/{}/", membership_type.to_string(), membership_id.to_string(), filter.to_string(), group_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GetGroupsForMemberResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_invited_individuals(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfGroupMemberApplication> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/InvitedIndividuals/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMemberApplication>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_members_of_group<S: Display+Into<String>>(&self, currentpage: i32, group_id: i64, member_type: Option<rustgie_types::groups_v2::RuntimeGroupMemberType>, name_search: Option<S>, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfGroupMember> {
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if member_type.is_some(){
            query_params.push(("memberType", member_type.unwrap().to_string()));
        }
        if name_search.is_some(){
            query_params.push(("nameSearch", name_search.unwrap().to_string()));
        }
        match Url::parse_with_params(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/", group_id.to_string()), query_params) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMember>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_pending_memberships(&self, currentpage: i32, group_id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfGroupMemberApplication> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/Pending/", group_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfGroupMemberApplication>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_potential_groups_for_member(&self, filter: rustgie_types::groups_v2::GroupPotentialMemberStatus, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupPotentialMembershipSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/User/Potential/{}/{}/{}/{}/", membership_type.to_string(), membership_id.to_string(), filter.to_string(), group_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupPotentialMembershipSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_recommended_groups(&self, create_date_range: rustgie_types::groups_v2::GroupDateRange, group_type: rustgie_types::groups_v2::GroupType, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::groups_v2::GroupV2Card>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Recommended/{}/{}/", group_type.to_string(), create_date_range.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<Vec<rustgie_types::groups_v2::GroupV2Card>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_get_user_clan_invite_setting(&self, m_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/GetUserClanInviteSetting/{}/", m_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_group_search(&self, request_body: rustgie_types::groups_v2::GroupQuery, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupSearchResponse> {
        match Url::parse("https://www.bungie.net/Platform/GroupV2/Search/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::groups_v2::GroupSearchResponse, rustgie_types::groups_v2::GroupQuery>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_individual_group_invite(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, request_body: rustgie_types::groups_v2::GroupApplicationRequest, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupApplicationResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/IndividualInvite/{}/{}/", group_id.to_string(), membership_type.to_string(), membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::groups_v2::GroupApplicationResponse, rustgie_types::groups_v2::GroupApplicationRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_individual_group_invite_cancel(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupApplicationResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/IndividualInviteCancel/{}/{}/", group_id.to_string(), membership_type.to_string(), membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<rustgie_types::groups_v2::GroupApplicationResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_kick_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupMemberLeaveResult> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/{}/{}/Kick/", group_id.to_string(), membership_type.to_string(), membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<rustgie_types::groups_v2::GroupMemberLeaveResult>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_recover_group_for_founder(&self, group_type: rustgie_types::groups_v2::GroupType, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::groups_v2::GroupMembershipSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/Recover/{}/{}/{}/", membership_type.to_string(), membership_id.to_string(), group_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::groups_v2::GroupMembershipSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn group_v2_unban_member(&self, group_id: i64, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<i32> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/GroupV2/{}/Members/{}/{}/Unban/", group_id.to_string(), membership_type.to_string(), membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<i32>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_accept_friend_request<S: Display+Into<String>>(&self, membership_id: S, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Accept/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_decline_friend_request<S: Display+Into<String>>(&self, membership_id: S, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Decline/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_get_friend_list(&self, access_token: Option<String>) -> RustgieResult<rustgie_types::social::friends::BungieFriendListResponse> {
        match Url::parse("https://www.bungie.net/Platform/Social/Friends/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::social::friends::BungieFriendListResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_get_friend_request_list(&self, access_token: Option<String>) -> RustgieResult<rustgie_types::social::friends::BungieFriendRequestListResponse> {
        match Url::parse("https://www.bungie.net/Platform/Social/Friends/Requests/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::social::friends::BungieFriendRequestListResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_get_platform_friend_list<S: Display+Into<String>>(&self, friend_platform: rustgie_types::social::friends::PlatformFriendType, page: S, access_token: Option<String>) -> RustgieResult<rustgie_types::social::friends::PlatformFriendResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/PlatformFriends/{}/{}/", friend_platform.to_string(), page.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::social::friends::PlatformFriendResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_issue_friend_request<S: Display+Into<String>>(&self, membership_id: S, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Add/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_remove_friend<S: Display+Into<String>>(&self, membership_id: S, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Remove/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn social_remove_friend_request<S: Display+Into<String>>(&self, membership_id: S, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Social/Friends/Requests/Remove/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_apply_missing_partner_offers_without_claim(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Partner/ApplyMissingOffers/{}/{}/", partner_application_id.to_string(), target_bnet_membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post::<bool>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_claim_partner_offer(&self, request_body: rustgie_types::tokens::PartnerOfferClaimRequest, access_token: Option<String>) -> RustgieResult<bool> {
        match Url::parse("https://www.bungie.net/Platform/Tokens/Partner/ClaimOffer/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<bool, rustgie_types::tokens::PartnerOfferClaimRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_bungie_rewards_for_user(&self, membership_id: i64, access_token: Option<String>) -> RustgieResult<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Rewards/GetRewardsForUser/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_bungie_rewards_list(&self, access_token: Option<String>) -> RustgieResult<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>> {
        match Url::parse("https://www.bungie.net/Platform/Tokens/Rewards/BungieRewards/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<String, rustgie_types::tokens::BungieRewardDisplay>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn tokens_get_partner_offer_sku_history(&self, partner_application_id: i32, target_bnet_membership_id: i64, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::tokens::PartnerOfferSkuHistoryResponse>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Tokens/Partner/History/{}/{}/", partner_application_id.to_string(), target_bnet_membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::tokens::PartnerOfferSkuHistoryResponse>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn trending_get_trending_categories(&self, access_token: Option<String>) -> RustgieResult<rustgie_types::trending::TrendingCategories> {
        match Url::parse("https://www.bungie.net/Platform/Trending/Categories/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::trending::TrendingCategories>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn trending_get_trending_category<S: Display+Into<String>>(&self, category_id: S, page_number: i32, access_token: Option<String>) -> RustgieResult<rustgie_types::SearchResultOfTrendingEntry> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Trending/Categories/{}/{}/", category_id.to_string(), page_number.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::SearchResultOfTrendingEntry>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn trending_get_trending_entry_detail<S: Display+Into<String>>(&self, identifier: S, trending_entry_type: rustgie_types::trending::TrendingEntryType, access_token: Option<String>) -> RustgieResult<rustgie_types::trending::TrendingDetail> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/Trending/Details/{}/{}/", trending_entry_type.to_string(), identifier.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::trending::TrendingDetail>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_available_themes(&self, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::config::UserTheme>> {
        match Url::parse("https://www.bungie.net/Platform/User/GetAvailableThemes/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::config::UserTheme>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_bungie_net_user_by_id(&self, id: i64, access_token: Option<String>) -> RustgieResult<rustgie_types::user::GeneralUser> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetBungieNetUserById/{}/", id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::GeneralUser>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_credential_types_for_target_account(&self, membership_id: i64, access_token: Option<String>) -> RustgieResult<Vec<rustgie_types::user::models::GetCredentialTypesForAccountResponse>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetCredentialTypesForTargetAccount/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<Vec<rustgie_types::user::models::GetCredentialTypesForAccountResponse>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_membership_data_by_id(&self, membership_id: i64, membership_type: rustgie_types::BungieMembershipType, access_token: Option<String>) -> RustgieResult<rustgie_types::user::UserMembershipData> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetMembershipsById/{}/{}/", membership_id.to_string(), membership_type.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::UserMembershipData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_membership_data_for_current_user(&self, access_token: Option<String>) -> RustgieResult<rustgie_types::user::UserMembershipData> {
        match Url::parse("https://www.bungie.net/Platform/User/GetMembershipsForCurrentUser/") {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::UserMembershipData>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_membership_from_hard_linked_credential<S: Display+Into<String>>(&self, credential: S, cr_type: rustgie_types::BungieCredentialType, access_token: Option<String>) -> RustgieResult<rustgie_types::user::HardLinkedUserMembership> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/{}/{}/", cr_type.to_string(), credential.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::HardLinkedUserMembership>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_get_sanitized_platform_display_names(&self, membership_id: i64, access_token: Option<String>) -> RustgieResult<HashMap<u8, String>> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/GetSanitizedPlatformDisplayNames/{}/", membership_id.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<HashMap<u8, String>>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_search_by_global_name_post(&self, page: i32, request_body: rustgie_types::user::UserSearchPrefixRequest, access_token: Option<String>) -> RustgieResult<rustgie_types::user::UserSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/Search/GlobalName/{}/", page.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_post_with_body::<rustgie_types::user::UserSearchResponse, rustgie_types::user::UserSearchPrefixRequest>(self, url, request_body, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }

    pub async fn user_search_by_global_name_prefix<S: Display+Into<String>>(&self, display_name_prefix: S, page: i32, access_token: Option<String>) -> RustgieResult<rustgie_types::user::UserSearchResponse> {
        match Url::parse(&*format!("https://www.bungie.net/Platform/User/Search/Prefix/{}/{}/", display_name_prefix.to_string(), page.to_string())) {
            Ok(url) => { Ok(RustgieClient::bungie_api_get::<rustgie_types::user::UserSearchResponse>(self, url, access_token).await?) }
            Err(_) => { Err("There was an error while parsing the URL".to_string()) }
        }
    }
}
