pub mod filters_logging {
    use deadpool_postgres::Pool;
    use log::info;
    use warp::header::headers_cloned;
    use warp::http::HeaderMap;
    use warp::{Filter, Rejection, Reply};

    use common::models::customer_prices::AddCustomerPriceEntry;

    use crate::customerprices::customerprices_handler::filters_customer_price::{
        insert_customer_price_handler, insert_dummy_data_customer_prices_handler,
        read_customerprice_entries, read_customerprice_entry,
    };
    use crate::db::server::with_db;

    pub fn price_route(
        pool: Pool,
    ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        let server1 = warp::path!("api" / "v1" / "customerprice" / String / i32);
        let customerprice_get = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(
                |customer_id: String, year: i32, pool: Pool, headers: HeaderMap| {
                    info!("GET /api/v1/customerprice/:customer_id/:year");
                    read_customerprice_entry(pool, customer_id, year, headers)
                },
            );

        let server3 = warp::path!("api" / "v1" / "customerprice");
        let customerprice_insert = server3
            .and(with_db(pool.clone()))
            .and(warp::post())
            .and(json_body_add_customer_price())
            .and(headers_cloned())
            .and_then(
                |pool: Pool, req: AddCustomerPriceEntry, headers: HeaderMap| {
                    info!("POST  /api/v1/customerprice/  matched");
                    insert_customer_price_handler(pool, req, headers)
                },
            );

        let server1 =
            warp::path!("api" / "v1" / "customerprice" / "insertdummydata" / u32 / u32 / u32);
        let insert_dummy_data = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(
                |offset: u32, limit: u32, count: u32, pool: Pool, headers: HeaderMap| {
                    info!("GET /api/v1/customerprice/insertdummydata");
                    insert_dummy_data_customer_prices_handler(offset, limit, count, pool, headers)
                },
            );

        let server1 = warp::path!("api" / "v1" / "customerprices" / String);
        let customerprice_get = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(|customer_id: String, pool: Pool, headers: HeaderMap| {
                info!("GET /api/v1/customerprices/:customer_id");
                read_customerprice_entries(pool, customer_id, headers)
            });

        customerprice_get
            .or(customerprice_insert)
            .or(insert_dummy_data)
    }

    fn json_body_add_customer_price(
    ) -> impl Filter<Extract = (AddCustomerPriceEntry,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}
