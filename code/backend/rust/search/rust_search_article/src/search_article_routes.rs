pub mod search_article_routes {
    use log::info;
    use warp::Filter;

    use common::models::article::SearchArticleRequest;

    use crate::search_article_handler::handler_search_article::search_article;

    pub fn search_article_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "v1" / "meili" / "article");
        let search_meili = server
            .and(warp::post())
            .and(search_article_request())
            .and_then(|req| {
                info!("POST /api/v1/meili/article matched");
                search_article(req, "meili".to_string())
            });

        let server = warp::path!("api" / "v1" / "solr" / "article");
        let search_solr = server
            .and(warp::post())
            .and(search_article_request())
            .and_then(|req| {
                info!("POST /api/solr/article matched");
                search_article(req, "solr".to_string())
            });

        search_meili.or(search_solr)
    }

    fn search_article_request() -> impl Filter<Extract=(SearchArticleRequest, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
