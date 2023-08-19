use std::fs;
use std::{fs::File, sync::Arc};
use tracing_subscriber::{filter, prelude::*};

pub fn setup_logs(verbose: bool) {
    // get/create cache dir
    let cache_dir = match dirs::cache_dir() {
        Some(mut path) => {
            path.push("falion");
            if let Err(error) = fs::create_dir_all(&path) {
                eprintln!("Failed to create cache dir. Error: {}", error);
                return;
            }
            path
        }
        None => {
            eprintln!("Failed to get cache dir!");
            return;
        }
    };
    // move the contents from latest.log (if any) to another file
    let latest_log = cache_dir.join("latest.log");
    let older_log =
        cache_dir.join(chrono::Utc::now().format("%d-%m-%YT%H.%M.%S").to_string() + ".log");
    if let Err(error) = fs::copy(latest_log, older_log) {
        if error.kind() != std::io::ErrorKind::NotFound {
            eprintln!(
                "Failed to copy latest log to another file. Error: {}",
                error
            );
            return;
        }
    }

    // setup tracing subscriber
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    // let latest_log = cache_dir.
    let file = match File::create(cache_dir.join("latest.log")) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to create a latest.log file. Error: {:#?}", error);
            return;
        }
    };
    let mut debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .with_line_number(true)
        .with_file(true)
        .with_thread_ids(true)
        .with_target(false);
    debug_log.set_ansi(false);

    // register the layers
    if !verbose {
        let filter_debug_log = filter::Targets::new()
            .with_target("falion", filter::LevelFilter::DEBUG)
            .with_target("hyper", filter::LevelFilter::WARN);
        let filter_stdout_log = filter::Targets::new()
            .with_target("falion", filter::LevelFilter::WARN)
            .with_target("falion::search", filter::LevelFilter::OFF)
            .with_target("hyper", filter::LevelFilter::WARN);
        tracing_subscriber::registry()
            .with(
                stdout_log
                    // Add an `INFO` filter to the stdout logging layer
                    .with_filter(filter_stdout_log)
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log.with_filter(filter_debug_log)), // Add a filter to *both* layers that rejects spans and
                                                                        // events whose targets start with `metrics`.
            )
            .init();
    } else {
        let filter_debug_log = filter::Targets::new()
            .with_target("falion", filter::LevelFilter::DEBUG)
            .with_target("hyper", filter::LevelFilter::DEBUG);
        let filter_stdout_log = filter::Targets::new()
            .with_target("falion", filter::LevelFilter::DEBUG)
            .with_target("hyper", filter::LevelFilter::WARN);
        tracing_subscriber::registry()
            .with(
                stdout_log
                    // Add an `INFO` filter to the stdout logging layer
                    .with_filter(filter_stdout_log)
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log.with_filter(filter_debug_log)), // Add a filter to *both* layers that rejects spans and
                                                                        // events whose targets start with `metrics`.
            )
            .init();
    }
}
