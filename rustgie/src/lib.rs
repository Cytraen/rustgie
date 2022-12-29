#![forbid(unsafe_code)]

pub mod endpoints;

pub use rustgie_types as types;

use anyhow::{anyhow, Result, Context};
use reqwest::Url;
use rustgie_types::api_response_::BungieApiResponse;
use std::collections::HashMap;

#[must_use]
pub struct RustgieClientBuilder {
    api_key: Option<String>,
    user_agent: Option<String>,
    oauth_client_id: Option<String>,
    oauth_client_secret: Option<String>,
}

impl RustgieClientBuilder {
    #[must_use]
    pub fn new() -> RustgieClientBuilder {
        RustgieClientBuilder {
            api_key: None,
            user_agent: None,
            oauth_client_id: None,
            oauth_client_secret: None,
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> RustgieClientBuilder {
        self.api_key = Option::from(api_key.to_string());
        self
    }

    pub fn with_user_agent(mut self, user_agent: &str) -> RustgieClientBuilder {
        const RUSTGIE_VERSION: &str = env!("CARGO_PKG_VERSION");
        self.user_agent = Option::from(format!(
            "{user_agent} rustgie/{RUSTGIE_VERSION} (+github.com/ashakoor/rustgie)"
        ));
        self
    }

    pub fn with_oauth_client_id(mut self, client_id: u32) -> RustgieClientBuilder {
        self.oauth_client_id = Option::from(client_id.to_string());
        self
    }

    pub fn with_oauth_client_secret(mut self, client_secret: &str) -> RustgieClientBuilder {
        self.oauth_client_secret = Option::from(client_secret.to_string());
        self
    }

    #[must_use]
    pub fn build(self) -> Result<RustgieClient> {
        let mut header_map = reqwest::header::HeaderMap::new();

        match self.api_key {
            None => return Err(anyhow!("An API key is required.")),
            Some(key) => {
                header_map.insert(
                    "X-API-Key",
                    reqwest::header::HeaderValue::try_from(key)?,
                );
            }
        }

        match self.user_agent {
            None => {}
            Some(ua) => {
                header_map.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::try_from(ua)?,
                );
            }
        }

        RustgieClient::new(
            header_map,
            self.oauth_client_id,
            self.oauth_client_secret,
        )
    }
}

impl Default for RustgieClientBuilder {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub struct RustgieClient {
    client: reqwest::Client,
    oauth_client_id: Option<String>,
    oauth_client_secret: Option<String>,
}

impl RustgieClient {
    fn new(
        default_headers: reqwest::header::HeaderMap,
        client_id: Option<String>,
        client_secret: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            client: reqwest::ClientBuilder::new()
                .brotli(true)
                .gzip(true)
                .deflate(true)
                .https_only(true)
                .cookie_store(true)
                .redirect(reqwest::redirect::Policy::none())
                .default_headers(default_headers)
                .build()?,
            oauth_client_id: client_id,
            oauth_client_secret: client_secret,
        })
    }

    #[must_use]
    pub fn builder() -> RustgieClientBuilder {
        RustgieClientBuilder::new()
    }

    async fn bungie_api_get<T: serde::de::DeserializeOwned>(
        &self,
        url: Url,
        access_token: Option<&str>,
    ) -> Result<T> {
        let request = self.client.get(url);

        match access_token {
            None => self.process_api_response::<T>(request).await,
            Some(at) => {
                self.process_api_response::<T>(request.bearer_auth(at.to_string()))
                    .await
            }
        }
    }

    async fn bungie_api_post<T: serde::de::DeserializeOwned>(
        &self,
        url: Url,
        access_token: Option<&str>,
    ) -> Result<T> {
        let request = self.client.post(url);

        match access_token {
            None => self.process_api_response::<T>(request).await,
            Some(at) => {
                self.process_api_response::<T>(request.bearer_auth(at))
                    .await
            }
        }
    }

    async fn bungie_api_post_with_body<T: serde::de::DeserializeOwned, U: serde::Serialize>(
        &self,
        url: Url,
        request_body: U,
        access_token: Option<&str>,
    ) -> Result<T> {
        let request = self.client.post(url).json(&request_body);

        match access_token {
            None => self.process_api_response::<T>(request).await,
            Some(at) => {
                self.process_api_response::<T>(request.bearer_auth(at))
                    .await
            }
        }
    }

    async fn process_api_response<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T> {
        let http_response = request.send().await.with_context(|| "There was an error connecting to the Bungie API")?;

        let headers = http_response.headers();

        if !headers.contains_key("Content-Type") {
            return Err(anyhow!("'Content-Type' header is not present"));
        }

        if !headers["Content-Type"]
                .to_str().with_context(|| "Could not parse Content-Type header from Bungie API as string")?
                .starts_with("application/json")
        {
            return Err(anyhow!("'Content-Type' of response was not 'application/json'"));
        }

        let deserialized_response = http_response.json::<BungieApiResponse<T>>()
        .await
        .with_context(|| "There was an error deserializing the JSON response")?;

        match deserialized_response.error_code {
            rustgie_types::exceptions::PlatformErrorCodes::Success => {
                match deserialized_response.response {
                    None => Err(anyhow!("The Bungie API did not include a response")),
                    Some(resp) => Ok(resp),
                }
            }
            error_code => Err(anyhow!(
                "The Bungie API returned a PlatformErrorCode of {error_code}"
            )),
        }
    }

    /////////////////////////////////////// OAUTH FLOW

    pub fn oauth_get_authorization_url_(
        &self,
        language_code: &str,
        state: Option<&str>,
    ) -> Result<String> {
        let mut query_params = Vec::<(&str, &str)>::new();

        match &self.oauth_client_id {
            None => return Err(anyhow!("OAuth client ID is required")),
            Some(client_id) => {
                query_params.push(("client_id", client_id));
            }
        }

        match state {
            None => {}
            Some(state_str) => {
                query_params.push(("state", state_str));
            }
        }

        query_params.push(("response_type", "code"));

        return Ok(Url::parse_with_params(
            &format!("https://www.bungie.net/{language_code}/OAuth/Authorize/"),
            query_params,
        ).with_context(|| "Error parsing OAuth authorization URL")?.to_string());

    }

    async fn process_oauth_response(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<rustgie_types::api_response_::BungieTokenResponse> {
        let http_response = request.send().await.with_context(|| "There was an error connecting to the Bungie API")?;

        let headers = http_response.headers();

        if !headers.contains_key("Content-Type") {
            return Err(anyhow!("'Content-Type' header is not present"));
        }

        if !headers["Content-Type"]
                .to_str().with_context(|| "Could not parse Content-Type header from Bungie API as string")?
                .starts_with("application/json")
        {
            return Err(anyhow!("'Content-Type' of response was not 'application/json'"));
        }

        let deserialized_response = http_response.json::<rustgie_types::api_response_::BungieTokenResponse>()
        .await.with_context(|| "There was an error deserializing the JSON response")?;

        match deserialized_response.access_token {
            None => Err(anyhow!("The Bungie API did not include an access token")),
            Some(_) => Ok(deserialized_response),
        }
    }

    pub async fn oauth_get_auth_token_(
        &self,
        auth_code: &str,
    ) -> Result<rustgie_types::api_response_::BungieTokenResponse> {
        let mut form = HashMap::<&str, &str>::new();

        match &self.oauth_client_id {
            None => return Err(anyhow!("OAuth client ID is required")),
            Some(client_id) => {
                form.insert("client_id", client_id);
            }
        }

        match &self.oauth_client_secret {
            None => {}
            Some(client_secret) => {
                form.insert("client_secret", client_secret);
            }
        }

        form.insert("grant_type", "authorization_code");
        form.insert("code", auth_code);

        self.process_oauth_response(
            self.client
                .post("https://www.bungie.net/Platform/App/OAuth/Token/")
                .form(&form),
        )
        .await
    }

    pub async fn oauth_refresh_auth_token_(
        &self,
        refresh_token: &str,
    ) -> Result<rustgie_types::api_response_::BungieTokenResponse> {
        let mut form = HashMap::<&str, &str>::new();

        match &self.oauth_client_id {
            None => return Err(anyhow!("OAuth client ID is required")),
            Some(client_id) => {
                form.insert("client_id", client_id);
            }
        }

        match &self.oauth_client_secret {
            None => return Err(anyhow!("OAuth client secret is required")),
            Some(client_secret) => {
                form.insert("client_secret", client_secret);
            }
        }

        form.insert("grant_type", "refresh_token");
        form.insert("refresh_token", refresh_token);

        self.process_oauth_response(
            self.client
                .post("https://www.bungie.net/Platform/App/OAuth/Token/")
                .form(&form),
        )
        .await
    }
}
