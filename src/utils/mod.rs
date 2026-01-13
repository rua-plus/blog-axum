use tracing_subscriber::{EnvFilter, Registry, fmt, prelude::__tracing_subscriber_SubscriberExt};

pub fn init_tracing() -> anyhow::Result<()> {
    // Check if we're in production
    let is_production = std::env::var("APP_ENV")
        .map(|env| env == "production")
        .unwrap_or(false);

    if is_production {
        let subscriber = Registry::default()
            .with(
                EnvFilter::from_default_env()
                    .add_directive("axum_tracing_example=info".parse()?)
                    .add_directive("tower_http=info".parse()?),
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
                    .add_directive("axum_tracing_example=debug".parse()?)
                    .add_directive("tower_http=debug".parse()?),
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
