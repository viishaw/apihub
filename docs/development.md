# ApiHub 开发指南

> 本指南帮助你在本地搭建 ApiHub 开发环境。

---

## 1. 环境要求

### 1.1 必需软件

| 软件 | 版本 | 安装方式 |
|------|------|----------|
| **Rust** | 1.75+ | https://rustup.rs |
| **Node.js** | 18+ | https://nodejs.org |
| **PostgreSQL** | 15+ | Docker 或本地安装 |
| **Redis** | 7+ | Docker 或本地安装 |
| **Docker** | 最新 | https://docker.com |

### 1.2 推荐工具

- **VS Code** + rust-analyzer 插件
- **DBeaver** 或 **pgAdmin**（数据库管理）
- **Redis Insight**（Redis 管理）

---

## 2. 快速开始

### 2.1 克隆项目

```bash
git clone https://github.com/yourusername/apihub.git
cd apihub
```

### 2.2 启动依赖服务（Docker）

```bash
# 启动 PostgreSQL 和 Redis
docker-compose up -d postgres redis

# 验证
docker-compose ps
```

### 2.3 配置环境变量

```bash
cp .env.example .env
```

编辑 `.env`：

```env
DATABASE_URL=postgres://apihub:apihub@localhost:5432/apihub
REDIS_URL=redis://localhost:6379
JWT_SECRET=dev-jwt-secret-do-not-use-in-production
MASTER_KEY=dev-master-key-do-not-use-in-production-32ch
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
RUST_LOG=debug
```

### 2.4 运行数据库迁移

```bash
cd crates/db
cargo run --bin migration
```

### 2.5 启动后端

```bash
cd crates/api
cargo run
```

后端启动在 http://localhost:3000

### 2.6 启动前端

```bash
cd frontend
npm install
npm run dev
```

前端启动在 http://localhost:5173

---

## 3. 项目结构

```
apihub/
├── crates/                     # Rust 后端
│   ├── api/                    # HTTP API 层
│   ├── core/                   # 核心业务逻辑
│   ├── db/                     # 数据库层
│   └── providers/              # API 提供商适配器
│
├── frontend/                   # React 前端
│   ├── src/
│   │   ├── api/               # API 调用
│   │   ├── components/        # 组件
│   │   ├── pages/             # 页面
│   │   ├── hooks/             # Hooks
│   │   ├── stores/            # 状态管理
│   │   └── types/             # 类型定义
│   └── package.json
│
├── docs/                       # 文档
└── scripts/                    # 脚本
```

---

## 4. 开发流程

### 4.1 创建新功能

```bash
# 1. 创建分支
git checkout -b feature/your-feature

# 2. 开发
# ... 编写代码 ...

# 3. 测试
cargo test
cd frontend && npm test

# 4. 提交
git add .
git commit -m "feat: 添加新功能"

# 5. 推送
git push origin feature/your-feature

# 6. 创建 PR
```

### 4.2 数据库迁移

```bash
# 创建新迁移
cd crates/db
sea-orm-cli migrate generate add_new_table

# 编辑迁移文件
# crates/db/migration/src/m20240120_000001_add_new_table.rs

# 运行迁移
cargo run --bin migration

# 回滚（开发时）
cargo run --bin migration -- down
```

### 4.3 添加新 API

1. 在 `crates/api/src/routes/` 创建路由文件
2. 在 `crates/api/src/routes/mod.rs` 注册路由
3. 在 `crates/core/` 实现业务逻辑
4. 在 `crates/db/src/repositories/` 添加数据访问
5. 编写测试

---

## 5. 测试

### 5.1 单元测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_calculate_credits

# 显示输出
cargo test -- --nocapture
```

### 5.2 集成测试

```bash
# 运行集成测试
cargo test --test integration

# 需要 Docker 启动测试数据库
docker-compose -f docker-compose.test.yml up -d
cargo test --test integration
```

### 5.3 前端测试

```bash
cd frontend
npm test
npm run test:e2e  # 端到端测试
```

---

## 6. 代码规范

### 6.1 Rust

```bash
# 格式化
cargo fmt

# Lint
cargo clippy

# 自动修复
cargo clippy --fix
```

**规范**：
- 使用 `rustfmt` 默认配置
- 避免 unwrap()，使用 `?` 或 `expect()`
- 公开函数必须有文档注释
- 错误类型实现 `thiserror::Error`

### 6.2 TypeScript

```bash
cd frontend

# 格式化
npm run format

# Lint
npm run lint

# 自动修复
npm run lint:fix
```

**规范**：
- 使用 Prettier 格式化
- 组件使用 PascalCase
- 函数使用 camelCase
- 常量使用 UPPER_SNAKE_CASE

---

## 7. 调试

### 7.1 后端调试

**VS Code launch.json**:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug API",
      "cargo": {
        "args": ["build", "--bin=apihub", "--package=apihub-api"],
        "filter": {
          "name": "apihub",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}/crates/api"
    }
  ]
}
```

### 7.2 日志调试

```rust
use tracing::{debug, info, warn, error};

// 使用
debug!("Processing request for user: {}", user_id);
info!("Request completed in {}ms", latency);
warn!("Key {} is rate limited", key_id);
error!("Failed to connect to database: {}", err);
```

### 7.3 数据库调试

```bash
# 连接数据库
psql -U apihub -d apihub -h localhost

# 查看表
\dt

# 查询数据
SELECT * FROM users LIMIT 5;
```

---

## 8. 常见问题

### 8.1 编译错误

**问题**: `linker 'cc' not found`
**解决**: 
```bash
# macOS
xcode-select --install

# Linux
sudo apt install build-essential
```

### 8.2 数据库连接失败

**问题**: `connection refused`
**解决**:
```bash
# 检查 PostgreSQL 是否运行
docker-compose ps postgres

# 检查连接字符串
echo $DATABASE_URL
```

### 8.3 端口占用

**问题**: `Address already in use`
**解决**:
```bash
# 查看端口占用
lsof -i :3000

# 杀掉进程
kill -9 <PID>
```

---

## 9. 性能分析

### 9.1 基准测试

```bash
# 运行基准测试
cargo bench
```

### 9.2 性能分析

```bash
# 生成火焰图
cargo flamegraph --root -- bin apihub

# 内存分析
cargo instruments -t Allocations -- bin apihub
```

---

## 10. 贡献代码

### 10.1 提交规范

使用 Conventional Commits:

- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `style:` 代码格式
- `refactor:` 重构
- `test:` 测试
- `chore:` 构建/工具

### 10.2 PR 规范

- 标题清晰描述变更
- 关联 Issue
- 添加测试
- 更新文档

---

*最后更新: 2024-01-20*
