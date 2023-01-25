use serde::{Deserialize, Serialize};

/// Many Destiny*Definition contracts - the "first order" entities of Destiny that have their own tables in the Manifest Database - also have displayable information. This is the base class for that display information.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyDisplayPropertiesDefinition {
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Note that "icon" is sometimes misleading, and should be interpreted in the context of the entity. For instance, in Destiny 1 the DestinyRecordBookDefinition's icon was a big picture of a book.
    /// But usually, it will be a small square image that you can use as... well, an icon.
    /// They are currently represented as 96px x 96px images.
    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "iconSequences")]
    pub icon_sequences: Option<Vec<crate::destiny::definitions::common::DestinyIconSequenceDefinition>>,

    /// If this item has a high-res icon (at least for now, many things won't), then the path to that icon will be here.
    #[serde(rename = "highResIcon")]
    pub high_res_icon: Option<String>,

    #[serde(rename = "hasIcon")]
    pub has_icon: bool,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyIconSequenceDefinition {
    #[serde(rename = "frames")]
    pub frames: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyPositionDefinition {
    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "z")]
    pub z: i32,
}
