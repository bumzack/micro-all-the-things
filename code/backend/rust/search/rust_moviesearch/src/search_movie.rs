pub mod filters_search_movie {
    use warp::{Filter, Reply};

    use common::meili_search::handlers_search_entity::{
        meili_search_movie, meili_search_movie_paginated,
    };
    use common::search::SearchPaginatedRequest;

    use crate::CLIENT;

    pub fn search_movie_route() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "movie" / String);
        let search = server.and(warp::get()).and_then(|name: String| {
            info!("GET /api/movie/:name  matched");
            meili_search_movie("movie".to_string(), name, &CLIENT)
        });

        let server1 = warp::path!("api" / "movie");
        let search_name = server1
            .and(warp::post())
            .and(search_movies_request())
            .and_then(|req: SearchPaginatedRequest| {
                //  info!("POST /api/movie/  matched");
                meili_search_movie_paginated("movie".to_string(), req, &CLIENT)
            });

        search_name.or(search)
    }

    fn search_movies_request() -> impl Filter<Extract=(SearchPaginatedRequest, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
