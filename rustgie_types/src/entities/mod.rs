use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct EntityActionResult {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "entityId")]
    pub entity_id: i64,

    #[serde(rename = "result")]
    pub result: crate::exceptions::PlatformErrorCodes,
}
