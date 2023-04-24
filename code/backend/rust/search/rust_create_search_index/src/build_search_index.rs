pub mod filters_build_index {
    use log::info;
    use warp::header::headers_cloned;
    use warp::Filter;

    use common::entity::entity::Engine;

    use crate::build_search_index_v1::build_index_v1;
    use crate::build_search_index_v2::build_index_v2;
    use crate::build_search_index_v3::build_index_v3;
    use crate::build_search_index_v4::build_index_v4;

    pub fn build_index_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let v1 = warp::path!("api" / "v1" / "searchindex" / "build")
            .and(warp::get())
            .and_then(|| {
                info!("GET /api/v1/searchindex/build matched");
                build_index_v1()
            });

        let v2 = warp::path!("api" / "v2" / "searchindex" / "build" / u32 / u32 / u32)
            .and(warp::get())
            .and_then(|start, pagesize, multiplier| {
                info!("GET /api/v2/searchindex/build/:start/:pagesize/:multiplier matched");
                build_index_v2(start, pagesize, multiplier)
            });

        let v3_meili =
            warp::path!("api" / "v3" / "meili" / "searchindex" / "build" / u32 / u32 / u32)
                .and(warp::get())
                .and(headers_cloned())
                .and_then(|start, pagesize, tasks, headers| {
                    let engine = Engine::Meili;
                    info!("GET /api/v3/meili/searchindex/build/:start/:pagesize/:tasks matched");
                    build_index_v3(engine, start, pagesize, tasks, headers)
                });

        let v3_solr =
            warp::path!("api" / "v3" / "solr" / "searchindex" / "build" / u32 / u32 / u32)
                .and(warp::get())
                .and(headers_cloned())
                .and_then(|start, pagesize, tasks, headers| {
                    let engine = Engine::Solr;
                    info!("GET /api/v3/solr/searchindex/build/:start/:pagesize/:tasks matched");
                    build_index_v3(engine, start, pagesize, tasks, headers)
                });

        let v4_meili =
            warp::path!("api" / "v4" / "meili" / "searchindex" / "build" / u32 / u32 / u32 / u32)
                .and(warp::get())
                .and(headers_cloned())
                .and_then(|start, pagesize, max_movies, tasks, headers| {
                    let engine = Engine::Meili;
                    info!("GET /api/v4/meili/searchindex/build/:start/:pagesize/:tasks matched");
                    info!(
                        " start {}, pagesize {}, max_movies {}, tasks {}",
                        start, pagesize, max_movies, tasks
                    );
                    build_index_v4(engine, start, pagesize, max_movies, tasks, headers)
                });

        let v4_solr =
            warp::path!("api" / "v4" / "solr" / "searchindex" / "build" / u32 / u32 / u32 / u32)
                .and(warp::get())
                .and(headers_cloned())
                .and_then(|start, pagesize, max_movies, tasks, headers| {
                    let engine = Engine::Solr;
                    info!("GET /api/v4/solr/searchindex/build/:start/:pagesize/:tasks matched");
                    info!(
                        " start {}, pagesize {}, max_movies {}, tasks {}",
                        start, pagesize, max_movies, tasks
                    );
                    build_index_v4(engine, start, pagesize, max_movies, tasks, headers)
                });

        v1.or(v2).or(v3_meili).or(v3_solr).or(v4_meili).or(v4_solr)
    }
}
