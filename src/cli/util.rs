use crate::util::setup_logs;
use clap::Parser;
use crossterm::terminal;
use std::io::Write;

#[tracing::instrument(skip_all)]
pub fn clean(stdout: &mut std::io::Stdout) {
    if let Err(error) = crossterm::terminal::disable_raw_mode() {
        tracing::warn!("Failed to disable termial raw mode! Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::style::ResetColor) {
        tracing::warn!("Failed to reset term collor. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::cursor::Show) {
        tracing::warn!("Failed to show back cursor. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::All)) {
        tracing::warn!("Failed to clear terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, terminal::ScrollUp(u16::MAX)) {
        tracing::warn!("Failed to scroll up the terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::cursor::MoveTo(0, 0)) {
        tracing::warn!("Failed to move terminal cursor. Error: {}", error);
    }

    if let Err(error) = stdout.flush() {
        tracing::warn!("Failed to flush stdout. Error: {}", error);
    };
}

#[tracing::instrument(skip_all)]
pub fn clear_terminal(stdout: &mut std::io::Stdout) {
    if let Err(error) = crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::All)) {
        tracing::warn!("Failed to clear terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, terminal::ScrollUp(u16::MAX)) {
        tracing::warn!("Failed to scroll up the terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::cursor::MoveTo(0, 0)) {
        tracing::warn!("Failed to move terminal cursor. Error: {}", error);
    }

    if let Err(error) = stdout.flush() {
        tracing::warn!("Failed to flush stdout. Error: {}", error);
    }
}

pub fn setup_cli() -> String {
    // initiate cli
    let cli = super::Cli::parse();

    // get values
    let query = cli.query.join(" ");
    let verbose = cli.verbose;
    let disable_logs = cli.disable_logs;

    // check if query is not shorter than 5 characters
    if query.len() < 5 {
        panic!("\n-> Your query is shorter than 5 characters <-\n");
    }

    // get stdout
    let mut stdout = std::io::stdout();

    // Pre-setup
    // enable terminal raw mode
    if let Err(err) = terminal::enable_raw_mode() {
        panic!("Failed to enable raw mode: {}", err);
    }

    // hide the cursor
    if let Err(error) = crossterm::execute!(&mut stdout, crossterm::cursor::Hide) {
        tracing::warn!("Failed to hide terminal cursor. Error: {}", error);
    };

    // enable (or not) logs based on flag
    if !disable_logs {
        setup_logs(verbose);
    }

    query
}
