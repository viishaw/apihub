//! Key 管理路由

use axum::{
    extract::{Extension, Path},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;

/// 创建 Key
pub async fn create_key(
    Extension(_state): Extension<Arc<AppState>>,
    Json(_req): Json<CreateKeyRequest>,
) -> Result<Json<SuccessResponse<KeyResponse>>> {
    // TODO: 实现创建 Key 逻辑
    // 1. 验证用户登录
    // 2. 加密 API Key
    // 3. 计算 Key 哈希（去重）
    // 4. 存储到数据库
    // 5. 给用户增加积分
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 获取群组内所有 Key
pub async fn list_keys(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<Vec<KeyResponse>>>> {
    // TODO: 实现获取 Key 列表逻辑
    // 1. 获取用户所在群组
    // 2. 查询群组内所有 Key
    // 3. 不返回实际的 Key，只返回元数据
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 获取我贡献的 Key
pub async fn my_keys(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<Vec<KeyResponse>>>> {
    // TODO: 实现获取我的 Key 逻辑
    // 1. 获取当前用户
    // 2. 查询用户贡献的所有 Key
    // 3. 返回 Key 列表
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 更新 Key
pub async fn update_key(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
    Json(_req): Json<UpdateKeyRequest>,
) -> Result<Json<SuccessResponse<KeyResponse>>> {
    // TODO: 实现更新 Key 逻辑
    // 1. 验证 Key 属于当前用户
    // 2. 更新 Key 信息
    // 3. 返回更新后的 Key
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 删除 Key
pub async fn delete_key(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现删除 Key 逻辑
    // 1. 验证 Key 属于当前用户
    // 2. 软删除 Key
    // 3. 扣除用户积分
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 暂停/恢复 Key
pub async fn toggle_key(
    Extension(_state): Extension<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现切换 Key 状态逻辑
    // 1. 验证 Key 属于当前用户
    // 2. 切换 is_active 状态
    // 3. 返回新状态
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}
