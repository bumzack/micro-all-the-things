use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::models::search_doc::IndexDocFacetDistribution;

#[derive(Debug, Deserialize, Serialize)]
pub struct SolrParams {
    pub q: Option<String>,
    #[serde(rename = "QTime")]
    pub indent: Option<bool>,
    pub start: Option<String>,
    pub limit: Option<String>,
    pub sort: Option<String>,
    #[serde(rename = "q.op")]
    pub q_op: Option<i32>,
    pub rows: Option<String>,
    #[serde(rename = "useParams")]
    pub use_params: Option<u32>,
    #[serde(rename = "_")]
    pub uid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseHeader {
    pub status: Option<i32>,
    #[serde(rename = "QTime")]
    pub q_time: Option<i32>,
    pub params: Option<SolrParams>,
    pub total: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolrResponseDocs<T> {
    #[serde(rename = "numFound")]
    pub num_found: Option<i32>,
    pub start: Option<i32>,
    #[serde(rename = "numFoundExact")]
    pub num_found_exact: Option<bool>,
    pub docs: Option<Vec<T>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FacetCounts {
    pub facet_fields: Option<IndexDocFacetDistribution>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolrResponse<T> {
    #[serde(rename = "responseHeader")]
    pub response_header: Option<ResponseHeader>,
    pub response: Option<SolrResponseDocs<T>>,
    pub facet_counts: Option<FacetCounts>,
}
