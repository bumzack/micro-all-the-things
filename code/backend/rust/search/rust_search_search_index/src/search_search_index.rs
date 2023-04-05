pub mod filters_search_search_index {
    use std::convert::Infallible;

    use warp::Filter;

    use common::logging_service_client::logging_service;
    use common::meili_search::handlers_search_entity::meili_search_searchindex;
    use common::search::SearchIndexRequest;

    use crate::CLIENT;

    pub fn search_index_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("api" / "searchindex" / "search")
            .and(warp::post())
            .and(search_index_request())
            .and_then(|req| {
                info!("POST /api/searchindex/search matched");
                search_index(req)
            })
    }

    fn search_index_request(
    ) -> impl Filter<Extract = (SearchIndexRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    pub async fn search_index(req: SearchIndexRequest) -> Result<impl warp::Reply, Infallible> {
        let msg = format!(
            "start search_index(). search_text '{}', offset {}, limit {}",
            req.q, req.offset, req.limit
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
        ];

        meili_search_searchindex(
            "searchindex".to_string(),
            req.q,
            req.limit,
            req.offset,
            facets,
            &CLIENT,
        )
        .await
    }
}
