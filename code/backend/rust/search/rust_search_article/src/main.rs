use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

use config::Config;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

use common::server::warp_cors::warp_stuff::warp_cors;

mod search_article_handler;
mod search_article_routes;
mod search_helper;
mod search_helper_prices;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

lazy_static::lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("/Users/bumzack/stoff/micro-all-the-things/code/backend/rust/config.toml"))
        .build()
        .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let root = root
        .or(search_article_routes::mod_search_article_routes::search_article_route())
        .with(warp_cors());

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
