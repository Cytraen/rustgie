use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DateRange {
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "start")]
    pub start: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "end")]
    pub end: OffsetDateTime,
}
