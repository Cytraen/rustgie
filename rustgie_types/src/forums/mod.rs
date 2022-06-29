use std::fmt::{Display, Formatter, Result};
use enumflags2::bitflags;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u32)
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u32)
    }
}
