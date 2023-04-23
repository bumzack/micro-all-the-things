pub mod handler_customer {
    use deadpool_postgres::Pool;
    use warp::header::headers_cloned;
    use warp::http::HeaderMap;
    use warp::{Filter, Rejection, Reply};

    use common::models::customer::AddCustomer;

    use crate::customer::customer_handler::handler_customer::{
        insert_customer_handler, insert_dummy_data_handler, read_customer_handler,
        read_customer_paginated_handler,
    };
    use crate::db::server::with_db;

    pub fn customer_route(
        pool: Pool,
    ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        let server1 = warp::path!("api" / "v1" / "customer" / String);
        let customer_get = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(|email: String, pool: Pool, headers: HeaderMap| {
                info!("GET /api/v1/customer/:email");
                read_customer_handler(pool, email, headers)
            });

        let server1 = warp::path!("api" / "v1" / "customer" / "paginated" / i32 / i32);
        let customer_get_paginated = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(|offset: i32, limit: i32, pool: Pool, headers: HeaderMap| {
                info!("GET /api/v1/customer/paginated/:offset/:limit");
                read_customer_paginated_handler(pool, offset, limit, headers)
            });

        let server3 = warp::path!("api" / "v1" / "customer");
        let customer_insert = server3
            .and(with_db(pool.clone()))
            .and(warp::post())
            .and(json_body_add_customer())
            .and(headers_cloned())
            .and_then(|pool: Pool, req: AddCustomer, headers: HeaderMap| {
                info!("POST  /api/customerprices/entry  matched");
                insert_customer_handler(pool, req, headers)
            });

        let server1 = warp::path!("api" / "v1" / "customer" / "insertdummydata" / u32 / u32 / u32);
        let insert_dummy_data = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(
                |offset: u32, limit: u32, count: u32, pool: Pool, headers: HeaderMap| {
                    info!("GET /api/v1/customer/insertdummydata");
                    insert_dummy_data_handler(offset, limit, count, pool, headers)
                },
            );

        customer_get
            .or(customer_insert)
            .or(insert_dummy_data)
            .or(customer_get_paginated)
    }

    fn json_body_add_customer(
    ) -> impl Filter<Extract = (AddCustomer,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}
