pub mod filters_search_movie {
    use warp::Filter;
    use common::meilisearch::handlers_search_entity;

    use crate::CLIENT;

    pub fn search_movie_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone
    {
        let server = warp::path!("api" / "movie" / String);
        let search = server
            .and(warp::get())
            .and_then(|name: String| handlers_search_entity::meili_search("movie".to_string(), name, &CLIENT));

        return search;
    }
}

