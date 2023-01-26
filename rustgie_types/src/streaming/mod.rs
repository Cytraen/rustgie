use anyhow::{anyhow, Result};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DropStateEnum {
    Claimed = 0,
    Applied = 1,
    Fulfilled = 2,
}

impl Display for DropStateEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for DropStateEnum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Claimed" => Ok(DropStateEnum::Claimed),
            "Applied" => Ok(DropStateEnum::Applied),
            "Fulfilled" => Ok(DropStateEnum::Fulfilled),
            _ => Err(anyhow!("Could not deserialize string '{}' to DropStateEnum", s)),
        }
    }
}
