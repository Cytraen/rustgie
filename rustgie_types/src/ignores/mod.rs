use std::fmt::{Display, Formatter, Result};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize)]
pub struct IgnoreResponse {
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,

    #[serde(rename = "ignoreFlags")]
    pub ignore_flags: crate::ignores::IgnoreStatus,
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IgnoreStatus {
    IgnoredUser = 1,
    IgnoredGroup = 2,
    IgnoredByGroup = 4,
    IgnoredPost = 8,
    IgnoredTag = 16,
    IgnoredGlobal = 32,
}

impl Display for IgnoreStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IgnoreLength {
    None = 0,
    Week = 1,
    TwoWeeks = 2,
    ThreeWeeks = 3,
    Month = 4,
    ThreeMonths = 5,
    SixMonths = 6,
    Year = 7,
    Forever = 8,
    ThreeMinutes = 9,
    Hour = 10,
    ThirtyDays = 11,
}

impl Display for IgnoreLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as i32)
    }
}
