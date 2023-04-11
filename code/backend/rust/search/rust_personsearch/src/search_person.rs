pub mod filters_search_person {
    use std::convert::Infallible;

    use log::info;
    use reqwest::Client;
    use warp::{Filter, Reply};

    use common::entity::entity::Entity;
    use common::meili::meili_entity::meili_entity_stuff::{meili_filter_entity, meili_read_doc};
    use common::models::person::{Person, SearchPersonList};
    use common::models::search_doc::SearchPaginatedRequest;
    use common::solr::solr_entity::solr_entity_stuff::{solr_filter_entity, solr_read_doc};

    use crate::CLIENT;

    pub fn filter_person_route() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
        let server3 = warp::path!("api" / "meili" / "person" / "filter");
        let search_nconsts_meili = server3
            .and(warp::post())
            .and(json_body_search_person_list())
            .and_then(|req: SearchPersonList| {
                info!("POST  /api/meili/person/filter      matched");
                filter_person(
                    "nconst".to_string(),
                    req.nconsts,
                    "meili".to_string(),
                    &CLIENT,
                )
            });

        let server3 = warp::path!("api" / "solr" / "person" / "filter");
        let search_nconsts_solr = server3
            .and(warp::post())
            .and(json_body_search_person_list())
            .and_then(|req: SearchPersonList| {
                info!("POST  /api/solr/person/filter       matched");
                filter_person(
                    "nconst".to_string(),
                    req.nconsts,
                    "solr".to_string(),
                    &CLIENT,
                )
            });

        let server = warp::path!("api" / "v1" / "meili" / "person");
        let search_name_meili = server
            .and(warp::post())
            .and(search_persons_request())
            .and_then(|req: SearchPaginatedRequest| {
                info!("POST /api/v1/meili/person/  matched");
                read_person_documents(req.offset, req.limit, "meili".to_string(), &CLIENT)
            });

        let server = warp::path!("api" / "v1" / "solr" / "person");
        let search_name_solr = server
            .and(warp::post())
            .and(search_persons_request())
            .and_then(|req: SearchPaginatedRequest| {
                info!("POST /api/v1/solr/person/  matched");
                read_person_documents(req.offset, req.limit, "solr".to_string(), &CLIENT)
            });

        search_nconsts_meili
            .or(search_nconsts_solr)
            .or(search_name_solr)
            .or(search_name_meili)
    }

    fn json_body_search_person_list() -> impl Filter<Extract=(SearchPersonList, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }

    pub async fn filter_person(
        filter_field: String,
        filter_values: Vec<String>,
        engine: String,
        client: &Client,
    ) -> Result<impl Reply, Infallible> {
        let persons = match engine.as_str() {
            "solr" => {
                solr_filter_entity::<Person>(Entity::PERSON, filter_field, filter_values, client)
                    .await
            }
            "meili" => {
                meili_filter_entity::<Person>(Entity::PERSON, filter_field, filter_values, client)
                    .await
            }
            _ => vec![],
        };

        Ok(warp::reply::json(&persons))
    }

    pub async fn read_person_documents(
        offset: u32,
        limit: u32,
        engine: String,
        client: &Client,
    ) -> Result<impl Reply, Infallible> {
        let persons = match engine.as_str() {
            "solr" => solr_read_doc::<Person>(Entity::PERSON, offset, limit, client).await,
            "meili" => meili_read_doc::<Person>(Entity::PERSON, offset, limit, client).await,
            _ => vec![],
        };

        Ok(warp::reply::json(&persons))
    }

    fn search_persons_request() -> impl Filter<Extract=(SearchPaginatedRequest, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
