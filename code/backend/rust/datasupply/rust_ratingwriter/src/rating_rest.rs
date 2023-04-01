pub mod filters_rating {
    use warp::Filter;

    use common::TsvLines;

    use super::handlers_entity;

    pub fn rating_route() ->  impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path("api").and(rating_post())
    }

    pub fn rating_post(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("rating")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_entity::post_rating)
    }

    fn json_body_tsv_line() -> impl Filter<Extract = (TsvLines,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_entity {
    use std::convert::Infallible;

    use common::handlers_entity::post_entity;
    use common::{Rating, TsvLines};

    use crate::CLIENT;

    pub async fn post_rating(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        post_entity::<Rating>(tsv_lines, "rating".to_string(), &CLIENT).await
    }
}
