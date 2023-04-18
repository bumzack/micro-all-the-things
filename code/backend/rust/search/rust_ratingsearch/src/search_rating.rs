pub mod filters_search_rating {
    use std::convert::Infallible;

    use log::info;
    use warp::{Filter, Reply};

    use common::entity::entity::{Engine, Entity};
    use common::meili::meili_entity::meili_entity_stuff::meili_filter_entity;
    use common::models::rating::Rating;
    use common::solr::solr_entity::solr_entity_stuff::solr_filter_entity;

    use crate::CLIENT;

    pub fn filter_rating_route() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "rating" / "filter" / String);
        let search_rating_meili = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/rating/filter/:tconst matched");
            filter_rating("tconst".to_string(), tconst, Engine::Meili)
        });

        let server = warp::path!("api" / "solr" / "rating" / "filter" / String);
        let search_rating_solr = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/rating/filter/:tconst matched");
            filter_rating("tconst".to_string(), tconst, Engine::Solr)
        });

        search_rating_meili.or(search_rating_solr)
    }

    pub async fn filter_rating(
        filter_field: String,
        filter_value: String,
        engine: Engine,
    ) -> Result<impl Reply, Infallible> {
        let principals = match engine {
            Engine::Solr => {
                solr_filter_entity::<Rating>(
                    Entity::RATING,
                    filter_field,
                    vec![filter_value],
                    &CLIENT,
                )
                    .await
            }
            Engine::Meili => {
                meili_filter_entity::<Rating>(
                    Entity::RATING,
                    filter_field,
                    vec![filter_value],
                    &CLIENT,
                )
                    .await
            }
        };

        Ok(warp::reply::json(&principals))
    }
}
