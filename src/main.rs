use std::env;
use warp::Filter;
extern crate log;
extern crate tokio;

mod filters;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=books=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "books=info");
    }
    pretty_env_logger::init();

    let db = models::blank_db();
    let api = filters::books(db);

    // View access logs by setting `RUST_LOG=books`.
    let routes = api.with(warp::log("books"));
    // Start up the server...
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

//Tests
#[cfg(test)]
mod tests;
