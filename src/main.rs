mod controllers;
mod store;

use dotenv::dotenv;
use warp::Filter;

fn post_json_body() -> impl Filter<Extract = (store::Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_json_body() -> impl Filter<Extract = (store::Id,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    // Get /hello/warp => 200 Ok with body "Hello, {input}!"
    // prettier-ignore
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    dotenv().ok();

    let test_env_str = std::env::var("TEST_ENV_STR").expect("TEST_ENV_STR must be set!");
    println!("ENV VAR: {}", test_env_str);

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json_body())
        .and(store_filter.clone())
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
