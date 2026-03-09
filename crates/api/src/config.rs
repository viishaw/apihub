//! 配置模块

use std::env;

/// 应用配置
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub master_key: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> crate::error::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL")
                .expect("REDIS_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            master_key: env::var("MASTER_KEY")
                .expect("MASTER_KEY must be set"),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
        })
    }
}
