use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DestinyTraitDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "traitCategoryId")]
    pub trait_category_id: Option<String>,

    #[serde(rename = "traitCategoryHash")]
    pub trait_category_hash: u32,

    /// An identifier for how this trait can be displayed. For example: a 'keyword' hint to show an explanation for certain related terms.
    #[serde(rename = "displayHint")]
    pub display_hint: Option<String>,

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
    /// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    #[serde(rename = "hash")]
    pub hash: u32,

    /// The index of the entity as it was found in the investment tables.
    #[serde(rename = "index")]
    pub index: i32,

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    #[serde(rename = "redacted")]
    pub redacted: bool,
}

#[derive(Deserialize, Serialize)]
pub struct DestinyTraitCategoryDefinition {
    #[serde(rename = "traitCategoryId")]
    pub trait_category_id: Option<String>,

    #[serde(rename = "traitHashes")]
    pub trait_hashes: Option<Vec<u32>>,

    #[serde(rename = "traitIds")]
    pub trait_ids: Option<Vec<String>>,

    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
    /// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    #[serde(rename = "hash")]
    pub hash: u32,

    /// The index of the entity as it was found in the investment tables.
    #[serde(rename = "index")]
    pub index: i32,

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    #[serde(rename = "redacted")]
    pub redacted: bool,
}
