use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::models::search_doc::{IndexDocFacetDistribution, SearchIndexDoc};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchCustomer {
    pub customer_id: Option<i32>,
    pub jwt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchArticleRequest {
    pub q: String,
    pub offset: u32,
    pub limit: u32,
    pub customer: SearchCustomer,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleSearchResult {
    pub article: SearchIndexDoc,
    pub price: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_price: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArticleResponse {
    pub articles: Option<Vec<ArticleSearchResult>>,
    pub facets: Option<IndexDocFacetDistribution>,
}
