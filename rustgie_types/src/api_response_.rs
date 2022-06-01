use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BungieApiResponse<T>
{
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
