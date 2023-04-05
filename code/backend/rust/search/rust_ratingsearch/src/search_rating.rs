pub mod filters_search_rating {
    use std::convert::Infallible;
    use std::future::Future;

    use warp::{Filter, Reply};

    use common::meili_filter::handlers_search_entity::meili_filter_rating;

    use crate::CLIENT;

    pub fn search_rating_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "rating" / String);
        server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/rating/:tconst matched");
            filter_entity("rating".to_string(), "tconst".to_string(), tconst)
        })
    }

    fn filter_entity(
        entity: String,
        attribute: String,
        value: String,
    ) -> impl Future<Output=Result<impl Reply + Sized, Infallible>> {
        info!("filter_entity  {attribute} =  {value}");
        let f = format!("\"{}\"  = \"{}\"", attribute, value);
        let filter: Vec<String> = vec![f];
        meili_filter_rating(entity, filter, &CLIENT)
    }
}
