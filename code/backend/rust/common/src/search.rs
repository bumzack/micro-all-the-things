use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MeiliSearchRequest {
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "hitsPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits_per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributesToRetrieve")]
    pub attributes_to_retrieve: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributesToCrop")]
    pub attributes_to_crop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cropLength")]
    pub crop_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cropMarker")]
    pub crop_marker: Option<String>,
    #[serde(rename = "attributesToHighlight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_highlight: Option<Vec<String>>,
    #[serde(rename = "highlightPreTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
    #[serde(rename = "highlightPostTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[serde(rename = "showMatchesPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_matches_position: Option<bool>,
    pub sort: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matchingStrategy")]
    pub matching_strategy: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MeiliSearchResult<T> {
    pub hits: Vec<T>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    #[serde(rename = "estimatedTotalHits")]
    pub estimated_total_hits: u32,
    #[serde(rename = "totalHits")]
    pub total_hits: Option<u32>,
    #[serde(rename = "totalPages")]
    pub total_pages: Option<u32>,
    #[serde(rename = "hitsPerPage")]
    pub hits_per_page: Option<u32>,
    pub page: Option<u32>,
    // TODO
    // pub facetDistribution: Object ???
    #[serde(rename = "processingTimeMs")]
    pub processing_time_ms: u32,
    pub query: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MeiliSearchRequestMovie {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchPaginatedRequest {
    pub q: String,
    pub offset: u32,
    pub limit: u32,
    pub sort: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchPersonList {
    pub nconsts: Vec<String>,
}


