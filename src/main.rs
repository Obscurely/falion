use std::io::{stdout, IsTerminal};
mod cli;
mod search;
mod ui;
mod util;

/// Main Falion execution
#[tokio::main]
async fn main() {
    // If the app is run from a terminal run the cli, otherwise the gui
    if stdout().is_terminal() {
        match util::is_parent_explorer() {
            Some(explorer) => {
                if explorer {
                    util::hide_console_window();
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
