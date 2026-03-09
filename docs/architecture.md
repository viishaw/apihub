# ApiHub 架构设计

> 本文档详细描述 ApiHub 的系统架构、核心模块和技术选型。

---

## 1. 系统概览

### 1.1 设计理念

| 原则 | 说明 |
|------|------|
| **简单优先** | 用户 5 分钟内完成部署 |
| **私有优先** | 每个部署独立，数据不共享 |
| **公平优先** | 积分机制防止白嫖 |
| **透明优先** | 所有用量、贡献可见 |

### 1.2 核心流程

```
用户请求 → 认证 → 排队（如需要） → 调度 → 代理 → 记录 → 积分更新
   ↓                      ↓         ↓       ↓
 [JWT验证]           [积分排序]  [选Key]  [转发API]
```

---

## 2. 技术栈

### 2.1 后端

| 组件 | 技术 | 版本 | 理由 |
|------|------|------|------|
| **语言** | Rust | 1.75+ | 高性能、类型安全、无 GC |
| **Web 框架** | Axum | 0.7+ | Tower 生态、高性能 |
| **ORM** | SeaORM | 0.12+ | 异步、类型安全、迁移支持 |
| **数据库** | PostgreSQL | 15+ | 稳定、功能完整 |
| **缓存** | Redis | 7+ | 排队、缓存、实时统计 |
| **异步运行时** | Tokio | 1.x | 事实标准 |

### 2.2 前端

| 组件 | 技术 | 版本 | 理由 |
|------|------|------|------|
| **框架** | React | 18+ | 生态成熟 |
| **UI 库** | Ant Design | 5+ | 企业级、组件丰富 |
| **语言** | TypeScript | 5+ | 类型安全 |
| **构建** | Vite | 5+ | 快速开发体验 |
| **状态管理** | Zustand | 4+ | 轻量、简单 |
| **请求** | TanStack Query | 5+ | 缓存、自动刷新 |

### 2.3 基础设施

| 组件 | 技术 | 理由 |
|------|------|------|
| **反向代理** | Caddy | 自动 HTTPS、配置简单 |
| **容器** | Docker | 一键部署 |
| **监控** | Prometheus + Grafana | 可选，生产环境用 |

---

## 3. 项目结构

```
apihub/
├── Cargo.toml                  # Workspace 根配置
├── Cargo.lock
├── README.md
├── LICENSE
├── Dockerfile
├── docker-compose.yml
├── .env.example
│
├── crates/                     # Rust crates
│   ├── api/                    # HTTP API 层
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs         # 入口
│   │       ├── routes/         # 路由
│   │       │   ├── mod.rs
│   │       │   ├── auth.rs     # 认证
│   │       │   ├── groups.rs   # 群组管理
│   │       │   ├── keys.rs     # Key 管理
│   │       │   ├── chat.rs     # 聊天接口
│   │       │   └── stats.rs    # 统计
│   │       ├── middleware/     # 中间件
│   │       │   ├── mod.rs
│   │       │   ├── auth.rs     # JWT 验证
│   │       │   └── logging.rs  # 日志
│   │       ├── error.rs        # 错误处理
│   │       └── config.rs       # 配置
│   │
│   ├── core/                   # 核心业务逻辑
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── scheduler/      # 调度器
│   │       │   ├── mod.rs
│   │       │   ├── round_robin.rs
│   │       │   ├── weighted.rs
│   │       │   └── failover.rs
│   │       ├── credits/        # 积分系统
│   │       │   ├── mod.rs
│   │       │   ├── calculator.rs
│   │       │   └── transaction.rs
│   │       ├── queue/          # 排队系统
│   │       │   ├── mod.rs
│   │       │   ├── redis.rs
│   │       │   └── priority.rs
│   │       └── crypto/         # 加密
│   │           ├── mod.rs
│   │           └── aes.rs
│   │
│   ├── db/                     # 数据库层
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/         # SeaORM 模型
│   │       │   ├── mod.rs
│   │       │   ├── group.rs
│   │       │   ├── user.rs
│   │       │   ├── api_key.rs
│   │       │   ├── usage_log.rs
│   │       │   └── credit.rs
│   │       ├── migration/      # 数据库迁移
│   │       │   ├── mod.rs
│   │       │   ├── m20240101_000001_initial.rs
│   │       │   └── ...
│   │       └── repositories/   # 数据访问层
│   │           ├── mod.rs
│   │           ├── group.rs
│   │           ├── user.rs
│   │           └── key.rs
│   │
│   └── providers/              # API 提供商适配器
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── traits.rs       # Provider trait
│           ├── openai.rs       # OpenAI
│           ├── anthropic.rs    # Claude
│           ├── gemini.rs       # Gemini
│           └── custom.rs       # 自定义 OpenAI 兼容
│
├── frontend/                   # React 前端
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── api/               # API 调用
│   │   │   ├── client.ts
│   │   │   ├── auth.ts
│   │   │   ├── groups.ts
│   │   │   ├── keys.ts
│   │   │   └── stats.ts
│   │   ├── components/        # 通用组件
│   │   │   ├── Layout/
│   │   │   ├── Header/
│   │   │   └── ...
│   │   ├── pages/             # 页面
│   │   │   ├── Login/
│   │   │   ├── Dashboard/
│   │   │   ├── Groups/
│   │   │   ├── Keys/
│   │   │   ├── Stats/
│   │   │   └── Settings/
│   │   ├── hooks/             # 自定义 hooks
│   │   ├── stores/            # Zustand stores
│   │   ├── types/             # TypeScript 类型
│   │   └── utils/             # 工具函数
│   └── public/
│
├── docs/                       # 文档
│   ├── architecture.md
│   ├── api-reference.md
│   ├── database.md
│   ├── development.md
│   └── deployment.md
│
└── scripts/                    # 脚本
    ├── dev.sh                  # 开发启动
    ├── build.sh                # 构建
    └── deploy.sh               # 部署
```

---

## 4. 核心模块设计

### 4.1 调度器 (Scheduler)

**职责**：从 Key 池中选择最合适的 Key

```rust
pub trait Scheduler: Send + Sync {
    /// 选择一个可用的 Key
    fn select_key(&self, request: &ChatRequest) -> Result<Arc<ApiKey>, SchedulerError>;
    
    /// 报告 Key 使用结果（用于故障转移）
    fn report_result(&self, key_id: Uuid, result: KeyResult);
    
    /// 获取 Key 状态
    fn get_key_status(&self, key_id: Uuid) -> KeyStatus;
}

/// 调度策略
pub enum ScheduleStrategy {
    /// 轮询（默认）
    RoundRobin,
    /// 加权轮询（按权重分配）
    WeightedRoundRobin,
    /// 最少使用（优先用最少用的 Key）
    LeastUsed,
    /// 随机
    Random,
}
```

**故障转移逻辑**：
```
1. 尝试 Key A
2. 如果失败（429/500/超时）：
   - 标记 Key A 为"冷却中"
   - 从池中临时移除 60 秒
   - 尝试 Key B
3. 如果所有 Key 都失败：
   - 返回错误
   - 加入排队等待重试
```

### 4.2 积分系统 (Credits)

**职责**：计算积分、记录交易、查询余额

```rust
/// 积分计算器
pub struct CreditsCalculator {
    rates: HashMap<Provider, f64>,  // $1 = N 积分
}

impl CreditsCalculator {
    /// 根据成本计算积分
    pub fn calculate(&self, cost_usd: f64, provider: &Provider) -> f64 {
        let rate = self.rates.get(provider).unwrap_or(&10.0);
        cost_usd * rate
    }
    
    /// 根据模型和 tokens 估算成本
    pub fn estimate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64 {
        // 参考 OpenAI 定价
        let (input_rate, output_rate) = match model {
            "gpt-4-turbo" => (0.01 / 1000.0, 0.03 / 1000.0),
            "gpt-4" => (0.03 / 1000.0, 0.06 / 1000.0),
            "gpt-3.5-turbo" => (0.0005 / 1000.0, 0.0015 / 1000.0),
            "claude-3-opus" => (0.015 / 1000.0, 0.075 / 1000.0),
            "claude-3-sonnet" => (0.003 / 1000.0, 0.015 / 1000.0),
            _ => (0.001 / 1000.0, 0.002 / 1000.0),
        };
        (input_tokens as f64 * input_rate) + (output_tokens as f64 * output_rate)
    }
}

/// 积分交易类型
pub enum CreditTransactionType {
    /// 贡献 Key 被使用，获得积分
    Contribution,
    /// 使用服务，消耗积分
    Usage,
    /// 管理员调整
    AdminAdjust,
    /// 系统奖励
    Bonus,
}
```

**积分规则**：
- 每次调用，贡献者获得 `cost_usd * rate` 积分
- 默认 rate：
  - OpenAI: 10 积分/$
  - Anthropic: 12 积分/$
  - Gemini: 8 积分/$
  - 其他: 10 积分/$

### 4.3 排队系统 (Queue)

**职责**：管理并发请求，按优先级排队

```rust
/// 排队管理器
pub struct QueueManager {
    redis: RedisClient,
}

impl QueueManager {
    /// 加入队列
    pub async fn enqueue(&self, request: QueueRequest) -> Result<Uuid, QueueError>;
    
    /// 获取下一个请求（按优先级）
    pub async fn dequeue(&self, group_id: Uuid) -> Result<Option<QueueEntry>, QueueError>;
    
    /// 获取队列长度
    pub async fn length(&self, group_id: Uuid) -> Result<usize, QueueError>;
    
    /// 获取用户在队列中的位置
    pub async fn position(&self, request_id: Uuid) -> Result<Option<usize>, QueueError>;
}

/// 优先级计算
pub fn calculate_priority(user: &User, wait_time_secs: u64) -> f64 {
    // 基础优先级 = 积分
    let base = user.credits;
    
    // 等待时间加成（每分钟 +0.1，最多 +5）
    let wait_bonus = (wait_time_secs as f64 / 60.0 * 0.1).min(5.0);
    
    // 新用户保护（前 24 小时 +10）
    let new_user_bonus = if user.created_at > Utc::now() - chrono::Duration::hours(24) {
        10.0
    } else {
        0.0
    };
    
    base + wait_bonus + new_user_bonus
}
```

**Redis 数据结构**：
```
# 队列（Sorted Set，按优先级排序）
apihub:queue:{group_id}
  score: priority (float)
  member: request_id (UUID)

# 请求详情（Hash）
apihub:request:{request_id}
  user_id, model, priority, created_at, status
```

### 4.4 代理层 (Proxy)

**职责**：转发请求到各 API 提供商

```rust
/// Provider trait
#[async_trait]
pub trait Provider: Send + Sync {
    /// 发送聊天请求
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError>;
    
    /// 发送流式请求
    async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream, ProviderError>;
    
    /// 获取可用模型列表
    fn models(&self) -> Vec<&str>;
    
    /// Provider 名称
    fn name(&self) -> &str;
}

/// OpenAI Provider
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

/// Anthropic Provider
pub struct AnthropicProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}
```

---

## 5. 数据流

### 5.1 聊天请求流程

```
1. 用户发送 POST /v1/chat/completions
   ↓
2. 中间件验证 JWT，提取 user_id
   ↓
3. 检查是否有可用 Key
   ├─ 有 → 继续
   └─ 无 → 返回错误
   ↓
4. 检查是否需要排队
   ├─ 需要排队 → 加入 Redis 队列，返回队列位置
   └─ 不需要 → 继续
   ↓
5. 调度器选择 Key
   ↓
6. Provider 发起请求
   ├─ 成功 → 记录日志，更新积分
   └─ 失败 → 故障转移或返回错误
   ↓
7. 返回响应
```

### 5.2 积分更新流程

```
1. 请求完成，获取 tokens 使用量
   ↓
2. 计算成本（estimate_cost）
   ↓
3. 计算积分（calculate_credits）
   ↓
4. 数据库事务：
   - 更新 api_keys.used_quota
   - 插入 usage_logs
   - 插入 credit_transactions
   - 更新 users.credits
   ↓
5. 更新 Redis 缓存
```

---

## 6. 安全设计

### 6.1 API Key 加密

```rust
/// AES-256-GCM 加密
pub fn encrypt_key(plaintext: &str, master_key: &[u8; 32]) -> Result<String, CryptoError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(master_key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())?;
    
    // 格式: base64(nonce || ciphertext)
    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    Ok(BASE64_STANDARD.encode(&result))
}

pub fn decrypt_key(encrypted: &str, master_key: &[u8; 32]) -> Result<String, CryptoError> {
    let bytes = BASE64_STANDARD.decode(encrypted)?;
    let (nonce, ciphertext) = bytes.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(master_key));
    let plaintext = cipher.decrypt(Nonce::from_slice(nonce), ciphertext)?;
    String::from_utf8(plaintext).map_err(|_| CryptoError::InvalidUtf8)
}
```

### 6.2 Key 去重

```rust
/// 计算Key哈希（用于去重）
pub fn hash_key(api_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    format!("{:x}", hasher.finalize())
}
```

在同一个群组内，相同的 Key 只能贡献一次。

### 6.3 配额保护

```rust
/// 检查配额
pub async fn check_quota(key: &ApiKey) -> Result<(), QuotaError> {
    if let Some(monthly_quota) = key.monthly_quota {
        if key.used_quota >= monthly_quota {
            return Err(QuotaError::Exceeded);
        }
    }
    Ok(())
}
```

---

## 7. 性能优化

### 7.1 连接池

- PostgreSQL: 使用 SeaORM 内置连接池
- Redis: 使用 deadpool-redis
- HTTP: 使用 reqwest 的连接池

### 7.2 缓存策略

| 数据 | 缓存位置 | TTL | 理由 |
|------|---------|-----|------|
| 用户信息 | Redis | 5 分钟 | 频繁读取 |
| Key 列表 | Redis | 1 分钟 | 需要较新 |
| 积分余额 | Redis | 实时 | 准确性要求高 |
| 排行榜 | Redis | 1 分钟 | 允许延迟 |

### 7.3 并发控制

```rust
/// 使用信号量限制并发
let semaphore = Arc::new(Semaphore::new(100)); // 最多 100 并发

async fn handle_chat(request: ChatRequest) -> Result<ChatResponse> {
    let _permit = semaphore.acquire().await?;
    // 处理请求
}
```

---

## 8. 可扩展性

### 8.1 添加新的 Provider

1. 实现 `Provider` trait
2. 在 `providers/mod.rs` 注册
3. 添加数据库迁移（如需新字段）

### 8.2 添加新的调度策略

1. 实现 `Scheduler` trait
2. 在配置中添加选项
3. 在 `SchedulerFactory` 中注册

### 8.3 水平扩展

ApiHub 设计为无状态（除 Redis 排队），可以水平扩展：

```
                    ┌─────────────┐
                    │   负载均衡   │
                    └─────────────┘
                          ↓
        ┌─────────────────┼─────────────────┐
        ↓                 ↓                 ↓
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│  ApiHub #1    │ │  ApiHub #2    │ │  ApiHub #3    │
└───────────────┘ └───────────────┘ └───────────────┘
        ↓                 ↓                 ↓
        └─────────────────┼─────────────────┘
                          ↓
                ┌─────────────────┐
                │  Shared Redis   │
                │  + PostgreSQL   │
                └─────────────────┘
```

---

## 9. 监控和日志

### 9.1 日志格式

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "apihub::api::routes::chat",
  "message": "Chat request completed",
  "fields": {
    "user_id": "uuid",
    "key_id": "uuid",
    "model": "gpt-4-turbo",
    "latency_ms": 1234,
    "tokens": 500
  }
}
```

### 9.2 Prometheus 指标

```rust
// 请求计数
lazy_static! {
    static ref REQUEST_COUNT: Counter = register_counter!(
        "apihub_requests_total",
        "Total number of requests"
    ).unwrap();
    
    static ref REQUEST_LATENCY: Histogram = register_histogram!(
        "apihub_request_latency_seconds",
        "Request latency in seconds"
    ).unwrap();
    
    static ref ACTIVE_KEYS: Gauge = register_gauge!(
        "apihub_active_keys",
        "Number of active API keys"
    ).unwrap();
}
```

---

## 10. 未来规划

### Phase 2

- [ ] WebRTC 直连（减少服务器负载）
- [ ] 多群组支持（一个用户多个群组）
- [ ] API 市场（公开共享 Key）
- [ ] 插件系统（自定义调度策略）

### Phase 3

- [ ] 移动端 App
- [ ] 桌面端客户端
- [ ] 联邦模式（多个 ApiHub 互联）

---

*最后更新: 2024-01-15*
