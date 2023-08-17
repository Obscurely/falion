mod search;
use clap::Parser;
use crossterm::style::{self, Stylize};
use crossterm::terminal;

#[tokio::main]
async fn main() {
    // initiate cli
    let cli = falion::Cli::parse();

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
        falion::clean(&mut stdout);
        panic!("Failed to enable raw mode: {}", err);
    }

    // enable (or not) logs based on flag
    if !disable_logs {
        falion::setup_logs(&mut stdout, verbose);
    }

    // debug log the query
    tracing::debug!("The input query: {}", &query);

    // Make objects
    let client = search::utils::client_with_special_settings();
    let stackoverflow = search::stackoverflow::StackOverflow::with_client(client.clone());
    let stackexchange = search::stackexchange::StackExchange::with_client(client.clone());
    let github_gist = search::github_gist::GithubGist::with_client(client.clone());
    let geeksforgeeks = search::geeksforgeeks::GeeksForGeeks::with_client(client.clone());
    let ddg_search = search::ddg_search::DdgSearch::with_client(client.clone());

    // Get results
    let stackoverflow_results = stackoverflow.get_multiple_questions_content(&query, Some(10));
    let stackexchange_results = stackexchange.get_multiple_questions_content(&query, Some(10));
    let github_gist_results = github_gist.get_multiple_gists_content(&query, Some(10));
    let geeksforgeeks_results = geeksforgeeks.get_multiple_pages_content(&query, Some(10));
    let ddg_search_results = ddg_search.get_multiple_pages_content(&query, Some(10));

    // await all results at the same time
    let results_awaited = futures::join!(
        stackoverflow_results,
        stackexchange_results,
        github_gist_results,
        geeksforgeeks_results,
        ddg_search_results
    );

    // transfer the awaited futures back
    let stackoverflow_results = results_awaited.0;
    let stackexchange_results = results_awaited.1;
    let github_gist_results = results_awaited.2;
    let geeksforgeeks_results = results_awaited.3;
    let ddg_search_results = results_awaited.4;

    // store wheter or not there are available results
    let stackoverflow_available = stackoverflow_results.is_ok();
    let stackexchange_available = stackexchange_results.is_ok();
    let github_gist_available = github_gist_results.is_ok();
    let geeksforgeeks_available = geeksforgeeks_results.is_ok();
    let ddg_search_available = ddg_search_results.is_ok();

    // set a current index of result sources
    let mut stackoverflow_index = 0;
    let mut stackexchange_index = 0;
    let mut github_gist_index = 0;
    let mut geeksforgeeks_index = 0;
    let mut ddg_search_index = 0;

    // actual cli
    // query print
    let query_print = format!("{} {}", "Your search query is:".green(), query.blue())
        .attribute(style::Attribute::Bold);
    // clear terminal
    falion::clear_terminal(&mut stdout);

    // display
    if let Err(error) = crossterm::queue!(&mut stdout, style::PrintStyledContent(query_print)) {
        tracing::warn!("There was an error printing some text. Error: {}", error);
    };

    crossterm::terminal::disable_raw_mode().unwrap();
    // falion::clean(&mut stdout);
}
