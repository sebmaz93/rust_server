#[allow(warnings, unused)]
pub mod db;

mod controllers;
mod store;

use crate::db::*;
use dotenv::dotenv;
use std::convert::Infallible;
use std::sync::Arc;
// use prisma_client_rust::NewClientError;
use warp::Filter;

fn post_json_body() -> impl Filter<Extract = (store::Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_json_body() -> impl Filter<Extract = (store::Id,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_prisma(
    prisma_client: Arc<&PrismaClient>,
) -> impl Filter<Extract = (Arc<&PrismaClient>,), Error = Infallible> + Clone {
    warp::any().map(move || prisma_client.clone())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Get /hello/warp => 200 Ok with body "Hello, {input}!"
    // prettier-ignore
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // PRISMA CLIENT
    let client: PrismaClient = PrismaClient::_builder()
        .build()
        .await
        .expect("Prisma client must start.");

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json_body())
        .and(store_filter.clone())
        .and(with_prisma(Arc::new(&client).clone()))
        .and_then(controllers::update_grocery_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(controllers::get_grocery_list);

    let update_items = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json_body())
        .and(store_filter.clone())
        .and(with_prisma(Arc::new(&client).clone()))
        .and_then(controllers::update_grocery_list);

    let delete_items = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json_body())
        .and(store_filter.clone())
        .and_then(controllers::delete_grocery_list_item);

    let routes = add_items.or(get_items).or(update_items).or(delete_items);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
