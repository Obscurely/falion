mod cli;
mod search;
mod ui;
mod util;

/// Main Falion execution
#[tokio::main]
async fn main() {
    // if falion is run from a terminal run the cli, if not run the ui.
    if atty::is(atty::Stream::Stdout) {
        cli::cli().await;
    } else {
        ui::ui();
    }
}
