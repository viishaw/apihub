//! 聊天路由

use axum::{
    extract::Extension,
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;

/// OpenAI Chat Completions
pub async fn completions(
    Extension(_state): Extension<Arc<AppState>>,
    Json(_req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>> {
    // TODO: 实现聊天逻辑
    // 1. 验证用户登录
    // 2. 检查用户积分
    // 3. 加入排队
    // 4. 选择可用的 Key
    // 5. 调用 AI API
    // 6. 记录使用量
    // 7. 扣除积分
    // 8. 给 Key 贡献者增加积分
    // 9. 返回响应
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// Anthropic Messages
pub async fn messages(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现 Anthropic 消息逻辑
    // 类似 completions，但使用 Anthropic API
    
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 列出可用模型
pub async fn list_models(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现列出模型逻辑
    // 1. 查询群组内可用的 provider
    // 2. 返回支持的模型列表
    
    // 暂时返回静态列表
    Ok(Json(serde_json::json!({
        "object": "list",
        "data": [
            {"id": "gpt-4-turbo", "object": "model", "owned_by": "openai"},
            {"id": "gpt-3.5-turbo", "object": "model", "owned_by": "openai"},
            {"id": "claude-3-opus-20240229", "object": "model", "owned_by": "anthropic"},
            {"id": "claude-3-sonnet-20240229", "object": "model", "owned_by": "anthropic"},
        ]
    })))
}

/// 排队状态
pub async fn queue_status(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现排队状态逻辑
    // 1. 查询 Redis 队列
    // 2. 返回队列长度和用户位置
    
    Ok(Json(serde_json::json!({
        "queue_length": 0,
        "your_position": null,
        "estimated_wait_seconds": 0
    })))
}
