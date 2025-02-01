use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("paralleler")]
pub struct Paralleler(sqlx::SqlitePool);
