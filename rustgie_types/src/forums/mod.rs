use anyhow::{anyhow, Result};
use enumflags2::bitflags;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ForumPostCategoryEnums {
    TextOnly = 1,
    Media = 2,
    Link = 4,
    Poll = 8,
    Question = 16,
    Answered = 32,
    Announcement = 64,
    ContentComment = 128,
    BungieOfficial = 256,
    NinjaOfficial = 512,
    Recruitment = 1024,
}

impl Display for ForumPostCategoryEnums {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for ForumPostCategoryEnums {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "TextOnly" => Ok(ForumPostCategoryEnums::TextOnly),
            "Media" => Ok(ForumPostCategoryEnums::Media),
            "Link" => Ok(ForumPostCategoryEnums::Link),
            "Poll" => Ok(ForumPostCategoryEnums::Poll),
            "Question" => Ok(ForumPostCategoryEnums::Question),
            "Answered" => Ok(ForumPostCategoryEnums::Answered),
            "Announcement" => Ok(ForumPostCategoryEnums::Announcement),
            "ContentComment" => Ok(ForumPostCategoryEnums::ContentComment),
            "BungieOfficial" => Ok(ForumPostCategoryEnums::BungieOfficial),
            "NinjaOfficial" => Ok(ForumPostCategoryEnums::NinjaOfficial),
            "Recruitment" => Ok(ForumPostCategoryEnums::Recruitment),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumPostCategoryEnums", s)),
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ForumFlagsEnum {
    BungieStaffPost = 1,
    ForumNinjaPost = 2,
    ForumMentorPost = 4,
    TopicBungieStaffPosted = 8,
    TopicBungieVolunteerPosted = 16,
    QuestionAnsweredByBungie = 32,
    QuestionAnsweredByNinja = 64,
    CommunityContent = 128,
}

impl Display for ForumFlagsEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for ForumFlagsEnum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "BungieStaffPost" => Ok(ForumFlagsEnum::BungieStaffPost),
            "ForumNinjaPost" => Ok(ForumFlagsEnum::ForumNinjaPost),
            "ForumMentorPost" => Ok(ForumFlagsEnum::ForumMentorPost),
            "TopicBungieStaffPosted" => Ok(ForumFlagsEnum::TopicBungieStaffPosted),
            "TopicBungieVolunteerPosted" => Ok(ForumFlagsEnum::TopicBungieVolunteerPosted),
            "QuestionAnsweredByBungie" => Ok(ForumFlagsEnum::QuestionAnsweredByBungie),
            "QuestionAnsweredByNinja" => Ok(ForumFlagsEnum::QuestionAnsweredByNinja),
            "CommunityContent" => Ok(ForumFlagsEnum::CommunityContent),
            _ => Err(anyhow!("Could not deserialize string '{}' to ForumFlagsEnum", s)),
        }
    }
}
