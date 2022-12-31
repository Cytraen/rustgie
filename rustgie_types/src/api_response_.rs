use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct BungieApiResponse<T> {
    #[serde(rename = "Response")]
    pub response: Option<T>,

    #[serde(rename = "ErrorCode")]
    pub error_code: crate::exceptions::PlatformErrorCodes,

    #[serde(rename = "ThrottleSeconds")]
    pub throttle_seconds: i32,

    #[serde(rename = "ErrorStatus")]
    pub error_status: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "MessageData")]
    pub message_data: HashMap<String, String>,

    #[serde(rename = "DetailedErrorTrace")]
    pub detailed_error_trace: Option<String>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct BungieTokenResponse {
    #[serde(rename = "access_token")]
    pub access_token: Option<String>,

    #[serde(rename = "token_type")]
    pub token_type: Option<String>,

    #[serde(rename = "expires_in")]
    pub expires_in: Option<i32>,

    #[serde(rename = "refresh_token")]
    pub refresh_token: Option<String>,

    #[serde(rename = "refresh_expires_in")]
    pub refresh_expires_in: Option<i32>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "membership_id")]
    pub membership_id: Option<i64>,

    #[serde(rename = "error")]
    pub error: Option<String>,

    #[serde(rename = "error_description")]
    pub error_description: Option<String>,
}
