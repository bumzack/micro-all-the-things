pub mod filters_search_movie {
    use std::convert::Infallible;

    use log::info;
    use reqwest::Client;
    use warp::{Filter, Reply};

    use common::entity::entity::{Engine, Entity};
    use common::meili::meili_entity::meili_entity_stuff::meili_filter_entity;
    use common::models::principal::Principal;
    use common::solr::solr_entity::solr_entity_stuff::solr_filter_entity;

    use crate::CLIENT;

    pub fn filter_principal_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "principal" / "filter" / "name" / String);
        let search_name_meili = server.and(warp::get()).and_then(|nconst: String| {
            info!("/api/meili/principal/filter/name/:nconst     matched");
            filter_principal("nconst".to_string(), nconst, Engine::Meili, &CLIENT)
        });

        let server2 = warp::path!("api" / "meili" / "principal" / "filter" / "title" / String);
        let filter_title_meili = server2.and(warp::get()).and_then(|tconst: String| {
            info!("/api/meili/principal/filter/title/:tconst     matched");
            filter_principal("tconst".to_string(), tconst, Engine::Meili, &CLIENT)
        });

        let server = warp::path!("api" / "solr" / "principal" / "filter" / "name" / String);
        let search_name_solr = server.and(warp::get()).and_then(|nconst: String| {
            info!("/api/solr/principal/filter/name/:nconst     matched");
            filter_principal("nconst".to_string(), nconst, Engine::Solr, &CLIENT)
        });

        let server2 = warp::path!("api" / "solr" / "principal" / "filter" / "title" / String);
        let filter_title_solr = server2.and(warp::get()).and_then(|tconst: String| {
            info!("/api/solr/principal/filter/title/:tconst     matched");
            filter_principal("tconst".to_string(), tconst, Engine::Solr, &CLIENT)
        });

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
    ) -> Result<impl Reply, Infallible> {
        let principals = match engine {
            Engine::Solr => {
                solr_filter_entity::<Principal>(
                    Entity::PRINCIPAL,
                    filter_field,
                    vec![filter_value],
                    client,
                )
                .await
            }
            Engine::Meili => {
                meili_filter_entity::<Principal>(
                    Entity::PRINCIPAL,
                    filter_field,
                    vec![filter_value],
                    client,
                )
                .await
            }
        };

        Ok(warp::reply::json(&principals))
    }
}
