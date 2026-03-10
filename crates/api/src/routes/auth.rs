//! 认证路由

use axum::{extract::Extension, Json};
use std::sync::Arc;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Duration, Utc};
use uuid::Uuid;
use sqlx::Row;

use crate::dto::*;
use crate::error::{Error, Result};
use crate::AppState;
use crate::config::Config;

/// 简单密码哈希（生产环境应使用 argon2）
fn hash_password(password: &str, secret: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(secret.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn verify_password(password: &str, secret: &str, hash: &str) -> bool {
    hash_password(password, secret) == hash
}

/// 注册
pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<SuccessResponse<AuthResponse>>> {
    if req.username.len() < 3 || req.username.len() > 32 {
        return Err(Error::InvalidCredentials);
    }
    
    let existing = sqlx::query("SELECT id FROM users WHERE username = $1 OR email = $2")
        .bind(&req.username).bind(&req.email)
        .fetch_optional(&state.db).await?;
    
    if existing.is_some() { return Err(Error::DuplicateKey); }
    
    let password_hash = hash_password(&req.password, &state.config.jwt_secret);
    
    let group_id = if let Some(ref code) = req.invite_code {
        let row = sqlx::query("SELECT id FROM groups WHERE invite_code = $1")
            .bind(code).fetch_optional(&state.db).await?
            .ok_or(Error::InvalidInviteCode)?;
        row.try_get::<Uuid, _>("id")?
    } else if let Some(ref name) = req.group_name {
        let code = generate_invite_code();
        let row = sqlx::query("INSERT INTO groups (name, invite_code) VALUES ($1, $2) RETURNING id")
            .bind(name).bind(&code).fetch_one(&state.db).await?;
        row.try_get::<Uuid, _>("id")?
    } else { return Err(Error::InvalidInviteCode); };
    
    let is_admin = req.invite_code.is_none();
    let row = sqlx::query(
        "INSERT INTO users (username, email, password_hash, group_id, is_admin) 
         VALUES ($1, $2, $3, $4, $5) RETURNING id, username, email, credits, is_admin, created_at"
    ).bind(&req.username).bind(&req.email).bind(&password_hash).bind(&group_id).bind(is_admin)
        .fetch_one(&state.db).await?;
    
    let user = UserResponse {
        id: row.try_get::<Uuid, _>("id")?,
        username: row.try_get::<String, _>("username")?,
        email: row.try_get::<String, _>("email")?,
        credits: row.try_get::<f64, _>("credits")?,
        is_admin: row.try_get::<bool, _>("is_admin")?,
        created_at: row.try_get::<chrono::DateTime<Utc>, _>("created_at")?,
    };
    
    if is_admin {
        sqlx::query("UPDATE groups SET owner_id = $1 WHERE id = $2")
            .bind(&user.id).bind(&group_id).execute(&state.db).await?;
    }
    
    let token = generate_token(user.id, &state.config)?;
    let group = get_group(&state.db, group_id).await?;
    
    Ok(Json(SuccessResponse::new(AuthResponse { user, group, token })))
}

/// 登录
pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<SuccessResponse<AuthResponse>>> {
    let row = sqlx::query(
        "SELECT id, username, email, password_hash, credits, is_active, is_admin, group_id, created_at 
         FROM users WHERE username = $1"
    ).bind(&req.username).fetch_optional(&state.db).await?
        .ok_or(Error::InvalidCredentials)?;
    
    let user_id = row.try_get::<Uuid, _>("id")?;
    let password_hash = row.try_get::<String, _>("password_hash")?;
    let group_id = row.try_get::<Option<Uuid>, _>("group_id")?;
    
    if !verify_password(&req.password, &state.config.jwt_secret, &password_hash) {
        return Err(Error::InvalidCredentials);
    }
    if !row.try_get::<bool, _>("is_active")? { return Err(Error::PermissionDenied); }
    
    let user = UserResponse {
        id: user_id,
        username: row.try_get::<String, _>("username")?,
        email: row.try_get::<String, _>("email")?,
        credits: row.try_get::<f64, _>("credits")?,
        is_admin: row.try_get::<bool, _>("is_admin")?,
        created_at: row.try_get::<chrono::DateTime<Utc>, _>("created_at")?,
    };
    
    let token = generate_token(user.id, &state.config)?;
    let group = match group_id { Some(gid) => get_group(&state.db, gid).await?, None => None };
    
    Ok(Json(SuccessResponse::new(AuthResponse { user, group, token })))
}

pub async fn me(Extension(_state): Extension<Arc<AppState>>) -> Result<Json<SuccessResponse<UserResponse>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

pub async fn change_password(Extension(_state): Extension<Arc<AppState>>) -> Result<Json<SuccessResponse<serde_json::Value>>> {
    Err(Error::Internal(anyhow::anyhow!("Not implemented")))
}

async fn get_group(db: &sqlx::PgPool, group_id: Uuid) -> Result<Option<GroupResponse>> {
    let row = sqlx::query("SELECT id, name, invite_code, owner_id FROM groups WHERE id = $1")
        .bind(&group_id).fetch_optional(db).await?;
    match row {
        Some(r) => Ok(Some(GroupResponse {
            id: r.try_get::<Uuid, _>("id")?,
            name: r.try_get::<String, _>("name")?,
            invite_code: r.try_get::<Option<String>, _>("invite_code")?,
            owner_id: r.try_get::<Option<Uuid>, _>("owner_id")?,
        })),
        None => Ok(None),
    }
}

fn generate_invite_code() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    (0..8).map(|_| CHARSET[rand::thread_rng().gen_range(0..CHARSET.len())] as char).collect()
}

fn generate_token(user_id: Uuid, config: &Config) -> Result<String> {
    #[derive(serde::Serialize)]
    struct Claims { sub: String, exp: usize }
    
    let exp = Utc::now().checked_add_signed(Duration::hours(24 * 7))
        .ok_or_else(|| Error::Internal(anyhow::anyhow!("Time error")))?.timestamp() as usize;
    
    encode(&Header::default(), &Claims { sub: user_id.to_string(), exp },
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()))
        .map_err(|_| Error::Internal(anyhow::anyhow!("Token generation failed")))
}
