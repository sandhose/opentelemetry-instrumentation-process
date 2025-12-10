#![doc = include_str!("../README.md")]
#![deny(clippy::all, clippy::pedantic)]

use std::sync::LazyLock;

use opentelemetry::InstrumentationScope;
use opentelemetry::metrics::Meter;

#[allow(dead_code, reason = "Might be unused on unsupported platforms")]
static METER: LazyLock<Meter> = LazyLock::new(|| {
    let scope = InstrumentationScope::builder(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_schema_url(opentelemetry_semantic_conventions::SCHEMA_URL)
        .build();

    opentelemetry::global::meter_with_scope(scope)
});

#[cfg(target_os = "linux")]
mod linux;

/// The error returned when process metrics initialisation fails.
#[derive(Debug)]
pub struct InitError {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not initialise process metrics")
    }
}

impl std::error::Error for InitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// Initialise process metrics collection.
///
/// # Errors
///
/// Returns an error if initialisation fails.
pub fn init() -> Result<(), InitError> {
    #[cfg(target_os = "linux")]
    self::linux::init().map_err(|source| InitError { source })?;

    Ok(())
}
