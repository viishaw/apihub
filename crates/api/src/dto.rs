//! DTO (Data Transfer Objects) 模块

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

// ============ 认证相关 ============

/// 注册请求
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1, max = 64))]
    pub group_name: String,
}

/// 加入群组请求
#[derive(Debug, Deserialize, Validate)]
pub struct JoinRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 6, max = 32))]
    pub invite_code: String,
}

/// 登录请求
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub group: Option<GroupResponse>,
    pub token: String,
}

/// 用户响应
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub credits: f64,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

/// 群组响应
#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub id: Uuid,
    pub name: String,
    pub invite_code: Option<String>,
    pub owner_id: Option<Uuid>,
}

// ============ Key 管理相关 ============

/// 创建 Key 请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateKeyRequest {
    #[validate(length(min = 1, max = 32))]
    pub provider: String,
    #[validate(length(min = 1))]
    pub api_key: String,
    #[validate(length(max = 64))]
    pub name: Option<String>,
    #[validate(url)]
    pub base_url: Option<String>,
    pub monthly_quota: Option<f64>,
}

/// Key 响应
#[derive(Debug, Serialize)]
pub struct KeyResponse {
    pub id: Uuid,
    pub provider: String,
    pub name: Option<String>,
    pub contributor: UserBrief,
    pub monthly_quota: Option<f64>,
    pub used_quota: f64,
    pub weight: i32,
    pub is_active: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// 用户简要信息
#[derive(Debug, Serialize)]
pub struct UserBrief {
    pub id: Uuid,
    pub username: String,
}

// ============ 聊天相关 ============

/// 聊天请求（OpenAI 格式）
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default)]
    pub temperature: f32,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub stream: bool,
}

/// 消息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// 聊天响应（OpenAI 格式）
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// 选择
#[derive(Debug, Serialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

/// Token 使用量
#[derive(Debug, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ============ 统计相关 ============

/// 用量统计响应
#[derive(Debug, Serialize)]
pub struct UsageStatsResponse {
    pub period: String,
    pub summary: UsageSummary,
    pub by_model: Vec<ModelUsage>,
    pub by_user: Vec<UserUsage>,
}

#[derive(Debug, Serialize)]
pub struct UsageSummary {
    pub total_requests: u64,
    pub total_tokens: u64,
    pub total_cost_usd: f64,
    pub unique_users: u32,
    pub avg_latency_ms: u32,
}

#[derive(Debug, Serialize)]
pub struct ModelUsage {
    pub model: String,
    pub requests: u64,
    pub tokens: u64,
    pub cost_usd: f64,
}

#[derive(Debug, Serialize)]
pub struct UserUsage {
    pub user_id: Uuid,
    pub username: String,
    pub requests: u64,
    pub tokens: u64,
    pub cost_usd: f64,
}

/// 排行榜响应
#[derive(Debug, Serialize)]
pub struct LeaderboardResponse {
    pub leaderboard: Vec<LeaderboardEntry>,
    pub your_rank: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub user_id: Uuid,
    pub username: String,
    pub credits: f64,
    pub key_count: u32,
    pub avatar: Option<String>,
}

// ============ 通用响应 ============

/// 成功响应
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

/// 分页信息
#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}
