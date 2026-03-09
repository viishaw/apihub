//! Database crate - 数据库层

pub mod models;
pub mod repositories;
pub mod migrations;

use sea_orm::{Database as SeaDatabase, DatabaseConnection, DbErr};

pub type Database = DatabaseConnection;

/// 连接数据库
pub async fn connect(database_url: &str) -> Result<Database, DbErr> {
    SeaDatabase::connect(database_url).await
}

/// 连接 Redis
pub async fn redis_connect(redis_url: &str) -> Result<redis::Client, redis::RedisError> {
    redis::Client::open(redis_url)
}

/// 运行迁移
pub async fn run_migrations(_db: &Database) -> Result<(), DbErr> {
    // TODO: 实现迁移
    Ok(())
}
