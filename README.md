# axum best

> For the optimal axum development experience

* [简体中文](README_ZH.md)
* [development tools](dev.md)
* [tech stack / crates](CRATES.md)
* [技术栈 / crates](CRATES_ZH.md)

## Project Overview

axum-best is a Rust web project template that demonstrates a layered architecture
for building scalable HTTP services with relational storage, caching,
structured logging, request validation, and a configurable middleware stack.
It ships with sensible defaults, ready-to-run Docker support, and an opinionated
baseline you can fork and grow.

For the underlying dependencies (versions, repositories, import status), see
[CRATES.md](CRATES.md).

## Key Features

- Layered architecture with one-way dependencies between handlers, services,
  repositories, and data access
- Request validation extractors for JSON, query, path, and form inputs
- Built-in middleware: request-id tracing, CORS, request decompression,
  per-request timeout, response compression (with per-path exclusions), and
  per-route rate limiting
- JSON structured logging with rotation
- Health check, Prometheus metrics, and OpenAPI / Swagger UI endpoints
- Pluggable background task pool with bounded concurrency
- Migration-driven database schema (offline query metadata committed)
- Multi-stage Dockerfile and Docker Compose setup with health checks
- Criterion benchmarks and a set of runnable example programs

## Project Structure

```plain
src/
├── app/              # App context: dependency assembly and lifecycle
├── auth/             # JWT claims and authentication middleware
├── conf/             # Configuration loading from TOML
├── core/             # Core types: AppState, AppResult/AppError wrappers
├── data/             # Repository implementations + connection/pool management
├── docs/             # OpenAPI / utoipa registration
├── errors/           # Predefined application errors
├── handlers/         # HTTP request handlers
├── logx/             # Structured logging initialization
├── models/           # Data models and entities (e.g. UserInfo)
├── observability/   # OpenTelemetry / metrics setup
├── repos/            # Repository traits (data access contracts)
├── routers/          # Route definitions and middleware stack
├── services/         # Business logic layer
├── transport/        # HTTP server setup and middleware helpers
│   └── middleware/   # Custom middleware functions
├── types/            # Request/response DTOs with validation
└── utils/            # Utility functions (hash, valid code, etc.)

benches/             # Criterion benchmark suites
examples/            # Runnable example programs
etc/                 # Configuration files
migrations/          # Database migration scripts
```

### Layers

1. **Transport** (`src/transport/`): TCP listener, HTTP server setup, reusable middleware.
2. **Handlers** (`src/handlers/`): HTTP endpoint handlers that parse input and call services.
3. **Services** (`src/services/`): Business logic; each service holds repository interfaces injected at startup.
4. **Repositories** (`src/repos/`): Data access contracts.
5. **Data** (`src/data/`): Concrete implementations over storage backends plus pool management.

Dependencies flow one way: `handlers → services → repos (traits) ← data (implementations)`.
`app::AppContext` wires concrete implementations into services and exposes them via `AppState`.

## Quick Start

### Prerequisites

- Rust 1.89+ (install via [rustup](https://rustup.rs/))
- MySQL 5.7.20+
- Redis 5.0+
- `sqlx-cli` with MySQL support

### Run

```bash
# 1. Clone
git clone https://github.com/TuMengStudios/axum-best.git
cd axum-best

# 2. Configure (edit etc/config.toml for DSN / Redis / WeChat / etc.)
cp .env.example .env  # for sqlx-cli + compile-time query check

# 3. Create database and run migrations
sqlx migrate run

# 4. Start the development server (listens on 0.0.0.0:8080)
cargo run
```

### Common Commands

```bash
cargo run                      # run the application
cargo test                     # tests
cargo nextest run              # tests with nextest
cargo bench                    # benchmarks
cargo watch -x run             # hot reload
cargo fmt                      # format
cargo clippy                   # lint
cargo build --release          # release build
```

## Configuration

Configuration is loaded from a TOML file (default: `etc/config.toml`):

```bash
cargo run -- --conf etc/config.toml
```

Sensible secrets (e.g. the JWT signing key) should be supplied through
environment variables in production. See the top of `etc/config.toml` for the full
list of sections and environment overrides.

## Deployment

```bash
# Docker Compose (uses host network, exposes 8080)
docker compose up --build

# Or build manually
docker build -t axum-best .
docker run -p 8080:8080 axum-best
```

Docker Compose and the Dockerfile both configure health checks against
`http://localhost:8080/health`.

For production, place a reverse proxy (nginx / Caddy) in front and manage the
process with systemd / supervisord.

## License

MIT
