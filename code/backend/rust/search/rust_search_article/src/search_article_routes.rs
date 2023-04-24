pub mod mod_search_article_routes {
    use std::convert::Infallible;
    use std::time::Instant;

    use log::info;
    use warp::{Filter, Reply};
    use warp::header::headers_cloned;
    use warp::http::HeaderMap;

    use common::entity::entity::Engine;
    use common::logging::tracing_headers::tracing_headers_stuff::{
        build_response_from_json, build_tracing_headers, get_trace_infos,
    };
    use common::models::article::SearchArticleRequest;

    use crate::search_article_handler::handler_search_article::search_article;

    pub const SERVICE_NAME: &str = "search_article";

    pub fn search_article_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "v1" / "meili" / "article");
        let search_meili = server
            .and(warp::post())
            .and(search_article_request())
            .and(headers_cloned())
            .and_then(|req, headers: HeaderMap| {
                info!("POST /api/v1/meili/article matched");
                search_articles(req, Engine::Meili, headers)
            });

        let server = warp::path!("api" / "v1" / "solr" / "article");
        let search_solr = server
            .and(warp::post())
            .and(search_article_request())
            .and(headers_cloned())
            .and_then(|req, headers: HeaderMap| {
                info!("POST /api/solr/article matched");
                search_articles(req, Engine::Solr, headers)
            });

        search_meili.or(search_solr)
    }

    async fn search_articles(
        req: SearchArticleRequest,
        engine: Engine,
        headers: HeaderMap,
    ) -> Result<impl Reply, Infallible> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());
        info!("header stuff from start of 'search_article'   initiated_by {}, uuid {}, processed_by  {}",&initiated_by,&uuid, &processed_by);

        let (search_article_response, processed_by_new) =
            search_article(req, engine, &initiated_by, &uuid, &processed_by).await;

        info!(
            "processed_by_new from search_article  {}",
            &processed_by_new
        );

        let msg = format!(
            "found {} articles",
            &search_article_response.articles.as_ref().unwrap().len()
        );

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by_new,
            &msg,
        );

        headers.iter().for_each(|(name, value)| {
            info!("header   {}  -->   {:?}", &name, value);
        });

        info!(
            "processed_by_new from search_article  {}",
            &processed_by_new
        );
        let response = build_response_from_json(search_article_response, headers);

        Ok(response)
    }

    fn search_article_request() -> impl Filter<Extract=(SearchArticleRequest, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
