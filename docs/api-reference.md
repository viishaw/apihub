# ApiHub API 参考文档

> 本文档描述 ApiHub 的所有 API 接口。

---

## 基础信息

### Base URL

```
http://localhost:3000/api/v1
```

### 认证

所有需要认证的接口使用 JWT Bearer Token：

```
Authorization: Bearer <token>
```

### 响应格式

#### 成功响应

```json
{
  "success": true,
  "data": { ... }
}
```

#### 错误响应

```json
{
  "success": false,
  "error": {
    "code": "INVALID_CREDENTIALS",
    "message": "用户名或密码错误",
    "details": { ... }
  }
}
```

### 通用状态码

| 状态码 | 说明 |
|--------|------|
| 200 | 成功 |
| 201 | 创建成功 |
| 400 | 请求参数错误 |
| 401 | 未认证 |
| 403 | 无权限 |
| 404 | 资源不存在 |
| 409 | 资源冲突（如重复） |
| 429 | 请求过于频繁 |
| 500 | 服务器内部错误 |

---

## 1. 认证 API

### 1.1 注册

**POST** `/auth/register`

创建新用户并创建群组（第一个用户自动成为群主）。

#### 请求

```json
{
  "username": "alice",
  "email": "alice@example.com",
  "password": "secure_password_123",
  "group_name": "我的小圈子"
}
```

#### 响应

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "alice",
      "email": "alice@example.com",
      "credits": 0.0,
      "is_admin": true,
      "created_at": "2024-01-15T10:00:00Z"
    },
    "group": {
      "id": "660e8400-e29b-41d4-a716-446655440000",
      "name": "我的小圈子",
      "invite_code": "ABC123XYZ",
      "owner_id": "550e8400-e29b-41d4-a716-446655440000"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

### 1.2 登录

**POST** `/auth/login`

#### 请求

```json
{
  "username": "alice",
  "password": "secure_password_123"
}
```

#### 响应

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "alice",
      "email": "alice@example.com",
      "credits": 150.5,
      "is_admin": true
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

### 1.3 加入群组

**POST** `/auth/join`

通过邀请码加入已有群组。

#### 请求

```json
{
  "username": "bob",
  "email": "bob@example.com",
  "password": "secure_password_456",
  "invite_code": "ABC123XYZ"
}
```

#### 响应

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "770e8400-e29b-41d4-a716-446655440000",
      "username": "bob",
      "email": "bob@example.com",
      "credits": 0.0,
      "is_admin": false
    },
    "group": {
      "id": "660e8400-e29b-41d4-a716-446655440000",
      "name": "我的小圈子"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

### 1.4 获取当前用户

**GET** `/auth/me`

需要认证。

#### 响应

```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "email": "alice@example.com",
    "credits": 150.5,
    "is_admin": true,
    "group": {
      "id": "660e8400-e29b-41d4-a716-446655440000",
      "name": "我的小圈子",
      "member_count": 5
    },
    "created_at": "2024-01-15T10:00:00Z",
    "last_active_at": "2024-01-20T15:30:00Z"
  }
}
```

### 1.5 修改密码

**PUT** `/auth/password`

需要认证。

#### 请求

```json
{
  "old_password": "secure_password_123",
  "new_password": "new_secure_password_789"
}
```

#### 响应

```json
{
  "success": true,
  "data": {
    "message": "密码修改成功"
  }
}
```

---

## 2. 群组 API

### 2.1 获取群组信息

**GET** `/groups/{group_id}`

需要认证。

#### 响应

```json
{
  "success": true,
  "data": {
    "id": "660e8400-e29b-41d4-a716-446655440000",
    "name": "我的小圈子",
    "invite_code": "ABC123XYZ",
    "owner": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "alice"
    },
    "member_count": 5,
    "key_count": 12,
    "total_credits": 1250.5,
    "created_at": "2024-01-15T10:00:00Z"
  }
}
```

### 2.2 获取群组成员

**GET** `/groups/{group_id}/members`

需要认证。

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| page | int | 否 | 页码，默认 1 |
| limit | int | 否 | 每页数量，默认 20 |

#### 响应

```json
{
  "success": true,
  "data": {
    "members": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "alice",
        "credits": 150.5,
        "key_count": 5,
        "is_admin": true,
        "joined_at": "2024-01-15T10:00:00Z"
      },
      {
        "id": "770e8400-e29b-41d4-a716-446655440000",
        "username": "bob",
        "credits": 85.0,
        "key_count": 2,
        "is_admin": false,
        "joined_at": "2024-01-16T14:30:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 5,
      "total_pages": 1
    }
  }
}
```

### 2.3 更新群组信息

**PUT** `/groups/{group_id}`

需要认证。只有群主可以修改。

#### 请求

```json
{
  "name": "新的群组名"
}
```

#### 响应

```json
{
  "success": true,
  "data": {
    "id": "660e8400-e29b-41d4-a716-446655440000",
    "name": "新的群组名"
  }
}
```

### 2.4 重新生成邀请码

**POST** `/groups/{group_id}/regenerate-invite`

需要认证。只有群主可以操作。

#### 响应

```json
{
  "success": true,
  "data": {
    "invite_code": "NEW123CODE"
  }
}
```

### 2.5 踢出成员

**DELETE** `/groups/{group_id}/members/{user_id}`

需要认证。只有群主可以操作。

#### 响应

```json
{
  "success": true,
  "data": {
    "message": "成员已移除"
  }
}
```

### 2.6 退出群组

**POST** `/groups/{group_id}/leave`

需要认证。群主不能退出。

#### 响应

```json
{
  "success": true,
  "data": {
    "message": "已退出群组"
  }
}
```

---

## 3. API Key 管理 API

### 3.1 贡献 API Key

**POST** `/keys`

需要认证。

#### 请求

```json
{
  "provider": "openai",
  "api_key": "sk-xxxxxxxxxxxxxxxx",
  "name": "我的 OpenAI Key",
  "base_url": "https://api.openai.com/v1",
  "monthly_quota": 20.0
}
```

#### 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| provider | string | 是 | 提供商: openai, anthropic, gemini, custom |
| api_key | string | 是 | API Key（将被加密存储） |
| name | string | 否 | Key 的名称，方便识别 |
| base_url | string | 否 | 自定义 API 地址 |
| monthly_quota | float | 否 | 每月配额（美元），超过后自动停用 |

#### 响应

```json
{
  "success": true,
  "data": {
    "id": "880e8400-e29b-41d4-a716-446655440000",
    "provider": "openai",
    "name": "我的 OpenAI Key",
    "monthly_quota": 20.0,
    "used_quota": 0.0,
    "is_active": true,
    "created_at": "2024-01-20T10:00:00Z"
  }
}
```

### 3.2 获取群组内所有 Key

**GET** `/keys`

需要认证。返回群组内所有 Key（不包含明文 Key）。

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| provider | string | 否 | 筛选提供商 |
| active_only | bool | 否 | 只返回活跃的 Key |

#### 响应

```json
{
  "success": true,
  "data": {
    "keys": [
      {
        "id": "880e8400-e29b-41d4-a716-446655440000",
        "provider": "openai",
        "name": "我的 OpenAI Key",
        "contributor": {
          "id": "550e8400-e29b-41d4-a716-446655440000",
          "username": "alice"
        },
        "monthly_quota": 20.0,
        "used_quota": 5.5,
        "weight": 1,
        "is_active": true,
        "last_used_at": "2024-01-20T15:30:00Z",
        "created_at": "2024-01-20T10:00:00Z"
      }
    ],
    "summary": {
      "total_keys": 12,
      "active_keys": 10,
      "by_provider": {
        "openai": 5,
        "anthropic": 4,
        "gemini": 3
      }
    }
  }
}
```

### 3.3 获取我贡献的 Key

**GET** `/keys/my`

需要认证。

#### 响应

```json
{
  "success": true,
  "data": {
    "keys": [
      {
        "id": "880e8400-e29b-41d4-a716-446655440000",
        "provider": "openai",
        "name": "我的 OpenAI Key",
        "monthly_quota": 20.0,
        "used_quota": 5.5,
        "is_active": true,
        "credits_earned": 55.0,
        "usage_count": 150
      }
    ]
  }
}
```

### 3.4 更新 Key

**PUT** `/keys/{key_id}`

需要认证。只有贡献者可以修改自己的 Key。

#### 请求

```json
{
  "name": "新的名称",
  "monthly_quota": 30.0,
  "weight": 2
}
```

#### 响应

```json
{
  "success": true,
  "data": {
    "id": "880e8400-e29b-41d4-a716-446655440000",
    "name": "新的名称",
    "monthly_quota": 30.0,
    "weight": 2
  }
}
```

### 3.5 撤回 Key

**DELETE** `/keys/{key_id}`

需要认证。只有贡献者可以撤回自己的 Key。

#### 响应

```json
{
  "success": true,
  "data": {
    "message": "Key 已撤回"
  }
}
```

### 3.6 暂停/恢复 Key

**POST** `/keys/{key_id}/toggle`

需要认证。只有贡献者可以操作。

#### 响应

```json
{
  "success": true,
  "data": {
    "id": "880e8400-e29b-41d4-a716-446655440000",
    "is_active": false
  }
}
```

---

## 4. 聊天 API

### 4.1 OpenAI 兼容接口

**POST** `/v1/chat/completions`

需要认证。完全兼容 OpenAI API 格式。

#### 请求

```json
{
  "model": "gpt-4-turbo",
  "messages": [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": "Hello!"}
  ],
  "temperature": 0.7,
  "max_tokens": 1000,
  "stream": false
}
```

#### 响应（非流式）

```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1705509600,
  "model": "gpt-4-turbo",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I help you today?"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 20,
    "completion_tokens": 10,
    "total_tokens": 30
  }
}
```

#### 响应（流式）

当 `stream: true` 时，返回 SSE 流：

```
data: {"id":"chatcmpl-123","choices":[{"delta":{"content":"Hello"},"index":0}]}

data: {"id":"chatcmpl-123","choices":[{"delta":{"content":"!"},"index":0}]}

data: [DONE]
```

### 4.2 Anthropic 兼容接口

**POST** `/v1/messages`

需要认证。兼容 Anthropic API 格式。

#### 请求

```json
{
  "model": "claude-3-opus-20240229",
  "max_tokens": 1024,
  "messages": [
    {"role": "user", "content": "Hello, Claude"}
  ]
}
```

#### 响应

```json
{
  "id": "msg_123",
  "type": "message",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "Hello! How can I help you today?"
    }
  ],
  "model": "claude-3-opus-20240229",
  "stop_reason": "end_turn",
  "usage": {
    "input_tokens": 10,
    "output_tokens": 20
  }
}
```

### 4.3 获取可用模型列表

**GET** `/v1/models`

需要认证。

#### 响应

```json
{
  "success": true,
  "data": {
    "models": [
      {
        "id": "gpt-4-turbo",
        "name": "GPT-4 Turbo",
        "provider": "openai",
        "available": true,
        "key_count": 3
      },
      {
        "id": "claude-3-opus",
        "name": "Claude 3 Opus",
        "provider": "anthropic",
        "available": true,
        "key_count": 2
      },
      {
        "id": "gemini-pro",
        "name": "Gemini Pro",
        "provider": "gemini",
        "available": false,
        "key_count": 0
      }
    ]
  }
}
```

### 4.4 查询队列状态

**GET** `/chat/queue`

需要认证。查看当前是否有排队请求。

#### 响应

```json
{
  "success": true,
  "data": {
    "queue_length": 3,
    "your_position": 2,
    "estimated_wait_seconds": 30,
    "your_request": {
      "id": "990e8400-e29b-41d4-a716-446655440000",
      "model": "gpt-4-turbo",
      "priority": 85.5,
      "created_at": "2024-01-20T15:30:00Z"
    }
  }
}
```

---

## 5. 统计 API

### 5.1 获取用量统计

**GET** `/stats/usage`

需要认证。

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| period | string | 否 | 时间范围: today, week, month, all，默认 week |
| user_id | uuid | 否 | 指定用户（仅管理员） |

#### 响应

```json
{
  "success": true,
  "data": {
    "period": "week",
    "summary": {
      "total_requests": 1250,
      "total_tokens": 1250000,
      "total_cost_usd": 25.5,
      "unique_users": 8,
      "avg_latency_ms": 1200
    },
    "by_model": [
      {
        "model": "gpt-4-turbo",
        "requests": 500,
        "tokens": 500000,
        "cost_usd": 15.0
      },
      {
        "model": "gpt-3.5-turbo",
        "requests": 750,
        "tokens": 750000,
        "cost_usd": 10.5
      }
    ],
    "by_user": [
      {
        "user_id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "alice",
        "requests": 300,
        "tokens": 300000,
        "cost_usd": 6.0
      }
    ],
    "daily": [
      {
        "date": "2024-01-20",
        "requests": 200,
        "tokens": 200000,
        "cost_usd": 4.0
      }
    ]
  }
}
```

### 5.2 获取贡献统计

**GET** `/stats/contribution`

需要认证。

#### 响应

```json
{
  "success": true,
  "data": {
    "summary": {
      "total_keys": 12,
      "active_keys": 10,
      "total_quota_usd": 200.0,
      "used_quota_usd": 50.0
    },
    "by_provider": [
      {
        "provider": "openai",
        "key_count": 5,
        "total_quota": 100.0,
        "used_quota": 25.0
      }
    ],
    "by_user": [
      {
        "user_id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "alice",
        "key_count": 5,
        "total_credits_earned": 250.0,
        "usage_by_others": 150
      }
    ]
  }
}
```

### 5.3 获取积分排行榜

**GET** `/stats/leaderboard`

需要认证。

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| limit | int | 否 | 返回数量，默认 10 |

#### 响应

```json
{
  "success": true,
  "data": {
    "leaderboard": [
      {
        "rank": 1,
        "user_id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "alice",
        "credits": 350.5,
        "key_count": 5,
        "avatar": "https://..."
      },
      {
        "rank": 2,
        "user_id": "770e8400-e29b-41d4-a716-446655440000",
        "username": "bob",
        "credits": 150.0,
        "key_count": 3,
        "avatar": null
      }
    ],
    "your_rank": 2
  }
}
```

### 5.4 获取积分记录

**GET** `/stats/credits`

需要认证。

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| page | int | 否 | 页码 |
| limit | int | 否 | 每页数量 |
| type | string | 否 | 筛选类型: contribution, usage, admin_adjust |

#### 响应

```json
{
  "success": true,
  "data": {
    "transactions": [
      {
        "id": "aa0e8400-e29b-41d4-a716-446655440000",
        "amount": 5.5,
        "type": "contribution",
        "reason": "Your OpenAI key was used by bob",
        "related_usage": {
          "model": "gpt-4-turbo",
          "tokens": 500
        },
        "created_at": "2024-01-20T15:30:00Z"
      },
      {
        "id": "bb0e8400-e29b-41d4-a716-446655440000",
        "amount": -2.0,
        "type": "usage",
        "reason": "Used GPT-4 Turbo",
        "created_at": "2024-01-20T14:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 50
    },
    "balance": 150.5
  }
}
```

---

## 6. 管理员 API

> 以下接口需要管理员权限。

### 6.1 获取所有用户

**GET** `/admin/users`

#### 响应

```json
{
  "success": true,
  "data": {
    "users": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "alice",
        "email": "alice@example.com",
        "credits": 350.5,
        "is_admin": true,
        "is_active": true,
        "created_at": "2024-01-15T10:00:00Z",
        "last_active_at": "2024-01-20T15:30:00Z"
      }
    ],
    "pagination": { ... }
  }
}
```

### 6.2 调整用户积分

**POST** `/admin/users/{user_id}/adjust-credits`

#### 请求

```json
{
  "amount": 50.0,
  "reason": "Bug 补偿"
}
```

#### 响应

```json
{
  "success": true,
  "data": {
    "new_balance": 400.5
  }
}
```

### 6.3 禁用/启用用户

**POST** `/admin/users/{user_id}/toggle`

#### 响应

```json
{
  "success": true,
  "data": {
    "is_active": false
  }
}
```

### 6.4 获取系统统计

**GET** `/admin/stats`

#### 响应

```json
{
  "success": true,
  "data": {
    "users": {
      "total": 10,
      "active_today": 5,
      "active_week": 8
    },
    "keys": {
      "total": 15,
      "active": 12,
      "by_provider": { "openai": 8, "anthropic": 7 }
    },
    "usage": {
      "requests_today": 500,
      "tokens_today": 500000,
      "cost_today_usd": 10.0
    },
    "queue": {
      "current_length": 2,
      "avg_wait_seconds": 15
    }
  }
}
```

---

## 7. WebSocket API

### 7.1 连接

```
ws://localhost:3000/ws
```

需要认证，在 query string 中传入 token：

```
ws://localhost:3000/ws?token=xxx
```

### 7.2 消息格式

#### 客户端 → 服务端

```json
{
  "type": "subscribe",
  "channel": "queue"
}
```

#### 服务端 → 客户端

```json
{
  "type": "queue_update",
  "data": {
    "position": 1,
    "estimated_wait_seconds": 10
  }
}
```

### 7.3 频道

| 频道 | 说明 |
|------|------|
| queue | 队列状态更新 |
| stats | 实时统计更新 |
| notifications | 系统通知 |

---

## 8. 错误码

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| INVALID_CREDENTIALS | 401 | 用户名或密码错误 |
| TOKEN_EXPIRED | 401 | Token 已过期 |
| PERMISSION_DENIED | 403 | 无权限 |
| USER_NOT_FOUND | 404 | 用户不存在 |
| GROUP_NOT_FOUND | 404 | 群组不存在 |
| KEY_NOT_FOUND | 404 | Key 不存在 |
| DUPLICATE_KEY | 409 | Key 已存在 |
| INVALID_INVITE_CODE | 400 | 邀请码无效 |
| QUOTA_EXCEEDED | 429 | 配额已用完 |
| NO_AVAILABLE_KEY | 503 | 没有可用的 Key |
| PROVIDER_ERROR | 502 | API 提供商返回错误 |

---

*最后更新: 2024-01-20*
