use crate::db::grocery_item;
use crate::db::PrismaClient;
use crate::store::{Id, Item};
use std::sync::Arc;
use warp::http;

pub async fn add_grocery_list_item(
    item: Item,
    db: Arc<PrismaClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    db.grocery_item()
        .create(item.name, item.quantity, vec![])
        .exec()
        .await
        .expect("must provide correct params");
    Ok(warp::reply::with_status(
        "Added items to grocery list",
        http::StatusCode::CREATED,
    ))
}

pub async fn get_grocery_list(db: Arc<PrismaClient>) -> Result<impl warp::Reply, warp::Rejection> {
    let result = db.grocery_item().find_many(vec![]).exec().await.unwrap();
    Ok(warp::reply::json(&result))
}

pub async fn update_grocery_list_item(
    item: Item,
    db: Arc<PrismaClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    db.grocery_item()
        .update(
            grocery_item::name::equals(item.name),
            vec![grocery_item::quantity::set(item.quantity)],
        )
        .exec()
        .await
        .expect("must provide correct params");
    Ok(warp::reply::with_status(
        "Updated item",
        http::StatusCode::OK,
    ))
}

pub async fn delete_grocery_list_item(
    id: Id,
    db: Arc<PrismaClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    db.grocery_item()
        .delete(grocery_item::id::equals(id.val))
        .exec()
        .await
        .expect("must provide correct ID");
    Ok(warp::reply::with_status(
        "Removed item",
        http::StatusCode::OK,
    ))
}
