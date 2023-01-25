use anyhow::{anyhow, Result};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct IgnoreResponse {
    #[serde(rename = "isIgnored")]
    pub is_ignored: bool,

    #[serde(rename = "ignoreFlags")]
    pub ignore_flags: enumflags2::BitFlags<crate::ignores::IgnoreStatus>,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for IgnoreStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "IgnoredUser" => Ok(IgnoreStatus::IgnoredUser),
            "IgnoredGroup" => Ok(IgnoreStatus::IgnoredGroup),
            "IgnoredByGroup" => Ok(IgnoreStatus::IgnoredByGroup),
            "IgnoredPost" => Ok(IgnoreStatus::IgnoredPost),
            "IgnoredTag" => Ok(IgnoreStatus::IgnoredTag),
            "IgnoredGlobal" => Ok(IgnoreStatus::IgnoredGlobal),
            _ => Err(anyhow!("Could not deserialize string '{}' to IgnoreStatus", s)),
        }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for IgnoreLength {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(IgnoreLength::None),
            "Week" => Ok(IgnoreLength::Week),
            "TwoWeeks" => Ok(IgnoreLength::TwoWeeks),
            "ThreeWeeks" => Ok(IgnoreLength::ThreeWeeks),
            "Month" => Ok(IgnoreLength::Month),
            "ThreeMonths" => Ok(IgnoreLength::ThreeMonths),
            "SixMonths" => Ok(IgnoreLength::SixMonths),
            "Year" => Ok(IgnoreLength::Year),
            "Forever" => Ok(IgnoreLength::Forever),
            "ThreeMinutes" => Ok(IgnoreLength::ThreeMinutes),
            "Hour" => Ok(IgnoreLength::Hour),
            "ThirtyDays" => Ok(IgnoreLength::ThirtyDays),
            _ => Err(anyhow!("Could not deserialize string '{}' to IgnoreLength", s)),
        }
    }
}
