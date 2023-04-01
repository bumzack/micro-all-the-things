pub mod filters_person {
    use warp::Filter;

    use common::TsvLines;

    use super::handlers_entity;

    pub fn person_route() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path("api").and(principal_post())
    }

    pub fn principal_post() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        warp::path!("person")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_entity::post_person)
    }

    fn json_body_tsv_line() -> impl Filter<Extract=(TsvLines, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_entity {
    use std::convert::Infallible;

    use common::{Person, TsvLines};
    use common::handlers_entity::post_entity;

    use crate::CLIENT;

    pub async fn post_person(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        post_entity::<Person>(tsv_lines, "person".to_string(), &CLIENT).await
    }
}
