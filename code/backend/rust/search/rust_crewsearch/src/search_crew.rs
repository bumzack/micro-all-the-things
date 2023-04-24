pub mod filters_search_crew {
    use std::convert::Infallible;
    use std::time::Instant;

    use log::info;
    use warp::{Filter, Reply};
    use warp::header::headers_cloned;
    use warp::hyper::HeaderMap;

    use common::entity::entity::{Engine, Entity};
    use common::logging::tracing_headers::tracing_headers_stuff::{
        build_response_from_json, build_tracing_headers, get_trace_infos,
    };
    use common::meili::meili_entity::meili_entity_stuff::meili_filter_entity;
    use common::models::crew::Crew;
    use common::solr::solr_entity::solr_entity_stuff::solr_filter_entity;

    use crate::CLIENT;

    const SERVICE_NAME: &str = "Search Crew Service";

    pub fn filter_crew_route() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "crew" / "filter" / String);
        let search_crew_meili = server.and(warp::get()).and(headers_cloned()).and_then(
            |tconst: String, headers: HeaderMap| {
                info!("/api/meili/crew/filter/:tconst matched");
                filter_crew("tconst".to_string(), tconst, Engine::Meili, headers)
            },
        );

        let server = warp::path!("api" / "solr" / "crew" / "filter" / String);
        let search_crew_solr = server.and(warp::get()).and(headers_cloned()).and_then(
            |tconst: String, headers: HeaderMap| {
                info!("/api/solr/crew/filter/:tconst matched");
                filter_crew("tconst".to_string(), tconst, Engine::Solr, headers)
            },
        );

        search_crew_meili.or(search_crew_solr)
    }

    pub async fn filter_crew(
        filter_field: String,
        filter_value: String,
        engine: Engine,
        headers: HeaderMap,
    ) -> Result<impl Reply, Infallible> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let crew = match engine {
            Engine::Solr => {
                solr_filter_entity::<Crew>(
                    Entity::CREW,
                    filter_field.clone(),
                    vec![filter_value.clone()],
                    &CLIENT,
                )
                    .await
            }
            Engine::Meili => {
                meili_filter_entity::<Crew>(
                    Entity::CREW,
                    filter_field.clone(),
                    vec![filter_value.clone()],
                    &CLIENT,
                )
                    .await
            }
        };

        let msg = format!(
            "filtered {} crew using engine {:?}, field {}, value {}",
            crew.len(),
            &engine,
            filter_field,
            filter_value
        );
        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(crew, headers);

        Ok(response)
    }
}
