mod search;
use clap::Parser;
use crossterm::terminal;

#[tokio::main]
async fn main() {
    // initiate cli
    let cli = falion::Cli::parse();

    // get stdout
    let mut stdout = std::io::stdout();

    // get values
    let _query = cli.query.join(" ");
    let _verbose = cli.verbose;

    // Pre-setup
    match terminal::disable_raw_mode() {
        Ok(_) => (),
        Err(err) => {
            falion::clean_exit(
                &mut stdout,
                format!("Failed to disable raw mode: {}", err).as_str(),
            );
        }
    }
}
