use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

/// Defines a canonical "Season" of Destiny: a range of a few months where the game highlights certain challenges, provides new loot, has new Clan-related rewards and celebrates various seasonal events.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinySeasonDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "backgroundImagePath")]
    pub background_image_path: Option<String>,

    #[serde(rename = "seasonNumber")]
    pub season_number: i32,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "startDate")]
    pub start_date: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default)]
    #[serde(rename = "endDate")]
    pub end_date: Option<OffsetDateTime>,

    #[serde(rename = "seasonPassHash")]
    pub season_pass_hash: Option<u32>,

    #[serde(rename = "seasonPassProgressionHash")]
    pub season_pass_progression_hash: Option<u32>,

    #[serde(rename = "artifactItemHash")]
    pub artifact_item_hash: Option<u32>,

    #[serde(rename = "sealPresentationNodeHash")]
    pub seal_presentation_node_hash: Option<u32>,

    #[serde(rename = "seasonalChallengesPresentationNodeHash")]
    pub seasonal_challenges_presentation_node_hash: Option<u32>,

    /// Optional - Defines the promotional text, images, and links to preview this season.
    #[serde(rename = "preview")]
    pub preview: Option<crate::destiny::definitions::seasons::DestinySeasonPreviewDefinition>,

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

/// Defines the promotional text, images, and links to preview this season.
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinySeasonPreviewDefinition {
    /// A localized description of the season.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// A relative path to learn more about the season. Web browsers should be automatically redirected to the user's Bungie.net locale. For example: "/SeasonOfTheChosen" will redirect to "/7/en/Seasons/SeasonOfTheChosen" for English users.
    #[serde(rename = "linkPath")]
    pub link_path: Option<String>,

    /// An optional link to a localized video, probably YouTube.
    #[serde(rename = "videoLink")]
    pub video_link: Option<String>,

    /// A list of images to preview the seasonal content. Should have at least three to show.
    #[serde(rename = "images")]
    pub images: Option<Vec<crate::destiny::definitions::seasons::DestinySeasonPreviewImageDefinition>>,
}

/// Defines the thumbnail icon, high-res image, and video link for promotional images
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinySeasonPreviewImageDefinition {
    /// A thumbnail icon path to preview seasonal content, probably 480x270.
    #[serde(rename = "thumbnailImage")]
    pub thumbnail_image: Option<String>,

    /// An optional path to a high-resolution image, probably 1920x1080.
    #[serde(rename = "highResImage")]
    pub high_res_image: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinySeasonPassDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    /// This is the progression definition related to the progression for the initial levels 1-100 that provide item rewards for the Season pass. Further experience after you reach the limit is provided in the "Prestige" progression referred to by prestigeProgressionHash.
    #[serde(rename = "rewardProgressionHash")]
    pub reward_progression_hash: u32,

    /// I know what you're thinking, but I promise we're not going to duplicate and drown you. Instead, we're giving you sweet, sweet power bonuses.
    /// Prestige progression is further progression that you can make on the Season pass after you gain max ranks, that will ultimately increase your power/light level over the theoretical limit.
    #[serde(rename = "prestigeProgressionHash")]
    pub prestige_progression_hash: u32,

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

/// Defines the properties of an 'Event Card' in Destiny 2, to coincide with a seasonal event for additional challenges, premium rewards, a new seal, and a special title. For example: Solstice of Heroes 2022.
#[serde_as]
#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyEventCardDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: Option<crate::destiny::definitions::common::DestinyDisplayPropertiesDefinition>,

    #[serde(rename = "linkRedirectPath")]
    pub link_redirect_path: Option<String>,

    #[serde(rename = "color")]
    pub color: Option<crate::destiny::misc::DestinyColor>,

    #[serde(rename = "images")]
    pub images: Option<crate::destiny::definitions::seasons::DestinyEventCardImages>,

    #[serde(rename = "triumphsPresentationNodeHash")]
    pub triumphs_presentation_node_hash: u32,

    #[serde(rename = "sealPresentationNodeHash")]
    pub seal_presentation_node_hash: u32,

    #[serde(rename = "ticketCurrencyItemHash")]
    pub ticket_currency_item_hash: u32,

    #[serde(rename = "ticketVendorHash")]
    pub ticket_vendor_hash: u32,

    #[serde(rename = "ticketVendorCategoryHash")]
    pub ticket_vendor_category_hash: u32,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "endTime")]
    pub end_time: i64,

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

#[derive(Deserialize, Serialize, PartialEq)]
pub struct DestinyEventCardImages {
    #[serde(rename = "unownedCardSleeveImagePath")]
    pub unowned_card_sleeve_image_path: Option<String>,

    #[serde(rename = "unownedCardSleeveWrapImagePath")]
    pub unowned_card_sleeve_wrap_image_path: Option<String>,

    #[serde(rename = "cardIncompleteImagePath")]
    pub card_incomplete_image_path: Option<String>,

    #[serde(rename = "cardCompleteImagePath")]
    pub card_complete_image_path: Option<String>,

    #[serde(rename = "cardCompleteWrapImagePath")]
    pub card_complete_wrap_image_path: Option<String>,

    #[serde(rename = "progressIconImagePath")]
    pub progress_icon_image_path: Option<String>,

    #[serde(rename = "themeBackgroundImagePath")]
    pub theme_background_image_path: Option<String>,
}
