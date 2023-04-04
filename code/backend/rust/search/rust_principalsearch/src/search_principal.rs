pub mod filters_search_movie {
    use std::convert::Infallible;
    use std::future::Future;

    use warp::{Filter, Reply};

    use common::meili_filter::handlers_search_entity::meili_filter_principal;

    use crate::CLIENT;

    pub fn search_principal_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let server1 = warp::path!("api" / "principal" / "name" / String);
        let search_name = server1.and(warp::get()).and_then(|nconst: String| {
            info!("/api/principal/name/:nconst     matched");
            filter_entity("principal".to_string(), "nconst".to_string(), nconst)
        });

        let server2 = warp::path!("api" / "principal" / "title" / String);
        let search_title = server2.and(warp::get()).and_then(|tconst: String| {
            info!("/api/principal/title/:tconst     matched");
            filter_entity("principal".to_string(), "tconst".to_string(), tconst)
        });
        search_name.or(search_title)
    }

    fn filter_entity(
        entity: String,
        attribute: String,
        value: String,
    ) -> impl Future<Output = Result<impl Reply + Sized, Infallible>> {
        //  println!("filter_entity  {attribute} =  {value}");
        let f = format!("\"{}\"  = \"{}\"", attribute, value);
        let filter: Vec<String> = vec![f];
        meili_filter_principal(entity, filter, &CLIENT)
    }
}
