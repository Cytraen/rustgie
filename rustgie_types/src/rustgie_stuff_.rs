use std::fmt::{Display, Formatter};

pub struct RustgieEnumFromStrError;

impl Display for RustgieEnumFromStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RustgieEnumFromStrError")
    }
}
