pub mod warp_stuff {
    use warp::cors::Builder;

    pub fn warp_cors() -> Builder {
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec![
                "User-Agent",
                "Sec-Fetch-Mode",
                "Referer",
                "Origin",
                "content-type",
                "Access-Control-Request-Method",
                "Access-Control-Request-Headers",
                "Access-Control-Allow-Headers",
                "Access-Control-Allow-Methods",
                "Access-Control-Allow-Origin",
                "Access-Control-Expose-Headers",
                "Access-Control-Request-Headers",
                "Access-Control-Request-Methods",
                "Accept-Encoding",
                "Accept-Language",
                "Accept-Post",
                "Access-Control-Allow-Credentials",
                "keep-alive",
            ])
            .allow_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "HEAD"])
    }
}
