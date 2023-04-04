#[macro_use]
extern crate log;

use std::io;

use warp::Filter;

mod search_crew;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(search_crew::filters_search_crew::search_crew_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("meilisearchcrew"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18205)).await;

    Ok(())
}
