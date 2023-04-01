pub mod filters_search_person {
    use std::convert::Infallible;
    use std::future::Future;

    use warp::{Filter, Reply};

    use common::meilisearch::handlers_search_entity;

    use crate::CLIENT;

    pub fn search_person_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone
    {
        let server1 = warp::path!( "api" / "person" / String);
        let search_name = server1
            .and(warp::get())
            .and_then(|name: String| {
                println!("/api/person/:name matched");
                handlers_search_entity::meili_search("person".to_string(), name, &CLIENT)
            });

        let server2 = warp::path!( "api" / "personbynconst" / String);
        let search_nconst = server2
            .and(warp::get())
            .and_then(|name: String| {
                println!("/api/person/bynconst/:-name matched");
                filter_entity(name)
            });

        search_name.or(search_nconst)
    }

    fn filter_entity(name: String) -> impl Future<Output=Result<impl Reply + Sized, Infallible>> {
        println!("filter_entity   {name}");
        let f = format!("\"{}\"  = \"{}\"", "nconst", name);
        let filter: Vec<String> = vec![f];
        handlers_search_entity::meili_filter("person".to_string(), filter, &CLIENT)
    }
}

