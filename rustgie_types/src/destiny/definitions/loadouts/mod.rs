use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutColorDefinition {
    #[serde(rename = "colorImagePath")]
    pub color_image_path: Option<String>,

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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutIconDefinition {
    #[serde(rename = "iconImagePath")]
    pub icon_image_path: Option<String>,

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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutNameDefinition {
    #[serde(rename = "name")]
    pub name: Option<String>,

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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyLoadoutConstantsDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// This is the same icon as the one in the display properties, offered here as well with a more descriptive name.
    #[serde(rename = "whiteIconImagePath")]
    pub white_icon_image_path: Option<String>,

    /// This is a color-inverted version of the whiteIconImagePath.
    #[serde(rename = "blackIconImagePath")]
    pub black_icon_image_path: Option<String>,

    /// The maximum number of loadouts available to each character. The loadouts component API response can return fewer loadouts than this, as more loadouts are unlocked by reaching higher Guardian Ranks.
    #[serde(rename = "loadoutCountPerCharacter")]
    pub loadout_count_per_character: i32,

    /// A list of the socket category hashes to be filtered out of loadout item preview displays.
    #[serde(rename = "loadoutPreviewFilterOutSocketCategoryHashes")]
    pub loadout_preview_filter_out_socket_category_hashes: Option<Vec<u32>>,

    /// A list of the socket type hashes to be filtered out of loadout item preview displays.
    #[serde(rename = "loadoutPreviewFilterOutSocketTypeHashes")]
    pub loadout_preview_filter_out_socket_type_hashes: Option<Vec<u32>>,

    /// A list of the loadout name hashes in index order, for convenience.
    #[serde(rename = "loadoutNameHashes")]
    pub loadout_name_hashes: Option<Vec<u32>>,

    /// A list of the loadout icon hashes in index order, for convenience.
    #[serde(rename = "loadoutIconHashes")]
    pub loadout_icon_hashes: Option<Vec<u32>>,

    /// A list of the loadout color hashes in index order, for convenience.
    #[serde(rename = "loadoutColorHashes")]
    pub loadout_color_hashes: Option<Vec<u32>>,

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
