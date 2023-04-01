pub mod filters_search_movie {
    use warp::Filter;
    use common::meilisearch::handlers_search_entity;

    use crate::CLIENT;

    pub fn create_search_index_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone
    {
        let server = warp::path!("api" / "serachindex" );
        let search = server
            .and(warp::get())
            .and_then(  handlers_search_entity::meili_search("movie".to_string(), name, &CLIENT));

        return search;
    }
}

