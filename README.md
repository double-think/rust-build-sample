# Rust Build Speed Tuning Sample

This project demonstrates the impact of dependency configuration on Rust build times. It contains two branches (`main` and `terrible`) that showcase different approaches to dependency management and their effects on compilation speed.

## Overview

This is a simple Axum-based web server application that serves as a learning tool for understanding how dependency choices affect build performance in Rust projects.

## Project Structure

```
rust-build-sample/
├── Cargo.toml          # Dependency configuration
├── mise.toml           # Rust version management (1.89)
└── src/
    ├── main.rs         # Entry point with Axum server
    ├── api_doc.rs      # OpenAPI documentation (optional)
    ├── app_error.rs    # Error handling
    ├── user_post.rs    # User creation endpoint
    └── validated_json.rs # JSON validation utilities
```

## Branch Comparison

### `main` Branch (Optimized)
The main branch represents a well-tuned configuration with minimal dependencies:

**Key characteristics:**
- Uses Rust edition 2024
- Selective feature flags for dependencies
- Only necessary features enabled
- Smaller dependency tree
- **Build time: ~13 seconds** (release mode)

**Dependency configuration:**
- `axum`: Only `multipart` feature
- `axum-extra`: Only `typed-header` feature
- `sea-orm`: PostgreSQL-only with `runtime-tokio-rustls`
- `tokio`: Only essential features (`macros`, `rt-multi-thread`, `signal`)
- `utoipa`/`utoipa-swagger-ui`: Optional features behind `apidoc` feature flag
- `validator`: Only `derive` feature

### `terrible` Branch (Unoptimized)
The terrible branch demonstrates poor dependency management practices:

**Key characteristics:**
- Uses Rust edition 2021 (more compatible but increases dependencies)
- All features enabled for most dependencies
- Significantly larger dependency tree
- **Build time: Significantly longer** due to compilation of unused features

**Dependency configuration:**
- `axum`: Full features including `macros`, `http1`, `http2`, `ws`, `tokio`, `tower-log`
- `axum-extra`: All features (`cookie`, `protobuf`, `query`, etc.)
- `sea-orm`: All database backends (`sqlx-all`) with `async-std` runtime
- `tokio`: Full features enabled
- `utoipa`/`utoipa-swagger-ui`: All features including `yaml`, `debug`, `debug-embed`
- `validator`: All features including `card`, `unic`

## Build Performance Tips

1. **Minimize feature flags**: Only enable features you actually use
2. **Choose specific database backends**: Don't use `sqlx-all` if you only need PostgreSQL
3. **Use consistent async runtimes**: Mixing `tokio` and `async-std` increases build times
4. **Make optional features truly optional**: Use feature flags for documentation tools
5. **Consider Rust edition**: Newer editions may have optimizations but check compatibility

## Running the Project

### Prerequisites
- Rust 1.89+ (managed via mise)
- PostgreSQL (for database features)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd rust-build-sample

# Install Rust via mise
mise install

# Build the optimized version
git checkout main
cargo build --release

# Build the unoptimized version
git checkout terrible
cargo build --release
```

### Running the Server

```bash
cargo run --release
```

The server will start on `http://0.0.0.0:3000`

**Available endpoints:**
- `GET /` - Hello World endpoint
- `POST /api/users` - Create user endpoint
- `GET /swagger-ui` - API documentation (when built with `--features apidoc`)

## Analyzing Build Times

To generate detailed build timing reports:

```bash
cargo clean
cargo build --release --timings
```

This creates an HTML report in `target/cargo-timings/` showing:
- Which crates take the longest to compile
- Parallelization opportunities
- Dependency bottlenecks

## Learning Resources

- [The Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) - Analyze binary size
- [cargo-tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) - Visualize dependency tree

## License

This is an educational example project for learning about Rust build optimization.