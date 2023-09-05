mod results;
mod util;
use super::search;
use super::search::ddg_search::DdgSearchError;
use super::search::geeksforgeeks::GfgError;
use super::search::github_gist::GithubGistError;
use super::search::stackexchange::SeError;
use super::search::stackoverflow::SofError;
use indexmap::IndexMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::task::JoinHandle;

slint::include_modules!();

type StackOverflowResults =
    Option<Result<IndexMap<String, JoinHandle<Result<Vec<String>, SofError>>>, SofError>>;
type StackExchangeResults =
    Option<Result<IndexMap<String, JoinHandle<Result<Vec<String>, SeError>>>, SeError>>;
type GithubGistResults = Option<
    Result<IndexMap<String, JoinHandle<Result<Vec<String>, GithubGistError>>>, GithubGistError>,
>;
type GeeksForGeeksResults =
    Option<Result<IndexMap<String, JoinHandle<Result<String, GfgError>>>, GfgError>>;
type DdgSearchResults =
    Option<Result<IndexMap<String, JoinHandle<Result<String, DdgSearchError>>>, DdgSearchError>>;

pub fn ui() {
    tracing::info!("User chose the GUI.");
    // main window
    let main_window = match MainWindow::new() {
        Ok(window) => window,
        Err(err) => {
            tracing::error!("There was an error creating the window. Error {}", err);
            panic!("Error creating the window. Error {}", err);
        }
    };

    // Make source objects
    let client = search::util::client_with_special_settings();
    let stackoverflow = Arc::new(search::stackoverflow::StackOverflow::with_client(
        client.clone(),
    ));
    let stackexchange = Arc::new(search::stackexchange::StackExchange::with_client(
        client.clone(),
    ));
    let github_gist = Arc::new(search::github_gist::GithubGist::with_client(client.clone()));
    let geeksforgeeks = Arc::new(search::geeksforgeeks::GeeksForGeeks::with_client(
        client.clone(),
    ));
    let ddg_search = Arc::new(search::ddg_search::DdgSearch::with_client(client.clone()));

    // make variables to store results
    let stackoverflow_results: Arc<Mutex<StackOverflowResults>> = Arc::new(Mutex::new(None));
    let stackexchange_results: Arc<Mutex<StackExchangeResults>> = Arc::new(Mutex::new(None));
    let github_gist_results: Arc<Mutex<GithubGistResults>> = Arc::new(Mutex::new(None));
    let geeksforgeeks_results: Arc<Mutex<GeeksForGeeksResults>> = Arc::new(Mutex::new(None));
    let ddg_search_results: Arc<Mutex<DdgSearchResults>> = Arc::new(Mutex::new(None));

    // make variables to store the current index
    let stackoverflow_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let stackexchange_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let github_gist_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let geeksforgeeks_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let ddg_search_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    // on query enter callback
    main_window.on_query_enter({
        tracing::info!("Query enter event hit.");
        // get weak pointer the ui in order to use it in an event loop
        let ui_thread = main_window.as_weak();
        let stackoverflow_clone = Arc::clone(&stackoverflow);
        let stackexchange_clone = Arc::clone(&stackexchange);
        let github_gist_clone = Arc::clone(&github_gist);
        let geeksforgeeks_clone = Arc::clone(&geeksforgeeks);
        let ddg_search_clone = Arc::clone(&ddg_search);
        let stackoverflow_results_clone = Arc::clone(&stackoverflow_results);
        let stackexchange_results_clone = Arc::clone(&stackexchange_results);
        let github_gist_results_clone = Arc::clone(&github_gist_results);
        let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results);
        let ddg_search_results_clone = Arc::clone(&ddg_search_results);
        let stackoverflow_index_clone = Arc::clone(&stackoverflow_index);
        let stackexchange_index_clone = Arc::clone(&stackexchange_index);
        let github_gist_index_clone = Arc::clone(&github_gist_index);
        let geeksforgeeks_index_clone = Arc::clone(&geeksforgeeks_index);
        let ddg_search_index_clone = Arc::clone(&ddg_search_index);
        move |text| {
            // clone the weak pointer in order to use it in event loops
            let ui_thread = ui_thread.clone();

            // check it the query is longer than 5 characters.
            if text.len() < 5 {
                tracing::info!("User tried searching for content with a query with less than 5 chars. Query: {}", text);
                if let Err(err) = slint::invoke_from_event_loop(move || {
                    let ui = util::get_ui(ui_thread);
                    ui.set_error("Error: Query is shorter than 5 characters, please provide a longer one.".into());
                }) {
                    util::slint_event_loop_panic(err);
                };
                return;
            }

            // log search query
            tracing::info!("Getting results for search query {}", &text);

            // reset results
            let ui_thread_clone = ui_thread.clone();
            results::reset_results(ui_thread_clone); 

            // clone any ARCs we need
            let stackoverflow_clone = Arc::clone(&stackoverflow_clone);
            let stackexchange_clone = Arc::clone(&stackexchange_clone);
            let github_gist_clone = Arc::clone(&github_gist_clone);
            let geeksforgeeks_clone = Arc::clone(&geeksforgeeks_clone);
            let ddg_search_clone = Arc::clone(&ddg_search_clone);
            let stackoverflow_results_clone = Arc::clone(&stackoverflow_results_clone);
            let stackexchange_results_clone = Arc::clone(&stackexchange_results_clone);
            let github_gist_results_clone = Arc::clone(&github_gist_results_clone);
            let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results_clone);
            let ddg_search_results_clone = Arc::clone(&ddg_search_results_clone);
            let stackoverflow_index_clone = Arc::clone(&stackoverflow_index_clone);
            let stackexchange_index_clone = Arc::clone(&stackexchange_index_clone);
            let github_gist_index_clone = Arc::clone(&github_gist_index_clone);
            let geeksforgeeks_index_clone = Arc::clone(&geeksforgeeks_index_clone);
            let ddg_search_index_clone = Arc::clone(&ddg_search_index_clone);

            // spawn task to get the results and show in a different thread.
            tokio::spawn(async move {
                let ui_thread = ui_thread.clone();
                // get result
                let so_res = stackoverflow_clone.get_multiple_questions_content(&text, Some(10));
                let se_res = stackexchange_clone.get_multiple_questions_content(&text, Some(10));
                let gg_res = github_gist_clone.get_multiple_gists_content(&text, Some(10));
                let gfg_res = geeksforgeeks_clone.get_multiple_pages_content(&text, Some(10));
                let ddg_res = ddg_search_clone.get_multiple_pages_content(&text, Some(10));

                // await all results at the same time
                let res = futures::join!(so_res, se_res, gg_res, gfg_res, ddg_res);

                // lock the mutex for the results in oder to update them
                let mut stackoverflow_results_clone_lock = match stackoverflow_results_clone.lock()
                {
                    Ok(res) => res,
                    Err(err) => {
                        util::poison_panic(err);
                        panic!(); // never actually reached
                    },
                };
                let mut stackexchange_results_clone_lock = match stackexchange_results_clone.lock()
                {
                    Ok(res) => res,
                    Err(err) => {
                        util::poison_panic(err);
                        panic!(); // never actually reached
                    },
                };
                let mut github_gist_results_clone_lock = match github_gist_results_clone.lock() {
                    Ok(res) => res,
                    Err(err) => {
                        util::poison_panic(err);
                        panic!(); // never actually reached
                    },
                };
                let mut geeksforgeeks_results_clone_lock = match geeksforgeeks_results_clone.lock()
                {
                    Ok(res) => res,
                    Err(err) => {
                        util::poison_panic(err);
                        panic!(); // never actually reached
                    },
                };
                let mut ddg_search_results_clone_lock = match ddg_search_results_clone.lock() {
                    Ok(res) => res,
                    Err(err) => {
                        util::poison_panic(err);
                        panic!(); // never actually reached
                    },
                };

                // resest index to 0
                results::reset_result_index(stackoverflow_index_clone);
                results::reset_result_index(stackexchange_index_clone);
                results::reset_result_index(github_gist_index_clone);
                results::reset_result_index(geeksforgeeks_index_clone);
                results::reset_result_index(ddg_search_index_clone);
 
                // update results with the new ones
                stackoverflow_results_clone_lock.replace(res.0);
                stackexchange_results_clone_lock.replace(res.1);
                github_gist_results_clone_lock.replace(res.2);
                geeksforgeeks_results_clone_lock.replace(res.3);
                ddg_search_results_clone_lock.replace(res.4);

                // display the results and enable their respective buttons
                // using if let and not handling none since we just set values above
                // stachoverflow
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = stackoverflow_results_clone_lock.as_ref() {
                    results::display_first_result(ui_thread_clone, results, results::ResultType::StackOverflow);
                }
                // stackexchange
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = stackexchange_results_clone_lock.as_ref() {
                    results::display_first_result(ui_thread_clone, results, results::ResultType::StackExchange);
                }
                // github gist
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = github_gist_results_clone_lock.as_ref() {
                    results::display_first_result(ui_thread_clone, results, results::ResultType::GithubGist);
                }
                // GeeksForGeeks
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = geeksforgeeks_results_clone_lock.as_ref() {
                    results::display_first_result(ui_thread_clone, results, results::ResultType::GeeksForGeeks);
                }
                // Ddg Search
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = ddg_search_results_clone_lock.as_ref() {
                    results::display_first_result(ui_thread_clone, results, results::ResultType::DdgSearch);
                }

                // Enable the next and bach buttons aswell
                let ui_thread_clone = ui_thread.clone();
                if let Err(err) = slint::invoke_from_event_loop(move || {
                    let ui = util::get_ui(ui_thread_clone);

                    ui.set_is_next(true);
                    ui.set_is_back(true);
                }) {
                    util::slint_event_loop_panic(err);
                };
                
                // log that we displayed the results successfully
                tracing::info!("Displayed the results successfully!");
            });
        }
    });

    main_window.on_next_enter({
        tracing::info!("On next enter event hit.");
        // clone the necessary ARCs
        let stackoverflow_results_clone = Arc::clone(&stackoverflow_results);
        let stackexchange_results_clone = Arc::clone(&stackexchange_results);
        let github_gist_results_clone = Arc::clone(&github_gist_results);
        let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results);
        let ddg_search_results_clone = Arc::clone(&ddg_search_results);
        let stackoverflow_index_clone = Arc::clone(&stackoverflow_index);
        let stackexchange_index_clone = Arc::clone(&stackexchange_index);
        let github_gist_index_clone = Arc::clone(&github_gist_index);
        let geeksforgeeks_index_clone = Arc::clone(&geeksforgeeks_index);
        let ddg_search_index_clone = Arc::clone(&ddg_search_index);
        // get a weak pointer to the main window
        let ui = main_window.as_weak();

        // actual closure
        move || {
            // clone again the necessary ARCs
            let stackoverflow_results_clone1 = Arc::clone(&stackoverflow_results_clone);
            let stackexchange_results_clone1 = Arc::clone(&stackexchange_results_clone);
            let github_gist_results_clone1 = Arc::clone(&github_gist_results_clone);
            let geeksforgeeks_results_clone1 = Arc::clone(&geeksforgeeks_results_clone);
            let ddg_search_results_clone1 = Arc::clone(&ddg_search_results_clone);
            let stackoverflow_index_clone1 = Arc::clone(&stackoverflow_index_clone);
            let stackexchange_index_clone1 = Arc::clone(&stackexchange_index_clone);
            let github_gist_index_clone1 = Arc::clone(&github_gist_index_clone);
            let geeksforgeeks_index_clone1 = Arc::clone(&geeksforgeeks_index_clone);
            let ddg_search_index_clone1 = Arc::clone(&ddg_search_index_clone);

            // try and up the index by one
            results::try_up_index(stackoverflow_results_clone1, stackoverflow_index_clone1);
            results::try_up_index(stackexchange_results_clone1, stackexchange_index_clone1);
            results::try_up_index(github_gist_results_clone1, github_gist_index_clone1);
            results::try_up_index(geeksforgeeks_results_clone1, geeksforgeeks_index_clone1);
            results::try_up_index(ddg_search_results_clone1, ddg_search_index_clone1);

            // clone ARCs again
            let stackoverflow_results_clone2 = Arc::clone(&stackoverflow_results_clone);
            let stackexchange_results_clone2 = Arc::clone(&stackexchange_results_clone);
            let github_gist_results_clone2 = Arc::clone(&github_gist_results_clone);
            let geeksforgeeks_results_clone2 = Arc::clone(&geeksforgeeks_results_clone);
            let ddg_search_results_clone2 = Arc::clone(&ddg_search_results_clone);
            let stackoverflow_index_clone2 = Arc::clone(&stackoverflow_index_clone);
            let stackexchange_index_clone2 = Arc::clone(&stackexchange_index_clone);
            let github_gist_index_clone2 = Arc::clone(&github_gist_index_clone);
            let geeksforgeeks_index_clone2 = Arc::clone(&geeksforgeeks_index_clone);
            let ddg_search_index_clone2 = Arc::clone(&ddg_search_index_clone);

            // redisplay results
            results::redisplay_result(
                ui.clone(),
                stackoverflow_results_clone2,
                stackoverflow_index_clone2,
                results::ResultType::StackOverflow,
            );
            results::redisplay_result(
                ui.clone(),
                stackexchange_results_clone2,
                stackexchange_index_clone2,
                results::ResultType::StackExchange,
            );
            results::redisplay_result(
                ui.clone(),
                github_gist_results_clone2,
                github_gist_index_clone2,
                results::ResultType::GithubGist,
            );
            results::redisplay_result(
                ui.clone(),
                geeksforgeeks_results_clone2,
                geeksforgeeks_index_clone2,
                results::ResultType::GeeksForGeeks,
            );
            results::redisplay_result(
                ui.clone(),
                ddg_search_results_clone2,
                ddg_search_index_clone2,
                results::ResultType::DdgSearch,
            );

            // log the end of the function
            tracing::info!("Up the results by one successfully and resdisplayed them.");
        }
    });

    main_window.on_back_enter({
        tracing::info!("On next enter event hit.");
        // clone the necessary ARCs
        let stackoverflow_results_clone = Arc::clone(&stackoverflow_results);
        let stackexchange_results_clone = Arc::clone(&stackexchange_results);
        let github_gist_results_clone = Arc::clone(&github_gist_results);
        let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results);
        let ddg_search_results_clone = Arc::clone(&ddg_search_results);
        let stackoverflow_index_clone = Arc::clone(&stackoverflow_index);
        let stackexchange_index_clone = Arc::clone(&stackexchange_index);
        let github_gist_index_clone = Arc::clone(&github_gist_index);
        let geeksforgeeks_index_clone = Arc::clone(&geeksforgeeks_index);
        let ddg_search_index_clone = Arc::clone(&ddg_search_index);
        // get a weak pointer to the main window
        let ui = main_window.as_weak();

        // actual closure
        move || {
            // clone again the necessary ARCs
            let stackoverflow_index_clone1 = Arc::clone(&stackoverflow_index_clone);
            let stackexchange_index_clone1 = Arc::clone(&stackexchange_index_clone);
            let github_gist_index_clone1 = Arc::clone(&github_gist_index_clone);
            let geeksforgeeks_index_clone1 = Arc::clone(&geeksforgeeks_index_clone);
            let ddg_search_index_clone1 = Arc::clone(&ddg_search_index_clone);

            // try and up the index by one
            results::try_down_index(stackoverflow_index_clone1);
            results::try_down_index(stackexchange_index_clone1);
            results::try_down_index(github_gist_index_clone1);
            results::try_down_index(geeksforgeeks_index_clone1);
            results::try_down_index(ddg_search_index_clone1);

            // clone ARCs again
            let stackoverflow_results_clone2 = Arc::clone(&stackoverflow_results_clone);
            let stackexchange_results_clone2 = Arc::clone(&stackexchange_results_clone);
            let github_gist_results_clone2 = Arc::clone(&github_gist_results_clone);
            let geeksforgeeks_results_clone2 = Arc::clone(&geeksforgeeks_results_clone);
            let ddg_search_results_clone2 = Arc::clone(&ddg_search_results_clone);
            let stackoverflow_index_clone2 = Arc::clone(&stackoverflow_index_clone);
            let stackexchange_index_clone2 = Arc::clone(&stackexchange_index_clone);
            let github_gist_index_clone2 = Arc::clone(&github_gist_index_clone);
            let geeksforgeeks_index_clone2 = Arc::clone(&geeksforgeeks_index_clone);
            let ddg_search_index_clone2 = Arc::clone(&ddg_search_index_clone);

            // redisplay results
            results::redisplay_result(
                ui.clone(),
                stackoverflow_results_clone2,
                stackoverflow_index_clone2,
                results::ResultType::StackOverflow,
            );
            results::redisplay_result(
                ui.clone(),
                stackexchange_results_clone2,
                stackexchange_index_clone2,
                results::ResultType::StackExchange,
            );
            results::redisplay_result(
                ui.clone(),
                github_gist_results_clone2,
                github_gist_index_clone2,
                results::ResultType::GithubGist,
            );
            results::redisplay_result(
                ui.clone(),
                geeksforgeeks_results_clone2,
                geeksforgeeks_index_clone2,
                results::ResultType::GeeksForGeeks,
            );
            results::redisplay_result(
                ui.clone(),
                ddg_search_results_clone2,
                ddg_search_index_clone2,
                results::ResultType::DdgSearch,
            );

            // log the end of the function
            tracing::info!("Up the results by one successfully and resdisplayed them.");
        }
    });

    // show the window
    if let Err(err) = main_window.run() {
        tracing::error!("There was an error displaying the window. Error {}", err);
        panic!("There was an error displaying the window. Error {}", err);
    };
}
