pub mod filters_movieaka {
    use warp::Filter;

    use common::tsv::tsv::TsvLines;

    use super::handlers_entity;

    pub fn movieaka_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path("api").and(movieaka_post())
    }

    pub fn movieaka_post(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("movieaka")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_entity::post_movieaka)
    }

    fn json_body_tsv_line() -> impl Filter<Extract = (TsvLines,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_entity {
    use std::convert::Infallible;

    use common::entity::entity::handlers_entity::post_entity;
    use common::entity::entity::Entity;
    use common::models::movieaka::MovieAkas;
    use common::tsv::tsv::TsvLines;

    use crate::CLIENT;

    pub async fn post_movieaka(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        post_entity::<MovieAkas>(tsv_lines, Entity::MOVIEAKA, &CLIENT).await
    }
}
