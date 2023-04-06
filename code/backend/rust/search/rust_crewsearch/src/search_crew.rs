pub mod filters_search_crew {
    use std::convert::Infallible;
    use std::future::Future;

    use warp::{Filter, Reply};

    use common::meili_filter::handlers_search_entity::meili_filter_crew;

    use crate::CLIENT;

    pub fn search_crew_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let server = warp::path!("api" / "crew" / String);
        server.and(warp::get()).and_then(|tconst: String| {
            info!("/api/crew/:tconst matched");
            filter_entity("crew".to_string(), "tconst".to_string(), tconst)
        })
    }

    fn filter_entity(
        entity: String,
        attribute: String,
        value: String,
    ) -> impl Future<Output = Result<impl Reply + Sized, Infallible>> {
        info!("filter_entity  {attribute} =  {value}");
        let f = format!("\"{}\"  = \"{}\"", attribute, value);
        let filter: Vec<String> = vec![f];
        meili_filter_crew(entity, filter, &CLIENT)
    }
}
