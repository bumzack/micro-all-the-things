pub mod filters_search_search_index {
    use std::convert::Infallible;
    use std::time::Instant;

    use log::info;
    use warp::header::headers_cloned;
    use warp::hyper::HeaderMap;
    use warp::Filter;

    use common::entity::entity::{Engine, Entity};
    use common::logging::tracing_headers::tracing_headers_stuff::{
        build_response_from_json, build_tracing_headers, get_trace_infos,
    };
    use common::meili::meili_entity::meili_entity_stuff::meili_search_entity_with_facets;
    use common::models::search_doc::{MovieSearchResult, SearchIndexDoc, SearchMovieIndexRequest};
    use common::solr::solr_entity::solr_entity_stuff::solr_search_entity_with_facets;

    use crate::CLIENT;

    const SERVICE_NAME: &str = "Search Index Service";

    pub fn search_index_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let server = warp::path!("api" / "v1" / "meili" / "searchindex" / "search");
        let search_meili = server
            .and(warp::post())
            .and(search_index_request())
            .and(headers_cloned())
            .and_then(|req, headers: HeaderMap| {
                info!("POST /api/v1/meili/searchindex/search matched");
                search_index(req, Engine::Meili, headers)
            });

        let server = warp::path!("api" / "v1" / "solr" / "searchindex" / "search");
        let search_solr = server
            .and(warp::post())
            .and(search_index_request())
            .and(headers_cloned())
            .and_then(|req, headers: HeaderMap| {
                info!("POST /api/v1/solr/searchindex/search matched");
                search_index(req, Engine::Solr, headers)
            });

        search_meili.or(search_solr)
    }

    fn search_index_request(
    ) -> impl Filter<Extract = (SearchMovieIndexRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    pub async fn search_index(
        req: SearchMovieIndexRequest,
        engine: Engine,
        headers: HeaderMap,
    ) -> Result<impl warp::Reply, Infallible> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let _msg = format!(
            "start search_index(). search_text '{}', offset {}, limit {}, engine {:?}",
            req.q, req.offset, req.limit, engine
        );

        let facets = vec![
            "genres".to_string(),
            "actors".to_string(),
            "directors".to_string(),
            "titles".to_string(),
            "characters".to_string(),
            //           "title_type".to_string(),
        ];

        let search_result = match engine {
            Engine::Solr => {
                let (movies, facets) = solr_search_entity_with_facets::<SearchIndexDoc>(
                    Entity::SEARCHINDEX,
                    req.q,
                    req.limit,
                    req.offset,
                    facets,
                    &CLIENT,
                )
                .await;
                let facets = match facets {
                    Some(f) => f.facet_fields,
                    None => None,
                };
                MovieSearchResult { movies, facets }
            }
            Engine::Meili => {
                let (movies, facets) = meili_search_entity_with_facets::<SearchIndexDoc>(
                    Entity::SEARCHINDEX,
                    req.q,
                    req.limit,
                    req.offset,
                    facets,
                    &CLIENT,
                )
                .await;

                MovieSearchResult { movies, facets }
            }
        };

        let msg = format!(
            "found {} movies  using {:?}",
            search_result.movies.len(),
            &engine
        );
        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(search_result, headers);

        Ok(response)
    }
}
