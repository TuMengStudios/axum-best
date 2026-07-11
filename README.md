# axum best

> For the optimal axum development experience

* [简体中文](README_ZH.md)
* [development tools](dev.md)

## Project Overview

axum-best is a Rust web project template built on [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/). It demonstrates a layered architecture for building scalable HTTP services with MySQL, Redis, structured logging, request validation, and common middleware.

### Key Features

- **High Performance**: Built on top of Tokio and Axum for asynchronous request handling
- **Layered Architecture**: Clear separation of Transport, Handler, Service, Repository, and Data layers
- **MySQL + SQLx**: Type-safe database operations with compile-time checked queries; offline query metadata is committed under `.sqlx`
- **Redis Caching**: Connection pooling via `r2d2` over the `redis` crate for caching and session-like storage
- **Request Validation**: Input validation with `validator` and `axum-valid`
- **Middleware Stack**: Request-ID tracing, CORS, request/response compression, decompression, and timeout handling
- **Structured Logging**: JSON log output with rotation, configurable via `etc/config.toml`
- **WeChat Mini-Program Login**: Placeholder login flow that maps a WeChat code to an existing user by `openid`
- **Email Binding**: Placeholder email-binding workflow with validation code generation
- **User CRUD**: Complete create, read, update, delete, and search operations for `user_info`
- **Utility Modules**: Numeric verification code generation, file digest (MD5/SHA1/SHA256/SHA512), and a generic `Primitive` enum
- **Docker Support**: Multi-stage Dockerfile and Docker Compose setup
- **Benchmarks**: Criterion benchmarks for utility functions and model generation

## Project Structure

```plain
src/
├── conf/           # Configuration loading from TOML
├── core/           # Core types: AppState, AppResult/AppError wrappers
├── data/           # MySQL and Redis connection/pool implementations
├── errors/         # Predefined application errors
├── handlers/       # HTTP request handlers
├── logx/           # Structured logging initialization
├── models/         # Data models and entities (e.g. UserInfo)
├── repos/          # Repository pattern / raw SQLx data access
├── routers/        # Route definitions and middleware stack
├── services/       # Business logic layer
├── srvCtx/         # Server context: builds state and starts the HTTP server
├── transport/      # HTTP server setup and middleware helpers
│   └── middleware/ # Custom middleware functions
├── types/          # Request/response DTOs with validation
└── utils/          # Utility functions (hash, valid code, etc.)

benches/           # Criterion benchmark suites
examples/          # Runnable example programs
etc/               # Configuration files
migrations/        # SQLx database migration scripts
```

### Architecture Layers

1. **Transport Layer** (`src/transport/`): TCP listener, HTTP server setup, and reusable middleware
2. **Handler Layer** (`src/handlers/`): HTTP endpoint handlers that parse input and call services
3. **Service Layer** (`src/services/`): Business logic implementation
4. **Repository Layer** (`src/repos/`): Data access abstraction over SQLx
5. **Data Layer** (`src/data/`): Database and cache connection/pool management

## API Endpoints

| Method | Path | Description |
| ------ | ---- | ----------- |
| `GET` | `/health` | Health check, returns `ok` |
| `GET` | `/foo` | Demo query endpoint with validated `key_word` |
| `GET` | `/user/{id}` | Get user by ID |
| `POST` | `/user/wx/login` | WeChat mini-program login placeholder |
| `GET` | `/user/random` | Create and return a random `UserInfo` |
| `POST` | `/user/email/pre` | Generate and store an email binding validation code |
| `POST` | `/user/email` | Bind an email address using the validation code |

## Development Guide

### Prerequisites

- Rust 1.89+ (install via [rustup](https://rustup.rs/))
- MySQL 5.7.20+
- Redis 5.0+
- `sqlx-cli` with MySQL support: `cargo install sqlx-cli --features mysql`

### Getting Started

1. **Clone the repository**

   ```bash
   git clone https://github.com/TuMengStudios/axum-best.git
   cd axum-best
   ```

2. **Configure the application**

   Edit `etc/config.toml` to set your MySQL DSN, Redis URL, HTTP listen address, log settings, and WeChat credentials.

3. **Create the database and run migrations**

   ```bash
   # Make sure MySQL is running
   sqlx migrate run
   ```

4. **Start the development server**

   ```bash
   cargo run
   ```

   The server listens on `http://0.0.0.0:8080` by default.

### Development Commands

```bash
# Run the application
cargo run

# Run tests
cargo test

# Run tests with nextest (requires cargo-nextest)
cargo nextest run

# Run benchmarks
cargo bench

# Hot reload during development (requires cargo-watch)
cargo watch -x run

# Format code
cargo fmt

# Check code quality
cargo clippy

# Build for release
cargo build --release
```

### Code Organization

- **Models**: Define your data structures in `src/models/`
- **Types**: Define request/response DTOs with validation in `src/types/`
- **Handlers**: Add new HTTP endpoints in `src/handlers/`
- **Services**: Implement business logic in `src/services/`
- **Repositories**: Add data access methods in `src/repos/`
- **Routes**: Register new routes in `src/routers/`

### Adding New Features

1. Define data models in `src/models/`
2. Define request/response types in `src/types/`
3. Create repository methods in `src/repos/`
4. Implement business logic in `src/services/`
5. Add HTTP handlers in `src/handlers/`
6. Register routes in `src/routers/`

## Configuration

Configuration is loaded from a TOML file (default: `etc/config.toml`). The file path can be overridden via the command line:

```bash
cargo run -- --conf etc/config.toml
```

Key sections in `etc/config.toml`:

- `[log]` — log level, rotation, directory, filename, and JSON/text format
- `[http]` — listen address and port (default `0.0.0.0:8080`)
- `[mysql]` — DSN, connection pool size, slow query thresholds
- `[redis]` — Redis URL and pool settings
- `[wechat]` — WeChat mini-program `appid` and `secret`

The `.env` file is used by `sqlx-cli` and the SQLx compile-time query checker (`DATABASE_URL=mysql://root:123456@localhost:3306/axum_best`).

## Examples

The `examples/` directory contains small runnable programs:

- `db_demo` — load config and test the MySQL connection
- `user_crud_demo` — demonstrate user repository CRUD usage
- `primitive_demo` — demonstrate the `Primitive` enum
- `test_valid_code` — test numeric verification code generation
- `test_random_uniqueness` — test random `UserInfo` generation uniqueness

Run an example with:

```bash
cargo run --example db_demo
```

## Benchmarks

Criterion benchmarks are located in `benches/`:

- `utils_benchmark` — benchmarks `gen_valid_code` and `file_digest`
- `models_user` — benchmarks `UserInfo::random()`

Run all benchmarks with:

```bash
cargo bench
```

## Deployment

### Docker Deployment

The project includes Docker support for easy deployment. The application listens on port `8080` inside the container.

1. **Build and run with Docker Compose**

   ```bash
   docker-compose up --build
   ```

2. **Build Docker image manually**

   ```bash
   docker build -t axum-best .
   docker run -p 8080:8080 axum-best
   ```

### Production Deployment

1. **Build optimized binary**

   ```bash
   cargo build --release
   ```

2. **Set up reverse proxy** (recommended: nginx or Caddy)
3. **Configure process manager** (recommended: systemd or supervisord)
4. **Set up monitoring and logging**

### Health Checks

The application includes a health check endpoint:

- `GET /health` — returns `ok`

Docker Compose and the Dockerfile both configure health checks against `http://localhost:8080/health`.

## Performance Considerations

- Use connection pooling for database and Redis operations
- Implement caching for frequently accessed data
- Enable compression for HTTP responses
- Use appropriate logging levels in production
- Monitor memory usage and connection counts

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests and ensure they pass
5. Submit a pull request

## License

This project is licensed under the MIT License.
