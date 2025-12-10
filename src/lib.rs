#![doc = include_str!("../README.md")]
#![deny(clippy::all, clippy::pedantic)]

use std::sync::LazyLock;

use opentelemetry::InstrumentationScope;
use opentelemetry::metrics::Meter;

static METER: LazyLock<Meter> = LazyLock::new(|| {
    let scope = InstrumentationScope::builder(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_schema_url(opentelemetry_semantic_conventions::SCHEMA_URL)
        .build();

    opentelemetry::global::meter_with_scope(scope)
});

#[cfg(target_os = "linux")]
mod linux;

/// Initialise process metrics collection.
///
/// # Errors
///
/// Returns an error if initialisation fails.
pub fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    #[cfg(target_os = "linux")]
    self::linux::init()?;

    Ok(())
}
