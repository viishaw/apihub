//! 统计路由

use axum::{
    extract::{Extension, Query},
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::Result;
use crate::AppState;

/// 用量统计
pub async fn usage(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<UsageStatsResponse>>> {
    // TODO: 实现用量统计逻辑
    todo!()
}

/// 贡献统计
pub async fn contribution(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现贡献统计逻辑
    todo!()
}

/// 排行榜
pub async fn leaderboard(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<LeaderboardResponse>>> {
    // TODO: 实现排行榜逻辑
    todo!()
}

/// 积分记录
pub async fn credits(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    // TODO: 实现积分记录逻辑
    todo!()
}
