use std::io::{stdout, IsTerminal};
mod cli;
mod search;
mod ui;
mod util;

/// Main Falion execution
#[tokio::main]
async fn main() {
    // if falion is run from a terminal run the cli, if not run the ui.
    if stdout().is_terminal() {
        cli::cli().await;
    } else {
        ui::ui();
    }
}
