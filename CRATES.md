# Crate Manifest (Dependency Matrix)

> [简体中文](CRATES_ZH.md)

> Derived from `Cargo.toml`. Includes crate name, version, purpose, import
> status, and repository link. Crates are grouped by architectural layer so
> the role of every entry is easy to audit.

> **Import status** legend:
>
> - **ws**: workspace dependency (`[workspace.dependencies]`)
> - **dep**: direct application-crate dependency (`[dependencies]`)
> - **dev**: development dependency (`[dev-dependencies]`)
> - **mem**: workspace member crate (`[workspace] members`)
> - **candidate / 未引入**: tracked but not yet pulled in

---

## 1. Web Framework / Runtime

| crate                                                                                   | version | purpose                                                              | import status |
| --------------------------------------------------------------------------------------- | ------- | -------------------------------------------------------------------- | ------------- |
| [`axum`](https://github.com/tokio-rs/axum)                                               | 0.8.6   | Modular HTTP framework on Tokio (features: `default`, `macros`, `query`) | dep       |
| [`tokio`](https://github.com/tokio-rs/tokio)                                             | 1.47.1  | Async runtime; `main` uses `full`; workspace restricted to `rt`/`sync`/`time` | ws + dep |
| [`tower`](https://github.com/tower-rs/tower)                                             | 0.5.2   | Service / middleware abstraction (features: `timeout`, `tracing`, `util`) | dep      |
| [`tower-http`](https://github.com/tower-rs/tower-http)                                   | 0.6.6   | Tower-ecosystem HTTP middleware: trace, compression, CORS, limit, timeout  | dep     |
| [`async-trait`](https://github.com/dtolnay/async-trait)                                 | 0.1     | Async functions in traits                                            | dep           |

## 2. Datastores (MySQL / Redis / External HTTP)

| crate                                                                               | version           | purpose                                                                                                                  | import status       |
| ----------------------------------------------------------------------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------ | ------------------- |
| [`sqlx`](https://github.com/launchbadge/sqlx)                                        | 0.8.6             | Async SQL with compile-time checked queries (features: `sqlx-mysql`, `mysql`, `chrono`, `runtime-tokio`)                  | dep                 |
| [`redis`](https://github.com/redis-rs/redis-rs)                                      | 0.32.7            | Redis client (features: `tokio-comp`, `json`)                                                                            | dep                 |
| [`bb8-redis`](https://github.com/davidgorges/bb8-redis)                              | 0.24              | `bb8` connection-pool adapter for Redis                                                                                  | dep                 |
| [`reqwest`](https://github.com/seanmonstar/reqwest)                                  | 0.12.23           | Async HTTP client (features: `json`, `rustls-tls`; native TLS disabled)                                                  | dep                 |
| [`rdkafka`](https://github.com/fede1024/rust-rdkafka)                                | 0.39 (candidate)  | Async Kafka client backed by `librdkafka`; tracked for evaluation when a message queue is needed                          | candidate / 未引入   |
| [`oauth2`](https://github.com/ramosbugs/oauth2-rs)                                   | 5.0 (candidate)   | Typed OAuth2 client (RFC 6749 / 7662 / 7009), supports PKCE and Device Authorization Flow; default feature targets `reqwest` | candidate / 未引入 |

## 3. Auth / Security / Validation / Rate Limiting

| crate                                                                               | version | purpose                                                  | import status |
| ----------------------------------------------------------------------------------- | ------- | -------------------------------------------------------- | ------------- |
| [`jsonwebtoken`](https://github.com/Keats/jsonwebtoken)                             | 9.3.1   | JWT signing and verification (HS256)                      | dep           |
| [`governor`](https://github.com/boinkor-net/governor)                                | 0.10    | GCRA rate limiter; per-route quotas                       | dep           |
| [`validator`](https://github.com/Keats/validator)                                    | 0.20.0  | `derive`-based input validation                           | dep           |
| [`regex`](https://github.com/rust-lang/regex)                                        | 1       | Path allowlist regular expressions                        | dep           |
| [`sha2`](https://github.com/RustCrypto/hashes)                                       | 0.10.9  | SHA-256 / 384 / 512 digests                               | dep           |
| [`sha1`](https://github.com/RustCrypto/hashes)                                       | 0.10.6  | SHA-1 digest                                              | dep           |
| [`md-5`](https://github.com/RustCrypto/hashes)                                       | 0.10.6  | MD5 digest                                                | dep           |
| [`digest`](https://github.com/RustCrypto/traits)                                     | 0.10.7  | Digest trait abstraction                                  | dep           |
| [`rand`](https://github.com/rust-random/rand)                                        | 0.9.2   | Random number generation (e.g. verification codes)        | dep           |

## 4. Observability (Logging / Tracing / Metrics)

| crate                                                                               | version | purpose                                                  | import status |
| ----------------------------------------------------------------------------------- | ------- | -------------------------------------------------------- | ------------- |
| [`tracing`](https://github.com/tokio-rs/tracing)                                    | 0.1.41  | Structured tracing (feature: `attributes`)               | ws + dep      |
| [`tracing-subscriber`](https://github.com/tokio-rs/tracing)                         | 0.3.20  | Subscriber + formatter (features: `chrono`, `json`)      | dep           |
| [`tracing-appender`](https://github.com/tokio-rs/tracing)                           | 0.2.3   | File appender with log rotation                          | dep           |
| [`tracing-opentelemetry`](https://github.com/tokio-rs/tracing-opentelemetry)        | 0.25.0  | Bridge from `tracing` to OpenTelemetry                   | dep           |
| [`opentelemetry`](https://github.com/open-telemetry/opentelemetry-rust)             | 0.24.0  | OTEL core API (feature: `trace`)                         | dep           |
| [`opentelemetry_sdk`](https://github.com/open-telemetry/opentelemetry-rust)         | 0.24.1  | OTEL SDK (features: `rt-tokio`, `trace`)                 | dep           |
| [`opentelemetry-otlp`](https://github.com/open-telemetry/opentelemetry-rust)         | 0.17.0  | OTLP gRPC exporter (features: `grpc-tonic`, `trace`)      | dep           |
| [`axum-prometheus`](https://github.com/ptrslr/axum-prometheus)                      | 0.10.1  | Prometheus metrics endpoint for Axum                    | dep           |
| [`log`](https://github.com/rust-lang/log)                                            | 0.4.28  | Classic logging facade (feature: `serde`)                | dep           |
| [`chrono`](https://github.com/chronotope/chrono)                                     | 0.4.38  | Date/time types (feature: `serde`)                       | dep           |

## 5. Configuration / CLI / Error Handling

| crate                                                                               | version  | purpose                                                                              | import status |
| ----------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------ | ------------- |
| [`clap`](https://github.com/clap-rs/clap)                                           | 4.4.6    | Command-line argument parser (features: `derive`, `wrap_help`, `env`, `string`, `default`) | dep       |
| [`toml`](https://github.com/toml-rs/toml)                                            | 0.9.7    | Parser for `etc/config.toml`                                                         | dep           |
| [`anyhow`](https://github.com/dtolnay/anyhow)                                        | 1.0.100  | Startup-time error wrapping                                                          | dep           |
| [`thiserror`](https://github.com/dtolnay/thiserror)                                  | 2        | Derives for library error types such as `AppError`                                   | ws           |
| [`smart-default`](https://github.com/idanarye/rust-smart-default)                    | 0.7.1    | `Default` value macro for config structs                                             | dep           |
| [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs)                 | 1.5.0    | Process-wide statics with spin-based init                                            | dep           |
| [`derivative`](https://github.com/mcarton/rust-derivative)                           | 2.1.3    | Custom `#[derive]` overrides (e.g. `#[derivative(Default)]`)                        | dep           |
| [`human-panic`](https://github.com/rust-cli/human-panic)                             | 2.0.3    | Human-readable panic output (feature: `color`)                                       | dep           |
| [`local-ip-address`](https://github.com/ajmwagar/local-ip-address)                   | 0.6.5    | Lookup local IP addresses                                                            | dep           |

## 6. Serialization / API Documentation

| crate                                                                               | version  | purpose                                                  | import status |
| ----------------------------------------------------------------------------------- | -------- | -------------------------------------------------------- | ------------- |
| [`serde`](https://github.com/serde-rs/serde)                                        | 1.0.228  | Serialize / deserialize framework (feature: `derive`)    | dep           |
| [`serde_json`](https://github.com/serde-rs/json)                                    | 1.0.145  | JSON encoder / decoder                                   | dep           |
| [`utoipa`](https://github.com/juhaku/utoipa)                                         | 5        | OpenAPI attribute macros (feature: `axum_extras`)        | dep           |
| [`utoipa-swagger-ui`](https://github.com/juhaku/utoipa)                             | 9        | Swagger UI integration (feature: `axum`)                | dep           |

## 7. Development / Testing / Benchmarking

| crate                                                                               | version | purpose                                          | import status |
| ----------------------------------------------------------------------------------- | ------- | ------------------------------------------------ | ------------- |
| [`criterion`](https://github.com/bheisler/criterion.rs)                             | 0.7.0   | Benchmarking framework (feature: `html_reports`) | dev           |
| [`manifest-dir-macros`](https://github.com/2b-t/manifest-dir-macros)                | 0.1.18  | Read `CARGO_MANIFEST_DIR` inside tests           | dev           |

## 8. Workspace Member Crates (`[workspace] members`)

| crate              | type                  | key dependencies                                     | import status | notes                                                                                                                                                                                  |
| ------------------ | --------------------- | ---------------------------------------------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/gitver`    | proc-macro            | `proc-macro2`, `quote`                               | mem           | Injects git version metadata at compile time; the Dockerfile keeps `.git/` so the proc-macro has data to read during build.                                                              |
| `crates/nextid`    | lib + criterion bench | `bson`, `hex`                                        | mem           | Generates snowflake-style distributed IDs inspired by the `bson` ObjectId; ships a benchmark suite.                                                                                       |
| `crates/noworkers` | lib                   | `thiserror`, `tokio`, `tracing` (all workspace deps) | mem + dep     | Project-owned facade over a bounded background-task pool (see `[worker].capacity` and `spawn_timeout_secs`).                                                                            |

---

## 9. Import Status Summary (Quick Lookup)

| status                     | count | notes                                    |
| -------------------------- | ----- | ---------------------------------------- |
| workspace deps (`ws`)      | 3     | `thiserror`, `tokio`, `tracing`          |
| direct deps (`dep`)        | ~40   | see sections 1–7                         |
| dev deps (`dev`)           | 2     | `criterion`, `manifest-dir-macros`       |
| workspace members (`mem`)  | 3     | `gitver`, `nextid`, `noworkers`          |
| candidate (未引入)         | 2     | `rdkafka`, `oauth2`                      |

> Dependency auditing: `deny.toml` is enforced via `cargo deny check` in pre-commit.

---

## 10. Notes

- Please keep this manifest in sync when upgrading or adding crates.
