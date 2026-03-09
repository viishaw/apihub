//! 数据库迁移

pub struct Migration;

impl Migration {
    pub fn up() -> Vec<&'static str> {
        vec![
            r#"
            CREATE TABLE groups (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(64) NOT NULL,
                owner_id UUID,
                invite_code VARCHAR(32) UNIQUE NOT NULL,
                settings JSONB DEFAULT '{}',
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            "#,
            r#"
            CREATE TABLE users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
                username VARCHAR(32) NOT NULL,
                email VARCHAR(128),
                password_hash VARCHAR(256) NOT NULL,
                credits FLOAT DEFAULT 0.0,
                is_admin BOOLEAN DEFAULT FALSE,
                is_active BOOLEAN DEFAULT TRUE,
                settings JSONB DEFAULT '{}',
                last_active_at TIMESTAMPTZ,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW(),
                UNIQUE (group_id, username),
                UNIQUE (group_id, email)
            );
            "#,
            r#"
            ALTER TABLE groups ADD CONSTRAINT fk_owner 
                FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE SET NULL;
            "#,
            r#"
            CREATE TABLE api_keys (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
                contributor_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                provider VARCHAR(32) NOT NULL,
                name VARCHAR(64),
                encrypted_key BYTEA NOT NULL,
                key_hash VARCHAR(64) NOT NULL,
                base_url VARCHAR(256),
                monthly_quota FLOAT,
                used_quota FLOAT DEFAULT 0.0,
                weight INT DEFAULT 1,
                is_active BOOLEAN DEFAULT TRUE,
                last_used_at TIMESTAMPTZ,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW(),
                UNIQUE (group_id, key_hash)
            );
            "#,
            r#"
            CREATE TABLE usage_logs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                key_id UUID NOT NULL REFERENCES api_keys(id) ON DELETE CASCADE,
                provider VARCHAR(32) NOT NULL,
                model VARCHAR(64) NOT NULL,
                input_tokens INT,
                output_tokens INT,
                total_tokens INT,
                cost_usd FLOAT,
                credits_earned FLOAT,
                latency_ms INT,
                status_code INT,
                error_message TEXT,
                created_at TIMESTAMPTZ DEFAULT NOW()
            );
            "#,
            r#"
            CREATE TABLE credit_transactions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                amount FLOAT NOT NULL,
                transaction_type VARCHAR(32) NOT NULL,
                reason VARCHAR(256),
                related_log_id UUID REFERENCES usage_logs(id) ON DELETE SET NULL,
                created_at TIMESTAMPTZ DEFAULT NOW()
            );
            "#,
            // 索引
            r#"CREATE INDEX idx_users_group ON users(group_id);"#,
            r#"CREATE INDEX idx_users_credits ON users(group_id, credits DESC);"#,
            r#"CREATE INDEX idx_keys_group ON api_keys(group_id);"#,
            r#"CREATE INDEX idx_keys_contributor ON api_keys(contributor_id);"#,
            r#"CREATE INDEX idx_keys_provider ON api_keys(group_id, provider);"#,
            r#"CREATE INDEX idx_keys_active ON api_keys(group_id, is_active);"#,
            r#"CREATE INDEX idx_logs_group_time ON usage_logs(group_id, created_at DESC);"#,
            r#"CREATE INDEX idx_logs_user ON usage_logs(user_id);"#,
            r#"CREATE INDEX idx_logs_model ON usage_logs(group_id, model);"#,
            r#"CREATE INDEX idx_credit_user ON credit_transactions(user_id);"#,
            r#"CREATE INDEX idx_credit_time ON credit_transactions(user_id, created_at DESC);"#,
        ]
    }
    
    pub fn down() -> Vec<&'static str> {
        vec![
            "DROP TABLE IF EXISTS credit_transactions;",
            "DROP TABLE IF EXISTS usage_logs;",
            "DROP TABLE IF EXISTS api_keys;",
            "DROP TABLE IF EXISTS users;",
            "DROP TABLE IF EXISTS groups;",
        ]
    }
}
