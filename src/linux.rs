use std::sync::Arc;

use opentelemetry::KeyValue;
use opentelemetry_semantic_conventions::attribute::{
    CPU_MODE, DISK_IO_DIRECTION, PROCESS_CONTEXT_SWITCH_TYPE,
};
use opentelemetry_semantic_conventions::metric::{
    PROCESS_CONTEXT_SWITCHES, PROCESS_CPU_TIME, PROCESS_DISK_IO, PROCESS_MEMORY_USAGE,
    PROCESS_MEMORY_VIRTUAL, PROCESS_OPEN_FILE_DESCRIPTOR_COUNT, PROCESS_THREAD_COUNT,
};
use procfs::process::Process;
use procfs::{WithCurrentSystemInfo, ticks_per_second};

use crate::METER;

macro_rules! expect {
    ($res:expr) => {
        match $res {
            Ok(v) => v,
            Err(e) => {
                opentelemetry::otel_warn!(
                    name: "opentelemetry-instrumentation-process",
                    error = format!("{e}"),
                );
                return;
            }
        }
    };
}

#[allow(clippy::too_many_lines)]
pub(crate) fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let me = Arc::new(Process::myself()?);

    #[allow(clippy::cast_precision_loss)]
    {
        let me = Arc::clone(&me);
        METER
            .f64_observable_counter(PROCESS_CPU_TIME)
            .with_unit("s")
            .with_description("Total CPU seconds broken down by different states.")
            .with_callback(move |instrument| {
                let stat = expect!(me.stat());
                instrument.observe(
                    stat.utime as f64 / ticks_per_second() as f64,
                    &[KeyValue::new(CPU_MODE, "user")],
                );
                instrument.observe(
                    stat.stime as f64 / ticks_per_second() as f64,
                    &[KeyValue::new(CPU_MODE, "system")],
                );
            })
            .build();
    }

    {
        let me = Arc::clone(&me);
        METER
            .u64_observable_gauge(PROCESS_MEMORY_USAGE)
            .with_unit("By")
            .with_description("The amount of physical memory in use.")
            .with_callback(move |instrument| {
                let stat = expect!(me.stat());
                instrument.observe(stat.rss_bytes().get(), &[]);
            })
            .build();
    }

    {
        let me = Arc::clone(&me);
        METER
            .u64_observable_gauge(PROCESS_MEMORY_VIRTUAL)
            .with_unit("By")
            .with_description("The amount of committed virtual memory.")
            .with_callback(move |instrument| {
                let stat = expect!(me.stat());
                instrument.observe(stat.vsize, &[]);
            })
            .build();
    }

    {
        let me = Arc::clone(&me);
        METER
            .u64_observable_counter(PROCESS_DISK_IO)
            .with_unit("By")
            .with_description("Disk bytes transferred.")
            .with_callback(move |instrument| {
                let stat = expect!(me.io());
                instrument.observe(stat.read_bytes, &[KeyValue::new(DISK_IO_DIRECTION, "read")]);
                instrument.observe(
                    stat.write_bytes,
                    &[KeyValue::new(DISK_IO_DIRECTION, "write")],
                );
            })
            .build();
    }

    {
        let me = Arc::clone(&me);
        METER
            .i64_observable_gauge(PROCESS_THREAD_COUNT)
            .with_unit("{thread}")
            .with_description("Process threads count.")
            .with_callback(move |instrument| {
                let stat = expect!(me.stat());
                instrument.observe(stat.num_threads, &[]);
            })
            .build();
    }

    {
        let me = Arc::clone(&me);
        METER
            .u64_observable_gauge(PROCESS_OPEN_FILE_DESCRIPTOR_COUNT)
            .with_unit("{file_descriptor}")
            .with_description("Number of file descriptors in use by the process.")
            .with_callback(move |instrument| {
                let count = expect!(me.fd_count());
                instrument.observe(count as u64, &[]);
            })
            .build();
    }

    {
        let me = Arc::clone(&me);
        METER
            .u64_observable_gauge(PROCESS_CONTEXT_SWITCHES)
            .with_unit("{context_switch}")
            .with_description("Number of times the process has been context switched.")
            .with_callback(move |instrument| {
                let status = expect!(me.status());
                if let Some(switches) = status.voluntary_ctxt_switches {
                    instrument.observe(
                        switches,
                        &[KeyValue::new(PROCESS_CONTEXT_SWITCH_TYPE, "voluntary")],
                    );
                }
                if let Some(switches) = status.nonvoluntary_ctxt_switches {
                    instrument.observe(
                        switches,
                        &[KeyValue::new(PROCESS_CONTEXT_SWITCH_TYPE, "involuntary")],
                    );
                }
            })
            .build();
    }

    Ok(())
}
