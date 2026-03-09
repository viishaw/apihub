//! 认证路由

use axum::{
    extract::Extension,
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::Result;
use crate::AppState;

/// 注册
pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<SuccessResponse<AuthResponse>>> {
    // TODO: 实现注册逻辑
    todo!()
}

/// 登录
pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<SuccessResponse<AuthResponse>>> {
    // TODO: 实现登录逻辑
    todo!()
}

/// 获取当前用户
pub async fn me(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<UserResponse>>> {
    // TODO: 实现获取用户逻辑
    todo!()
}

/// 修改密码
pub async fn change_password(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现修改密码逻辑
    todo!()
}
