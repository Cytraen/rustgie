pub mod models;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct ContentItemPublicContract {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "contentId")]
    pub content_id: i64,

    #[serde(rename = "cType")]
    pub c_type: Option<String>,

    #[serde(rename = "cmsPath")]
    pub cms_path: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "creationDate")]
    pub creation_date: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "modifyDate")]
    pub modify_date: OffsetDateTime,

    #[serde(rename = "allowComments")]
    pub allow_comments: bool,

    #[serde(rename = "hasAgeGate")]
    pub has_age_gate: bool,

    #[serde(rename = "minimumAge")]
    pub minimum_age: i32,

    #[serde(rename = "ratingImagePath")]
    pub rating_image_path: Option<String>,

    #[serde(rename = "author")]
    pub author: Option<crate::user::GeneralUser>,

    #[serde(rename = "autoEnglishPropertyFallback")]
    pub auto_english_property_fallback: bool,

    /// Firehose content is really a collection of metadata and "properties", which are the potentially-but-not-strictly localizable data that comprises the meat of whatever content is being shown.
    /// As Cole Porter would have crooned, "Anything Goes" with Firehose properties. They are most often strings, but they can theoretically be anything. They are JSON encoded, and could be JSON structures, simple strings, numbers etc... The Content Type of the item (cType) will describe the properties, and thus how they ought to be deserialized.
    #[serde(rename = "properties")]
    pub properties: Option<HashMap<String, crate::destiny::definitions::DestinyDefinition>>,

    #[serde(rename = "representations")]
    pub representations: Option<Vec<crate::content::ContentRepresentation>>,

    /// NOTE: Tags will always be lower case.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "commentSummary")]
    pub comment_summary: Option<crate::content::CommentSummary>,
}

#[derive(Deserialize, Serialize)]
pub struct ContentRepresentation {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    pub path: Option<String>,

    #[serde(rename = "validationString")]
    pub validation_string: Option<String>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct CommentSummary {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "topicId")]
    pub topic_id: i64,

    #[serde(rename = "commentCount")]
    pub comment_count: i32,
}
