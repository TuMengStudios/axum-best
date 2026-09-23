# Crate 清单（依赖矩阵）

> [english doc](CRATES.md)

> 依据 `Cargo.toml` 归纳，包含 crate 名、版本、作用、引入状态与仓库链接。
> 按架构分层分组，便于快速核对每个 crate 的归属。

> **引入状态** 图例：
>
> - **ws**：workspace 依赖（`[workspace.dependencies]`）
> - **dep**：应用 crate 直接依赖（`[dependencies]`）
> - **dev**：开发依赖（`[dev-dependencies]`）
> - **mem**：workspace 子 crate 成员（`[workspace] members`）
> - **candidate / 未引入**：已记录但暂未正式引入

---

## 1. Web 框架 / 运行时

| crate                                                                                   | 版本      | 作用                                                                       | 引入状态 |
| --------------------------------------------------------------------------------------- | --------- | -------------------------------------------------------------------------- | -------- |
| [`axum`](https://github.com/tokio-rs/axum)                                               | 0.8.6     | 基于 Tokio 的模块化 HTTP 框架（特性：`default`、`macros`、`query`）        | dep      |
| [`tokio`](https://github.com/tokio-rs/tokio)                                             | 1.47.1    | 异步运行时；`main` 启用 `full`，workspace 限制为 `rt`/`sync`/`time`         | ws + dep |
| [`tower`](https://github.com/tower-rs/tower)                                             | 0.5.2     | 服务 / 中间件抽象（特性：`timeout`、`tracing`、`util`）                    | dep      |
| [`tower-http`](https://github.com/tower-rs/tower-http)                                   | 0.6.6     | Tower 生态 HTTP 中间件：trace、压缩、CORS、limit、timeout                  | dep      |
| [`async-trait`](https://github.com/dtolnay/async-trait)                                 | 0.1       | 在 trait 中提供 async fn 语法                                              | dep      |

## 2. 数据存储（MySQL / Redis / 外部 HTTP）

| crate                                                                               | 版本                | 作用                                                                                                                  | 引入状态          |
| ----------------------------------------------------------------------------------- | ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------- |
| [`sqlx`](https://github.com/launchbadge/sqlx)                                        | 0.8.6               | 编译期校验的异步 SQL（特性：`sqlx-mysql`、`mysql`、`chrono`、`runtime-tokio`）                                       | dep               |
| [`redis`](https://github.com/redis-rs/redis-rs)                                      | 0.32.7              | Redis 客户端（特性：`tokio-comp`、`json`）                                                                            | dep               |
| [`bb8-redis`](https://github.com/davidgorges/bb8-redis)                              | 0.24                | `bb8` 通用连接池的 Redis 适配                                                                                         | dep               |
| [`reqwest`](https://github.com/seanmonstar/reqwest)                                  | 0.12.23             | 异步 HTTP 客户端（特性：`json`、`rustls-tls`，关闭 native TLS）                                                       | dep               |
| [`rdkafka`](https://github.com/fede1024/rust-rdkafka)                                | 0.39（候选）        | 基于 `librdkafka` 的异步 Kafka 客户端；记录在案，待需要消息队列时引入                                                | candidate / 未引入 |
| [`oauth2`](https://github.com/ramosbugs/oauth2-rs)                                   | 5.0（候选）         | 类型化 OAuth2 客户端（RFC 6749 / 7662 / 7009），支持 PKCE 与 Device Flow；默认特性对应 `reqwest`                       | candidate / 未引入 |

## 3. 鉴权 / 安全 / 校验 / 限流

| crate                                                                               | 版本      | 作用                                            | 引入状态 |
| ----------------------------------------------------------------------------------- | --------- | ----------------------------------------------- | -------- |
| [`jsonwebtoken`](https://github.com/Keats/jsonwebtoken)                             | 9.3.1     | JWT 签发与校验（HS256）                         | dep      |
| [`governor`](https://github.com/boinkor-net/governor)                                | 0.10      | GCRA 限流器，支持按路由配额                     | dep      |
| [`validator`](https://github.com/Keats/validator)                                    | 0.20.0    | `derive` 风格的输入校验                         | dep      |
| [`regex`](https://github.com/rust-lang/regex)                                        | 1         | 路径白名单正则                                  | dep      |
| [`sha2`](https://github.com/RustCrypto/hashes)                                       | 0.10.9    | SHA-256 / 384 / 512 摘要                        | dep      |
| [`sha1`](https://github.com/RustCrypto/hashes)                                       | 0.10.6    | SHA-1 摘要                                      | dep      |
| [`md-5`](https://github.com/RustCrypto/hashes)                                       | 0.10.6    | MD5 摘要                                        | dep      |
| [`digest`](https://github.com/RustCrypto/traits)                                     | 0.10.7    | 摘要 trait 抽象                                 | dep      |
| [`rand`](https://github.com/rust-random/rand)                                        | 0.9.2     | 随机数生成（如验证码）                          | dep      |

## 4. 可观测性（日志 / Trace / Metrics）

| crate                                                                               | 版本      | 作用                                            | 引入状态 |
| ----------------------------------------------------------------------------------- | --------- | ----------------------------------------------- | -------- |
| [`tracing`](https://github.com/tokio-rs/tracing)                                    | 0.1.41    | 结构化追踪（特性：`attributes`）                | ws + dep |
| [`tracing-subscriber`](https://github.com/tokio-rs/tracing)                         | 0.3.20    | 日志订阅与格式化（特性：`chrono`、`json`）     | dep      |
| [`tracing-appender`](https://github.com/tokio-rs/tracing)                           | 0.2.3     | 文件 appender，支持日志轮转                    | dep      |
| [`tracing-opentelemetry`](https://github.com/tokio-rs/tracing-opentelemetry)        | 0.25.0    | `tracing` → OpenTelemetry 桥接                  | dep      |
| [`opentelemetry`](https://github.com/open-telemetry/opentelemetry-rust)             | 0.24.0    | OTEL 核心 API（特性：`trace`）                  | dep      |
| [`opentelemetry_sdk`](https://github.com/open-telemetry/opentelemetry-rust)         | 0.24.1    | OTEL SDK（特性：`rt-tokio`、`trace`）           | dep      |
| [`opentelemetry-otlp`](https://github.com/open-telemetry/opentelemetry-rust)         | 0.17.0    | OTLP gRPC 导出器（特性：`grpc-tonic`、`trace`） | dep      |
| [`axum-prometheus`](https://github.com/ptrslr/axum-prometheus)                      | 0.10.1    | Axum 的 Prometheus 指标端点                     | dep      |
| [`log`](https://github.com/rust-lang/log)                                            | 0.4.28    | 经典日志门面（特性：`serde`）                   | dep      |
| [`chrono`](https://github.com/chronotope/chrono)                                     | 0.4.38    | 时间字段（特性：`serde`）                       | dep      |

## 5. 配置 / CLI / 错误处理

| crate                                                                               | 版本       | 作用                                                                            | 引入状态 |
| ----------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------- | -------- |
| [`clap`](https://github.com/clap-rs/clap)                                           | 4.4.6      | 命令行参数解析（特性：`derive`、`wrap_help`、`env`、`string`、`default`）        | dep      |
| [`toml`](https://github.com/toml-rs/toml)                                            | 0.9.7      | 解析 `etc/config.toml`                                                          | dep      |
| [`anyhow`](https://github.com/dtolnay/anyhow)                                        | 1.0.100    | 启动期错误包装                                                                  | dep      |
| [`thiserror`](https://github.com/dtolnay/thiserror)                                  | 2          | 库内错误类型派生（如 `AppError`）                                               | ws       |
| [`smart-default`](https://github.com/idanarye/rust-smart-default)                    | 0.7.1      | 配置结构 `Default` 值宏                                                          | dep      |
| [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs)                 | 1.5.0      | 进程级静态（spin 初始化）                                                        | dep      |
| [`derivative`](https://github.com/mcarton/rust-derivative)                           | 2.1.3      | 自定义 `#[derive]` 行为（如 `#[derivative(Default)]`）                          | dep      |
| [`human-panic`](https://github.com/rust-cli/human-panic)                             | 2.0.3      | 人性化 panic 输出（特性：`color`）                                              | dep      |
| [`local-ip-address`](https://github.com/ajmwagar/local-ip-address)                   | 0.6.5      | 获取本机 IP                                                                     | dep      |

## 6. 序列化 / API 文档

| crate                                                                               | 版本       | 作用                                                  | 引入状态 |
| ----------------------------------------------------------------------------------- | ---------- | ----------------------------------------------------- | -------- |
| [`serde`](https://github.com/serde-rs/serde)                                        | 1.0.228    | 序列化 / 反序列化框架（特性：`derive`）               | dep      |
| [`serde_json`](https://github.com/serde-rs/json)                                    | 1.0.145    | JSON 编解码                                           | dep      |
| [`utoipa`](https://github.com/juhaku/utoipa)                                         | 5          | OpenAPI 注解宏（特性：`axum_extras`）                 | dep      |
| [`utoipa-swagger-ui`](https://github.com/juhaku/utoipa)                             | 9          | Swagger UI 集成（特性：`axum`）                       | dep      |

## 7. 开发 / 测试 / 基准

| crate                                                                               | 版本      | 作用                                          | 引入状态 |
| ----------------------------------------------------------------------------------- | --------- | --------------------------------------------- | -------- |
| [`criterion`](https://github.com/bheisler/criterion.rs)                             | 0.7.0     | 基准测试框架（特性：`html_reports`）          | dev      |
| [`manifest-dir-macros`](https://github.com/2b-t/manifest-dir-macros)                | 0.1.18    | 在测试中读取 `CARGO_MANIFEST_DIR`             | dev      |

## 8. Workspace 子 crate（`[workspace] members`）

| crate              | 类型                   | 关键依赖                                              | 引入状态 | 说明                                                                                                              |
| ------------------ | ---------------------- | ------------------------------------------------------ | -------- | ----------------------------------------------------------------------------------------------------------------- |
| `crates/gitver`    | proc-macro             | `proc-macro2`、`quote`                                 | mem      | 编译期注入 git 版本元数据；Dockerfile 保留 `.git/` 以便 proc-macro 构建时可读                                      |
| `crates/nextid`    | lib + criterion bench  | `bson`、`hex`                                          | mem      | 基于 `bson` ObjectId 思路生成类雪花分布式 ID；提供基准测试                                                        |
| `crates/noworkers` | lib                    | `thiserror`、`tokio`、`tracing`（均为 workspace 依赖） | mem + dep| 项目自有后台任务门面，封装有界并发池（见 `[worker].capacity` 与 `spawn_timeout_secs`）                              |

---

## 9. 引入状态一览（速查）

| 状态                     | 数量 | 说明                                    |
| ------------------------ | ---- | --------------------------------------- |
| workspace 依赖（`ws`）   | 3    | `thiserror`、`tokio`、`tracing`         |
| 直接依赖（`dep`）        | ~40  | 参见 §1–§7                              |
| 开发依赖（`dev`）        | 2    | `criterion`、`manifest-dir-macros`      |
| workspace 子 crate（`mem`）| 3  | `gitver`、`nextid`、`noworkers`         |
| 候选（未引入）           | 2    | `rdkafka`、`oauth2`                     |

> 依赖审计：`deny.toml` 由 pre-commit 中的 `cargo deny check` 强制执行。

---

## 10. 备注

- 升级或新增 crate 后请同步更新本表。
