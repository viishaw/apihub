//! 认证中间件

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

/// JWT 认证中间件
pub async fn auth_layer(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: 实现 JWT 验证
    Ok(next.run(request).await)
}
