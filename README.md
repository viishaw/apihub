# ApiHub

> **私有化部署的 AI API 共享池，积分驱动，公平调度。**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-green.svg)](Dockerfile)

---

## 🎯 项目简介

ApiHub 是一个**自托管**的 AI API 共享平台，让家庭、朋友、团队可以共享彼此的 API Key，通过**积分机制**实现公平调度。

### 核心特性

- 🔐 **私有群组** - 邀请制，只有信任的人才能加入
- 💰 **积分驱动** - 贡献 API Key 获得积分，积分高者优先调度
- 🔄 **智能调度** - 轮询、权重、故障自动转移
- 📊 **透明统计** - 每个人都能看到用量、贡献、排行榜
- 🛡️ **安全加密** - API Key 加密存储，随时可撤回
- 🐳 **一键部署** - Docker 支持，5 分钟启动

### 解决的痛点

| 痛点 | ApiHub 的解决方案 |
|------|-------------------|
| 订阅太多，买不起全套餐 | 朋友共享，每人贡献一部分 |
| 只想用一次生图，却要买一月 | 用别人的，按量付费 |
| 公共共享池不安全 | 私有群组，只邀请信任的人 |
| 白嫖党太多 | 积分机制，贡献换额度 |

---

## 🚀 快速开始

### 方式一： Docker（推荐）

```bash
# 克隆项目
git clone https://github.com/yourusername/apihub.git
cd apihub

# 配置环境变量
cp .env.example .env
# 编辑 .env，设置数据库密码、JWT 密钥等

# 启动
docker-compose up -d

# 访问
open http://localhost:3000
```

### 方式二： 源码编译

```bash
# 前置要求
# - Rust 1.75+
# - Node.js 18+
# - PostgreSQL 15+
# - Redis 7+

# 后端
cd backend
cargo build --release
./target/release/apihub

# 前端
cd frontend
npm install
npm run build
```

---

## 📚 文档目录

| 文档 | 说明 |
|------|------|
| [开发指南](docs/development.md) | 本地开发环境搭建 |
| [API 参考](docs/api-reference.md) | 完整 API 文档 |
| [架构设计](docs/architecture.md) | 系统架构详解 |
| [数据库设计](docs/database.md) | 数据模型和迁移 |
| [部署指南](docs/deployment.md) | 生产环境部署 |
| [贡献指南](CONTRIBUTING.md) | 如何参与开发 |

---

## 🤝 贡献

欢迎贡献代码、报告 Bug、提出建议！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

## 🙏 致谢

- [Axum](https://github.com/tokio-rs/axum) - 高性能 Web 框架
- [SeaORM](https://www.sea-ql.org/SeaORM/) - 异步 ORM
- [Ant Design](https://ant.design/) - React UI 组件库
- 所有贡献者

---

**Made with ❤️ by Vincent and contributors**
