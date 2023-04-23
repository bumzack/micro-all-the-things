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
    use common::meili::meili_entity::meili_entity_stuff::meili_filter_entity;
    use common::models::principal::Principal;
    use common::solr::solr_entity::solr_entity_stuff::solr_filter_entity;

    use crate::CLIENT;

    const SERVICE_NAME: &str = "Search Principal Service";

    pub fn filter_principal_route(
    ) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "principal" / "filter" / "name" / String);
        let search_name_meili = server.and(warp::get()).and(headers_cloned()).and_then(
            |nconst: String, headers: HeaderMap| {
                info!("/api/meili/principal/filter/name/:nconst     matched");
                filter_principal(
                    "nconst".to_string(),
                    nconst,
                    Engine::Meili,
                    &CLIENT,
                    headers,
                )
            },
        );

        let server2 = warp::path!("api" / "meili" / "principal" / "filter" / "title" / String);
        let filter_title_meili = server2.and(warp::get()).and(headers_cloned()).and_then(
            |tconst: String, headers: HeaderMap| {
                info!("/api/meili/principal/filter/title/:tconst     matched");
                filter_principal(
                    "tconst".to_string(),
                    tconst,
                    Engine::Meili,
                    &CLIENT,
                    headers,
                )
            },
        );

        let server = warp::path!("api" / "solr" / "principal" / "filter" / "name" / String);
        let search_name_solr = server.and(warp::get()).and(headers_cloned()).and_then(
            |nconst: String, headers: HeaderMap| {
                info!("/api/solr/principal/filter/name/:nconst     matched");
                filter_principal("nconst".to_string(), nconst, Engine::Solr, &CLIENT, headers)
            },
        );

        let server2 = warp::path!("api" / "solr" / "principal" / "filter" / "title" / String);
        let filter_title_solr = server2.and(warp::get()).and(headers_cloned()).and_then(
            |tconst: String, headers: HeaderMap| {
                info!("/api/solr/principal/filter/title/:tconst     matched");
                filter_principal("tconst".to_string(), tconst, Engine::Solr, &CLIENT, headers)
            },
        );

        filter_title_meili
            .or(filter_title_solr)
            .or(search_name_meili)
            .or(search_name_solr)
    }

    pub async fn filter_principal(
        filter_field: String,
        filter_value: String,
        engine: Engine,
        client: &Client,
        headers: HeaderMap,
    ) -> Result<impl Reply, Infallible> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let principals = match engine {
            Engine::Solr => {
                solr_filter_entity::<Principal>(
                    Entity::PRINCIPAL,
                    filter_field.clone(),
                    vec![filter_value.clone()],
                    client,
                )
                .await
            }
            Engine::Meili => {
                meili_filter_entity::<Principal>(
                    Entity::PRINCIPAL,
                    filter_field.clone(),
                    vec![filter_value.clone()],
                    client,
                )
                .await
            }
        };

        let msg = format!(
            "filtered {} principals using engine {:?}, field {}, value {}",
            principals.len(),
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

        let response = build_response_from_json(principals, headers);

        Ok(response)
    }
}
