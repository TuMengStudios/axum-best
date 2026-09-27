# 使用官方 Rust 镜像作为构建环境（使用 1.98 稳定版本；SeaORM 2 要求 rustc 1.94+）
FROM rust:1.98-alpine AS builder

# 安装必要的构建依赖（包括静态 SSL 库、MySQL 开发包和 git——gitver proc-macro 需要）
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static mariadb-connector-c-dev git

# 创建工作目录
WORKDIR /app

# 复制整个项目（.dockerignore 已排除 target/、IDE、文档等；.git/ 保留供 gitver 使用）
COPY . .

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

# 启动应用
CMD ["/app/axum-best", "--conf", "/app/etc/config.toml"]
