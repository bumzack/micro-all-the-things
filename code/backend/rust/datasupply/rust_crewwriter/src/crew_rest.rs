pub mod filters_crew {
    use warp::Filter;
    use common::tsv::TsvLines;


    use super::handlers_entity;

    pub fn crew_route() ->  impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path("api").and(crew_post())
    }

    pub fn crew_post() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path!("crew")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_entity::post_crew)
    }

    fn json_body_tsv_line() -> impl Filter<Extract = (TsvLines,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_entity {
    use std::convert::Infallible;
    use common::crew::Crew;
    use common::entity::handlers_entity::post_entity;
    use common::tsv::TsvLines;


    use crate::CLIENT;

    pub async fn post_crew(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        post_entity::<Crew>(tsv_lines, "crew".to_string(), &CLIENT).await
    }
}
