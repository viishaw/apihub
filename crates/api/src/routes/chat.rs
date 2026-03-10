//! 聊天路由

use axum::{extract::Extension, Json};
use std::sync::Arc;
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;
use sqlx::Row;

use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;

/// OpenAI Chat Completions
pub async fn completions(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>> {
    // 1. 获取用户 ID（TODO: 从 JWT 获取）
    let user_id = Uuid::nil();
    
    // 2. 检查用户积分
    let user_row = sqlx::query("SELECT credits, group_id FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(Error::UserNotFound)?;
    
    let credits = user_row.try_get::<f64, _>("credits")?;
    let group_id = user_row.try_get::<Option<Uuid>, _>("group_id")?.ok_or(Error::PermissionDenied)?;
    
    if credits < 10.0 {
        return Err(Error::QuotaExceeded);
    }
    
    // 3. 选择可用的 Key
    let key_row = sqlx::query(
        "SELECT id, encrypted_key, provider FROM api_keys 
         WHERE group_id = $1 AND is_active = true 
         ORDER BY RANDOM() LIMIT 1"
    )
    .bind(&group_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(Error::NoAvailableKey)?;
    
    let key_id = key_row.try_get::<Uuid, _>("id")?;
    let encrypted_key = key_row.try_get::<String, _>("encrypted_key")?;
    let provider = key_row.try_get::<String, _>("provider")?;
    
    // 4. 调用 AI API
    let client = Client::new();
    let api_key = decrypt_key(&encrypted_key, &state.config.master_key)?;
    
    let response = match provider.as_str() {
        "openai" => {
            let res = client
                .post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&json!({
                    "model": req.model,
                    "messages": req.messages,
                    "temperature": req.temperature,
                    "max_tokens": req.max_tokens,
                }))
                .send()
                .await
                .map_err(|e| Error::ProviderError(e.to_string()))?;
            
            res.json::<serde_json::Value>()
                .await
                .map_err(|e| Error::ProviderError(e.to_string()))?
        }
        "anthropic" => {
            let res = client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &api_key)
                .header("anthropic-version", "2023-06-01")
                .json(&json!({
                    "model": req.model,
                    "messages": req.messages,
                    "max_tokens": req.max_tokens.unwrap_or(1024),
                }))
                .send()
                .await
                .map_err(|e| Error::ProviderError(e.to_string()))?;
            
            res.json::<serde_json::Value>()
                .await
                .map_err(|e| Error::ProviderError(e.to_string()))?
        }
        _ => return Err(Error::ProviderError(format!("Unknown provider: {}", provider))),
    };
    
    // 5. 解析响应
    let chat_response = parse_openai_response(&response)?;
    
    // 6. 记录使用量
    let cost = calculate_cost(&req.model, chat_response.usage.prompt_tokens, chat_response.usage.completion_tokens);
    
    sqlx::query(
        "INSERT INTO usage_logs (user_id, key_id, provider, model, prompt_tokens, completion_tokens, total_tokens, cost)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&user_id)
    .bind(&key_id)
    .bind(&provider)
    .bind(&req.model)
    .bind(chat_response.usage.prompt_tokens as i32)
    .bind(chat_response.usage.completion_tokens as i32)
    .bind(chat_response.usage.total_tokens as i32)
    .bind(cost)
    .execute(&state.db)
    .await?;
    
    // 7. 扣除用户积分
    sqlx::query("UPDATE users SET credits = credits - $1 WHERE id = $2")
        .bind(cost)
        .bind(&user_id)
        .execute(&state.db)
        .await?;
    
    // 8. 给 Key 贡献者增加积分（10%）
    sqlx::query(
        "UPDATE users SET credits = credits + $1 * 0.1 
         WHERE id = (SELECT user_id FROM api_keys WHERE id = $2)"
    )
    .bind(cost)
    .bind(&key_id)
    .execute(&state.db)
    .await?;
    
    Ok(Json(chat_response))
}

/// Anthropic Messages
pub async fn messages(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

/// 列出可用模型
pub async fn list_models(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
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
    Ok(Json(serde_json::json!({
        "queue_length": 0,
        "your_position": null,
        "estimated_wait_seconds": 0
    })))
}

// ============ 辅助函数 ============

fn decrypt_key(encrypted: &str, _master_key: &str) -> Result<String> {
    // TODO: 实现真正的解密
    // 目前直接返回（实际应该使用 AES-256-GCM 解密）
    Ok(encrypted.to_string())
}

fn parse_openai_response(response: &serde_json::Value) -> Result<ChatResponse> {
    let id = response["id"].as_str().unwrap_or("chatcmpl-0").to_string();
    let model = response["model"].as_str().unwrap_or("").to_string();
    let created = response["created"].as_i64().unwrap_or(0) as i64;
    
    let choices = response["choices"].as_array()
        .ok_or_else(|| Error::ProviderError("Invalid response format".to_string()))?
        .iter()
        .map(|c| Choice {
            index: c["index"].as_u64().unwrap_or(0) as u32,
            message: Message {
                role: c["message"]["role"].as_str().unwrap_or("assistant").to_string(),
                content: c["message"]["content"].as_str().unwrap_or("").to_string(),
            },
            finish_reason: c["finish_reason"].as_str().unwrap_or("stop").to_string(),
        })
        .collect();
    
    let usage = Usage {
        prompt_tokens: response["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
        completion_tokens: response["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
        total_tokens: response["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
    };
    
    Ok(ChatResponse {
        id,
        object: "chat.completion".to_string(),
        created,
        model,
        choices,
        usage,
    })
}

fn calculate_cost(model: &str, prompt_tokens: u32, completion_tokens: u32) -> f64 {
    // 价格表（美元/1K tokens）
    let (input_price, output_price) = match model {
        "gpt-4-turbo" => (0.01, 0.03),
        "gpt-3.5-turbo" => (0.0005, 0.0015),
        "claude-3-opus-20240229" => (0.015, 0.075),
        "claude-3-sonnet-20240229" => (0.003, 0.015),
        _ => (0.001, 0.002), // 默认价格
    };
    
    let input_cost = (prompt_tokens as f64 / 1000.0) * input_price;
    let output_cost = (completion_tokens as f64 / 1000.0) * output_price;
    
    // 转换为积分（$1 = 10 credits）
    (input_cost + output_cost) * 10.0
}
