use std::io;
use std::net::SocketAddr;

use config::Config;
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

use crate::tsv_rest::filters_tsv;

mod tsv_rest;

lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name(
            "/home/bumzack/micro-all-the-things/code/backend/rust/config.toml"
        ))
        .build()
        .unwrap();
}

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let root = warp::path::end().map(|| "Welcome to the rust TSV reader!");

    let root = root.or(filters_tsv::tsv_request_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("tsvfilereader"));
    // Start up the server...

    let host: String = CONFIG.get("server_host").expect("expected server host");
    let port: String = CONFIG.get("server_port").expect("expected server host");

    info!("host {}", &host);
    info!("port {}", &port);
    let h = format!("{}:{}", host, port);
    info!("host complete {}", &h);

    let server: SocketAddr = h.parse().expect("expected to be a valid SocketAddr");

    warp::serve(routes).run(server).await;

    Ok(())
}
