//! 数据库迁移

use sqlx::PgPool;
use anyhow::Result;

pub async fn run(pool: &PgPool) -> Result<()> {
    // 创建用户表
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(32) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            credits DOUBLE PRECISION DEFAULT 0,
            is_admin BOOLEAN DEFAULT FALSE,
            is_active BOOLEAN DEFAULT TRUE,
            group_id UUID,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // 创建群组表
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS groups (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(64) NOT NULL,
            invite_code VARCHAR(32) UNIQUE,
            owner_id UUID REFERENCES users(id),
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // 添加外键
    sqlx::query(r#"
        ALTER TABLE users 
        ADD CONSTRAINT fk_group 
        FOREIGN KEY (group_id) REFERENCES groups(id)
    "#).execute(pool).await?;

    // 创建 API Keys 表
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS api_keys (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            provider VARCHAR(32) NOT NULL,
            encrypted_key TEXT NOT NULL,
            key_hash VARCHAR(64) NOT NULL,
            name VARCHAR(64),
            base_url TEXT,
            monthly_quota DOUBLE PRECISION,
            used_quota DOUBLE PRECISION DEFAULT 0,
            weight INTEGER DEFAULT 1,
            is_active BOOLEAN DEFAULT TRUE,
            user_id UUID NOT NULL REFERENCES users(id),
            group_id UUID NOT NULL REFERENCES groups(id),
            last_used_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // 创建使用日志表
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS usage_logs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id),
            key_id UUID REFERENCES api_keys(id),
            provider VARCHAR(32) NOT NULL,
            model VARCHAR(64) NOT NULL,
            prompt_tokens INTEGER,
            completion_tokens INTEGER,
            total_tokens INTEGER,
            cost DOUBLE PRECISION,
            latency_ms INTEGER,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // 创建积分交易表
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS credit_transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id),
            amount DOUBLE PRECISION NOT NULL,
            balance_after DOUBLE PRECISION NOT NULL,
            reason VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
    "#).execute(pool).await?;

    // 创建索引
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_group ON users(group_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_keys_group ON api_keys(group_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_keys_user ON api_keys(user_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_usage_user ON usage_logs(user_id)").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_usage_created ON usage_logs(created_at)").execute(pool).await?;

    Ok(())
}
