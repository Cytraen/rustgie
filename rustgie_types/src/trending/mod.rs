use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize)]
pub struct TrendingCategories {
    #[serde(rename = "categories")]
    pub categories: Option<Vec<crate::trending::TrendingCategory>>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingCategory {
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,

    #[serde(rename = "entries")]
    pub entries: Option<crate::SearchResultOfTrendingEntry>,

    #[serde(rename = "categoryId")]
    pub category_id: Option<String>,
}

/// The list entry view for trending items. Returns just enough to show the item on the trending page.
#[derive(Deserialize, Serialize)]
pub struct TrendingEntry {
    /// The weighted score of this trending item.
    #[serde(rename = "weight")]
    pub weight: f64,

    #[serde(rename = "isFeatured")]
    pub is_featured: bool,

    /// We don't know whether the identifier will be a string, a uint, or a long... so we're going to cast it all to a string. But either way, we need any trending item created to have a single unique identifier for its type.
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    /// An enum - unfortunately - dictating all of the possible kinds of trending items that you might get in your result set, in case you want to do custom rendering or call to get the details of the item.
    #[serde(rename = "entityType")]
    pub entity_type: crate::trending::TrendingEntryType,

    /// The localized "display name/article title/'primary localized identifier'" of the entity.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// If the entity has a localized tagline/subtitle/motto/whatever, that is found here.
    #[serde(rename = "tagline")]
    pub tagline: Option<String>,

    #[serde(rename = "image")]
    pub image: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "startDate")]
    pub start_date: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "endDate")]
    pub end_date: Option<OffsetDateTime>,

    #[serde(rename = "link")]
    pub link: Option<String>,

    /// If this is populated, the entry has a related WebM video to show. I am 100% certain I am going to regret putting this directly on TrendingEntry, but it will work so yolo
    #[serde(rename = "webmVideo")]
    pub webm_video: Option<String>,

    /// If this is populated, the entry has a related MP4 video to show. I am 100% certain I am going to regret putting this directly on TrendingEntry, but it will work so yolo
    #[serde(rename = "mp4Video")]
    pub mp4_video: Option<String>,

    /// If isFeatured, this image will be populated with whatever the featured image is. Note that this will likely be a very large image, so don't use it all the time.
    #[serde(rename = "featureImage")]
    pub feature_image: Option<String>,

    /// If the item is of entityType TrendingEntryType.Container, it may have items - also Trending Entries - contained within it. This is the ordered list of those to display under the Container's header.
    #[serde(rename = "items")]
    pub items: Option<Vec<crate::trending::TrendingEntry>>,

    /// If the entry has a date at which it was created, this is that date.
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "creationDate")]
    pub creation_date: Option<OffsetDateTime>,
}

/// The known entity types that you can have returned from Trending.
#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TrendingEntryType {
    News = 0,
    DestinyItem = 1,
    DestinyActivity = 2,
    DestinyRitual = 3,
    SupportArticle = 4,
    Creation = 5,
    Stream = 6,
    Update = 7,
    Link = 8,
    ForumTag = 9,
    Container = 10,
    Release = 11,
}

impl Display for TrendingEntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for TrendingEntryType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "News" => Ok(TrendingEntryType::News),
            "DestinyItem" => Ok(TrendingEntryType::DestinyItem),
            "DestinyActivity" => Ok(TrendingEntryType::DestinyActivity),
            "DestinyRitual" => Ok(TrendingEntryType::DestinyRitual),
            "SupportArticle" => Ok(TrendingEntryType::SupportArticle),
            "Creation" => Ok(TrendingEntryType::Creation),
            "Stream" => Ok(TrendingEntryType::Stream),
            "Update" => Ok(TrendingEntryType::Update),
            "Link" => Ok(TrendingEntryType::Link),
            "ForumTag" => Ok(TrendingEntryType::ForumTag),
            "Container" => Ok(TrendingEntryType::Container),
            "Release" => Ok(TrendingEntryType::Release),
            _ => Err(anyhow!("Could not deserialize string '{}' to TrendingEntryType", s)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TrendingDetail {
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    #[serde(rename = "entityType")]
    pub entity_type: crate::trending::TrendingEntryType,

    #[serde(rename = "news")]
    pub news: Option<crate::trending::TrendingEntryNews>,

    #[serde(rename = "support")]
    pub support: Option<crate::trending::TrendingEntrySupportArticle>,

    #[serde(rename = "destinyItem")]
    pub destiny_item: Option<crate::trending::TrendingEntryDestinyItem>,

    #[serde(rename = "destinyActivity")]
    pub destiny_activity: Option<crate::trending::TrendingEntryDestinyActivity>,

    #[serde(rename = "destinyRitual")]
    pub destiny_ritual: Option<crate::trending::TrendingEntryDestinyRitual>,

    #[serde(rename = "creation")]
    pub creation: Option<crate::trending::TrendingEntryCommunityCreation>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingEntryNews {
    #[serde(rename = "article")]
    pub article: Option<crate::content::ContentItemPublicContract>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingEntrySupportArticle {
    #[serde(rename = "article")]
    pub article: Option<crate::content::ContentItemPublicContract>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingEntryDestinyItem {
    #[serde(rename = "itemHash")]
    pub item_hash: u32,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingEntryDestinyActivity {
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,

    #[serde(rename = "status")]
    pub status: Option<crate::destiny::activities::DestinyPublicActivityStatus>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingEntryDestinyRitual {
    #[serde(rename = "image")]
    pub image: Option<String>,

    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "subtitle")]
    pub subtitle: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "dateStart")]
    pub date_start: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "dateEnd")]
    pub date_end: Option<OffsetDateTime>,

    /// A destiny event does not necessarily have a related Milestone, but if it does the details will be returned here.
    #[serde(rename = "milestoneDetails")]
    pub milestone_details: Option<crate::destiny::milestones::DestinyPublicMilestone>,

    /// A destiny event will not necessarily have milestone "custom content", but if it does the details will be here.
    #[serde(rename = "eventContent")]
    pub event_content: Option<crate::destiny::milestones::DestinyMilestoneContent>,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct TrendingEntryCommunityCreation {
    #[serde(rename = "media")]
    pub media: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "author")]
    pub author: Option<String>,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "authorMembershipId")]
    pub author_membership_id: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "postId")]
    pub post_id: i64,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "upvotes")]
    pub upvotes: i32,
}
