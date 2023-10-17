use std::io::{stdout, IsTerminal};
mod cli;
mod search;
mod ui;
mod util;

/// Main Falion execution
#[tokio::main]
async fn main() {
    if stdout().is_terminal() {
        cli::cli().await;
    } else {
        ui::ui();
    }
}
