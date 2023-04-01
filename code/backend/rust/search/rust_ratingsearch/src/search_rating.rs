pub mod filters_search_rating {
    use std::convert::Infallible;
    use std::future::Future;

    use warp::{Filter, Reply};

    use common::meilisearch::handlers_search_entity;

    use crate::CLIENT;

    pub fn search_rating_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone
    {
        let server = warp::path!("api" / "rating" / String);
        server
            .and(warp::get())
            .and_then(|tconst: String| {
                println!("/api/rating/:tconst matched");
                filter_entity("rating".to_string(), "tconst".to_string(), tconst)
            })
    }

    fn filter_entity(entity: String, attribute: String, value: String) -> impl Future<Output=Result<impl Reply + Sized, Infallible>> {
        println!("filter_entity  {attribute} =  {value}");
        let f = format!("\"{}\"  = \"{}\"", attribute, value);
        let filter: Vec<String> = vec![f];
        handlers_search_entity::meili_filter(entity, filter, &CLIENT)
    }
}

