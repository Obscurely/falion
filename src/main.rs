mod search;
use clap::Parser;
use crossterm::terminal;

#[tokio::main]
async fn main() {
    // initiate cli
    let cli = falion::Cli::parse();

    // get values
    let query = cli.query.join(" ");
    let verbose = cli.verbose;
    let disable_logs = cli.disable_logs;

    // get stdout
    let mut stdout = std::io::stdout();

    // Pre-setup
    // make sure terminal raw mode is not enabled
    match terminal::disable_raw_mode() {
        Ok(_) => (),
        Err(err) => {
            falion::clean(&mut stdout);
            panic!("Failed to disable raw mode: {}", err);
        }
    }

    if !disable_logs {
        falion::setup_logs(&mut stdout, verbose);
    }

    tracing::debug!("The input query: {} | Verbose flag: {}", &query, &verbose);
}
