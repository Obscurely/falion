use std::fs;
use std::{fs::File, sync::Arc};
use tracing_subscriber::{filter, prelude::*};

/// Setup logging using tracing crate.
///
/// # Arguments
///
/// * `verbose` if set to true the stdout log output will also show debug information. Log written
/// to file always have debug output option set.
pub fn setup_logs(verbose: bool) {
    // get/create cache dir
    let cache_dir = match dirs::cache_dir() {
        Some(mut path) => {
            path.push("falion");
            if let Err(error) = fs::create_dir_all(&path) {
                eprintln!("Failed to create cache dir. Error: {}", error);
                return;
            }

            // put the logs in Temp folder from local appdata on windows
            #[cfg(windows)]
            {
                path.push("Temp");
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

/// Check if the parent process in explorer.exe on windows.
#[cfg(windows)]
pub fn is_parent_explorer() -> Option<bool> {
    use std::process::Command;
    // Use the "wmic" command to retrieve the parent process IDs
    let output = Command::new("wmic")
        .args(["process", "get", "ParentProcessId"])
        .output()
        .ok()?;

    // Parse the output to extract the parent process ID
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut parent_id = output_str.trim().lines().rev();
    // the third process from the back in the list is the actual parent process
    parent_id.next();
    parent_id.next();
    let parent_id = parent_id.next()?.trim();

    let output = Command::new("wmic")
        .args([
            "process",
            "where",
            format!("processId={}", parent_id).as_str(),
            "get",
            "name",
        ])
        .output()
        .ok()?;

    if String::from_utf8_lossy(&output.stdout).contains("explorer.exe") {
        Some(true)
    } else {
        Some(false)
    }
}

/// Returns None since we don't need to get the parent process on linux
#[cfg(not(windows))]
pub fn is_parent_explorer() -> Option<bool> {
    None
}

#[cfg(windows)]
pub fn hide_console_window() {
    use std::ptr;
    let window = unsafe { kernel32::GetConsoleWindow() };
    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms633548%28v=vs.85%29.aspx
    if window != ptr::null_mut() {
        unsafe { user32::ShowWindow(window, winapi::um::winuser::SW_HIDE) };
    }
}

#[cfg(not(windows))]
pub fn hide_console_window() {}
