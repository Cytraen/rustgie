use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// DestinyManifest is the external-facing contract for just the properties needed by those calling the Destiny Platform.
#[derive(Deserialize, Serialize)]
pub struct DestinyManifest {
    #[serde(rename = "version")]
    pub version: Option<String>,

    #[serde(rename = "mobileAssetContentPath")]
    pub mobile_asset_content_path: Option<String>,

    #[serde(rename = "mobileGearAssetDataBases")]
    pub mobile_gear_asset_data_bases: Option<Vec<crate::destiny::config::GearAssetDataBaseDefinition>>,

    #[serde(rename = "mobileWorldContentPaths")]
    pub mobile_world_content_paths: Option<HashMap<String, String>>,

    /// This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a path to the aggregated world definitions (warning: large file!)
    #[serde(rename = "jsonWorldContentPaths")]
    pub json_world_content_paths: Option<HashMap<String, String>>,

    /// This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a dictionary, where the key is a definition type by name, and the value is the path to the file for that definition. WARNING: This is unsafe and subject to change - do not depend on data in these files staying around long-term.
    #[serde(rename = "jsonWorldComponentContentPaths")]
    pub json_world_component_content_paths: Option<HashMap<String, HashMap<String, String>>>,

    #[serde(rename = "mobileClanBannerDatabasePath")]
    pub mobile_clan_banner_database_path: Option<String>,

    #[serde(rename = "mobileGearCDN")]
    pub mobile_gear_c_d_n: Option<HashMap<String, String>>,

    /// Information about the "Image Pyramid" for Destiny icons. Where possible, we create smaller versions of Destiny icons. These are found as subfolders under the location of the "original/full size" Destiny images, with the same file name and extension as the original image itself. (this lets us avoid sending largely redundant path info with every entity, at the expense of the smaller versions of the image being less discoverable)
    #[serde(rename = "iconImagePyramidInfo")]
    pub icon_image_pyramid_info: Option<Vec<crate::destiny::config::ImagePyramidEntry>>,
}

#[derive(Deserialize, Serialize)]
pub struct GearAssetDataBaseDefinition {
    #[serde(rename = "version")]
    pub version: i32,

    #[serde(rename = "path")]
    pub path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ImagePyramidEntry {
    /// The name of the subfolder where these images are located.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The factor by which the original image size has been reduced.
    #[serde(rename = "factor")]
    pub factor: f32,
}
