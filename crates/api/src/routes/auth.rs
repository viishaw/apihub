//! 认证路由

use axum::{
    extract::Extension,
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;

/// 注册
pub async fn register(
    Extension(_state): Extension<Arc<AppState>>,
    Json(_req): Json<RegisterRequest>,
) -> Result<Json<SuccessResponse<AuthResponse>>> {
    // TODO: 实现注册逻辑
    // 1. 验证输入
    // 2. 检查群组是否存在或创建新群组
    // 3. 创建用户
    // 4. 生成 JWT
    // 5. 返回用户信息和 token
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 登录
pub async fn login(
    Extension(_state): Extension<Arc<AppState>>,
    Json(_req): Json<LoginRequest>,
) -> Result<Json<SuccessResponse<AuthResponse>>> {
    // TODO: 实现登录逻辑
    // 1. 查找用户
    // 2. 验证密码
    // 3. 生成 JWT
    // 4. 返回用户信息和 token
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 获取当前用户
pub async fn me(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<UserResponse>>> {
    // TODO: 从 JWT 获取用户信息
    // 1. 验证 JWT
    // 2. 查询用户信息
    // 3. 返回用户信息
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 修改密码
pub async fn change_password(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现修改密码逻辑
    // 1. 验证旧密码
    // 2. 更新密码
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}
