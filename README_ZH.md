# axum best

[english doc](README.md)
> 为最佳的 axum 开发体验而设计

## 项目概述

axum-best 是一个使用 Rust 和 Axum Web 框架构建的高性能 Web 应用程序框架。它提供了一个强大的基础，用于构建可扩展和可维护的 Web 服务，并遵循最佳实践。

### 主要特性

- **高性能**: 基于 Tokio 和 Axum 构建，实现最大吞吐量
- **清晰架构**: 遵循领域驱动设计原则的结构化代码库
- **数据库支持**: 集成 MySQL 和 SQLx，实现类型安全的数据库操作
- **缓存层**: 基于 Redis 的缓存，提升性能
- **全面错误处理**: 贯穿应用程序的结构化错误处理
- **配置管理**: 具有环境特定设置的灵活配置系统
- **Docker 支持**: 使用 Docker 和 Docker Compose 进行容器化部署
- **基准测试**: 内置性能测试工具

## 项目结构

```plain
src/
├── conf/           # 配置管理
├── core/           # 核心应用逻辑和状态管理
├── data/           # 数据访问层 (MySQL, Redis等)
├── errors/         # 错误类型和处理
├── handlers/       # HTTP 请求处理器
├── logx/           # 日志工具
├── models/         # 数据模型和实体
├── repos/          # 仓储模式实现
├── routers/        # 路由定义
├── services/       # 业务逻辑层
├── srvCtx/         # 服务上下文和依赖注入
├── transport/      # HTTP 传输层和中间件
├── types/          # 自定义类型定义
└── utils/          # 工具函数

benches/           # 基准测试
examples/          # 示例使用代码
migrations/        # 数据库迁移脚本
etc/               # 配置文件
```

### 架构层次

1. **传输层** (`src/transport/`): HTTP 服务器设置、中间件和请求/响应处理
2. **处理器层** (`src/handlers/`): HTTP 端点处理器，协调服务调用
3. **服务层** (`src/services/`): 业务逻辑实现
4. **仓储层** (`src/repos/`): 数据访问抽象
5. **数据层** (`src/data/`): 数据库和缓存实现

## 开发指南

### 系统要求

- Rust 1.89+ (通过 [rustup](https://rustup.rs/) 安装)

> 国内用户推荐使用 [rsproxy](https://rsproxy.cn/)

- MySQL 5.7.20+
- Redis 5.0+

### 快速开始

1. **克隆仓库**

   ```bash
   git clone https://github.com/TuMengStudios/axum-best.git
   cd axum-best
   ```

2. **设置环境变量**

   创建一个包含数据库和 Redis 配置的 `.env` 文件。

3. **运行数据库迁移**

   ```bash
   # 确保 MySQL 正在运行
   sqlx migrate run
   ```

4. **启动开发服务器**

   ```bash
   cargo run
   ```

### 开发命令

```bash
# 运行应用程序
cargo run

# 运行测试
cargo test

# 运行基准测试
cargo bench

# 格式化代码
cargo fmt

# 检查代码质量
cargo clippy

# 构建发布版本
cargo build --release
```

### 代码组织

- **模型**: 在 `src/models/` 中定义数据结构
- **处理器**: 在 `src/handlers/` 中添加新的 HTTP 端点
- **服务**: 在 `src/services/` 中实现业务逻辑
- **仓储**: 在 `src/repos/` 中添加数据访问方法
- **路由**: 在 `src/routers/` 中注册新路由

### 添加新功能

1. 在 `src/models/` 中定义数据模型
2. 在 `src/repos/` 中创建仓储方法
3. 在 `src/services/` 中实现业务逻辑
4. 在 `src/handlers/` 中添加 HTTP 处理器
5. 在 `src/routers/` 中注册路由

## 部署

### Docker 部署

项目包含 Docker 支持，便于部署：

1. **使用 Docker Compose 构建和运行**

   ```bash
   docker-compose up --build
   ```

2. **手动构建 Docker 镜像**

   ```bash
   docker build -t axum-best .
   docker run -p 3000:3000 axum-best
   ```

### 环境配置

创建一个包含以下变量的 `.env` 文件：

```env
DATABASE_URL=mysql://username:password@localhost:3306/database_name
```

### 生产部署

1. **构建优化二进制文件**

   ```bash
   cargo build --release
   ```

2. **设置反向代理** (推荐: nginx 或 Caddy)
3. **配置进程管理器** (推荐: systemd 或 supervisord)
4. **设置监控和日志**

### 健康检查

应用程序包含健康检查端点：

- `GET /health` - 基本应用程序健康状态

## 配置

配置通过以下方式管理：

- `etc/config.toml` - 主配置文件
- 环境变量 (覆盖配置文件设置)
- 命令行参数

## 性能考虑

- 为数据库操作使用连接池
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
