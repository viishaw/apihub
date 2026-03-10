//! 数据库层

pub mod models;
pub mod repositories;
pub mod migrations;

use sqlx::{PgPool, postgres::PgPoolOptions};
use redis::Client as RedisClient;
use anyhow::Result;

/// 连接 PostgreSQL
pub async fn connect_postgres(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    
    // 运行迁移
    migrations::run(&pool).await?;
    
    Ok(pool)
}

/// 连接 Redis
pub async fn connect_redis(redis_url: &str) -> Result<RedisClient> {
    let client = RedisClient::open(redis_url)?;
    // 测试连接
    let mut conn = client.get_connection()?;
    let _: String = redis::cmd("PING").query(&mut conn)?;
    Ok(client)
}
