pub mod filters_price {
    use deadpool_postgres::Pool;
    use warp::header::headers_cloned;
    use warp::http::HeaderMap;
    use warp::{Filter, Rejection, Reply};

    use crate::db::server::with_db;
    use crate::prices::prices_handler::handlers_price::{insert_dummy_data, read_price_entry};

    pub fn price_route(
        pool: Pool,
    ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        let server1 = warp::path!("api" / "v1" / "price" / String);
        let search_name = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(|tconst: String, pool: Pool, headers: HeaderMap| {
                info!("GET /api/v1/price/:tconst");
                read_price_entry(pool, tconst, headers.clone())
            });

        let server1 = warp::path!("api" / "v1" / "price" / "insertdummydata" / u32 / u32 / u32);
        let insert_dummy_data = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(
                |offset: u32, limit: u32, count: u32, pool: Pool, headers: HeaderMap| {
                    info!("GET /api/v1/price/insertdummydata/:offet/:limit/:count");
                    insert_dummy_data(offset, limit, count, pool, headers.clone())
                },
            );

        search_name.or(insert_dummy_data)
    }
}
