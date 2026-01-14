.PHONY: default build run test clean fmt clippy doc

# Default target
default: build

# Build the project
build:
	cargo build

# Build release version
release:
	cargo build --release

# Run the project
run:
	cargo run

# Run release version
run-release:
	cargo run --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy --all-targets --all-features

# Generate documentation
doc:
	cargo doc --open

# Run with debug logging
run-debug:
	RUST_LOG=debug cargo run

# Run with trace logging
run-trace:
	RUST_LOG=trace cargo run

# Check code without building
check:
	cargo check

# Install dependencies (if needed)
install:
	cargo fetch
