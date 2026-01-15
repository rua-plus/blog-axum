# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

blog-axum is a modern blog application built with the Axum framework. It's designed as a high-performance, maintainable blog system with content management, user management, and commenting features.

## Common Development Commands

### Build and Run
```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release

# Build only
cargo build

# Build release
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture
```

### Code Quality
```bash
# Format code
cargo fmt

# Check code style
cargo clippy

# Generate documentation
cargo doc --open
```

### Logging
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run with trace logging
RUST_LOG=trace cargo run

# Run with specific module logging
RUST_LOG=blog_axum=debug,tower_http=debug cargo run
```

### Using Makefile
```bash
# Build the project
make build

# Run the project
make run

# Run tests
make test

# Format code
make fmt

# Run clippy linter
make clippy

# Run with debug logging
make run-debug

# Run with trace logging
make run-trace

# Clean build artifacts
make clean

# Generate documentation
make doc

# Check code without building
make check

# Install dependencies
make install
```

## Architecture Overview

### Core Components

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

- **Unified Response Format**: All API responses follow consistent structure with `success`, `code`, `message`, `data`, and `request_id` fields
- **Request Tracking**: Each request gets a unique UUID for end-to-end tracing
- **Version Integration**: Git commit hash automatically embedded in responses via build script
- **Error Classification**: Business error codes (40000-50000 range) separate from HTTP status codes

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
