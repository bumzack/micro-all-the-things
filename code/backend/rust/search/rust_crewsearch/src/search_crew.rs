pub mod filters_search_crew {
    use std::convert::Infallible;

    use log::info;
    use warp::{Filter, Reply};

    use common::entity::entity::{Engine, Entity};
    use common::meili::meili_entity::meili_entity_stuff::meili_filter_entity;
    use common::models::crew::Crew;
    use common::solr::solr_entity::solr_entity_stuff::solr_filter_entity;

    use crate::CLIENT;

    pub fn filter_crew_route() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "crew" / "filter" / String);
        let search_crew_meili = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/meili/crew/filter/:tconst matched");
            filter_crew("tconst".to_string(), tconst, Engine::Meili)
        });

        let server = warp::path!("api" / "solr" / "crew" / "filter" / String);
        let search_crew_solr = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/solr/crew/filter/:tconst matched");
            filter_crew("tconst".to_string(), tconst, Engine::Solr)
        });

        search_crew_meili.or(search_crew_solr)
    }

    pub async fn filter_crew(
        filter_field: String,
        filter_value: String,
        engine: Engine,
    ) -> Result<impl Reply, Infallible> {
        let persons = match engine {
            Engine::Solr => {
                solr_filter_entity::<Crew>(Entity::CREW, filter_field, vec![filter_value], &CLIENT)
                    .await
            }
            Engine::Meili => {
                meili_filter_entity::<Crew>(Entity::CREW, filter_field, vec![filter_value], &CLIENT)
                    .await
            }
        };

        Ok(warp::reply::json(&persons))
    }
}
