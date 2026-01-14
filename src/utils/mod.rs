pub mod config;

use tracing_subscriber::{EnvFilter, Registry, fmt, prelude::__tracing_subscriber_SubscriberExt};

/// Initializes the tracing/logging system for the application.
///
/// This function sets up the global tracing subscriber with different configurations
/// for production and development environments.
///
/// # Environment Variables
///
/// - `RUA_ENV`: If set to "production", enables production logging mode
/// - `RUA_BLOG`: Sets the log level (e.g., "debug", "info", "warn", "error")
///   Defaults to "info" in production, "debug" in development
///
/// # Production vs Development Behavior
///
/// ## Production
/// - JSON-formatted logs for better parsing by log aggregation systems
/// - ANSI colors disabled
/// - Includes thread IDs and names for better concurrency tracking
/// - No file/line numbers to reduce log size
///
/// ## Development
/// - Pretty-printed console output with colors
/// - Includes file names and line numbers for easier debugging
/// - Thread IDs and names included
/// - ANSI colors enabled for better readability
///
/// # Returns
///
/// Returns `Ok(())` if tracing initialization succeeds, or an error if the
/// global subscriber cannot be set.
///
/// # Examples
///
/// ```
/// // Initialize tracing with default settings
/// init_tracing()?;
///
/// // In production with JSON logs
/// // RUA_ENV=production RUA_BLOG=info cargo run
///
/// // In development with debug logs
/// // RUA_BLOG=debug cargo run
/// ```
pub fn init_tracing() -> anyhow::Result<()> {
    // Check if we're in production by reading the RUA_ENV environment variable
    let is_production = std::env::var("RUA_ENV")
        .map(|env| env == "production")
        .unwrap_or(false);

    // Determine the default log level based on environment
    // Production defaults to "info" for less verbose logging
    // Development defaults to "debug" for detailed debugging information
    let default_log_level = if is_production { "info" } else { "debug" };

    // Read log level from RUA_BLOG environment variable, falling back to environment-appropriate default
    let log_level = std::env::var("RUA_BLOG").unwrap_or(default_log_level.to_string());

    if is_production {
        let subscriber = Registry::default()
            .with(
                EnvFilter::from_default_env()
                    .add_directive(format!("blog_axum={}", log_level).parse()?)
                    .add_directive(format!("tower_http={}", log_level).parse()?),
            )
            .with(
                fmt::Layer::new()
                    .json()
                    .with_ansi(false)
                    .with_current_span(false)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true),
            );

        tracing::subscriber::set_global_default(subscriber)?;
    } else {
        // Development: Pretty console logging
        let subscriber = Registry::default()
            .with(
                EnvFilter::from_default_env()
                    .add_directive(format!("blog_axum={}", log_level).parse()?)
                    .add_directive(format!("tower_http={}", log_level).parse()?),
            )
            .with(
                fmt::Layer::new()
                    .pretty()
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_ansi(true)
                    .with_target(true)
                    .with_line_number(true)
                    .with_file(true),
            );

        tracing::subscriber::set_global_default(subscriber)?;
    }

    Ok(())
}
