use std::collections::HashMap;
use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeiliSearchRequest {
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits_per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_retrieve: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_crop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_marker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_highlight: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_matches_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_strategy: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeiliSearchResult<T> {
    pub hits: Vec<T>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub estimated_total_hits: Option<u32>,
    pub total_hits: Option<u32>,
    pub total_pages: Option<u32>,
    pub hits_per_page: Option<u32>,
    pub page: Option<u32>,
    // pub facet_distribution: Option<HashMap<String, HashMap<String, usize>>>,
    pub facet_distribution: Option<IndexDocFacetDistribution>,
    pub processing_time_ms: i32,
    pub query: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexDocFacetDistribution {
    pub actors: Option<HashMap<String, usize>>,
    pub directors: Option<HashMap<String, usize>>,
    pub genres: Option<HashMap<String, usize>>,
    pub titles: Option<HashMap<String, usize>>,
    pub characters: Option<HashMap<String, usize>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeiliDocReadResult<T> {
    pub results: Vec<T>,
    pub offset: u32,
    pub limit: u32,
    pub total: u32,
}
