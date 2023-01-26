use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SearchResult {
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PagedQuery {
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i32,

    #[serde(rename = "currentPage")]
    pub current_page: i32,

    #[serde(rename = "requestContinuationToken")]
    pub request_continuation_token: Option<String>,
}
