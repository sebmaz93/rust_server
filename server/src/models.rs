use crate::db::PrismaClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type Db = Arc<PrismaClient>;

pub async fn init_db() -> Db {
    Arc::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("Prisma client must start."),
    )
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GroceryItem {
    id: Option<String>,
    pub name: String,
    pub quantity: i32,
}
