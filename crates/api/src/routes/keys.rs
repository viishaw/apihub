//! Key 管理路由

use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::Result;
use crate::AppState;

/// 创建 Key
pub async fn create_key(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateKeyRequest>,
) -> Result<Json<SuccessResponse<KeyResponse>>> {
    // TODO: 实现创建 Key 逻辑
    todo!()
}

/// 获取群组内所有 Key
pub async fn list_keys(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现获取 Key 列表逻辑
    todo!()
}

/// 获取我贡献的 Key
pub async fn my_keys(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<Vec<KeyResponse>>>> {
    // TODO: 实现获取我的 Key 逻辑
    todo!()
}

/// 更新 Key
pub async fn update_key(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<KeyResponse>>> {
    // TODO: 实现更新 Key 逻辑
    todo!()
}

/// 删除 Key
pub async fn delete_key(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现删除 Key 逻辑
    todo!()
}

/// 暂停/恢复 Key
pub async fn toggle_key(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现切换 Key 状态逻辑
    todo!()
}
