# 使用官方 Rust 镜像作为构建环境（使用 1.89 稳定版本）
FROM rust:1.89-alpine AS builder

# 安装必要的构建依赖（包括静态 SSL 库和 MySQL 开发包）
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static mariadb-connector-c-dev

# 创建工作目录
WORKDIR /app

# 复制 Cargo 配置文件
COPY .cargo ./.cargo

COPY .env ./.env

# 复制 Cargo.toml 和 Cargo.lock
COPY Cargo.toml Cargo.lock ./

# 复制基准测试文件
COPY benches ./benches

# 复制 SQLx 查询缓存
COPY .sqlx ./.sqlx

# 创建空的 src 目录和 main.rs 文件来缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖（这一步会被缓存）
RUN cargo build --release

# 现在复制实际的源代码
COPY src ./src
COPY etc ./etc
COPY migrations ./migrations
ENV CARGO_TARGET_DIR=~/.cargo/target
RUN cargo install sqlx-cli --features mysql
# 设置 SQLx 离线模式
ENV SQLX_OFFLINE=true

# 构建应用
RUN cargo build --release

# 使用轻量级运行时镜像（使用固定版本）
FROM alpine:3.20

# 安装运行时依赖
RUN apk add --no-cache ca-certificates

# 创建非 root 用户
RUN addgroup -S app && adduser -S app -G app

# 创建工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/axum-best /app/axum-best

# 复制配置文件
COPY --from=builder /app/etc /app/etc
COPY --from=builder /app/migrations /app/migrations

# 创建日志目录
RUN mkdir -p /app/logs && chown -R app:app /app

# 切换到非 root 用户
USER app

# 暴露端口
EXPOSE 8080

# 设置健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# 启动应用
CMD ["./axum-best", "--conf", "etc/config.toml"]
