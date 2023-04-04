pub mod filters_build_index {
    use warp::Filter;

    use crate::build_search_index_v1::build_index_v1;
    use crate::build_search_index_v2::build_index_v2;

    pub fn build_index_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let v1 = warp::path!("api" / "v1" / "searchindex" / "build")
            .and(warp::get())
            .and_then(|| {
                info!("GET /api/v1/searchindex/build matched");
                build_index_v1()
            });

        let v2 = warp::path!("api" / "v2" / "searchindex" / "build" / u32)
            .and(warp::get())
            .and_then(|multiplier| {
                info!("GET /api/v2/searchindex/build matched");
                build_index_v2(multiplier)
            });

        v1.or(v2)
    }
}
