use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchIndexDoc {
    pub id: String,
    pub tconst: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_type: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchIndexRequest {
    pub q: String,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchPaginatedRequest {
    pub q: String,
    pub offset: u32,
    pub limit: u32,
    pub sort: Vec<String>,
}
