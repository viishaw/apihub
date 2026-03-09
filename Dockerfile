# 构建阶段 - 后端
FROM rust:1.75 as backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release

# 构建阶段 - 前端
FROM node:20-alpine as frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY frontend ./
RUN npm run build

# 运行阶段
FROM debian:bookworm-slim
WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 复制后端二进制
COPY --from=backend-builder /app/target/release/api /app/apihub

# 复制前端构建产物
COPY --from=frontend-builder /app/dist /app/dist

# 创建启动脚本
RUN echo '#!/bin/bash\n./apihub --host 0.0.0.0 --port 3000\n' > /app/start.sh && \
    chmod +x /app/start.sh

# 暴露端口
EXPOSE 3000

# 启动应用
CMD ["./apihub"]
