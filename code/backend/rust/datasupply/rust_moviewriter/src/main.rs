use std::io;

use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

use crate::movie_rest::filters_movie;

mod movie_rest;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();
    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(filters_movie::movie_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("moviewriter"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18100)).await;

    Ok(())
}
