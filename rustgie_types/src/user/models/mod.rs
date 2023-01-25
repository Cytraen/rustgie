use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct GetCredentialTypesForAccountResponse {
    #[serde(rename = "credentialType")]
    pub credential_type: crate::BungieCredentialType,

    #[serde(rename = "credentialDisplayName")]
    pub credential_display_name: Option<String>,

    #[serde(rename = "isPublic")]
    pub is_public: bool,

    #[serde(rename = "credentialAsString")]
    pub credential_as_string: Option<String>,
}
