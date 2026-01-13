use tracing_subscriber::{EnvFilter, Registry, fmt, prelude::__tracing_subscriber_SubscriberExt};

pub fn init_tracing() -> anyhow::Result<()> {
    // Check if we're in production
    let is_production = std::env::var("RUA_ENV")
        .map(|env| env == "production")
        .unwrap_or(false);

    // Read log level from RUA_BLOG environment variable, defaulting to info for production and debug for development
    let default_log_level = if is_production { "info" } else { "debug" };
    let log_level = std::env::var("RUA_BLOG").unwrap_or(default_log_level.to_string());

    if is_production {
        let subscriber = Registry::default()
            .with(
                EnvFilter::from_default_env()
                    .add_directive(format!("axum_tracing_example={}", log_level).parse()?)
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
                    .add_directive(format!("axum_tracing_example={}", log_level).parse()?)
                    .add_directive(format!("tower_http={}", log_level).parse()?),
            )
            .with(
                fmt::Layer::new()
                    .pretty()
                    .with_ansi(true)
                    .with_target(true)
                    .with_line_number(true)
                    .with_file(true),
            );

        tracing::subscriber::set_global_default(subscriber)?;
    }

    Ok(())
}
