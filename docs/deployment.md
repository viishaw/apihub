# ApiHub 部署指南

> 本指南帮助你在生产环境中部署 ApiHub。

---

## 1. 部署架构

```
┌─────────────────────────────────────────────────────────────┐
│                       互联网                                 │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│                   Caddy (反向代理)                           │
│                 - 自动 HTTPS                                │
│                 - WebSocket 支持                            │
│                 - 静态文件服务                               │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│                   ApiHub Backend                            │
│                 - Rust + Axum                               │
│                 - 单二进制文件                               │
└─────────────────────────────────────────────────────────────┘
              ↓                      ↓
┌──────────────────────┐   ┌──────────────────────┐
│     PostgreSQL       │   │       Redis          │
│    - 数据持久化       │   │    - 排队/缓存        │
└──────────────────────┘   └──────────────────────┘
```

---

## 2. 方式一：Docker Compose（推荐）

### 2.1 安装 Docker

```bash
# Ubuntu/Debian
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER

# 验证
docker --version
docker-compose --version
```

### 2.2 克隆项目

```bash
git clone https://github.com/yourusername/apihub.git
cd apihub
```

### 2.3 配置环境变量

```bash
cp .env.example .env
```

编辑 `.env`：

```env
# 数据库
POSTGRES_USER=apihub
POSTGRES_PASSWORD=你的强密码
POSTGRES_DB=apihub
DATABASE_URL=postgres://apihub:你的强密码@postgres:5432/apihub

# Redis
REDIS_URL=redis://redis:6379

# 安全
JWT_SECRET=你的JWT密钥至少32字符
MASTER_KEY=你的主密钥用于加密APIKey至少32字符

# 服务器
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# 域名（用于自动 HTTPS）
DOMAIN=apihub.yourdomain.com
```

### 2.4 启动

```bash
# 启动所有服务
docker-compose up -d

# 查看日志
docker-compose logs -f apihub

# 查看状态
docker-compose ps
```

### 2.5 验证

```bash
# 健康检查
curl http://localhost:3000/health

# 访问
open http://localhost:3000
```

---

## 3. 方式二：手动部署

### 3.1 安装依赖

```bash
# PostgreSQL
sudo apt install postgresql postgresql-contrib

# Redis
sudo apt install redis-server

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Node.js (前端构建)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install nodejs
```

### 3.2 配置 PostgreSQL

```bash
# 创建用户和数据库
sudo -u postgres psql
```

```sql
CREATE USER apihub WITH PASSWORD '你的密码';
CREATE DATABASE apihub OWNER apihub;
GRANT ALL PRIVILEGES ON DATABASE apihub TO apihub;
\q
```

### 3.3 配置 Redis

```bash
# 启动 Redis
sudo systemctl enable redis-server
sudo systemctl start redis-server

# 验证
redis-cli ping
```

### 3.4 构建后端

```bash
cd apihub/crates/api
cargo build --release
```

### 3.5 构建前端

```bash
cd apihub/frontend
npm install
npm run build
```

### 3.6 配置环境变量

```bash
export DATABASE_URL="postgres://apihub:你的密码@localhost:5432/apihub"
export REDIS_URL="redis://localhost:6379"
export JWT_SECRET="你的JWT密钥"
export MASTER_KEY="你的主密钥"
export SERVER_HOST="0.0.0.0"
export SERVER_PORT="3000"
```

### 3.7 运行迁移

```bash
cd apihub/crates/db
cargo run --bin migration
```

### 3.8 启动服务

```bash
# 直接运行
./target/release/apihub

# 或使用 systemd
sudo nano /etc/systemd/system/apihub.service
```

```ini
[Unit]
Description=ApiHub Server
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=apihub
WorkingDirectory=/opt/apihub
Environment="DATABASE_URL=postgres://apihub:密码@localhost:5432/apihub"
Environment="REDIS_URL=redis://localhost:6379"
Environment="JWT_SECRET=你的密钥"
Environment="MASTER_KEY=你的主密钥"
ExecStart=/opt/apihub/apihub
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable apihub
sudo systemctl start apihub
```

---

## 4. 配置 HTTPS

### 4.1 使用 Caddy（自动）

```bash
# 安装 Caddy
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install caddy
```

### 4.2 Caddyfile

```
apihub.yourdomain.com {
    reverse_proxy localhost:3000
    
    # 静态文件
    handle /assets/* {
        root * /opt/apihub/frontend/dist
        file_server
    }
}
```

### 4.3 启动 Caddy

```bash
sudo systemctl enable caddy
sudo systemctl start caddy
```

---

## 5. 生产环境优化

### 5.1 PostgreSQL 优化

编辑 `/etc/postgresql/15/main/postgresql.conf`：

```ini
# 连接
max_connections = 100

# 内存
shared_buffers = 256MB
effective_cache_size = 1GB

# 日志
logging_collector = on
log_directory = 'pg_log'
log_filename = 'postgresql-%Y-%m-%d.log'
```

### 5.2 Redis 优化

编辑 `/etc/redis/redis.conf`：

```ini
# 内存
maxmemory 256mb
maxmemory-policy allkeys-lru

# 持久化
appendonly yes
appendfsync everysec
```

### 5.3 系统优化

```bash
# 增加文件描述符限制
sudo nano /etc/security/limits.conf
```

```
* soft nofile 65536
* hard nofile 65536
```

---

## 6. 监控和日志

### 6.1 日志管理

ApiHub 输出 JSON 格式日志：

```bash
# 查看日志
journalctl -u apihub -f

# 日志轮转（/etc/logrotate.d/apihub）
/var/log/apihub/*.log {
    daily
    rotate 7
    compress
    missingok
    notifempty
}
```

### 6.2 Prometheus 监控（可选）

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'apihub'
    static_configs:
      - targets: ['localhost:3000']
```

访问 `/metrics` 端点获取指标。

---

## 7. 备份策略

### 7.1 数据库备份

```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/backup/apihub"
DATE=$(date +%Y%m%d)

mkdir -p $BACKUP_DIR

# PostgreSQL 备份
pg_dump apihub | gzip > $BACKUP_DIR/db_$DATE.sql.gz

# 保留 7 天
find $BACKUP_DIR -name "*.sql.gz" -mtime +7 -delete
```

### 7.2 定时备份

```bash
crontab -e
```

```
0 2 * * * /opt/apihub/backup.sh
```

---

## 8. 更新升级

### 8.1 Docker 方式

```bash
# 拉取最新镜像
docker-compose pull

# 重新部署
docker-compose up -d
```

### 8.2 手动方式

```bash
# 拉取代码
git pull

# 构建后端
cargo build --release

# 构建前端
cd frontend && npm run build

# 运行迁移
cd ../crates/db && cargo run --bin migration

# 重启服务
sudo systemctl restart apihub
```

---

## 9. 故障排查

### 9.1 常见问题

| 问题 | 排查 |
|------|------|
| 无法启动 | 检查环境变量、端口占用 |
| 数据库连接失败 | 检查 PostgreSQL 服务、连接字符串 |
| Redis 连接失败 | 检查 Redis 服务 |
| API Key 加密失败 | 检查 MASTER_KEY 是否设置 |

### 9.2 日志分析

```bash
# 查看错误日志
journalctl -u apihub | grep -i error

# 查看慢请求
journalctl -u apihub | grep "latency_ms.*[5-9][0-9][0-9][0-9]"
```

---

## 10. 安全建议

- 使用强密码
- 定期更新系统和依赖
- 启用防火墙（ufw）
- 配置 fail2ban 防暴力破解
- 定期备份数据
- 监控异常请求

---

*最后更新: 2024-01-20*
