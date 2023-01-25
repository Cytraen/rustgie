use serde::{Deserialize, Serialize};

/// Raw data about the customization options chosen for a character's face and appearance.
/// You can look up the relevant class/race/gender combo in DestinyCharacterCustomizationOptionDefinition for the character, and then look up these values within the CustomizationOptions found to pull some data about their choices. Warning: not all of that data is meaningful. Some data has useful icons. Others have nothing, and are only meant for 3D rendering purposes (which we sadly do not expose yet)
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCharacterCustomization {
    #[serde(rename = "personality")]
    pub personality: u32,

    #[serde(rename = "face")]
    pub face: u32,

    #[serde(rename = "skinColor")]
    pub skin_color: u32,

    #[serde(rename = "lipColor")]
    pub lip_color: u32,

    #[serde(rename = "eyeColor")]
    pub eye_color: u32,

    #[serde(rename = "hairColors")]
    pub hair_colors: Option<Vec<u32>>,

    #[serde(rename = "featureColors")]
    pub feature_colors: Option<Vec<u32>>,

    #[serde(rename = "decalColor")]
    pub decal_color: u32,

    #[serde(rename = "wearHelmet")]
    pub wear_helmet: bool,

    #[serde(rename = "hairIndex")]
    pub hair_index: i32,

    #[serde(rename = "featureIndex")]
    pub feature_index: i32,

    #[serde(rename = "decalIndex")]
    pub decal_index: i32,
}

/// A minimal view of a character's equipped items, for the purpose of rendering a summary screen or showing the character in 3D.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyCharacterPeerView {
    #[serde(rename = "equipment")]
    pub equipment: Option<Vec<crate::destiny::character::DestinyItemPeerView>>,
}

/// Bare minimum summary information for an item, for the sake of 3D rendering the item.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyItemPeerView {
    /// The hash identifier of the item in question. Use it to look up the DestinyInventoryItemDefinition of the item for static rendering data.
    #[serde(rename = "itemHash")]
    pub item_hash: u32,

    /// The list of dyes that have been applied to this item.
    #[serde(rename = "dyes")]
    pub dyes: Option<Vec<crate::destiny::DyeReference>>,
}
