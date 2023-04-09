pub mod filters_search_rating {
    use std::convert::Infallible;

    use log::info;
    use warp::{Filter, Reply};

    use common::meili::meili_filter::meili_filter_rating::meili_filter_rating_vec;
    use common::solr::solr_filter::solr_filter_rating::solr_filter_rating_vec;

    use crate::CLIENT;

    pub fn filter_rating_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "rating" / "filter" / String);
        let search_rating_meili = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/rating/filter/:tconst matched");
            filter_rating("rating".to_string(), tconst, "meili".to_string())
        });

        let server = warp::path!("api" / "rating" / "filter" / String);
        let search_rating_solr = server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/rating/filter/:tconst matched");
            filter_rating("rating".to_string(), "tconst".to_string(), tconst)
        });

        search_rating_meili.or(search_rating_solr)
    }

    pub async fn filter_rating(
        filter_field: String,
        filter_value: String,
        engine: String,
    ) -> Result<impl Reply, Infallible> {
        let principals = match engine.as_str() {
            "solr" => solr_filter_rating_vec(filter_field, vec![filter_value], &CLIENT).await,
            "meili" => meili_filter_rating_vec(filter_field, vec![filter_value], &CLIENT).await,
            _ => vec![],
        };

        Ok(warp::reply::json(&principals))
    }
}
