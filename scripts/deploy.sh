#!/bin/bash
set -e

echo "=== ApiHub 部署脚本 ==="

# 检查依赖
echo "[1/6] 检查依赖..."
command -v docker || { echo "Error: Docker not installed"; exit 1; }
command -v docker-compose || { echo "Error: Docker Compose not installed"; exit 1; }

# 拉取最新代码
echo "[2/6] 拉取最新代码..."
cd /tmp/apihub
git pull || git clone --depth 1 https://github.com/viishaw/apihub.git .

# 创建环境变量
echo "[3/6] 创建环境变量..."
cat > .env << 'EOF'
POSTGRES_USER=apihub
POSTGRES_PASSWORD=apihub_secret_2024
POSTGRES_DB=apihub
DATABASE_URL=postgres://apihub:apihub_secret_2024@postgres:5432/apihub
REDIS_URL=redis://redis:6379
JWT_SECRET=apihub_jwt_secret_at_least_32_characters_long
MASTER_KEY=apihub_master_key_at_least_32_characters_long
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info
EOF

# 启动数据库和 Redis
echo "[4/6] 启动数据库和 Redis..."
docker-compose up -d postgres redis

# 等待数据库启动
echo "[5/6] 等待数据库启动..."
sleep 10

# 检查服务状态
echo "[6/6] 检查服务状态..."
docker-compose ps

# 输出访问地址
echo ""
echo "=== 部署完成 ==="
echo "API: http://43.136.39.231:3000"
echo "健康检查: http://43.136.39.231:3000/health"
echo ""
echo "下一步："
echo "1. 构建并启动应用: cd /tmp/apihub && docker-compose up -d apihub"
echo "2. 查看日志: docker-compose logs -f apihub"
