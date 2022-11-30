use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
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
    type Err = crate::rustgie_stuff_::RustgieEnumFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Applied" => Ok(DropStateEnum::Applied),
            "Fulfilled" => Ok(DropStateEnum::Fulfilled),
            _ => Err(crate::rustgie_stuff_::RustgieEnumFromStrError),
        }
    }
}
