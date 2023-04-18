use std::io;

use config::Config;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

mod search_search_index;

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

    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let root = root.or(search_search_index::filters_search_search_index::search_index_route());

    let routes = root.with(warp::log("search_search_index"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18320)).await;

    Ok(())
}
