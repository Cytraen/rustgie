use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct DestinyStringVariablesComponent {
    #[serde(rename = "integerValuesByHash")]
    pub integer_values_by_hash: Option<HashMap<u32, i32>>,
}
