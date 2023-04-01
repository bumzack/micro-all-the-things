use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
    query: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MeiliSearchRequestMovie {
    pub title: String,
}

pub mod handlers_search_entity {
    use std::convert::Infallible;

    use reqwest::StatusCode;
    use serde_json::json;
    use warp::hyper;

    use crate::meilisearch::MeiliSearchRequest;

    pub async fn meili_search(entity: String, search_text: String, client: &reqwest::Client) -> Result<impl warp::Reply, Infallible> {
        println!(
            "searching for entity {} and search_term '{}'", &entity, &search_text
        );

        let search_request = MeiliSearchRequest {
            q: search_text,
            offset: None,
            limit: None,
            page: None,
            hits_per_page: None,
            filter: None,
            facets: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            crop_marker: None,
            crop_length: None,
            attributes_to_highlight: None,
            highlight_pre_tag: None,
            highlight_post_tag: None,
            show_matches_position: None,
            sort: None,
            matching_strategy: None,
        };

        let json = json!(&search_request).to_string();

        exec_meilisearch_search(&entity, json, client).await
    }

    pub async fn meili_filter(entity: String, filter: Vec<String>, client: &reqwest::Client) -> Result<impl warp::Reply, Infallible> {
        // funny stuff
        //             .fold(String::new(), |a, b| a + b + " // ");
        let f = &filter.iter()
            .fold(String::new(), |a, b| a + &*b + " // ");

        println!(
            "searching for entity {} using filter '{}'", &entity, &f
        );

        let search_request = MeiliSearchRequest {
            q: "*".to_string(),
            offset: None,
            limit: None,
            page: None,
            hits_per_page: None,
            filter: Some(filter),
            facets: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            crop_marker: None,
            crop_length: None,
            attributes_to_highlight: None,
            highlight_pre_tag: None,
            highlight_post_tag: None,
            show_matches_position: None,
            sort: None,
            matching_strategy: None,
        };

        println!("search request {:?}", &search_request);

        let json = json!(&search_request).to_string();

        exec_meilisearch_search(&entity, json, client).await
    }

    async fn exec_meilisearch_search(entity_name: &String, json: String, client: &reqwest::Client) -> Result<impl warp::Reply, Infallible> {
        let index = format!(
            "http://meilisearch01.bumzack.at/indexes/{}/search",
            &entity_name
        );

        let response = client
            .post(&index)
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        match &response {
            Ok(res) => {
                let code = res.status().clone();
                if code == StatusCode::OK {
                    println!("meilisearch search request success");
                } else {
                    let x = res.headers().clone();
                    // let b = res.text().await.unwrap();
                    println!("meilisearch search request != OK. status {:?}", code);
                    println!("meilisearch search request != OK. headers {:?}", x);
                    // println!("meilisearch search request != OK. response body {:?}", &b);
                }
            }
            Err(e) => println!("error in request to meilisearch {:?}", e),
        };

        // 🙏 https://github.com/seanmonstar/warp/issues/38
        let stream = response.unwrap().bytes_stream();
        let body = hyper::Body::wrap_stream(stream);
        Ok(warp::reply::Response::new(body))
    }
}

