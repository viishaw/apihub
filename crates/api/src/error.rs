//! 错误处理模块

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

/// 应用错误类型
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Group not found")]
    GroupNotFound,
    
    #[error("Key not found")]
    KeyNotFound,
    
    #[error("Duplicate key")]
    DuplicateKey,
    
    #[error("Invalid invite code")]
    InvalidInviteCode,
    
    #[error("Quota exceeded")]
    QuotaExceeded,
    
    #[error("No available key")]
    NoAvailableKey,
    
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Error::InvalidCredentials => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS", self.to_string()),
            Error::TokenExpired => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED", self.to_string()),
            Error::PermissionDenied => (StatusCode::FORBIDDEN, "PERMISSION_DENIED", self.to_string()),
            Error::UserNotFound => (StatusCode::NOT_FOUND, "USER_NOT_FOUND", self.to_string()),
            Error::GroupNotFound => (StatusCode::NOT_FOUND, "GROUP_NOT_FOUND", self.to_string()),
            Error::KeyNotFound => (StatusCode::NOT_FOUND, "KEY_NOT_FOUND", self.to_string()),
            Error::DuplicateKey => (StatusCode::CONFLICT, "DUPLICATE_KEY", self.to_string()),
            Error::InvalidInviteCode => (StatusCode::BAD_REQUEST, "INVALID_INVITE_CODE", self.to_string()),
            Error::QuotaExceeded => (StatusCode::TOO_MANY_REQUESTS, "QUOTA_EXCEEDED", self.to_string()),
            Error::NoAvailableKey => (StatusCode::SERVICE_UNAVAILABLE, "NO_AVAILABLE_KEY", self.to_string()),
            Error::ProviderError(_) => (StatusCode::BAD_GATEWAY, "PROVIDER_ERROR", self.to_string()),
            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", "Internal database error"),
            Error::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "REDIS_ERROR", "Internal cache error"),
            Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Internal server error"),
        };

        let body = json!({
            "success": false,
            "error": {
                "code": code,
                "message": message,
            }
        });

        (status, Json(body)).into_response()
    }
}
