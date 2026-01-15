# AGENTS.md - Development Guide for blog-axum

This guide provides essential information for agentic coding agents working with the blog-axum codebase. It covers build commands, linting, testing, code style guidelines, and project conventions.

## Build, Lint, and Test Commands

### Build Commands
```bash
# Build the project (debug mode)
cargo build

# Build in release mode
cargo build --release

# Run the project (debug mode)
cargo run

# Run in release mode
cargo run --release

# Check code without building
cargo check
```

### Linting and Formatting
```bash
# Run clippy linter (strict mode)
cargo clippy --all-targets --all-features

# Format code according to Rustfmt rules
cargo fmt

# Check formatting without applying changes
cargo fmt --check
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a single test (replace <test_name> with actual test name)
cargo test <test_name>

# Run tests for a specific module
cargo test -p <crate_name>

# Run tests in release mode
cargo test --release
```

### Makefile Shortcuts
For convenience, use the provided Makefile:
```bash
# Build (debug)
make build

# Run (debug)
make run

# Run tests
make test

# Format code
make fmt

# Run clippy
make clippy

# Build release
make release

# Run in release mode
make run-release

# Run with debug logging
make run-debug

# Run with trace logging
make run-trace

# Clean build artifacts
make clean

# Generate documentation
make doc
```

## Code Style Guidelines

### Imports
- Use standard Rust import conventions
- Group imports by category: std, external crates, local modules
- Use `use crate::` for local module imports
- Example:
  ```rust
  // src/main.rs
  use std::net::SocketAddr;
  use std::sync::Arc;

  use axum::{
      extract::State,
      http::StatusCode,
      response::IntoResponse,
      routing::get,
      Router,
  };
  use tracing_subscriber::layer::SubscriberExt;

  use crate::{
      middlewares::build_middleware_stack,
      response::success,
      utils::{config::load_config, init_tracing},
  };
  ```

### Naming Conventions
- **Functions**: snake_case (e.g., `init_tracing`, `load_config`)
- **Structs/Enums**: PascalCase (e.g., `AppConfig`, `StatusCode`)
- **Constants**: UPPER_SNAKE_CASE (e.g., `GIT_VERSION`)
- **Modules**: snake_case (e.g., `middlewares`, `utils`)
- **Traits**: PascalCase (e.g., `IntoResponse`)
- **Fields/Variables**: snake_case (e.g., `postgres_config`, `jwt_secret`)

### Type Annotations
- Use explicit type annotations for public API
- Prefer inferred types for local variables when clear
- Example:
  ```rust
  // src/response.rs
  pub struct SuccessResponse<T: Serialize> {
      pub success: bool,
      pub code: u32,
      pub message: String,
      pub data: T,
      pub timestamp: u64,
      pub request_id: String,
      pub version: String,
  }
  ```

### Error Handling
- Use `anyhow` for application-level errors
- Return `Result<T, AppError>` for fallible operations
- Use custom error types for domain-specific errors
- Example from src/utils/config.rs:
  ```rust
  pub fn load_config() -> Result<AppConfig, ConfigError> {
      let config_file = env::var("CONFIG_FILE").unwrap_or_else(|_| "config.toml".into());
      
      Config::builder()
          .add_source(File::with_name(&config_file))
          .add_source(Environment::with_prefix("APP").separator("_"))
          .build()?
          .try_deserialize()
  }
  ```

### Formatting Rules
- Follow Rustfmt defaults (run `cargo fmt` to apply)
- 4 spaces per indentation level
- Max line length: 100 characters
- Blank lines between logical code blocks
- Example:
  ```rust
  // src/middlewares/mod.rs
  pub fn build_trace_layer() -> TraceLayer<Span> {
      TraceLayer::new_for_http()
          .make_span_with(|request: &Request<_>| {
              tracing::info_span!(
                  "HTTP Request",
                  method = ?request.method(),
                  path = ?request.uri().path(),
                  request_id = %get_request_id(request),
              )
          })
          .on_request(|_request: &Request<_>, _span: &Span| {
              tracing::debug!("Request received")
          })
          .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
              tracing::info!(
                  status = ?response.status(),
                  latency = %latency.as_millis(),
                  "Response sent"
              );
          })
  }
  ```

## Project Structure and Architecture

### Core Modules

1. **Entry Point** (`src/main.rs`)
   - Initializes tracing/logging system
   - Loads application configuration
   - Sets up middleware stack
   - Creates and starts HTTP server

2. **Response System** (`src/response.rs`)
   - Unified response structures for success/error/pagination
   - Custom status codes (business logic codes, not HTTP)
   - Automatic timestamp and request ID generation
   - Git version integration

3. **Configuration** (`src/utils/config.rs`)
   - TOML-based configuration with `config.toml`
   - Environment variable support via `CONFIG_FILE`
   - PostgreSQL and JWT configuration sections

4. **Middleware** (`src/middlewares/mod.rs`)
   - Request ID tracking for distributed tracing
   - HTTP request/response logging via Tower HTTP

### Key Design Patterns

1. **Unified Response Format**: All API responses follow consistent structure with `success`, `code`, `message`, `data`, and `request_id` fields
2. **Request Tracking**: Each request gets a unique UUID for end-to-end tracing
3. **Version Integration**: Git commit hash automatically embedded in responses via build script
4. **Error Classification**: Business error codes (40000-50201 range) separate from HTTP status codes

### Configuration Flow

1. Build script (`build.rs`) captures Git version
2. Application loads `config.toml` (or custom path via `CONFIG_FILE`)
3. Configuration merged with environment variables
4. Server binds to `0.0.0.0:8000` with configured middleware stack

### Middleware Stack

1. `request_id_middleware` - Adds X-Request-ID header
2. `TraceLayer` - HTTP request/response logging

## Adding New Features

### New API Endpoint
1. Add route in `main.rs` `Router::new()` chain
2. Create handler function returning appropriate response type
3. Use `StatusCode::{success,created,etc}()` for consistent responses

### New Configuration
1. Add struct field to `AppConfig` in `src/utils/config.rs`
2. Update `config.toml` with default values
3. Access via `app_config.your_new_field`

### New Middleware
1. Create middleware function in `src/middlewares/mod.rs`
2. Add to middleware stack in `main.rs` with `.layer()`

## Important Notes

- The project uses a Git submodule in `lib/blog/` - ensure `--recurse-submodules` when cloning
- Response structures automatically include Git version from build time
- All responses include millisecond timestamps and request IDs for tracking
- Business error codes are 5-digit numbers (e.g., 40000, 40100) distinct from HTTP status codes
- Logging format automatically switches between JSON (production) and colored output (development)

## Testing Guidelines

### Writing Tests
- Place tests in the same module with `#[cfg(test)]` attribute
- Use `tokio::test` for async tests
- Example from src/utils/config.rs:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      use tempfile::NamedTempFile;
      use std::fs::write;

      #[test]
      fn test_load_config() {
          let temp_file = NamedTempFile::new().expect("Failed to create temp file");
          let config_content = r#"
              [postgresql]
              host = 'localhost'
              port = 5432
              user = 'test'
              password = 'test'
              database = 'test_db'

              [jwt]
              secret = 'test-secret'
              expires_in = '7d'
          "#;
          
          write(temp_file.path(), config_content).expect("Failed to write temp config");
          
          std::env::set_var("CONFIG_FILE", temp_file.path().to_str().unwrap());
          
          let config = load_config().expect("Failed to load test config");
          
          assert_eq!(config.postgresql.host, "localhost");
          assert_eq!(config.jwt.secret, "test-secret");
      }
  }
  ```

### Running Tests
- Run all tests: `cargo test`
- Run specific test: `cargo test test_load_config`
- Run tests with output: `cargo test -- --nocapture`