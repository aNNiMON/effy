use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, prelude::*};

/// Initialize logging, currently logging to current directory
pub(crate) fn init_tracing() -> Option<WorkerGuard> {
    match std::env::var_os("RUST_LOG") {
        None => return None,
        Some(v) if v.is_empty() || v == "off" => return None,
        _ => {}
    }
    let file_appender = tracing_appender::rolling::never(".", "effy.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("effy=info")))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .with_writer(non_blocking),
        )
        .try_init()
        .ok()?;
    Some(guard)
}
