# OpenTelemetry process instrumentation

A process instrumentation for the `opentelemetry` Rust crate that captures key metrics about the running process, such as CPU and memory usage.

[![Crates.io](https://img.shields.io/crates/v/opentelemetry-instrumentation-process.svg)](https://crates.io/crates/opentelemetry-instrumentation-process)
[![Documentation](https://img.shields.io/docsrs/opentelemetry-instrumentation-process.svg)](https://docs.rs/opentelemetry-instrumentation-process)
[![Apache 2.0 License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
opentelemetry-instrumentation-process = "0.1.0"
```

## Usage

```rust
opentelmetry_instrumentation_process::init().unwrap();
```

## OS support

This library only supports Linux for now. It is a no-op on other operating systems.
