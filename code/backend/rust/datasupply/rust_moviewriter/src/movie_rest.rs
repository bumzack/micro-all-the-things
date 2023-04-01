pub mod filters_movie {
    use warp::Filter;

    use common::TsvLines;

    use super::handlers_entity;

    pub fn movie_route() ->  impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path("api").and(movie_post())
    }

    pub fn movie_post(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("movie")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_entity::post_movie)
    }

    fn json_body_tsv_line() -> impl Filter<Extract = (TsvLines,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_entity {
    use std::convert::Infallible;

    use common::handlers_entity::post_entity;
    use common::{Movie, Principal, TsvLines};

    use crate::CLIENT;

    pub async fn post_movie(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        post_entity::<Movie>(tsv_lines, "movie".to_string(), &CLIENT).await
    }
}
