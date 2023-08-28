mod cli;
mod search;
mod ui;
mod util;

/// Main Falion execution
#[tokio::main]
async fn main() {
    // setup cli and get query
    let query = cli::setup_cli();

    // debug log the query
    tracing::debug!("The input query: {}", &query);

    // Make objects
    let client = search::util::client_with_special_settings();
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

    // run cli
    // cli::cli(
    //     query,
    //     stackoverflow_results,
    //     stackexchange_results,
    //     github_gist_results,
    //     geeksforgeeks_results,
    //     ddg_search_results,
    // )
    // .await;
    ui::ui().unwrap();
}
