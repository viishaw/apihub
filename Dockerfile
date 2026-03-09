# Frontend build stage
FROM node:18-alpine AS frontend-builder

WORKDIR /app
RUN npm install pnpm -g

COPY frontend/package*.json ./frontend/
WORKDIR /app/frontend
RUN pnpm install

COPY frontend ./
RUN pnpm build

# Backend build stage
FROM rust:1.75-alpine AS backend-builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release

# Final stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/api /app/api

# Copy frontend build
COPY --from=frontend-builder /app/frontend/dist /app/dist

# Copy startup script
COPY scripts/startup.sh /app/startup.sh
RUN chmod +x /app/startup.sh

EXPOSE 3000

CMD ["/app/startup.sh"]
