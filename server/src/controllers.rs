use crate::db::PrismaClient;
use crate::store::{Id, Item, Store};
use std::sync::Arc;
use warp::http;

pub async fn get_grocery_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.grocery_list.read();
    Ok(warp::reply::json(&*result))
}

pub async fn update_grocery_list(
    item: Item,
    store: Store,
    prisma_client: Arc<PrismaClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);
    Ok(warp::reply::with_status(
        "Added items to grocery list",
        http::StatusCode::CREATED,
    ))
    // client
    //     .user()
    //     .create("SebMaz".to_string(), vec![])
    //     .exec()
    //     .await
    //     .expect("error creating user");
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
