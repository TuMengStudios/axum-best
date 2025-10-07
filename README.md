# axum best

> For the optimal axum development experience

[简体中文](README_ZH.md)

## Project Overview

axum-best is a high-performance web application framework built with Rust and the Axum web framework. It provides a robust foundation for building scalable and maintainable web services with best practices in mind.

### Key Features

- **High Performance**: Built on top of Tokio and Axum for maximum throughput
- **Clean Architecture**: Well-structured codebase following domain-driven design principles
- **Database Support**: MySQL integration with SQLx for type-safe database operations
- **Caching Layer**: Redis-based caching for improved performance
- **Comprehensive Error Handling**: Structured error handling throughout the application
- **Configuration Management**: Flexible configuration system with environment-specific settings
- **Docker Support**: Containerized deployment with Docker and Docker Compose
- **Benchmarking**: Built-in benchmarking tools for performance testing

## Project Structure

```plain
src/
├── conf/           # Configuration management
├── core/           # Core application logic and state management
├── data/           # Data access layer (MySQL, Redis)
├── errors/         # Error types and handling
├── handlers/       # HTTP request handlers
├── logx/           # Logging utilities
├── models/         # Data models and entities
├── repos/          # Repository pattern implementations
├── routers/        # Route definitions
├── services/       # Business logic layer
├── srvCtx/         # Service context and dependency injection
├── transport/      # HTTP transport layer and middleware
├── types/          # Custom type definitions
└── utils/          # Utility functions

benches/           # Benchmark tests
examples/          # Example usage code
migrations/        # Database migration scripts
etc/               # Configuration files
```

### Architecture Layers

1. **Transport Layer** (`src/transport/`): HTTP server setup, middleware, and request/response handling
2. **Handler Layer** (`src/handlers/`): HTTP endpoint handlers that orchestrate service calls
3. **Service Layer** (`src/services/`): Business logic implementation
4. **Repository Layer** (`src/repos/`): Data access abstraction
5. **Data Layer** (`src/data/`): Database and cache implementations

## Development Guide

### Prerequisites

- Rust 1.89+ (install via [rustup](https://rustup.rs/))
- MySQL 5.7.20+
- Redis 5.0+

### Getting Started

1. **Clone the repository**

   ```bash
   git clone https://github.com/TuMengStudios/axum-best.git
   cd axum-best
   ```

2. **Set up environment variables**

   Create a `.env` file with your database and Redis configuration.

3. **Run database migrations**

   ```bash
   # Make sure MySQL is running
   sqlx migrate run
   ```

4. **Start the development server**

   ```bash
   cargo run
   ```

### Development Commands

```bash
# Run the application
cargo run

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Check code quality
cargo clippy

# Build for release
cargo build --release
```

### Code Organization

- **Models**: Define your data structures in `src/models/`
- **Handlers**: Add new HTTP endpoints in `src/handlers/`
- **Services**: Implement business logic in `src/services/`
- **Repositories**: Add data access methods in `src/repos/`
- **Routes**: Register new routes in `src/routers/`

### Adding New Features

1. Define data models in `src/models/`
2. Create repository methods in `src/repos/`
3. Implement business logic in `src/services/`
4. Add HTTP handlers in `src/handlers/`
5. Register routes in `src/routers/`

## Deployment

### Docker Deployment

The project includes Docker support for easy deployment:

1. **Build and run with Docker Compose**

   ```bash
   docker-compose up --build
   ```

2. **Build Docker image manually**

   ```bash
   docker build -t axum-best .
   docker run -p 3000:3000 axum-best
   ```

### Environment Configuration

Create a `.env` file with the following variables:

```env
DATABASE_URL=mysql://username:password@localhost:3306/database_name
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

The application includes health check endpoints:

- `GET /health` - Basic application health

## Configuration

Configuration is managed through:

- `etc/config.toml` - Main configuration file
- Environment variables (override config file settings)
- Command-line arguments

## Performance Considerations

- Use connection pooling for database operations
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
