pub mod filters_search_search_index {
    use std::convert::Infallible;

    use log::info;
    use warp::Filter;

    use common::entity::entity::{Engine, Entity};
    use common::logging::logging_service_client::logging_service;
    use common::meili::meili_entity::meili_entity_stuff::meili_search_entity_with_facets;
    use common::models::search_doc::{MovieSearchResult, SearchIndexDoc, SearchMovieIndexRequest};
    use common::solr::solr_entity::solr_entity_stuff::solr_search_entity_with_facets;

    use crate::CLIENT;

    pub fn search_index_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "v1" / "meili" / "searchindex" / "search");
        let search_meili = server
            .and(warp::post())
            .and(search_index_request())
            .and_then(|req| {
                info!("POST /api/v1/meili/searchindex/search matched");
                search_index(req, Engine::Meili)
            });

        let server = warp::path!("api" / "v1" / "solr" / "searchindex" / "search");
        let search_solr = server
            .and(warp::post())
            .and(search_index_request())
            .and_then(|req| {
                info!("POST /api/solr/searchindex/search matched");
                search_index(req, Engine::Solr)
            });

        search_meili.or(search_solr)
    }

    fn search_index_request() -> impl Filter<Extract=(SearchMovieIndexRequest, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    pub async fn search_index(
        req: SearchMovieIndexRequest,
        engine: Engine,
    ) -> Result<impl warp::Reply, Infallible> {
        let msg = format!(
            "start search_index(). search_text '{}', offset {}, limit {}, engine {:?}",
            req.q, req.offset, req.limit, engine
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

        Ok(warp::reply::json(&search_result))
    }
}
