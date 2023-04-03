use std::io;
use std::collections::HashMap;
use std::io;

use config::Config;
use warp::Filter;

mod build_search_index;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}


lazy_static::lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("/home/bumzack/micro-all-the-things/code/backend/rust/search/rust_create_search_index/config.toml"))
        .build()
        .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!(
        "{:?}",
        CONFIG
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(build_search_index::filters_search_movie::build_index_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("build_search_index"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18300)).await;

    Ok(())
}
