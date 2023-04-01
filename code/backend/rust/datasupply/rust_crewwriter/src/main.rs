use std::io;

use warp::Filter;

use crate::crew_rest::filters_crew;

mod crew_rest;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(filters_crew::crew_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("crewwriter"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18102)).await;

    Ok(())
}
