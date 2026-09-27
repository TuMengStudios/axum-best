# axum best

* [english doc](README.md)
* [开发工具](dev.md)
* [tech stack / crates](CRATES.md)
* [技术栈 / crates](CRATES_ZH.md)

> 为最佳的 axum 开发体验而设计

## 项目概述

axum-best 是一个 Rust Web 项目模板，演示了用于构建可扩展 HTTP 服务的分层架构。
默认集成关系型存储、缓存、结构化日志、输入校验、可配置中间件以及后台任务池。
提供合理开箱即用的默认项、多阶段 Dockerfile、可运行示例与基准测试，可直接 fork
作为业务项目的起点。

底层依赖（版本、仓库、引入状态）见 [CRATES.md](CRATES.md)。

## 主要特性

- 清晰的分层架构：`handlers → services → repos（trait）← data（实现）`，依赖单向
- 提供 JSON / Query / Path / Form 输入校验提取器
- 内置中间件：请求 ID 链路追踪、CORS、请求解压缩、超时、响应压缩（按路径排除）、按路由限流
- 结构化 JSON 日志，支持滚动轮转
- 健康检查、Prometheus 指标、OpenAPI / Swagger UI
- 有界并发后台任务池
- 数据库 schema 走迁移脚本，placeholder 即校验
- 多阶段 Dockerfile 与 Docker Compose，含健康检查
- Criterion 基准测试与可运行示例

## 项目结构

```plain
src/
├── app/              # 应用上下文：依赖组装与生命周期
├── auth/             # JWT Claims 与认证中间件
├── conf/             # TOML 配置加载
├── core/             # 核心类型：AppState、AppResult/AppError
├── data/             # 仓储实现 + 连接/池管理
├── docs/             # OpenAPI / utoipa 注册
├── errors/           # 预定义应用错误
├── handlers/         # HTTP 处理器
├── logx/             # 结构化日志初始化
├── models/           # 数据模型与实体（如 UserInfo）
├── observability/   # OpenTelemetry / 指标
├── repos/            # 仓储接口（数据访问契约）
├── routers/          # 路由定义与中间件栈
├── services/         # 业务逻辑层
├── transport/        # HTTP 服务器与中间件辅助
│   └── middleware/   # 自定义中间件
├── types/            # 带校验的请求/响应 DTO
└── utils/            # 工具函数（哈希、验证码等）

benches/             # Criterion 基准测试套件
examples/            # 可运行示例程序
etc/                 # 配置文件
migrations/          # 数据库迁移脚本
```

### 架构层次

1. **传输层** (`src/transport/`)：TCP 监听、HTTP 服务器与可复用中间件
2. **处理器层** (`src/handlers/`)：HTTP 端点处理器，解析输入并调用服务层
3. **服务层** (`src/services/`)：业务逻辑实现，service 持有启动时注入的仓储接口
4. **仓储层** (`src/repos/`)：数据访问契约
5. **数据层** (`src/data/`)：基于存储后端的具体实现与连接池管理

依赖单向：`handlers → services → repos（trait）← data（实现）`；由 `app::AppContext`
在启动时将具体实现注入 service 并装入 `AppState`。

## 快速开始

### 系统要求

- Rust 1.94+（[rustup](https://rustup.rs/)；国内推荐 [rsproxy](https://rsproxy.cn/)）
- MySQL 5.7.20+
- Redis 5.0+
- MySQL 客户端（`mysql`，用于执行迁移）

### 启动

```bash
# 1. 克隆
git clone https://github.com/TuMengStudios/axum-best.git
cd axum-best

# 2. 配置（按需修改 etc/config.toml 中的 DSN / Redis / 微信 / 等）
cp .env.example .env  # 可选：本地环境变量覆盖

# 3. 建库并按时间顺序执行迁移
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS axum_best DEFAULT CHARACTER SET utf8mb4"
for f in $(printf '%s\n' migrations/*.sql | sort -t_ -k1,1n); do
  mysql -u root -p axum_best < "$f"
done

# 4. 启动开发服务器（默认 0.0.0.0:8080）
cargo run
```

### 常用命令

```bash
cargo run                # 运行应用
cargo test               # 测试
cargo nextest run        # nextest 测试
cargo bench              # 基准测试
cargo watch -x run       # 热重载
cargo fmt                # 格式化
cargo clippy             # 静态检查
cargo build --release    # release 构建
```

## 配置

从 TOML 文件加载（默认 `etc/config.toml`），可通过命令行覆盖：

```bash
cargo run -- --conf etc/config.toml
```

敏感配置（如 JWT 签名密钥）生产环境建议使用环境变量注入，避免写入配置文件。
完整段位与环境变量覆盖规则见 `etc/config.toml` 顶部注释。

## 部署

```bash
# Docker Compose（host 网络，暴露 8080）
docker compose up --build

# 或手动构建
docker build -t axum-best .
docker run -p 8080:8080 axum-best
```

Docker Compose 与 Dockerfile 均配置了针对 `http://localhost:8080/health` 的健康检查。

生产部署建议在前面挂反向代理（nginx / Caddy），并用 systemd / supervisord 管理进程。

## 许可证

MIT
