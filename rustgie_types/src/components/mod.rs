use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The base class for any component-returning object that may need to indicate information about the state of the component being returned.
#[derive(Deserialize, Serialize)]
pub struct ComponentResponse {
    #[serde(rename = "privacy")]
    pub privacy: crate::components::ComponentPrivacySetting,

    /// If true, this component is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
}

/// A set of flags for reason(s) why the component populated in the way that it did. Inspect the individual flags for the reasons.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
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
    type Err = crate::rustgie_stuff_::RustgieEnumFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Public" => Ok(ComponentPrivacySetting::Public),
            "Private" => Ok(ComponentPrivacySetting::Private),
            _ => Err(crate::rustgie_stuff_::RustgieEnumFromStrError),
        }
    }
}
