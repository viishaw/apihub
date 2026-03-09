# ApiHub 数据库设计

> 本文档详细描述 ApiHub 的数据库模型、关系和迁移策略。

---

## 1. 数据库选型

| 数据库 | 用途 | 理由 |
|--------|------|------|
| **PostgreSQL** | 主数据库 | ACID、JSONB、成熟稳定 |
| **Redis** | 缓存 + 排队 | 高性能、数据结构丰富 |

---

## 2. ER 图

```
┌─────────────┐       ┌─────────────┐       ┌─────────────┐
│   groups    │       │    users    │       │   api_keys  │
├─────────────┤       ├─────────────┤       ├─────────────┤
│ id (PK)     │←──────│ group_id(FK)│       │ id (PK)     │
│ name        │       │ id (PK)     │──────→│ group_id(FK)│
│ owner_id(FK)│       │ username    │       │ contributor │
│ invite_code │       │ email       │       │ provider    │
│ created_at  │       │ credits     │       │ encrypted   │
└─────────────┘       │ is_admin    │       │ quota       │
                      └─────────────┘       └─────────────┘
                             │                     │
                             │                     │
                      ┌──────┴─────────────────────┘
                      │
              ┌───────┴────────┐
              │  usage_logs    │
              ├────────────────┤
              │ id (PK)        │
              │ user_id (FK)   │
              │ key_id (FK)    │
              │ model          │
              │ tokens         │
              │ cost           │
              │ created_at     │
              └────────────────┘
                      │
              ┌───────┴────────┐
              │credit_transact │
              ├────────────────┤
              │ id (PK)        │
              │ user_id (FK)   │
              │ amount         │
              │ type           │
              │ created_at     │
              └────────────────┘
```

---

## 3. 表结构

### 3.1 groups（群组表）

```sql
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL,
    owner_id UUID NOT NULL,
    invite_code VARCHAR(32) UNIQUE NOT NULL,
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT fk_owner FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX idx_groups_invite_code ON groups(invite_code);
CREATE INDEX idx_groups_owner ON groups(owner_id);

-- 注释
COMMENT ON TABLE groups IS '群组表';
COMMENT ON COLUMN groups.invite_code IS '邀请码，用于加入群组';
COMMENT ON COLUMN groups.settings IS '群组设置（JSON格式）';
```

**settings 字段结构**：

```json
{
  "max_members": 50,
  "allow_key_contribution": true,
  "default_quota_per_user": 100.0,
  "credits_rate": {
    "openai": 10.0,
    "anthropic": 12.0,
    "gemini": 8.0
  }
}
```

### 3.2 users（用户表）

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL,
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
    
    CONSTRAINT fk_group FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    CONSTRAINT uq_group_username UNIQUE (group_id, username),
    CONSTRAINT uq_group_email UNIQUE (group_id, email)
);

-- 索引
CREATE INDEX idx_users_group ON users(group_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_credits ON users(group_id, credits DESC);

-- 注释
COMMENT ON TABLE users IS '用户表';
COMMENT ON COLUMN users.credits IS '积分余额';
COMMENT ON COLUMN users.is_admin IS '是否为群组管理员（群主）';
```

**settings 字段结构**:

```json
{
  "theme": "light",
  "language": "zh-CN",
  "notifications": {
    "email": true,
    "web": true
  }
}
```

### 3.3 api_keys（API Key 表）

```sql
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL,
    contributor_id UUID NOT NULL,
    provider VARCHAR(32) NOT NULL,
    name VARCHAR(64),
    encrypted_key TEXT NOT NULL,
    key_hash VARCHAR(64) NOT NULL,
    base_url VARCHAR(256),
    monthly_quota FLOAT,
    used_quota FLOAT DEFAULT 0.0,
    weight INT DEFAULT 1,
    is_active BOOLEAN DEFAULT TRUE,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT fk_key_group FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    CONSTRAINT fk_key_contributor FOREIGN KEY (contributor_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT uq_group_key_hash UNIQUE (group_id, key_hash)
);

-- 索引
CREATE INDEX idx_keys_group ON api_keys(group_id);
CREATE INDEX idx_keys_contributor ON api_keys(contributor_id);
CREATE INDEX idx_keys_provider ON api_keys(group_id, provider);
CREATE INDEX idx_keys_active ON api_keys(group_id, is_active);

-- 注释
COMMENT ON TABLE api_keys IS 'API Key 表';
COMMENT ON COLUMN api_keys.encrypted_key IS 'AES-256 加密的 API Key';
COMMENT ON COLUMN api_keys.key_hash IS '用于去重的 SHA256 哈希';
COMMENT ON COLUMN api_keys.monthly_quota IS '每月配额（美元）';
COMMENT ON COLUMN api_keys.weight IS '调度权重';
```

### 3.4 usage_logs（使用日志表）

```sql
CREATE TABLE usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL,
    user_id UUID NOT NULL,
    key_id UUID NOT NULL,
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
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT fk_log_group FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    CONSTRAINT fk_log_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_log_key FOREIGN KEY (key_id) REFERENCES api_keys(id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX idx_logs_group_time ON usage_logs(group_id, created_at DESC);
CREATE INDEX idx_logs_user ON usage_logs(user_id);
CREATE INDEX idx_logs_key ON usage_logs(key_id);
CREATE INDEX idx_logs_model ON usage_logs(group_id, model);

-- 注释
COMMENT ON TABLE usage_logs IS '使用日志表';
COMMENT ON COLUMN usage_logs.credits_earned IS '贡献者获得的积分';
```

### 3.5 credit_transactions（积分交易表）

```sql
CREATE TABLE credit_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL,
    user_id UUID NOT NULL,
    amount FLOAT NOT NULL,
    type VARCHAR(32) NOT NULL,
    reason VARCHAR(256),
    related_log_id UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT fk_credit_group FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    CONSTRAINT fk_credit_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_credit_log FOREIGN KEY (related_log_id) REFERENCES usage_logs(id) ON DELETE SET NULL
);

-- 索引
CREATE INDEX idx_credit_user ON credit_transactions(user_id);
CREATE INDEX idx_credit_type ON credit_transactions(user_id, type);
CREATE INDEX idx_credit_time ON credit_transactions(user_id, created_at DESC);

-- 注释
COMMENT ON TABLE credit_transactions IS '积分交易表';
COMMENT ON COLUMN credit_transactions.amount IS '正数=获得，负数=消耗';
COMMENT ON COLUMN credit_transactions.type IS 'contribution, usage, admin_adjust, bonus';
```

---

## 4. 视图（可选)

### 4.1 用户统计视图

```sql
CREATE VIEW user_stats AS
SELECT 
    u.id,
    u.group_id,
    u.username,
    u.credits,
    COUNT(DISTINCT ak.id) AS key_count,
    COUNT(DISTINCT ul.id) AS request_count,
    COALESCE(SUM(ul.total_tokens), 0) AS total_tokens,
    COALESCE(SUM(ul.cost_usd), 0) AS total_cost
FROM users u
LEFT JOIN api_keys ak ON ak.contributor_id = u.id AND ak.is_active = TRUE
LEFT JOIN usage_logs ul ON ul.user_id = u.id
GROUP BY u.id, u.group_id, u.username, u.credits;
```

### 4.2 群组统计视图

```sql
CREATE VIEW group_stats AS
SELECT 
    g.id,
    g.name,
    COUNT(DISTINCT u.id) AS member_count,
    COUNT(DISTINCT ak.id) AS key_count,
    COUNT(DISTINCT CASE WHEN ak.is_active THEN ak.id END) AS active_key_count,
    COALESCE(SUM(ak.monthly_quota), 0) AS total_quota,
    COALESCE(SUM(ak.used_quota), 0) AS used_quota,
    COALESCE(SUM(u.credits), 0) AS total_credits
FROM groups g
LEFT JOIN users u ON u.group_id = g.id
LEFT JOIN api_keys ak ON ak.group_id = g.id
GROUP BY g.id, g.name;
```

---

## 5. 索引优化建议

### 5.1 查询优化
```sql
-- 排行榜查询（按群组）
CREATE INDEX idx_leaderboard ON users(group_id, credits DESC);

-- 按时间范围查询使用记录
CREATE INDEX idx_logs_time_range ON usage_logs(group_id, created_at DESC);

-- 按模型统计
CREATE INDEX idx_logs_model_stats ON usage_logs(group_id, model, created_at);
```

### 5.2 分区（可选，大数据量时）
```sql
-- 按月分区 usage_logs
CREATE TABLE usage_logs (
    -- ...
) PARTITION BY RANGE (created_at);
```

---

## 6. Redis 数据结构

### 6.1 排队队列
```
# Sorted Set（按优先级排序）
apihub:queue:{group_id}
  score: priority (float)
  member: request_id (UUID)

# 请求详情（Hash）
apihub:request:{request_id}
  user_id: UUID
  model: string
  priority: float
  created_at: timestamp
  status: string
```

### 6.2 用户缓存
```
# 用户信息（Hash）
apihub:user:{user_id}
  id, username, credits, is_admin, group_id
  TTL: 300s
```

### 6.3 Key 池缓存
```
# 活跃 Key 列表（Set）
apihub:keys:active:{group_id}
  member: key_id (UUID)

# Key 详情（Hash）
apihub:key:{key_id}
  provider, weight, last_used_at, is_active
  TTL: 60s
```

### 6.4 实时统计
```
# 今日统计（Hash）
apihub:stats:today:{group_id}
  requests: int
  tokens: int
  cost: float

# 排行榜（Sorted Set）
apihub:leaderboard:{group_id}
  score: credits
  member: user_id
```

---

## 7. 迁移脚本
使用 SeaORM 迁移：

```rust
// crates/db/migration/src/m20240101_000001_initial.rs
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Groups::Table)
                    .col(
                        ColumnDef::new(Groups::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default_value(Expr::custom(FuncCall::new(
                                "gen_random_uuid",
                                [],
                            ))),
                    )
                    .col(ColumnDef::new(Groups::Name).string_len(64).not_null())
                    .col(ColumnDef::new(Groups::InviteCode).string_len(32).unique_key().not_null())
                    .col(ColumnDef::new(Groups::Settings).json().default_value("{}"))
                    .col(
                        ColumnDef::new(Groups::CreatedAt)
                            .timestamp_with_time_zone()
                            .default_value(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await
    }
}
```

---

## 8. 数据库维护
### 8.1 备份策略
```bash
# 每日备份
pg_dump apihub > /backup/apihub_$(date +%Y%m%d).sql

# 保留 7 天
find /backup -name "apihub_*.sql" -mtime +7 -delete
```

### 8.2 清理策略
```sql
-- 清理 90 天前的日志
DELETE FROM usage_logs WHERE created_at < NOW() - INTERVAL '90 days';

-- 归档到冷存储（可选）
INSERT INTO usage_logs_archive SELECT * FROM usage_logs WHERE created_at < NOW() - INTERVAL '30 days';
DELETE FROM usage_logs WHERE created_at < NOW() - INTERVAL '30 days';
```

---

*最后更新: 2024-01-20*
