use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContentTypeDescription {
    #[serde(rename = "cType")]
    pub c_type: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "contentDescription")]
    pub content_description: Option<String>,

    #[serde(rename = "previewImage")]
    pub preview_image: Option<String>,

    #[serde(rename = "priority")]
    pub priority: i32,

    #[serde(rename = "reminder")]
    pub reminder: Option<String>,

    #[serde(rename = "properties")]
    pub properties: Option<Vec<crate::content::models::ContentTypeProperty>>,

    #[serde(rename = "tagMetadata")]
    pub tag_metadata: Option<Vec<crate::content::models::TagMetadataDefinition>>,

    #[serde(rename = "tagMetadataItems")]
    pub tag_metadata_items: Option<HashMap<String, crate::content::models::TagMetadataItem>>,

    #[serde(rename = "usageExamples")]
    pub usage_examples: Option<Vec<String>>,

    #[serde(rename = "showInContentEditor")]
    pub show_in_content_editor: bool,

    #[serde(rename = "typeOf")]
    pub type_of: Option<String>,

    #[serde(rename = "bindIdentifierToProperty")]
    pub bind_identifier_to_property: Option<String>,

    #[serde(rename = "boundRegex")]
    pub bound_regex: Option<String>,

    #[serde(rename = "forceIdentifierBinding")]
    pub force_identifier_binding: bool,

    #[serde(rename = "allowComments")]
    pub allow_comments: bool,

    #[serde(rename = "autoEnglishPropertyFallback")]
    pub auto_english_property_fallback: bool,

    #[serde(rename = "bulkUploadable")]
    pub bulk_uploadable: bool,

    #[serde(rename = "previews")]
    pub previews: Option<Vec<crate::content::models::ContentPreview>>,

    #[serde(rename = "suppressCmsPath")]
    pub suppress_cms_path: bool,

    #[serde(rename = "propertySections")]
    pub property_sections: Option<Vec<crate::content::models::ContentTypePropertySection>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContentTypeProperty {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "rootPropertyName")]
    pub root_property_name: Option<String>,

    #[serde(rename = "readableName")]
    pub readable_name: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<String>,

    #[serde(rename = "propertyDescription")]
    pub property_description: Option<String>,

    #[serde(rename = "localizable")]
    pub localizable: bool,

    #[serde(rename = "fallback")]
    pub fallback: bool,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "order")]
    pub order: i32,

    #[serde(rename = "visible")]
    pub visible: bool,

    #[serde(rename = "isTitle")]
    pub is_title: bool,

    #[serde(rename = "required")]
    pub required: bool,

    #[serde(rename = "maxLength")]
    pub max_length: i32,

    #[serde(rename = "maxByteLength")]
    pub max_byte_length: i32,

    #[serde(rename = "maxFileSize")]
    pub max_file_size: i32,

    #[serde(rename = "regexp")]
    pub regexp: Option<String>,

    #[serde(rename = "validateAs")]
    pub validate_as: Option<String>,

    #[serde(rename = "rssAttribute")]
    pub rss_attribute: Option<String>,

    #[serde(rename = "visibleDependency")]
    pub visible_dependency: Option<String>,

    #[serde(rename = "visibleOn")]
    pub visible_on: Option<String>,

    #[serde(rename = "datatype")]
    pub datatype: crate::content::models::ContentPropertyDataTypeEnum,

    #[serde(rename = "attributes")]
    pub attributes: Option<HashMap<String, String>>,

    #[serde(rename = "childProperties")]
    pub child_properties: Option<Vec<crate::content::models::ContentTypeProperty>>,

    #[serde(rename = "contentTypeAllowed")]
    pub content_type_allowed: Option<String>,

    #[serde(rename = "bindToProperty")]
    pub bind_to_property: Option<String>,

    #[serde(rename = "boundRegex")]
    pub bound_regex: Option<String>,

    #[serde(rename = "representationSelection")]
    pub representation_selection: Option<HashMap<String, String>>,

    #[serde(rename = "defaultValues")]
    pub default_values: Option<Vec<crate::content::models::ContentTypeDefaultValue>>,

    #[serde(rename = "isExternalAllowed")]
    pub is_external_allowed: bool,

    #[serde(rename = "propertySection")]
    pub property_section: Option<String>,

    #[serde(rename = "weight")]
    pub weight: i32,

    #[serde(rename = "entitytype")]
    pub entitytype: Option<String>,

    #[serde(rename = "isCombo")]
    pub is_combo: bool,

    #[serde(rename = "suppressProperty")]
    pub suppress_property: bool,

    #[serde(rename = "legalContentTypes")]
    pub legal_content_types: Option<Vec<String>>,

    #[serde(rename = "representationValidationString")]
    pub representation_validation_string: Option<String>,

    #[serde(rename = "minWidth")]
    pub min_width: i32,

    #[serde(rename = "maxWidth")]
    pub max_width: i32,

    #[serde(rename = "minHeight")]
    pub min_height: i32,

    #[serde(rename = "maxHeight")]
    pub max_height: i32,

    #[serde(rename = "isVideo")]
    pub is_video: bool,

    #[serde(rename = "isImage")]
    pub is_image: bool,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContentPropertyDataTypeEnum {
    None = 0,
    Plaintext = 1,
    Html = 2,
    Dropdown = 3,
    List = 4,
    Json = 5,
    Content = 6,
    Representation = 7,
    Set = 8,
    File = 9,
    FolderSet = 10,
    Date = 11,
    MultilinePlaintext = 12,
    DestinyContent = 13,
    Color = 14,
}

impl Display for ContentPropertyDataTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

impl FromStr for ContentPropertyDataTypeEnum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(ContentPropertyDataTypeEnum::None),
            "Plaintext" => Ok(ContentPropertyDataTypeEnum::Plaintext),
            "Html" => Ok(ContentPropertyDataTypeEnum::Html),
            "Dropdown" => Ok(ContentPropertyDataTypeEnum::Dropdown),
            "List" => Ok(ContentPropertyDataTypeEnum::List),
            "Json" => Ok(ContentPropertyDataTypeEnum::Json),
            "Content" => Ok(ContentPropertyDataTypeEnum::Content),
            "Representation" => Ok(ContentPropertyDataTypeEnum::Representation),
            "Set" => Ok(ContentPropertyDataTypeEnum::Set),
            "File" => Ok(ContentPropertyDataTypeEnum::File),
            "FolderSet" => Ok(ContentPropertyDataTypeEnum::FolderSet),
            "Date" => Ok(ContentPropertyDataTypeEnum::Date),
            "MultilinePlaintext" => Ok(ContentPropertyDataTypeEnum::MultilinePlaintext),
            "DestinyContent" => Ok(ContentPropertyDataTypeEnum::DestinyContent),
            "Color" => Ok(ContentPropertyDataTypeEnum::Color),
            _ => Err(anyhow!("Could not deserialize string '{}' to ContentPropertyDataTypeEnum", s)),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContentTypeDefaultValue {
    #[serde(rename = "whenClause")]
    pub when_clause: Option<String>,

    #[serde(rename = "whenValue")]
    pub when_value: Option<String>,

    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TagMetadataDefinition {
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "order")]
    pub order: i32,

    #[serde(rename = "items")]
    pub items: Option<Vec<crate::content::models::TagMetadataItem>>,

    #[serde(rename = "datatype")]
    pub datatype: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "isRequired")]
    pub is_required: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TagMetadataItem {
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "tagText")]
    pub tag_text: Option<String>,

    #[serde(rename = "groups")]
    pub groups: Option<Vec<String>>,

    #[serde(rename = "isDefault")]
    pub is_default: bool,

    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContentPreview {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    pub path: Option<String>,

    #[serde(rename = "itemInSet")]
    pub item_in_set: bool,

    #[serde(rename = "setTag")]
    pub set_tag: Option<String>,

    #[serde(rename = "setNesting")]
    pub set_nesting: i32,

    #[serde(rename = "useSetId")]
    pub use_set_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContentTypePropertySection {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "readableName")]
    pub readable_name: Option<String>,

    #[serde(rename = "collapsed")]
    pub collapsed: bool,
}
