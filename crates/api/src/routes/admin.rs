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
    // TODO: 实现列出用户逻辑
    // 1. 验证用户是管理员
    // 2. 查询群组内所有用户
    // 3. 返回用户列表
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 调整积分
pub async fn adjust_credits(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_user_id): Path<Uuid>,
    Json(_req): Json<AdjustCreditsRequest>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现调整积分逻辑
    // 1. 验证用户是管理员
    // 2. 调整目标用户积分
    // 3. 记录交易
    // 4. 返回新积分
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 启用/禁用用户
pub async fn toggle_user(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_user_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现切换用户状态逻辑
    // 1. 验证用户是管理员
    // 2. 切换用户 is_active 状态
    // 3. 返回新状态
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 系统统计
pub async fn stats(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现系统统计逻辑
    // 1. 验证用户是管理员
    // 2. 统计群组数据
    // 3. 返回统计数据
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}
