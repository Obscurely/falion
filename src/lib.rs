pub mod search;
use clap::Parser;
use crossterm::terminal;
use std::fs;
use std::{fs::File, sync::Arc};
use tracing_subscriber::{filter, prelude::*};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Search query
    pub query: Vec<String>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable logs
    #[arg(short, long)]
    pub disable_logs: bool,
}

pub fn clean(stdout: &mut std::io::Stdout) {
    let _ = terminal::disable_raw_mode();
    let _ = crossterm::execute!(stdout, crossterm::style::ResetColor);
    let _ = crossterm::execute!(stdout, terminal::Clear(terminal::ClearType::All));
    let _ = crossterm::execute!(stdout, terminal::ScrollUp(u16::MAX));
    let _ = crossterm::execute!(stdout, crossterm::cursor::MoveTo(0, 0));
}

pub fn setup_logs(stdout: &mut std::io::Stdout, verbose: bool) {
    // get/create cache dir
    let cache_dir = match dirs::cache_dir() {
        Some(mut path) => {
            path.push("falion");
            if let Err(error) = fs::create_dir_all(&path) {
                clean(stdout);
                panic!("Failed to create cache dir. Error: {}", error);
            }
            path
        }
        None => {
            clean(stdout);
            panic!("Failed to get cache dir!");
        }
    };
    // move the contents from latest.log (if any) to another file
    let latest_log = cache_dir.join("latest.log");
    let older_log =
        cache_dir.join(chrono::Utc::now().format("%d-%m-%YT%H.%M.%S").to_string() + ".log");
    if let Err(error) = fs::copy(latest_log, older_log) {
        if error.kind() != std::io::ErrorKind::NotFound {
            clean(stdout);
            panic!(
                "Failed to copy latest log to another file. Error: {}",
                error
            );
        }
    }

    // setup tracing subscriber
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    // let latest_log = cache_dir.
    let file = match File::create(cache_dir.join("latest.log")) {
        Ok(file) => file,
        Err(error) => {
            clean(stdout);
            panic!("Failed to create a latest.log file. Error: {:#?}", error)
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
        tracing_subscriber::registry()
            .with(
                stdout_log
                    // Add an `INFO` filter to the stdout logging layer
                    .with_filter(filter::LevelFilter::ERROR)
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log), // Add a filter to *both* layers that rejects spans and
                                          // events whose targets start with `metrics`.
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(
                stdout_log
                    // Add an `INFO` filter to the stdout logging layer
                    .with_filter(filter::LevelFilter::DEBUG)
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log), // Add a filter to *both* layers that rejects spans and
                                          // events whose targets start with `metrics`.
            )
            .init();
    }
}
