pub mod models;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use time::OffsetDateTime;

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct ContentRepresentation {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    pub path: Option<String>,

    #[serde(rename = "validationString")]
    pub validation_string: Option<String>,
}

#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct CommentSummary {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "topicId")]
    pub topic_id: i64,

    #[serde(rename = "commentCount")]
    pub comment_count: i32,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct NewsArticleRssResponse {
    #[serde(rename = "NewsArticles")]
    pub news_articles: Option<Vec<crate::content::NewsArticleRssItem>>,

    #[serde(rename = "CurrentPaginationToken")]
    pub current_pagination_token: i32,

    #[serde(rename = "NextPaginationToken")]
    pub next_pagination_token: Option<i32>,

    #[serde(rename = "ResultCountThisPage")]
    pub result_count_this_page: i32,

    #[serde(rename = "CategoryFilter")]
    pub category_filter: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct NewsArticleRssItem {
    #[serde(rename = "Title")]
    pub title: Option<String>,

    #[serde(rename = "Link")]
    pub link: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "PubDate")]
    pub pub_date: OffsetDateTime,

    #[serde(rename = "UniqueIdentifier")]
    pub unique_identifier: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "HtmlContent")]
    pub html_content: Option<String>,

    #[serde(rename = "ImagePath")]
    pub image_path: Option<String>,

    #[serde(rename = "OptionalMobileImagePath")]
    pub optional_mobile_image_path: Option<String>,
}
