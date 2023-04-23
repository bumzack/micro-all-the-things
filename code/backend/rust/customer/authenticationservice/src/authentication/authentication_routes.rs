pub mod handler_authentication {
    use deadpool_postgres::Pool;
    use warp::{Filter, Rejection, Reply};
    use warp::header::headers_cloned;
    use warp::http::HeaderMap;

    use common::models::authentication::{LogInRequest, LogOutRequest};

    use crate::authentication::authentication_handler::handler_authentication::{
        check_authenticated_handler, login_handler, logout_handler,
    };
    use crate::db::server::with_db;

    pub fn authentication_route(
        pool: Pool,
    ) -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
        let server1 = warp::path!("api" / "v1" / "authenticated" / i32);
        let authentication = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and(headers_cloned())
            .and_then(|customer_id: i32, pool: Pool, headers: HeaderMap| {
                info!("GET /api/v1/authenticated/:customer_id");
                check_authenticated_handler(pool, customer_id, headers.clone())
            });

        let server1 = warp::path!("api" / "v1" / "authentication" / "login");
        let authentication_login = server1
            .and(with_db(pool.clone()))
            .and(warp::post())
            .and(json_body_login())
            .and(headers_cloned())
            .and_then(|pool: Pool, login_request, headers: HeaderMap| {
                info!("POST /api/v1/authentication/login");
                login_handler(pool, login_request, headers.clone())
            });

        let server1 = warp::path!("api" / "v1" / "authentication" / "logout");
        let authentication_logout = server1
            .and(with_db(pool))
            .and(warp::post())
            .and(json_body_logout())
            .and(headers_cloned())
            .and_then(|pool: Pool, logout_request: LogOutRequest, headers: HeaderMap| {
                info!("POST /api/v1/authentication/logout");
                logout_handler(pool, logout_request, headers.clone())
            });

        // let server1 =
        //     warp::path!("api" / "v1" / "authentication" / "insertdummydata" / u32 / u32 / u32);
        // let insert_dummy_data = server1
        //     .and(with_db(pool.clone()))
        //     .and(warp::get())
        //     .and_then(|offset: u32, limit: u32, count: u32, pool: Pool| {
        //         info!("GET /api/v1/authentication/insertdummydata");
        //         insert_dummy_data_handler(offset, limit, count, pool)
        //     });

        authentication
            .or(authentication_login)
            .or(authentication_logout)
    }

    fn json_body_login() -> impl Filter<Extract=(LogInRequest, ), Error=warp::Rejection> + Clone
    {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }

    fn json_body_logout() -> impl Filter<Extract=(LogOutRequest, ), Error=warp::Rejection> + Clone
    {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}
