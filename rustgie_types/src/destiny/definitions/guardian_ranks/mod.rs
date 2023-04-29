use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyGuardianRankDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "rankNumber")]
    pub rank_number: i32,

    #[serde(rename = "presentationNodeHash")]
    pub presentation_node_hash: u32,

    #[serde(rename = "foregroundImagePath")]
    pub foreground_image_path: Option<String>,

    #[serde(rename = "overlayImagePath")]
    pub overlay_image_path: Option<String>,

    #[serde(rename = "overlayMaskImagePath")]
    pub overlay_mask_image_path: Option<String>,

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
pub struct DestinyGuardianRankConstantsDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "rankCount")]
    pub rank_count: i32,

    #[serde(rename = "rootNodeHash")]
    pub root_node_hash: u32,

    #[serde(rename = "iconBackgrounds")]
    pub icon_backgrounds: Option<crate::destiny::definitions::guardian_ranks::DestinyGuardianRankIconBackgroundsDefinition>,

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
pub struct DestinyGuardianRankIconBackgroundsDefinition {
    #[serde(rename = "backgroundEmptyBorderedImagePath")]
    pub background_empty_bordered_image_path: Option<String>,

    #[serde(rename = "backgroundEmptyBlueGradientBorderedImagePath")]
    pub background_empty_blue_gradient_bordered_image_path: Option<String>,

    #[serde(rename = "backgroundFilledBlueBorderedImagePath")]
    pub background_filled_blue_bordered_image_path: Option<String>,

    #[serde(rename = "backgroundFilledBlueGradientBorderedImagePath")]
    pub background_filled_blue_gradient_bordered_image_path: Option<String>,

    #[serde(rename = "backgroundFilledBlueLowAlphaImagePath")]
    pub background_filled_blue_low_alpha_image_path: Option<String>,

    #[serde(rename = "backgroundFilledBlueMediumAlphaImagePath")]
    pub background_filled_blue_medium_alpha_image_path: Option<String>,

    #[serde(rename = "backgroundFilledGrayMediumAlphaBorderedImagePath")]
    pub background_filled_gray_medium_alpha_bordered_image_path: Option<String>,

    #[serde(rename = "backgroundFilledGrayHeavyAlphaBorderedImagePath")]
    pub background_filled_gray_heavy_alpha_bordered_image_path: Option<String>,

    #[serde(rename = "backgroundFilledWhiteMediumAlphaImagePath")]
    pub background_filled_white_medium_alpha_image_path: Option<String>,

    #[serde(rename = "backgroundFilledWhiteImagePath")]
    pub background_filled_white_image_path: Option<String>,

    #[serde(rename = "backgroundPlateWhiteImagePath")]
    pub background_plate_white_image_path: Option<String>,

    #[serde(rename = "backgroundPlateBlackImagePath")]
    pub background_plate_black_image_path: Option<String>,

    #[serde(rename = "backgroundPlateBlackAlphaImagePath")]
    pub background_plate_black_alpha_image_path: Option<String>,
}
