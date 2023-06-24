#[allow(warnings, unused)]
mod db;
mod filters;
mod handlers;
mod models;

// use dotenv::dotenv;
use models::{init_db, Db};
use warp::Filter;

#[tokio::main]
async fn main() {
    // dotenv().ok();

    // PRISMA CLIENT
    let db: Db = init_db().await;

    let api = filters::grocery_items(db);

    let routes = api.with(warp::log("grocery"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
