//! 统计路由

use axum::{
    extract::{Extension, Query},
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;

/// 用量统计
pub async fn usage(
    Extension(_state): Extension<Arc<AppState>>,
    Query(_params): Query<UsageQuery>,
) -> Result<Json<SuccessResponse<UsageStatsResponse>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 贡献统计
pub async fn contribution(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 排行榜
pub async fn leaderboard(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<LeaderboardResponse>>> {
    // TODO: 获取用户所在群组
    // 暂时返回空排行榜
    Ok(Json(SuccessResponse::new(LeaderboardResponse {
        leaderboard: vec![],
        your_rank: None,
    })))
}

/// 积分记录
pub async fn credits(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}
