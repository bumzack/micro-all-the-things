use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

use config::Config;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

mod search_article_handler;
mod search_article_routes;
mod search_helper;
mod search_helper_prices;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .unwrap();
}

lazy_static::lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("/home/bumzack/micro-all-the-things/code/backend/rust/config.toml"))
        .build()
        .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Debug).init();

    let cors = warp::cors()
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
        ])
        .allow_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "HEAD"]);

    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let root = root
        .or(search_article_routes::mod_search_article_routes::search_article_route())
        .with(cors);

    let host: String = CONFIG
        .get("searcharticle_service_host")
        .expect("expected searcharticle_service_host variable");

    let port: u16 = CONFIG
        .get("searcharticle_service_port")
        .expect("expected searcharticle_service_port variable");

    let host = format!("{host}:{port}");

    info!("searcharticle host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(root).run(*addr).await;

    Ok(())
}
