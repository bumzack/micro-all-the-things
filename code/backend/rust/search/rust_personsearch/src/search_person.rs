pub mod filters_search_person {
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
    use common::meili::meili_entity::meili_entity_stuff::{meili_filter_entity, meili_read_doc};
    use common::models::person::{Person, SearchPersonList};
    use common::models::search_doc::SearchPaginatedRequest;
    use common::solr::solr_entity::solr_entity_stuff::{solr_filter_entity, solr_read_doc};

    use crate::CLIENT;

    const SERVICE_NAME: &str = "Search Person Service";

    pub fn filter_person_route(
    ) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
        let server3 = warp::path!("api" / "meili" / "person" / "filter");
        let search_nconsts_meili = server3
            .and(warp::post())
            .and(json_body_search_person_list())
            .and(headers_cloned())
            .and_then(|req: SearchPersonList, headers: HeaderMap| {
                info!("POST  /api/meili/person/filter      matched");
                filter_person(
                    "nconst".to_string(),
                    req.nconsts,
                    Engine::Meili,
                    &CLIENT,
                    headers,
                )
            });

        let server3 = warp::path!("api" / "solr" / "person" / "filter");
        let search_nconsts_solr = server3
            .and(warp::post())
            .and(json_body_search_person_list())
            .and(headers_cloned())
            .and_then(|req: SearchPersonList, headers: HeaderMap| {
                info!("POST  /api/solr/person/filter       matched");
                filter_person(
                    "nconst".to_string(),
                    req.nconsts,
                    Engine::Solr,
                    &CLIENT,
                    headers,
                )
            });

        let server = warp::path!("api" / "v1" / "meili" / "person");
        let search_name_meili = server
            .and(warp::post())
            .and(search_persons_request())
            .and(headers_cloned())
            .and_then(|req: SearchPaginatedRequest, headers: HeaderMap| {
                info!("POST /api/v1/meili/person/  matched");
                read_person_documents(req.offset, req.limit, Engine::Meili, &CLIENT, headers)
            });

        let server = warp::path!("api" / "v1" / "solr" / "person");
        let search_name_solr = server
            .and(warp::post())
            .and(search_persons_request())
            .and(headers_cloned())
            .and_then(|req: SearchPaginatedRequest, headers: HeaderMap| {
                info!("POST /api/v1/solr/person/  matched");
                read_person_documents(req.offset, req.limit, Engine::Solr, &CLIENT, headers)
            });

        search_nconsts_meili
            .or(search_nconsts_solr)
            .or(search_name_solr)
            .or(search_name_meili)
    }

    fn json_body_search_person_list(
    ) -> impl Filter<Extract = (SearchPersonList,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }

    pub async fn filter_person(
        filter_field: String,
        filter_values: Vec<String>,
        engine: Engine,
        client: &Client,
        headers: HeaderMap,
    ) -> Result<impl Reply, Infallible> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let persons = match engine {
            Engine::Solr => {
                solr_filter_entity::<Person>(Entity::PERSON, filter_field, filter_values, client)
                    .await
            }
            Engine::Meili => {
                meili_filter_entity::<Person>(Entity::PERSON, filter_field, filter_values, client)
                    .await
            }
        };

        let msg = format!(
            "filtered {} persons using engine {:?}",
            persons.len(),
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

        let response = build_response_from_json(persons, headers);

        Ok(response)
    }

    pub async fn read_person_documents(
        offset: u32,
        limit: u32,
        engine: Engine,
        client: &Client,
        headers: HeaderMap,
    ) -> Result<impl Reply, Infallible> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let persons = match engine {
            Engine::Solr => {
                let doc =
                    solr_read_doc::<Person>(Entity::PERSON, offset, limit, None, client).await;
                doc.0
            }
            Engine::Meili => meili_read_doc::<Person>(Entity::PERSON, offset, limit, client).await,
        };

        let msg = format!("read {} persons using engine {:?}", persons.len(), &engine);
        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(persons, headers);

        Ok(response)
    }

    fn search_persons_request(
    ) -> impl Filter<Extract = (SearchPaginatedRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
