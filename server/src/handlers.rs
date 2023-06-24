use crate::db::grocery_item;
use crate::models::{Db, GroceryItem};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::with_status;

pub async fn add_grocery_item(item: GroceryItem, db: Db) -> Result<impl warp::Reply, Infallible> {
    let res = db
        .grocery_item()
        .create(item.name, item.quantity, vec![])
        .exec()
        .await;

    match res {
        Ok(_res) => Ok(with_status("Item Added", StatusCode::CREATED)),
        Err(_err) => Ok(with_status("Error Adding item", StatusCode::BAD_REQUEST)),
    }
}

pub async fn get_grocery_items(db: Db) -> Result<impl warp::Reply, Infallible> {
    let res = db.grocery_item().find_many(vec![]).exec().await.unwrap();
    Ok(warp::reply::json(&res))
}

pub async fn update_grocery_item(
    id: String,
    item: GroceryItem,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let res = db
        .grocery_item()
        .update(
            grocery_item::id::equals(id),
            vec![
                grocery_item::name::set(item.name),
                grocery_item::quantity::set(item.quantity),
            ],
        )
        .exec()
        .await;

    match res {
        Ok(_res) => Ok(with_status("Item Updated", StatusCode::OK)),
        Err(_err) => Ok(with_status(
            "Must provide correct params",
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn delete_grocery_item(id: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let res = db
        .grocery_item()
        .delete(grocery_item::id::equals(id))
        .exec()
        .await;

    match res {
        Ok(_res) => Ok(with_status("Item Updated", StatusCode::NO_CONTENT)),
        Err(_err) => Ok(with_status(
            "Must provide correct params",
            StatusCode::NOT_FOUND,
        )),
    }
}
