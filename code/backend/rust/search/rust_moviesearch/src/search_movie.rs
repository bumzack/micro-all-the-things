pub mod filters_search_movie {
    use std::convert::Infallible;
    use std::time::Instant;

    use log::info;
    use reqwest::Client;
    use warp::header::headers_cloned;
    use warp::hyper::HeaderMap;
    use warp::{Filter, Reply};

    use common::entity::entity::{Engine, Entity};
    use common::logging::tracing_headers::tracing_headers_stuff::{
        build_response_from_json, build_tracing_headers, get_trace_infos,
    };
    use common::meili::meili_entity::meili_entity_stuff::{meili_read_doc, meili_search_entity};
    use common::models::movie::Movie;
    use common::models::search_doc::SearchPaginatedRequest;
    use common::solr::solr_entity::solr_entity_stuff::{solr_read_doc, solr_search_entity};

    use crate::CLIENT;

    const SERVICE_NAME: &str = "Search Movie";

    pub fn search_movie_route(
    ) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "movie" / String);
        let search_meili = server.and(warp::get()).and(headers_cloned()).and_then(
            |search_text: String, headers: HeaderMap| {
                info!("GET /api/meili/movie/:search_text  matched");
                search_movie(search_text, Engine::Meili, &CLIENT, headers.clone())
            },
        );

        let server = warp::path!("api" / "solr" / "movie" / String);
        let search_solr = server.and(warp::get()).and(headers_cloned()).and_then(
            |search_text: String, headers: HeaderMap| {
                info!("GET /api/solr/movie/:search_text  matched");
                search_movie(search_text, Engine::Solr, &CLIENT, headers.clone())
            },
        );

        let server = warp::path!("api" / "meili" / "movie");
        let search_name_meili = server
            .and(warp::post())
            .and(search_movies_request())
            .and(headers_cloned())
            .and_then(|req: SearchPaginatedRequest, headers: HeaderMap| {
                info!("POST /api/meili/movie/  matched");
                read_movie_documents(
                    req.offset,
                    req.limit,
                    Engine::Meili,
                    &CLIENT,
                    headers.clone(),
                )
            });

        let server = warp::path!("api" / "solr" / "movie");
        let search_name_solr = server
            .and(warp::post())
            .and(search_movies_request())
            .and(headers_cloned())
            .and_then(|req: SearchPaginatedRequest, headers: HeaderMap| {
                info!("POST /api/solr/movie/  matched");
                read_movie_documents(
                    req.offset,
                    req.limit,
                    Engine::Solr,
                    &CLIENT,
                    headers.clone(),
                )
            });

        search_meili
            .or(search_solr)
            .or(search_name_meili)
            .or(search_name_solr)
    }

    pub async fn search_movie(
        search_text: String,
        engine: Engine,
        client: &Client,
        headers: HeaderMap,
    ) -> Result<impl Reply, Infallible> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let movies = match engine {
            Engine::Solr => {
                solr_search_entity::<Movie>(
                    Entity::MOVIE,
                    search_text.clone(),
                    0,
                    50,
                    vec![],
                    client,
                )
                .await
            }
            Engine::Meili => {
                meili_search_entity::<Movie>(
                    Entity::MOVIE,
                    search_text.clone(),
                    0,
                    50,
                    vec![],
                    client,
                )
                .await
            }
        };

        let msg = format!(
            "found {} movies for search_text: {}",
            movies.len(),
            &search_text
        );
        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(movies, headers);

        Ok(response)
    }

    pub async fn read_movie_documents(
        offset: u32,
        limit: u32,
        engine: Engine,
        client: &Client,
        headers: HeaderMap,
    ) -> Result<impl Reply, Infallible> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let movies = match engine {
            Engine::Solr => solr_read_doc::<Movie>(Entity::MOVIE, offset, limit, client).await,
            Engine::Meili => meili_read_doc::<Movie>(Entity::MOVIE, offset, limit, client).await,
        };

        let msg = format!(
            "found {} movies for engine: {:?}, offset: {}, limit {}",
            movies.len(),
            &engine,
            offset,
            limit
        );

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(movies, headers);

        Ok(response)
    }

    fn search_movies_request(
    ) -> impl Filter<Extract = (SearchPaginatedRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
