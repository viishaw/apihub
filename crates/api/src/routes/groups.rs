//! 群组路由

use axum::{
    extract::{Extension, Path},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;

/// 获取群组信息
pub async fn get_group(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<GroupResponse>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 获取群组成员
pub async fn get_members(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Vec<UserResponse>>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 更新群组信息
pub async fn update_group(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
    Json(_req): Json<UpdateGroupRequest>,
) -> Result<Json<SuccessResponse<GroupResponse>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 重新生成邀请码
pub async fn regenerate_invite(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 移除成员
pub async fn remove_member(
    Extension(_state): Extension<Arc<AppState>>,
    Path((_group_id, _user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 退出群组
pub async fn leave_group(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 再生成邀请码（别名）
pub async fn regenerate_invite_code(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    regenerate_invite(Extension(state), Path(id)).await
}
