use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The base class for any component-returning object that may need to indicate information about the state of the component being returned.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ComponentResponse {
    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

/// A set of flags for reason(s) why the component populated in the way that it did. Inspect the individual flags for the reasons.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentPrivacySetting {
    None = 0,
    Public = 1,
    Private = 2,
}

impl Display for ComponentPrivacySetting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ComponentPrivacySetting {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(ComponentPrivacySetting::None),
            "Public" => Ok(ComponentPrivacySetting::Public),
            "Private" => Ok(ComponentPrivacySetting::Private),
            _ => Err(anyhow!("Could not deserialize string '{}' to ComponentPrivacySetting", s)),
        }
    }
}
