pub mod filters_search_search_index {
    use std::convert::Infallible;

    use log::info;
    use warp::Filter;

    use common::logging::logging_service_client::logging_service;
    use common::meili::meili_search::meili_search_searchindex::meili_search_searchindex_vec;
    use common::models::search_doc::SearchIndexRequest;
    use common::solr::solr_search::solr_search_search_index::solr_search_search_index_vec;

    use crate::CLIENT;

    pub fn search_index_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "searchindex" / "search");
        let search_meili = server
            .and(warp::post())
            .and(search_index_request())
            .and_then(|req| {
                info!("POST /api/meili/searchindex/search matched");
                search_index(req, "meili".to_string())
            });

        let server = warp::path!("api" / "solr" / "searchindex" / "search");
        let search_solr = server
            .and(warp::post())
            .and(search_index_request())
            .and_then(|req| {
                info!("POST /api/solr/searchindex/search matched");
                search_index(req, "solr".to_string())
            });

        search_meili.or(search_solr)
    }

    fn search_index_request(
    ) -> impl Filter<Extract = (SearchIndexRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    pub async fn search_index(
        req: SearchIndexRequest,
        engine: String,
    ) -> Result<impl warp::Reply, Infallible> {
        let msg = format!(
            "start search_index(). search_text '{}', offset {}, limit {}, engine {}",
            req.q, req.offset, req.limit, &engine
        );

        logging_service::log_entry(
            "rust_search_search_index".to_string(),
            "INFO".to_string(),
            &msg,
        )
        .await;

        let facets = vec![
            "genres".to_string(),
            "actors".to_string(),
            "directors".to_string(),
            "titles".to_string(),
            "characters".to_string(),
            "titleType".to_string(),
        ];

        let search_docs = match engine.as_str() {
            "solr" => {
                solr_search_search_index_vec(req.q, req.limit, req.offset, facets, &CLIENT).await
            }
            "meili" => {
                meili_search_searchindex_vec(req.q, req.limit, req.offset, facets, &CLIENT).await
            }
            _ => vec![],
        };

        Ok(warp::reply::json(&search_docs))
    }
}
