use std::fmt::{Display, Formatter, Result};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumPostCategoryEnums {
    None = 0,
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
        write!(f, "{}", *self as i32)
    }
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForumFlagsEnum {
    None = 0,
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
        write!(f, "{}", *self as i32)
    }
}
