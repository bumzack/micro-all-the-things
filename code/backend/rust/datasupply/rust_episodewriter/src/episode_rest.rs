pub mod filters_episode {
    use warp::Filter;

    use common::tsv::TsvLines;

    use super::handlers_entity;

    pub fn episode_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        warp::path("api").and(principal_post())
    }

    pub fn principal_post() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        warp::path!("episode")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_entity::post_episode)
    }

    fn json_body_tsv_line() -> impl Filter<Extract=(TsvLines, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_entity {
    use std::convert::Infallible;

    use common::entity::handlers_entity::post_entity;
    use common::episode::Episode;
    use common::tsv::TsvLines;

    use crate::CLIENT;

    pub async fn post_episode(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        post_entity::<Episode>(tsv_lines, "episode".to_string(), &CLIENT).await
    }
}
