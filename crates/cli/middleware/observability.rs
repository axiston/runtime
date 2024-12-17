#[must_use]
fn build_env_filter() -> tracing_subscriber::EnvFilter {
    let current = std::env::var("RUST_LOG")
        .or_else(|_| std::env::var("OTEL_LOG_LEVEL"))
        .unwrap_or_else(|_| "info".to_string());

    let env = format!("{},server=trace,otel=debug,tower_http=debug", current);
    std::env::set_var("RUST_LOG", env);
    tracing_subscriber::EnvFilter::from_default_env()
}

pub async fn initialize_tracing() -> anyhow::Result<()> {
    use tracing_subscriber::fmt::layer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    // Setups a temporary subscriber to log output during setup.
    let env_filter = build_env_filter();
    let fmt_layer = layer().pretty();
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    let _guard = tracing::subscriber::set_default(subscriber);
    tracing::trace!(target: "server:otel", "initialized temporary subscriber");

    // TODO: Enable OpenTelemetry.
    // https://github.com/davidB/tracing-opentelemetry-instrumentation-sdk

    // Setups an actual subscriber.
    let env_filter = build_env_filter();
    let fmt_layer = layer().pretty();
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::trace!(target: "server:otel", "initialized subscriber");
    Ok(())
}
