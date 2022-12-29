use anyhow::{anyhow, Result};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumTopicsCategoryFiltersEnum {
    Links = 1,
    Questions = 2,
    AnsweredQuestions = 4,
    Media = 8,
    TextOnly = 16,
    Announcement = 32,
    BungieOfficial = 64,
    Polls = 128,
}

impl Display for ForumTopicsCategoryFiltersEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for ForumTopicsCategoryFiltersEnum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Links" => Ok(ForumTopicsCategoryFiltersEnum::Links),
            "Questions" => Ok(ForumTopicsCategoryFiltersEnum::Questions),
            "AnsweredQuestions" => Ok(ForumTopicsCategoryFiltersEnum::AnsweredQuestions),
            "Media" => Ok(ForumTopicsCategoryFiltersEnum::Media),
            "TextOnly" => Ok(ForumTopicsCategoryFiltersEnum::TextOnly),
            "Announcement" => Ok(ForumTopicsCategoryFiltersEnum::Announcement),
            "BungieOfficial" => Ok(ForumTopicsCategoryFiltersEnum::BungieOfficial),
            "Polls" => Ok(ForumTopicsCategoryFiltersEnum::Polls),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumTopicsCategoryFiltersEnum", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumTopicsQuickDateEnum {
    All = 0,
    LastYear = 1,
    LastMonth = 2,
    LastWeek = 3,
    LastDay = 4,
}

impl Display for ForumTopicsQuickDateEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ForumTopicsQuickDateEnum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "All" => Ok(ForumTopicsQuickDateEnum::All),
            "LastYear" => Ok(ForumTopicsQuickDateEnum::LastYear),
            "LastMonth" => Ok(ForumTopicsQuickDateEnum::LastMonth),
            "LastWeek" => Ok(ForumTopicsQuickDateEnum::LastWeek),
            "LastDay" => Ok(ForumTopicsQuickDateEnum::LastDay),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumTopicsQuickDateEnum", s)),
        }
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumTopicsSortEnum {
    Default = 0,
    LastReplied = 1,
    MostReplied = 2,
    Popularity = 3,
    Controversiality = 4,
    Liked = 5,
    HighestRated = 6,
    MostUpvoted = 7,
}

impl Display for ForumTopicsSortEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for ForumTopicsSortEnum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Default" => Ok(ForumTopicsSortEnum::Default),
            "LastReplied" => Ok(ForumTopicsSortEnum::LastReplied),
            "MostReplied" => Ok(ForumTopicsSortEnum::MostReplied),
            "Popularity" => Ok(ForumTopicsSortEnum::Popularity),
            "Controversiality" => Ok(ForumTopicsSortEnum::Controversiality),
            "Liked" => Ok(ForumTopicsSortEnum::Liked),
            "HighestRated" => Ok(ForumTopicsSortEnum::HighestRated),
            "MostUpvoted" => Ok(ForumTopicsSortEnum::MostUpvoted),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumTopicsSortEnum", s)),
        }
    }
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct PostResponse {
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "lastReplyTimestamp")]
    pub last_reply_timestamp: OffsetDateTime,

    #[serde(rename = "IsPinned")]
    pub is_pinned: bool,

    #[serde(rename = "urlMediaType")]
    pub url_media_type: crate::forum::ForumMediaType,

    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,

    #[serde(rename = "popularity")]
    pub popularity: crate::forum::ForumPostPopularity,

    #[serde(rename = "isActive")]
    pub is_active: bool,

    #[serde(rename = "isAnnouncement")]
    pub is_announcement: bool,

    #[serde(rename = "userRating")]
    pub user_rating: i32,

    #[serde(rename = "userHasRated")]
    pub user_has_rated: bool,

    #[serde(rename = "userHasMutedPost")]
    pub user_has_muted_post: bool,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "latestReplyPostId")]
    pub latest_reply_post_id: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "latestReplyAuthorId")]
    pub latest_reply_author_id: i64,

    #[serde(rename = "ignoreStatus")]
    pub ignore_status: Option<crate::ignores::IgnoreResponse>,

    #[serde(rename = "locale")]
    pub locale: Option<String>,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumMediaType {
    None = 0,
    Image = 1,
    Video = 2,
    Youtube = 3,
}

impl Display for ForumMediaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ForumMediaType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(ForumMediaType::None),
            "Image" => Ok(ForumMediaType::Image),
            "Video" => Ok(ForumMediaType::Video),
            "Youtube" => Ok(ForumMediaType::Youtube),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumMediaType", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumPostPopularity {
    Empty = 0,
    Default = 1,
    Discussed = 2,
    CoolStory = 3,
    HeatingUp = 4,
    Hot = 5,
}

impl Display for ForumPostPopularity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ForumPostPopularity {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Empty" => Ok(ForumPostPopularity::Empty),
            "Default" => Ok(ForumPostPopularity::Default),
            "Discussed" => Ok(ForumPostPopularity::Discussed),
            "CoolStory" => Ok(ForumPostPopularity::CoolStory),
            "HeatingUp" => Ok(ForumPostPopularity::HeatingUp),
            "Hot" => Ok(ForumPostPopularity::Hot),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumPostPopularity", s)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PostSearchResponse {
    #[serde(rename = "relatedPosts")]
    pub related_posts: Option<Vec<crate::forum::PostResponse>>,

    #[serde(rename = "authors")]
    pub authors: Option<Vec<crate::user::GeneralUser>>,

    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::groups_v2::GroupResponse>>,

    #[serde(rename = "searchedTags")]
    pub searched_tags: Option<Vec<crate::tags::models::contracts::TagResponse>>,

    #[serde(rename = "polls")]
    pub polls: Option<Vec<crate::forum::PollResponse>>,

    #[serde(rename = "recruitmentDetails")]
    pub recruitment_details: Option<Vec<crate::forum::ForumRecruitmentDetail>>,

    #[serde(rename = "availablePages")]
    pub available_pages: Option<i32>,

    #[serde(rename = "results")]
    pub results: Option<Vec<crate::forum::PostResponse>>,

    #[serde(rename = "totalResults")]
    pub total_results: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(rename = "query")]
    pub query: Option<crate::queries::PagedQuery>,

    #[serde(rename = "replacementContinuationToken")]
    pub replacement_continuation_token: Option<String>,

    /// If useTotalResults is true, then totalResults represents an accurate count.
    /// If False, it does not, and may be estimated/only the size of the current page.
    /// Either way, you should probably always only trust hasMore.
    /// This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
    #[serde(rename = "useTotalResults")]
    pub use_total_results: bool,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct PollResponse {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "topicId")]
    pub topic_id: i64,

    #[serde(rename = "results")]
    pub results: Option<Vec<crate::forum::PollResult>>,

    #[serde(rename = "totalVotes")]
    pub total_votes: i32,
}

#[derive(Deserialize, Serialize)]
pub struct PollResult {
    #[serde(rename = "answerText")]
    pub answer_text: Option<String>,

    #[serde(rename = "answerSlot")]
    pub answer_slot: i32,

    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "lastVoteDate")]
    pub last_vote_date: OffsetDateTime,

    #[serde(rename = "votes")]
    pub votes: i32,

    #[serde(rename = "requestingUserVoted")]
    pub requesting_user_voted: bool,
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct ForumRecruitmentDetail {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "topicId")]
    pub topic_id: i64,

    #[serde(rename = "microphoneRequired")]
    pub microphone_required: bool,

    #[serde(rename = "intensity")]
    pub intensity: crate::forum::ForumRecruitmentIntensityLabel,

    #[serde(rename = "tone")]
    pub tone: crate::forum::ForumRecruitmentToneLabel,

    #[serde(rename = "approved")]
    pub approved: bool,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[serde(rename = "conversationId")]
    pub conversation_id: Option<i64>,

    #[serde(rename = "playerSlotsTotal")]
    pub player_slots_total: i32,

    #[serde(rename = "playerSlotsRemaining")]
    pub player_slots_remaining: i32,

    #[serde(rename = "Fireteam")]
    pub fireteam: Option<Vec<crate::user::GeneralUser>>,

    #[serde(rename = "kickedPlayerIds")]
    pub kicked_player_ids: Option<Vec<i64>>,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumRecruitmentIntensityLabel {
    None = 0,
    Casual = 1,
    Professional = 2,
}

impl Display for ForumRecruitmentIntensityLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for ForumRecruitmentIntensityLabel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(ForumRecruitmentIntensityLabel::None),
            "Casual" => Ok(ForumRecruitmentIntensityLabel::Casual),
            "Professional" => Ok(ForumRecruitmentIntensityLabel::Professional),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumRecruitmentIntensityLabel", s)),
        }
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumRecruitmentToneLabel {
    None = 0,
    FamilyFriendly = 1,
    Rowdy = 2,
}

impl Display for ForumRecruitmentToneLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for ForumRecruitmentToneLabel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(ForumRecruitmentToneLabel::None),
            "FamilyFriendly" => Ok(ForumRecruitmentToneLabel::FamilyFriendly),
            "Rowdy" => Ok(ForumRecruitmentToneLabel::Rowdy),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumRecruitmentToneLabel", s)),
        }
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumPostSortEnum {
    Default = 0,
    OldestFirst = 1,
}

impl Display for ForumPostSortEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ForumPostSortEnum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Default" => Ok(ForumPostSortEnum::Default),
            "OldestFirst" => Ok(ForumPostSortEnum::OldestFirst),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumPostSortEnum", s)),
        }
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CommunityContentSortMode {
    Trending = 0,
    Latest = 1,
    HighestRated = 2,
}

impl Display for CommunityContentSortMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for CommunityContentSortMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Trending" => Ok(CommunityContentSortMode::Trending),
            "Latest" => Ok(CommunityContentSortMode::Latest),
            "HighestRated" => Ok(CommunityContentSortMode::HighestRated),
            _ => Err(anyhow!("Could not deserialize string '{}' to CommunityContentSortMode", s)),
        }
    }
}
