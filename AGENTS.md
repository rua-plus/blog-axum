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
  use anyhow::Context;
  use axum::{Router, extract::State, middleware, routing::get};
  use sqlx::PgPool;
  use tracing::{debug, info};

  use crate::response::{StatusCode, SuccessResponse};
  use crate::utils::{config, init_tracing};
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
  pub struct SuccessResponse<T> {
      pub success: bool,
      pub code: StatusCode,
      pub message: String,
      pub timestamp: u64,
      pub request_id: String,
      pub data: Option<T>,
      pub version: Option<String>,
  }
  ```

### Error Handling
- Use custom `AppError` type wrapping `anyhow::Error` for application-level errors
- Return `AppResult<T>` (type alias for `anyhow::Result<T, AppError>`) for fallible operations
- Implement `From` trait for common error types (sqlx, config, jwt)
- Example from src/error.rs:
  ```rust
  #[derive(Debug)]
  pub struct AppError(Error);

  impl AppError {
      pub fn new<E: Into<Error>>(err: E) -> Self {
          AppError(err.into())
      }
  }

  impl IntoResponse for AppError {
      fn into_response(self) -> Response {
          error!("{:?}", self);
          StatusCode::internal_error()
              .with_debug(self.0.to_string())
              .into_response()
      }
  }

  pub type AppResult<T> = anyhow::Result<T, AppError>;
  ```

### Formatting Rules
- Follow Rustfmt defaults (run `cargo fmt` to apply)
- 4 spaces per indentation level
- Max line length: 100 characters
- Blank lines between logical code blocks

## Project Structure and Architecture

### Core Modules

1. **Entry Point** (`src/main.rs`)
   - Initializes tracing/logging system
   - Loads application configuration
   - Creates database connection pool
   - Initializes JWT service
   - Sets up middleware stack
   - Creates and starts HTTP server

2. **Response System** (`src/response.rs`)
   - Unified response structures for success/error/pagination
   - Custom status codes (business logic codes, not HTTP)
   - Automatic timestamp and request ID generation
   - Git version integration
   - Key structs: `SuccessResponse<T>`, `ErrorResponse`, `PaginationResponse<T>`

3. **Error Handling** (`src/error.rs`)
   - Custom `AppError` type wrapping `anyhow::Error`
   - Conversion from common error sources (sqlx, config, jwt)
   - `AppResult<T>` type alias for simplified error handling

4. **Extractors** (`src/extractors.rs`)
   - `ValidatedJson<T>`: Validates request JSON against validator crate rules
   - `Auth`: Extracts and validates JWT tokens from Authorization header
   - Custom rejection handling for validation and authentication errors

5. **Middleware** (`src/middlewares/mod.rs`)
   - Request ID tracking for distributed tracing
   - HTTP request/response logging via Tower HTTP TraceLayer

6. **Routes** (`src/routes/`)
   - `mod.rs`: Route definitions and creation function
   - `users.rs`: User management endpoints (list, login, create)

7. **Models** (`src/models/mod.rs`)
   - Data models (User struct) with validation using validator crate
   - Database schema definitions for SQLx

8. **Utilities** (`src/utils/`)
   - `config.rs`: Configuration loader (TOML + environment variables)
   - `jwt.rs`: JWT token service (creation, validation)
   - `password.rs`: Password hashing/verification using Argon2
   - `mod.rs`: General utility functions

### Key Design Patterns

1. **Unified Response Format**: All API responses follow consistent structure with `success`, `code`, `message`, `data`, and `request_id` fields
2. **Request Tracking**: Each request gets a unique UUID for end-to-end tracing
3. **Version Integration**: Git commit hash automatically embedded in responses via build script
4. **Error Classification**: Business error codes (40000-50201 range) separate from HTTP status codes
5. **Validation**: Request validation using validator crate with custom extractor
6. **Authentication**: JWT token validation via custom extractor

### Configuration Flow

1. Build script (`build.rs`) captures Git version
2. Application loads `config.toml` (or custom path via `CONFIG_FILE`)
3. Configuration merged with environment variables
4. Server binds to `0.0.0.0:8000` with configured middleware stack

### Middleware Stack

1. `request_id_middleware` - Adds X-Request-ID header
2. `TraceLayer` - HTTP request/response logging with latency tracking

## Adding New Features

### New API Endpoint
1. Add route in `src/routes/mod.rs`
2. Create handler function in appropriate routes file (e.g., `src/routes/users.rs`)
3. Use `StatusCode::{success,created,etc}()` for consistent responses
4. For authenticated routes, use `Auth` extractor
5. For validated requests, use `ValidatedJson<T>` extractor

### New Configuration
1. Add struct field to `AppConfig` in `src/utils/config.rs`
2. Update `config.toml` with default values
3. Access via `app_config.your_new_field`

### New Middleware
1. Create middleware function in `src/middlewares/mod.rs`
2. Add to middleware stack in `main.rs` with `.layer()`

### New Data Model
1. Add struct to `src/models/mod.rs` with validation attributes
2. Implement necessary traits (Debug, Clone, Serialize, Deserialize)
3. Add database queries in appropriate utils module

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
- Example from src/response.rs:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_success_response_creation() {
          let data = "test data";
          let response = StatusCode::success(Some(data));

          assert!(response.success);
          assert_eq!(response.code, StatusCode::Success);
          assert_eq!(response.message, "Success");
          assert!(response.timestamp > 0);
          assert!(!response.request_id.is_empty());
          assert_eq!(response.data.unwrap(), data);
      }
  }
  ```

### Running Tests
- Run all tests: `cargo test`
- Run specific test: `cargo test test_success_response_creation`
- Run tests with output: `cargo test -- --nocapture`

## Key Technologies

- **Framework**: Axum 0.8.8 (async web framework)
- **Runtime**: Tokio 1.49.0 (async runtime)
- **Database**: SQLx with PostgreSQL
- **Authentication**: JWT tokens (jsonwebtoken crate)
- **Password Hashing**: Argon2 (argon2 crate)
- **Validation**: validator crate (with derive macros)
- **Configuration**: config crate (TOML + environment variables)
- **Tracing/Logging**: tracing and tracing-subscriber
- **HTTP Middleware**: tower-http (trace layer)