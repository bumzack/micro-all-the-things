pub mod filters_search_crew {
    use std::convert::Infallible;

    use log::info;
    use warp::{Filter, Reply};

    use common::meili::meili_filter::meili_filter_crew::meili_filter_crew_vec;
    use common::solr::solr_filter::solr_filter_crew::solr_filter_crew_vec;

    use crate::CLIENT;

    pub fn filter_crew_route() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "crew" / "filter" / String);
        let search_crew_meili = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/meili/crew/filter/:tconst matched");
            filter_crew("tconst".to_string(), tconst, "meili".to_string())
        });

        let server = warp::path!("api" / "solr" / "crew" / "filter" / String);
        let search_crew_solr = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/solr/crew/filter/:tconst matched");
            filter_crew("tconst".to_string(), tconst, "solr".to_string())
        });

        search_crew_meili.or(search_crew_solr)
    }

    pub async fn filter_crew(
        filter_field: String,
        filter_value: String,
        engine: String,
    ) -> Result<impl Reply, Infallible> {
        let persons = match engine.as_str() {
            "solr" => solr_filter_crew_vec(filter_field, vec![filter_value], &CLIENT).await,
            "meili" => meili_filter_crew_vec(filter_field, vec![filter_value], &CLIENT).await,
            _ => vec![],
        };

        Ok(warp::reply::json(&persons))
    }
}
