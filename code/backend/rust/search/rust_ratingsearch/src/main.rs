#[macro_use]
extern crate log;

use std::io;

use warp::Filter;

mod search_rating;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(search_rating::filters_search_rating::search_rating_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("meilisearchrating"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18205)).await;

    Ok(())
}
