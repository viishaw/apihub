//! 管理员路由

use axum::{
    extract::{Extension, Path},
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::Result;
use crate::AppState;

/// 列出用户
pub async fn list_users(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现列出用户逻辑
    todo!()
}

/// 调整积分
pub async fn adjust_credits(
    Extension(state): Extension<Arc<AppState>>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现调整积分逻辑
    todo!()
}

/// 启用/禁用用户
pub async fn toggle_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现切换用户状态逻辑
    todo!()
}

/// 系统统计
pub async fn stats(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现系统统计逻辑
    todo!()
}
