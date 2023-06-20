use crate::store::{Id, Item, Store};
use std::collections::HashMap;
use warp::http;

pub async fn get_grocery_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.grocery_list.read();
    Ok(warp::reply::json(&*result))
}

pub async fn update_grocery_list(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);
    Ok(warp::reply::with_status(
        "Added items to grocery list",
        http::StatusCode::CREATED,
    ))
}

pub async fn delete_grocery_list_item(
    id: Id,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().remove(&id.name);
    Ok(warp::reply::with_status(
        format!("Removed {} from grocery list", &id.name),
        http::StatusCode::OK,
    ))
}
