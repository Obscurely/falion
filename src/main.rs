mod search;
use clap::Parser;
use crossterm::style::{self, Stylize};
use crossterm::terminal;
use indexmap::IndexMap;

#[tokio::main]
async fn main() {
    let now = std::time::Instant::now();
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

    // set a current index of result resources
    let mut stackoverflow_index = 0;
    let mut stackexchange_index = 0;
    let mut github_gist_index = 0;
    let mut geeksforgeeks_index = 0;
    let mut ddg_search_index = 0;

    // set results awaited vars
    // let mut stackoverflow_results_awaited = IndexMap::new();
    // let mut stackexchange_results_awaited = IndexMap::new();
    // let mut github_gist_results_awaited = IndexMap::new();
    // let mut geeksforgeeks_results_awaited = IndexMap::new();
    // let mut ddg_search_results_awaited = IndexMap::new();

    // actual cli
    // reusable prints
    let query_print = format!("{} {}", "Your search query is:".green(), query.blue()).stylize();
    let sof_print = format!("{} {} ", "(1)".green(), "[  StackOverFlow  ]".yellow());
    let se_print = format!("{} {} ", "(2)".green(), "[  StackExchange  ]".yellow());
    let gg_print = format!("{} {} ", "(3)".green(), "[   Github Gist   ]".yellow());
    let gfg_print = format!("{} {} ", "(4)".green(), "[  GeeksForGeeks  ]".yellow());
    let ddg_print = format!("{} {} ", "(5)".green(), "[DuckDuckGo Search]".yellow());
    // clear terminal
    falion::clear_terminal(&mut stdout);

    // display query
    if let Err(error) = crossterm::queue!(
        &mut stdout,
        style::PrintStyledContent(query_print),
        style::Print("\n\r")
    ) {
        tracing::warn!("There was an error printing some text. Error: {}", error);
    };

    // display resources
    falion::print_resource(
        &mut stdout,
        stackoverflow_index,
        &sof_print,
        &stackoverflow_results,
    );
    falion::print_resource(
        &mut stdout,
        stackexchange_index,
        &se_print,
        &stackexchange_results,
    );
    falion::print_resource(
        &mut stdout,
        github_gist_index,
        &gg_print,
        &github_gist_results,
    );
    falion::print_resource(
        &mut stdout,
        geeksforgeeks_index,
        &gfg_print,
        &geeksforgeeks_results,
    );
    falion::print_resource(
        &mut stdout,
        ddg_search_index,
        &ddg_print,
        &ddg_search_results,
    );

    let total = std::time::Instant::now() - now;

    crossterm::terminal::disable_raw_mode().unwrap();
    // falion::clean(&mut stdout);
    println!("Total execution time: {}ms", total.as_millis());
}
