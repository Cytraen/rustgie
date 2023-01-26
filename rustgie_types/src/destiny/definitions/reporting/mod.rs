use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// If you're going to report someone for a Terms of Service violation, you need to choose a category and reason for the report. This definition holds both the categories and the reasons within those categories, for simplicity and my own laziness' sake.
/// Note tha this means that, to refer to a Reason by reasonHash, you need a combination of the reasonHash *and* the associated ReasonCategory's hash: there are some reasons defined under multiple categories.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyReportReasonCategoryDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// The specific reasons for the report under this category.
    #[serde(rename = "reasons")]
    pub reasons: Option<HashMap<u32, crate::destiny::definitions::reporting::DestinyReportReasonDefinition>>,

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

/// A specific reason for being banned. Only accessible under the related category (DestinyReportReasonCategoryDefinition) under which it is shown. Note that this means that report reasons' reasonHash are not globally unique: and indeed, entries like "Other" are defined under most categories for example.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DestinyReportReasonDefinition {
    /// The identifier for the reason: they are only guaranteed unique under the Category in which they are found.
    #[serde(rename = "reasonHash")]
    pub reason_hash: u32,

    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,
}
