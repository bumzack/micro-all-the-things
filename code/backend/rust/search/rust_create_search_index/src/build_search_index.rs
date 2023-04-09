pub mod filters_build_index {
    use log::info;
    use warp::Filter;

    use crate::build_search_index_v1::build_index_v1;
    use crate::build_search_index_v2::build_index_v2;
    use crate::build_search_index_v3::build_index_v3;

    pub fn build_index_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
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
            warp::path!("api" / "v3" / "meili" / "searchindex" / "build"  / u32 / u32 / u32)
                .and(warp::get())
                .and_then(|start, pagesize, tasks| {
                    info!("GET /api/v3/meili/searchindex/build/:start/:pagesize/:tasks matched");
                    build_index_v3("meili".to_string(), start, pagesize, tasks)
                });

        let v3_solr =
            warp::path!("api" / "v3" / "solr" / "searchindex" / "build"  / u32 / u32 / u32)
                .and(warp::get())
                .and_then(|start, pagesize, tasks| {
                    info!("GET /api/v3/solr/searchindex/build/:start/:pagesize/:tasks matched");
                    build_index_v3("solr".to_string(), start, pagesize, tasks)
                });

        v1.or(v2).or(v3_meili).or(v3_solr)
    }
}
