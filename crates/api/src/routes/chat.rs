//! 聊天路由

use axum::{
    extract::Extension,
    Json,
};
use std::sync::Arc;
use crate::dto::*;
use crate::error::Result;
use crate::AppState;

/// OpenAI Chat Completions
pub async fn completions(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>> {
    // TODO: 实现聊天逻辑
    todo!()
}

/// Anthropic Messages
pub async fn messages(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现 Anthropic 消息逻辑
    todo!()
}

/// 列出可用模型
pub async fn list_models(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现列出模型逻辑
    todo!()
}

/// 排队状态
pub async fn queue_status(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现排队状态逻辑
    todo!()
}
