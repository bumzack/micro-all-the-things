pub mod filters_search_movieaka {
    use std::convert::Infallible;

    use log::info;
    use reqwest::Client;
    use warp::{Filter, Reply};

    use common::entity::entity::Entity;
    use common::meili::meili_entity::meili_entity_stuff::meili_filter_entity;
    use common::models::movieaka::MovieAkas;
    use common::solr::solr_entity::solr_entity_stuff::solr_filter_entity;

    use crate::CLIENT;

    pub fn filter_movieaka_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "movieaka" / "filter" / String);
        let search_name_meili = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/meili/movieaka/filter/:titleId     matched");
            filter_movieaka("titleId".to_string(), tconst, "meili".to_string(), &CLIENT)
        });

        let server = warp::path!("api" / "solr" / "movieaka" / "filter" / String);
        let search_name_solr = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/solr/movieaka/filter/:titleId     matched");
            filter_movieaka("titleId".to_string(), tconst, "solr".to_string(), &CLIENT)
        });

        search_name_meili.or(search_name_solr)
    }

    pub async fn filter_movieaka(
        filter_field: String,
        filter_value: String,
        engine: String,
        client: &Client,
    ) -> Result<impl Reply, Infallible> {
        let principals = match engine.as_str() {
            "solr" => {
                solr_filter_entity::<MovieAkas>(
                    Entity::MOVIEAKA,
                    filter_field,
                    vec![filter_value],
                    client,
                )
                    .await
            }
            "meili" => {
                meili_filter_entity::<MovieAkas>(
                    Entity::MOVIEAKA,
                    filter_field,
                    vec![filter_value],
                    client,
                )
                    .await
            }
            _ => vec![],
        };

        Ok(warp::reply::json(&principals))
    }
}
