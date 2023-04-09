pub mod filters_search_movie {
    use std::convert::Infallible;

    use log::info;
    use reqwest::Client;
    use warp::{Filter, Reply};

    use common::meili::meili_read_docs::meilisearch_read_doc::meili_search_read_doc_movie_vec;
    use common::meili::meili_search::meili_search_movie::meili_search_movie_vec;
    use common::models::search_doc::SearchPaginatedRequest;
    use common::solr::solr_read_docs::solr_read_doc::solr_read_doc_movie_vec;
    use common::solr::solr_search::solr_search_movie::solr_search_movie_vec;

    use crate::CLIENT;

    pub fn search_movie_route() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "movie" / String);
        let search_meili = server.and(warp::get()).and_then(|search_text: String| {
            info!("GET /api/meili/movie/:search_text  matched");
            search_movie(search_text, "meili".to_string(), &CLIENT)
        });

        let server = warp::path!("api" / "solr" / "movie" / String);
        let search_solr = server.and(warp::get()).and_then(|search_text: String| {
            info!("GET /api/solr/movie/:search_text  matched");
            search_movie(search_text, "solr".to_string(), &CLIENT)
        });

        let server = warp::path!("api" / "meili" / "movie");
        let search_name_meili = server
            .and(warp::post())
            .and(search_movies_request())
            .and_then(|req: SearchPaginatedRequest| {
                info!("POST /api/meili/movie/  matched");
                read_movie_documents(req.offset, req.limit, "meili".to_string(), &CLIENT)
            });

        let server = warp::path!("api" / "solr" / "movie");
        let search_name_solr = server
            .and(warp::post())
            .and(search_movies_request())
            .and_then(|req: SearchPaginatedRequest| {
                info!("POST /api/solr/movie/  matched");
                read_movie_documents(req.offset, req.limit, "solr".to_string(), &CLIENT)
            });

        search_meili
            .or(search_solr)
            .or(search_name_meili)
            .or(search_name_solr)
    }

    pub async fn search_movie(
        search_text: String,
        engine: String,
        client: &Client,
    ) -> Result<impl Reply, Infallible> {
        let movies = match engine.as_str() {
            "solr" => solr_search_movie_vec(search_text, client).await,
            "meili" => meili_search_movie_vec(search_text, client).await,
            _ => vec![],
        };

        Ok(warp::reply::json(&movies))
    }

    pub async fn read_movie_documents(
        offset: u32,
        limit: u32,
        engine: String,
        client: &Client,
    ) -> Result<impl Reply, Infallible> {
        let movies = match engine.as_str() {
            "solr" => solr_read_doc_movie_vec(offset, limit, client).await,
            "meili" => meili_search_read_doc_movie_vec(offset, limit, client).await,
            _ => vec![],
        };

        Ok(warp::reply::json(&movies))
    }

    fn search_movies_request() -> impl Filter<Extract=(SearchPaginatedRequest, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
