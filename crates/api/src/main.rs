//! ApiHub - AI API 共享池
//! 
//! 私有化部署的 AI API 共享平台，积分驱动，公平调度。

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post, put, delete},
    Router,
    Extension,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    request_id::SetRequestIdLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod middleware;
mod config;
mod error;
mod dto;

use crate::config::Config;
use crate::error::Result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "apihub=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting ApiHub...");

    // 加载配置
    let config = Config::from_env()?;
    tracing::info!("📝 Configuration loaded");

    // 连接数据库
    let db = db::connect(&config.database_url).await?;
    tracing::info!("📦 Database connected");

    // 连接 Redis
    let redis = db::redis_connect(&config.redis_url).await?;
    tracing::info!("🔴 Redis connected");

    // 运行迁移
    db::run_migrations(&db).await?;
    tracing::info!("🔄 Migrations completed");

    // 创建应用状态
    let state = Arc::new(AppState {
        config: config.clone(),
        db,
        redis,
    });

    // 构建路由
    let app = Router::new()
        // 健康检查
        .route("/health", get(health_check))
        
        // 认证路由
        .route("/api/v1/auth/register", post(routes::auth::register))
        .route("/api/v1/auth/login", post(routes::auth::login))
        .route("/api/v1/auth/me", get(routes::auth::me))
        .route("/api/v1/auth/password", put(routes::auth::change_password))
        
        // 群组路由
        .route("/api/v1/groups/:id", get(routes::groups::get_group))
        .route("/api/v1/groups/:id/members", get(routes::groups::get_members))
        .route("/api/v1/groups/:id", put(routes::groups::update_group))
        .route("/api/v1/groups/:id/regenerate-invite", post(routes::groups::regenerate_invite))
        .route("/api/v1/groups/:id/members/:user_id", delete(routes::groups::remove_member))
        .route("/api/v1/groups/:id/leave", post(routes::groups::leave_group))
        
        // Key 管理路由
        .route("/api/v1/keys", post(routes::keys::create_key))
        .route("/api/v1/keys", get(routes::keys::list_keys))
        .route("/api/v1/keys/my", get(routes::keys::my_keys))
        .route("/api/v1/keys/:id", put(routes::keys::update_key))
        .route("/api/v1/keys/:id", delete(routes::keys::delete_key))
        .route("/api/v1/keys/:id/toggle", post(routes::keys::toggle_key))
        
        // 聊天路由（OpenAI 兼容）
        .route("/v1/chat/completions", post(routes::chat::completions))
        .route("/v1/messages", post(routes::chat::messages))
        .route("/v1/models", get(routes::chat::list_models))
        .route("/api/v1/chat/queue", get(routes::chat::queue_status))
        
        // 统计路由
        .route("/api/v1/stats/usage", get(routes::stats::usage))
        .route("/api/v1/stats/contribution", get(routes::stats::contribution))
        .route("/api/v1/stats/leaderboard", get(routes::stats::leaderboard))
        .route("/api/v1/stats/credits", get(routes::stats::credits))
        
        // 管理员路由
        .route("/api/v1/admin/users", get(routes::admin::list_users))
        .route("/api/v1/admin/users/:id/adjust-credits", post(routes::admin::adjust_credits))
        .route("/api/v1/admin/users/:id/toggle", post(routes::admin::toggle_user))
        .route("/api/v1/admin/stats", get(routes::admin::stats))
        
        // 中间件
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::x_request_id(uuid::Uuid::new_v4().to_string()))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(Extension(state));

    // 启动服务器
    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port).parse()?;
    tracing::info!("🌐 Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: db::Database,
    pub redis: redis::Client,
}

/// 健康检查
async fn health_check() -> &'static str {
    "OK"
}
