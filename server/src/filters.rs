use crate::handlers;
use crate::models::{Db, GroceryItem};
use warp::Filter;

pub fn grocery_items(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    get_grocery_items(db.clone())
        .or(add_grocery_item(db.clone()))
        .or(update_grocery_item(db.clone()))
        .or(delete_grocery_item(db))
}

pub fn add_grocery_item(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("v1/groceries")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::add_grocery_item)
}

pub fn get_grocery_items(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("v1/groceries")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_grocery_items)
}

pub fn update_grocery_item(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("v1/groceries" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_grocery_item)
}

pub fn delete_grocery_item(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // making endpoint admin only
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("v1/groceries" / String)
        .and(admin_only)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_grocery_item)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (GroceryItem,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
