use crate::types::{CliError, CliResult};
use tracing_subscriber::{EnvFilter, Registry, fmt::format::JsonFields, prelude::*};

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init(environment: &str) -> CliResult<()> {
    let (is_production, filter) = match environment {
        "production" => (
            true,
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error")),
        ),
        _ => (
            false,
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        ),
    };

    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true);

    if is_production {
        let layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .event_format(format.json())
            .fmt_fields(JsonFields::new());

        let subscriber = Registry::default().with(filter).with(layer);

        tracing::subscriber::set_global_default(subscriber)
            .map_err(|err| CliError::ConfigError(err.to_string()))?;
    } else {
        let layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .event_format(format.pretty())
            .with_writer(std::io::stdout);

        let subscriber = Registry::default().with(filter).with(layer);

        tracing::subscriber::set_global_default(subscriber)
            .map_err(|err| CliError::ConfigError(err.to_string()))?;
    }

    Ok(())
}
