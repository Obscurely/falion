use crate::util::setup_logs;
use clap::Parser;
use crossterm::terminal;
use std::io::Write;

/// Reset the terminal basically. Disable raw mode, reset colors, show cursor, clear screen
/// scroll up the terminal, move the cursor to the beginning.
///
/// # Arguments
///
/// * `stdout` - std::io::stdout(), you should have one in main that you reference to all your
/// functions for ideal performance and queue commands to it.
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
    if let Err(error) = crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::Purge)) {
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

/// Clear the terminal, scroll up, and move cursor to the beginning.
///
/// # Arguments
///
/// * `stdout` - std::io::stdout() you should have one in main that you reference to all your
/// functions for ideal performance and queue commands to it.
#[tracing::instrument(skip_all)]
pub fn clear_terminal(stdout: &mut std::io::Stdout) {
    if let Err(error) = crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::Purge)) {
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

/// Setup the cli. Setup the arguments for bin, get the given values and panic if a query equal or
/// long to 5 in length hasn't been given. Enable terminal raw mode, hide the cursor, setup
/// logging. Create an std::io::Stdout instance.
///
/// # Errors
///
/// If the user hasn't provided a query shorter than 5 chars or none at all
pub fn setup_cli() -> Result<String, std::io::Error> {
    // initiate cli
    let cli = super::Cli::parse();

    // first check for ui
    if cli.ui {
        crate::ui::ui();
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "User chose to run gui from cli",
        ));
    }

    // next check if for keybinds
    if cli.keybinds {
        print_keybindings();
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "User chose to print the keybinds list",
        ));
    }

    // get values
    let query = cli.query.join(" ");
    let verbose = cli.verbose;
    let disable_logs = cli.disable_logs;

    // check if query is not shorter than 5 characters
    if query.len() < 5 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "query shorter than 5 chars provided",
        ));
    }

    // Pre-setup
    // enable terminal raw mode
    if let Err(err) = terminal::enable_raw_mode() {
        panic!("Failed to enable raw mode: {}", err);
    }

    // enable (or not) logs based on flag
    if !disable_logs {
        setup_logs(verbose);
    }

    Ok(query)
}

/// Simple println statement to print the keybinds for the cli
pub fn print_keybindings() {
    let keybinds_list = r#"
Keybinds list for falion.
Note: where you see ".." it means from that to that i.e "1..5" would mean from 1 to 5.
   
Main menu:
[1..5] = Access that resource.
SHIFT + [1..5] = Go to the next element in the list of that resource.
ALT + [1..5] = Go to the previous element in the list of that resource.
n = Move to the next element in the list of every resource.
SHIFT + n = Move back to the previous element in the list of every resource.
CTRL + c = Clear terminal and exit.

Sub menus for the resources:
n = Move to the next element in the content list (like questions & answers).
SHIFT + n = Move back to the previous element in the content list.
q = Go back to the main menu.
CTRL + c = Clear terminal and exit.
    "#;
    println!("{keybinds_list}");
}
