//! 管理员路由

use axum::{
    extract::{Extension, Path},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;

/// 列出用户
pub async fn list_users(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 调整积分
pub async fn adjust_credits(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_user_id): Path<Uuid>,
    Json(_req): Json<AdjustCreditsRequest>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 启用/禁用用户
pub async fn toggle_user(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_user_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 系统统计
pub async fn stats(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}
