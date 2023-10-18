use std::io::{stdout, IsTerminal};
mod cli;
mod search;
mod ui;
mod util;

#[cfg(windows)]
fn is_parent_explorer() -> Option<bool> {
    use std::process::Command;
    // Use the "wmic" command to retrieve the parent process ID
    let output = Command::new("wmic")
        .args(["process", "get", "ParentProcessId"])
        .output()
        .ok()?;
    
    // Parse the output to extract the parent process ID
    let output_str = String::from_utf8_lossy(&output.stdout);
    let parent_id: &str = output_str.trim().lines().last()?;

    let output = Command::new("wmic")
        .args(["process", "where", format!("processId={}", parent_id).as_str(), "get", "name"])
        .output()
        .ok()?;

    if String::from_utf8_lossy(&output.stdout).contains("explorer.exe") {
        Some(true)
    } else {
        Some(false)
    }
}

#[cfg(not(windows))]
fn is_parent_explorer() -> Option<bool> {
    None
}

/// Main Falion execution
#[tokio::main]
async fn main() {
    if stdout().is_terminal() {
        match is_parent_explorer() {
            Some(explorer) => {
                if explorer {
                    ui::ui();
                } else {
                    cli::cli().await;
                }
            }
            None => cli::cli().await,
        }
    } else {
        ui::ui();
    }
}
