# axum best

* [english doc](README.md)
* [开发工具](dev.md)

> 为最佳的 axum 开发体验而设计

## 项目概述

axum-best 是一个基于 [Axum](https://github.com/tokio-rs/axum) 和 [Tokio](https://tokio.rs/) 构建的 Rust Web 项目模板。它展示了用于构建可扩展 HTTP 服务的分层架构，包含 MySQL、Redis、结构化日志、请求校验和常用中间件等能力。

### 主要特性

- **高性能**：基于 Tokio 和 Axum 实现异步请求处理
- **清晰的分层架构**：传输层、处理器层、服务层、仓储层、数据层职责分离
- **MySQL + SQLx**：类型安全的数据库操作，查询在编译期检查；`.sqlx` 目录中提交了离线查询元数据
- **Redis 缓存**：通过 `bb8-redis` 异步连接池（tokio 原生，r2d2 风格 API，基于 `redis` crate）实现缓存与会话类存储
- **请求校验**：使用 `validator` 进行输入校验（由项目自带的 `ValidJson` / `ValidQuery` / `ValidPath` / `ValidForm` 提取器封装）
- **JWT 认证**：使用 `jsonwebtoken` 校验 Bearer Token；受保护处理器可直接声明 `Claims` 参数获取用户信息
- **中间件栈**：请求 ID 链路追踪、CORS、请求解压缩、超时（`http.timeout_secs`）、响应压缩控制（支持按路径排除，前缀或 `regex:` 正则）以及按路由限流（见 `etc/config.toml` 的 `http.timeout_excluded_paths` / `http.compression_excluded_paths`）
- **GCRA 限流**：使用 `governor` 实现进程内 GCRA 限流，支持按路由分别配置客户端 IP 和已认证用户 ID 的配额
- **结构化日志**：支持 JSON 输出与日志轮转，通过 `etc/config.toml` 配置
- **微信小程序登录**：占位式登录流程，通过微信 code 与 `openid` 映射到已有用户
- **邮箱绑定**：带验证码生成的占位式邮箱绑定流程
- **用户 CRUD**：`user_info` 的增删改查及搜索完整示例
- **工具模块**：数字验证码生成、文件摘要（MD5/SHA1/SHA256/SHA512）、通用 `Primitive` 枚举
- **Docker 支持**：多阶段 Dockerfile 与 Docker Compose 配置
- **基准测试**：针对工具函数与模型生成的 Criterion 基准测试

## 项目结构

```plain
src/
├── conf/           # 从 TOML 加载配置
├── core/           # 核心类型：AppState、AppResult/AppError 包装器
├── data/           # 仓储实现（SQLx/Redis）与连接池管理
├── errors/         # 预定义应用错误
├── handlers/       # HTTP 请求处理器
├── logx/           # 结构化日志初始化
├── models/         # 数据模型与实体（如 UserInfo）
├── repos/          # 仓储接口定义（数据访问契约）
├── routers/        # 路由定义与中间件栈
├── services/       # 业务逻辑层
├── app.rs          # 应用上下文：组装依赖、持有 AppState 与 HTTP server
├── transport/      # HTTP 服务器设置与中间件辅助函数
│   └── middleware/ # 自定义中间件函数
├── types/          # 带校验的请求/响应 DTO
└── utils/          # 工具函数（哈希、验证码等）

benches/           # Criterion 基准测试套件
examples/          # 可运行的示例程序
etc/               # 配置文件
migrations/        # SQLx 数据库迁移脚本
```

### 架构层次

1. **传输层** (`src/transport/`)：TCP 监听器、HTTP 服务器设置与可复用中间件
2. **处理器层** (`src/handlers/`)：HTTP 端点处理器，解析输入并调用服务层
3. **服务层** (`src/services/`)：业务逻辑实现，service 持有启动时注入的仓储接口（`Arc<dyn UserRepo>`）
4. **仓储层** (`src/repos/`)：仓储接口定义（数据访问契约）
5. **数据层** (`src/data/`)：基于 SQLx/Redis 的仓储实现与连接池管理

依赖方向单向：`handlers → services → repos（trait）← data（实现）`；由 `app::AppContext` 在启动时将具体实现注入 service 并装入 `AppState`。

## API 端点

| 方法 | 路径 | 说明 |
| ---- | ---- | ---- |
| `GET` | `/health` | 健康检查，返回 `ok` |
| `GET` | `/foo` | 带 `key_word` 校验的演示查询接口 |
| `GET` | `/user/{id}` | 根据 ID 获取用户 |
| `POST` | `/user/wx/login` | 微信小程序登录占位接口 |
| `GET` | `/user/random` | 创建并返回一个随机 `UserInfo` |
| `POST` | `/user/email/pre` | 生成并存储邮箱绑定验证码 |
| `POST` | `/user/email` | 使用验证码绑定邮箱 |

## 限流

项目通过 [`governor`](https://github.com/boinkor-net/governor) 使用 GCRA 算法实现限流。每个 route 都可以拥有独立的限流器和配额。超过配额的请求会返回 `429 Too Many Requests`，并使用项目统一的错误响应格式。

需要登录的 route 使用规范化后的请求路径和 `Claims.user_id` 生成限流 key。健康检查则使用客户端 IP 单独限流。当前各 route 的配额如下：

| 路由 | Key | 配额 |
| ---- | --- | ---- |
| `/health` | 客户端 IP + path | 10 秒 2 次 |
| `/user/{id}` | 用户 ID + path | 5 秒 2 次 |
| `/user/email` | 用户 ID + path | 10 秒 3 次 |
| `/user/email/pre` | 用户 ID + path | 10 秒 2 次 |
| `/user/random` | 用户 ID + path | 5 秒 2 次 |
| `/foo` | 用户 ID + path | 5 秒 2 次 |

当前限流状态保存在进程内存中，适合单实例 API 部署。若服务水平扩展到多个实例，需要接入 Redis 等共享后端，才能让所有实例共享同一份配额。

## 开发指南

### 系统要求

- Rust 1.89+（通过 [rustup](https://rustup.rs/) 安装）

> 国内用户推荐使用 [rsproxy](https://rsproxy.cn/)

- MySQL 5.7.20+
- Redis 5.0+
- `sqlx-cli`（MySQL 支持）：`cargo install sqlx-cli --features mysql`

### 快速开始

1. **克隆仓库**

   ```bash
   git clone https://github.com/TuMengStudios/axum-best.git
   cd axum-best
   ```

2. **配置应用**

   编辑 `etc/config.toml`，设置 MySQL DSN、Redis 地址、HTTP 监听地址、日志配置以及微信小程序凭证。

3. **创建数据库并运行迁移**

   ```bash
   # 确保 MySQL 正在运行
   sqlx migrate run
   ```

4. **启动开发服务器**

   ```bash
   cargo run
   ```

   默认监听 `http://0.0.0.0:8080`。

### 开发命令

```bash
# 运行应用程序
cargo run

# 运行测试
cargo test

# 使用 nextest 运行测试（需安装 cargo-nextest）
cargo nextest run

# 运行基准测试
cargo bench

# 开发时热重载（需安装 cargo-watch）
cargo watch -x run

# 格式化代码
cargo fmt

# 检查代码质量
cargo clippy

# 构建发布版本
cargo build --release
```

### 代码组织

- **模型**：在 `src/models/` 中定义数据结构
- **类型**：在 `src/types/` 中定义带校验的请求/响应 DTO
- **处理器**：在 `src/handlers/` 中添加新的 HTTP 端点
- **服务**：在 `src/services/` 中实现业务逻辑
- **仓储**：在 `src/repos/` 中定义数据访问 trait，在 `src/data/` 中提供实现
- **路由**：在 `src/routers/` 中注册新路由
- **限流**：在 `src/routers/` 中使用 `RateLimitLayer::with_quota` 或 `RateLimitLayer::with_login_quota` 配置 route 级别的限流

### 添加新功能

1. 在 `src/models/` 中定义数据模型
2. 在 `src/types/` 中定义请求/响应类型
3. 在 `src/repos/` 中定义仓储 trait，并在 `src/data/` 中实现
4. 在 `src/services/` 中实现业务逻辑
5. 在 `src/handlers/` 中添加 HTTP 处理器
6. 在 `src/routers/` 中注册路由

## 配置

配置从 TOML 文件加载（默认：`etc/config.toml`）。配置文件路径可通过命令行参数覆盖：

```bash
cargo run -- --conf etc/config.toml
```

`etc/config.toml` 主要配置项：

- `[log]` — 日志级别、轮转策略、目录、文件名、JSON/文本格式
- `[http]` — 监听地址与端口（默认 `0.0.0.0:8080`）
- `[mysql]` — DSN、连接池大小、慢查询阈值
- `[redis]` — Redis 地址与连接池配置
- `[wechat]` — 微信小程序 `appid` 与 `secret`
- `[jwt]` — JWT 签名密钥 `secret` 与令牌有效期 `expiration_secs`；也可以使用环境变量 `JWT_SECRET` 和 `JWT_EXPIRATION_SECS` 覆盖配置文件中的值。生产环境建议通过 `JWT_SECRET` 配置密钥，避免写入配置文件
- 限流配置 — 当前 route 级别的限流规则配置在 `src/routers/mod.rs` 中，暂不从 TOML 加载

`.env` 文件供 `sqlx-cli` 和 SQLx 编译期查询检查使用（`DATABASE_URL=mysql://root:123456@localhost:3306/axum_best`）。

## 示例

`examples/` 目录包含多个可运行的小程序：

- `db_demo` — 加载配置并测试 MySQL 连接
- `user_crud_demo` — 演示用户仓储的 CRUD 用法
- `primitive_demo` — 演示 `Primitive` 枚举
- `test_valid_code` — 测试数字验证码生成
- `test_random_uniqueness` — 测试随机 `UserInfo` 生成的唯一性

运行示例：

```bash
cargo run --example db_demo
```

## 基准测试

`benches/` 目录包含 Criterion 基准测试：

- `utils_benchmark` — 对 `gen_valid_code` 和 `file_digest` 进行基准测试
- `models_user` — 对 `UserInfo::random()` 进行基准测试

运行所有基准测试：

```bash
cargo bench
```

## 部署

### Docker 部署

项目包含 Docker 支持，容器内应用监听 `8080` 端口。

1. **使用 Docker Compose 构建和运行**

   ```bash
   docker-compose up --build
   ```

2. **手动构建 Docker 镜像**

   ```bash
   docker build -t axum-best .
   docker run -p 8080:8080 axum-best
   ```

### 生产部署

1. **构建优化二进制文件**

   ```bash
   cargo build --release
   ```

2. **设置反向代理**（推荐：nginx 或 Caddy）
3. **配置进程管理器**（推荐：systemd 或 supervisord）
4. **设置监控和日志**

### 健康检查

应用包含健康检查端点：

- `GET /health` — 返回 `ok`

Docker Compose 与 Dockerfile 均配置了针对 `http://localhost:8080/health` 的健康检查。

## 性能考虑

- 为数据库与 Redis 操作使用连接池
- 为频繁访问的数据实现缓存
- 启用 HTTP 响应压缩
- 在生产环境中使用适当的日志级别
- 监控内存使用和连接数

## 贡献指南

1. Fork 仓库
2. 创建功能分支
3. 进行更改
4. 添加测试并确保通过
5. 提交拉取请求

## 许可证

本项目采用 MIT 许可证。
