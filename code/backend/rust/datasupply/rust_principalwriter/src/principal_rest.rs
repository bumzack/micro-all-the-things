pub mod filters_principal {
    use warp::Filter;

    use common::tsv::tsv::TsvLines;

    use super::handlers_entity;

    pub fn principal_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        warp::path("api").and(principal_post())
    }

    pub fn principal_post() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        warp::path!("principal")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_entity::post_principal)
    }

    fn json_body_tsv_line() -> impl Filter<Extract=(TsvLines, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_entity {
    use std::convert::Infallible;

    use common::entity::entity::Entity;
    use common::entity::entity::handlers_entity::post_entity;
    use common::models::principal::Principal;
    use common::tsv::tsv::TsvLines;

    use crate::CLIENT;

    pub async fn post_principal(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        post_entity::<Principal>(tsv_lines, Entity::PRINCIPAL, &CLIENT).await
    }
}
