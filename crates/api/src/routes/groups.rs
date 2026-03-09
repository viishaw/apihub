//! 群组路由

use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::Result;
use crate::AppState;

/// 获取群组信息
pub async fn get_group(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<GroupResponse>>> {
    // TODO: 实现获取群组逻辑
    todo!()
}

/// 获取群组成员
pub async fn get_members(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现获取成员逻辑
    todo!()
}

/// 更新群组信息
pub async fn update_group(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<GroupResponse>>> {
    // TODO: 实现更新群组逻辑
    todo!()
}

/// 重新生成邀请码
pub async fn regenerate_invite(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现重新生成邀请码逻辑
    todo!()
}

/// 移除成员
pub async fn remove_member(
    Extension(state): Extension<Arc<AppState>>,
    Path((group_id, user_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现移除成员逻辑
    todo!()
}

/// 退出群组
pub async fn leave_group(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现退出群组逻辑
    todo!()
}
