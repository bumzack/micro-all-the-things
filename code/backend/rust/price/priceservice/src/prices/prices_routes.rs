pub mod filters_price {
    use deadpool_postgres::Pool;
    use log::info;
    use warp::header::headers_cloned;
    use warp::http::HeaderMap;
    use warp::{Filter, Rejection, Reply};

    use common::models::prices::SearchPricesRequest;

    use crate::db::server::with_db;
    use crate::prices::prices_handler::handlers_price::{
        insert_dummy_data, read_price_entries, read_price_entry,
    };

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
                read_price_entry(pool, tconst, headers)
            });

        let server1 = warp::path!("api" / "v1" / "price" / "insertdummydata" / u32 / u32 / u32);
        let insert_dummy_data = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(
                |offset: u32, limit: u32, count: u32, pool: Pool, headers: HeaderMap| {
                    info!("GET /api/v1/price/insertdummydata/:offet/:limit/:count");
                    insert_dummy_data(offset, limit, count, pool, headers)
                },
            );

        let s = warp::path!("api" / "v2" / "prices");
        let search_prices = s
            .and(search_prices_request())
            .and(with_db(pool.clone()))
            .and(warp::post())
            .and(headers_cloned())
            .and_then(|req: SearchPricesRequest, pool: Pool, headers: HeaderMap| {
                info!("POST /api/v2/prices");
                read_price_entries(req, pool, headers)
            });

        search_name.or(insert_dummy_data).or(search_prices)
    }

    fn search_prices_request(
    ) -> impl Filter<Extract = (SearchPricesRequest,), Error = Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
