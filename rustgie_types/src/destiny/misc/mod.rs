use serde::{Deserialize, Serialize};

/// Represents a color whose RGBA values are all represented as values between 0 and 255.
#[derive(Deserialize, Serialize)]
pub struct DestinyColor {
    #[serde(rename = "red")]
    pub red: u8,

    #[serde(rename = "green")]
    pub green: u8,

    #[serde(rename = "blue")]
    pub blue: u8,

    #[serde(rename = "alpha")]
    pub alpha: u8,
}
