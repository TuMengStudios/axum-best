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
- **Redis 缓存**：通过 `r2d2` 连接池（基于 `redis` crate）实现缓存与会话类存储
- **请求校验**：使用 `validator` 和 `axum-valid` 进行输入校验
- **中间件栈**：请求 ID 链路追踪、CORS、请求/响应压缩、解压缩和超时控制
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
├── data/           # MySQL 与 Redis 连接池实现
├── errors/         # 预定义应用错误
├── handlers/       # HTTP 请求处理器
├── logx/           # 结构化日志初始化
├── models/         # 数据模型与实体（如 UserInfo）
├── repos/          # 仓储模式 / 基于 SQLx 的原始数据访问
├── routers/        # 路由定义与中间件栈
├── services/       # 业务逻辑层
├── srvCtx/         # 服务上下文：构建状态并启动 HTTP 服务
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
3. **服务层** (`src/services/`)：业务逻辑实现
4. **仓储层** (`src/repos/`)：基于 SQLx 的数据访问抽象
5. **数据层** (`src/data/`)：数据库与缓存的连接池管理

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
- **仓储**：在 `src/repos/` 中添加数据访问方法
- **路由**：在 `src/routers/` 中注册新路由

### 添加新功能

1. 在 `src/models/` 中定义数据模型
2. 在 `src/types/` 中定义请求/响应类型
3. 在 `src/repos/` 中创建仓储方法
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
