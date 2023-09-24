mod dyn_content;
mod results;
mod static_content;
mod util;
use super::search;
use super::search::ddg_search::DdgSearchError;
use super::search::geeksforgeeks::GfgError;
use super::search::github_gist::GithubGistError;
use super::search::stackexchange::SeError;
use super::search::stackoverflow::SofError;
use dashmap::DashMap;
use results::display;
use results::helper;
use results::index;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

slint::include_modules!();

const MAIN_VIEW: i32 = 0;
const DYN_CONTENT_VIEW: i32 = 1;
const STATIC_CONTENT_VIEW: i32 = 2;

type StackOverflowResults =
    Option<Result<Vec<(String, JoinHandle<Result<Vec<String>, SofError>>)>, SofError>>;
type StackExchangeResults =
    Option<Result<Vec<(String, JoinHandle<Result<Vec<String>, SeError>>)>, SeError>>;
type GithubGistResults = Option<
    Result<Vec<(String, JoinHandle<Result<Vec<String>, GithubGistError>>)>, GithubGistError>,
>;
type GeeksForGeeksResults =
    Option<Result<Vec<(String, JoinHandle<Result<String, GfgError>>)>, GfgError>>;
type DdgSearchResults =
    Option<Result<Vec<(String, JoinHandle<Result<String, DdgSearchError>>)>, DdgSearchError>>;

type Results<T, E> = Result<Vec<(String, T)>, E>;
type ResultsStaticType<E, F> = Result<Vec<(String, JoinHandle<Result<String, E>>)>, F>;
type ResultsDynType<E, F> = Result<Vec<(String, JoinHandle<Result<Vec<String>, E>>)>, F>;

/// The main ui function that executes the window and sets it up.
#[tracing::instrument(skip_all)]
pub fn ui() {
    // firt setup logs
    crate::util::setup_logs(true);
    // continue
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
    let stackoverflow_results: Arc<RwLock<StackOverflowResults>> = Arc::new(RwLock::new(None));
    let stackexchange_results: Arc<RwLock<StackExchangeResults>> = Arc::new(RwLock::new(None));
    let github_gist_results: Arc<RwLock<GithubGistResults>> = Arc::new(RwLock::new(None));
    let geeksforgeeks_results: Arc<RwLock<GeeksForGeeksResults>> = Arc::new(RwLock::new(None));
    let ddg_search_results: Arc<RwLock<DdgSearchResults>> = Arc::new(RwLock::new(None));

    // make variables to store awaite results
    // create vars
    let stackoverflow_results_awaited: Arc<DashMap<String, Vec<String>>> =
        Arc::new(DashMap::with_capacity(5));
    let stackexchange_results_awaited: Arc<DashMap<String, Vec<String>>> =
        Arc::new(DashMap::with_capacity(5));
    let github_gist_results_awaited: Arc<DashMap<String, Vec<String>>> =
        Arc::new(DashMap::with_capacity(5));
    let geeksforgeeks_results_awaited: Arc<DashMap<String, String>> =
        Arc::new(DashMap::with_capacity(5));
    let ddg_search_results_awaited: Arc<DashMap<String, String>> =
        Arc::new(DashMap::with_capacity(5));

    // make variables to store the current index
    let stackoverflow_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let stackexchange_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let github_gist_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let geeksforgeeks_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let ddg_search_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));

    // make variables to store the current content index
    let stackoverflow_content_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let stackexchange_content_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let github_gist_content_index: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));

    // Event for when a search enter is hit
    main_window.on_query_enter({
        tracing::info!("Query enter event hit.");
        // get weak pointer the ui in order to use it in an event loop
        let ui_thread = main_window.as_weak();
        // get necessary ARC clones
        // resource objects
        let stackoverflow_clone = Arc::clone(&stackoverflow);
        let stackexchange_clone = Arc::clone(&stackexchange);
        let github_gist_clone = Arc::clone(&github_gist);
        let geeksforgeeks_clone = Arc::clone(&geeksforgeeks);
        let ddg_search_clone = Arc::clone(&ddg_search);
        // results
        let stackoverflow_results_clone = Arc::clone(&stackoverflow_results);
        let stackexchange_results_clone = Arc::clone(&stackexchange_results);
        let github_gist_results_clone = Arc::clone(&github_gist_results);
        let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results);
        let ddg_search_results_clone = Arc::clone(&ddg_search_results);
        // results awaited
        let stackoverflow_results_awaited_clone = Arc::clone(&stackoverflow_results_awaited);
        let stackexchange_results_awaited_clone = Arc::clone(&stackexchange_results_awaited);
        let github_gist_results_awaited_clone = Arc::clone(&github_gist_results_awaited);
        let geeksforgeeks_results_awaited_clone = Arc::clone(&geeksforgeeks_results_awaited);
        let ddg_search_results_awaited_clone = Arc::clone(&ddg_search_results_awaited);
        // indexes
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
            results::reset_results(ui_thread.clone());

            // disable search
            helper::disable_search(ui_thread.clone());

            // clone any ARCs we need
            // resource objects
            let stackoverflow_clone = Arc::clone(&stackoverflow_clone);
            let stackexchange_clone = Arc::clone(&stackexchange_clone);
            let github_gist_clone = Arc::clone(&github_gist_clone);
            let geeksforgeeks_clone = Arc::clone(&geeksforgeeks_clone);
            let ddg_search_clone = Arc::clone(&ddg_search_clone);
            // results
            let stackoverflow_results_clone = Arc::clone(&stackoverflow_results_clone);
            let stackexchange_results_clone = Arc::clone(&stackexchange_results_clone);
            let github_gist_results_clone = Arc::clone(&github_gist_results_clone);
            let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results_clone);
            let ddg_search_results_clone = Arc::clone(&ddg_search_results_clone);
            // results awaited
            let stackoverflow_results_awaited_clone = Arc::clone(&stackoverflow_results_awaited_clone);
            let stackexchange_results_awaited_clone = Arc::clone(&stackexchange_results_awaited_clone);
            let github_gist_results_awaited_clone = Arc::clone(&github_gist_results_awaited_clone);
            let geeksforgeeks_results_awaited_clone = Arc::clone(&geeksforgeeks_results_awaited_clone);
            let ddg_search_results_awaited_clone = Arc::clone(&ddg_search_results_awaited_clone);
            // indexes
            let stackoverflow_index_clone = Arc::clone(&stackoverflow_index_clone);
            let stackexchange_index_clone = Arc::clone(&stackexchange_index_clone);
            let github_gist_index_clone = Arc::clone(&github_gist_index_clone);
            let geeksforgeeks_index_clone = Arc::clone(&geeksforgeeks_index_clone);
            let ddg_search_index_clone = Arc::clone(&ddg_search_index_clone);

            // spawn task to get the results and show in a different thread.
            tokio::spawn(async move {
                // get result
                let so_res = stackoverflow_clone.get_multiple_questions_content(&text, Some(5));
                let se_res = stackexchange_clone.get_multiple_questions_content(&text, Some(5));
                let gg_res = github_gist_clone.get_multiple_gists_content(&text, Some(5));
                let gfg_res = geeksforgeeks_clone.get_multiple_pages_content(&text, Some(5));
                let ddg_res = ddg_search_clone.get_multiple_pages_content(&text, Some(5));

                // await all results at the same time
                let res = futures::join!(so_res, se_res, gg_res, gfg_res, ddg_res);

                // lock the mutex for the results in oder to update them
                let locked = futures::join!(
                    stackoverflow_results_clone.write(),
                    stackexchange_results_clone.write(),
                    github_gist_results_clone.write(),
                    geeksforgeeks_results_clone.write(),
                    ddg_search_results_clone.write(),
                );

                // take out the locks
                let mut stackoverflow_results_clone_lock = locked.0;
                let mut stackexchange_results_clone_lock = locked.1;
                let mut github_gist_results_clone_lock = locked.2;
                let mut geeksforgeeks_results_clone_lock = locked.3;
                let mut ddg_search_results_clone_lock = locked.4;

                // clear awaited results
                stackoverflow_results_awaited_clone.clear();
                stackexchange_results_awaited_clone.clear();
                github_gist_results_awaited_clone.clear();
                geeksforgeeks_results_awaited_clone.clear();
                ddg_search_results_awaited_clone.clear();

                // resest index to 0
                futures::join!(
                    index::reset_result_index(stackoverflow_index_clone),
                    index::reset_result_index(stackexchange_index_clone),
                    index::reset_result_index(github_gist_index_clone),
                    index::reset_result_index(geeksforgeeks_index_clone),
                    index::reset_result_index(ddg_search_index_clone),
                );

                // update results with the new ones
                stackoverflow_results_clone_lock.replace(res.0);
                stackexchange_results_clone_lock.replace(res.1);
                github_gist_results_clone_lock.replace(res.2);
                geeksforgeeks_results_clone_lock.replace(res.3);
                ddg_search_results_clone_lock.replace(res.4);

                // display the results and enable their respective buttons
                // using if let and not handling none since we just set values above
                // stachoverflow
                if let Some(results) = stackoverflow_results_clone_lock.as_ref() {
                    display::display_first_result(ui_thread.clone(), results, results::ResultType::StackOverflow);
                }
                // stackexchange
                if let Some(results) = stackexchange_results_clone_lock.as_ref() {
                    display::display_first_result(ui_thread.clone(), results, results::ResultType::StackExchange);
                }
                // github gist
                if let Some(results) = github_gist_results_clone_lock.as_ref() {
                    display::display_first_result(ui_thread.clone(), results, results::ResultType::GithubGist);
                }
                // GeeksForGeeks
                if let Some(results) = geeksforgeeks_results_clone_lock.as_ref() {
                    display::display_first_result(ui_thread.clone(), results, results::ResultType::GeeksForGeeks);
                }
                // Ddg Search
                if let Some(results) = ddg_search_results_clone_lock.as_ref() {
                    display::display_first_result(ui_thread.clone(), results, results::ResultType::DdgSearch);
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

                // enable back search
                helper::enable_search(ui_thread.clone());

                // log that we displayed the results successfully
                tracing::info!("Displayed the results successfully!");
            });
        }
    });

    // Event for when the button to move all results up by one is hit
    main_window.on_next_enter({
        tracing::info!("On next enter eventphit.");
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
            // clone the necessary ARCs
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
            // get a weak pointer to the main window
            let ui = ui.clone();
            tokio::task::spawn_blocking(move || {
                // try and up the index by one
                index::try_up_index(
                    Arc::clone(&stackoverflow_results_clone),
                    Arc::clone(&stackoverflow_index_clone),
                );
                index::try_up_index(
                    Arc::clone(&stackexchange_results_clone),
                    Arc::clone(&stackexchange_index_clone),
                );
                index::try_up_index(
                    Arc::clone(&github_gist_results_clone),
                    Arc::clone(&github_gist_index_clone),
                );
                index::try_up_index(
                    Arc::clone(&geeksforgeeks_results_clone),
                    Arc::clone(&geeksforgeeks_index_clone),
                );
                index::try_up_index(
                    Arc::clone(&ddg_search_results_clone),
                    Arc::clone(&ddg_search_index_clone),
                );

                // redisplay results
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&stackoverflow_results_clone),
                    Arc::clone(&stackoverflow_index_clone),
                    results::ResultType::StackOverflow,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&stackexchange_results_clone),
                    Arc::clone(&stackexchange_index_clone),
                    results::ResultType::StackExchange,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&github_gist_results_clone),
                    Arc::clone(&github_gist_index_clone),
                    results::ResultType::GithubGist,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&geeksforgeeks_results_clone),
                    Arc::clone(&geeksforgeeks_index_clone),
                    results::ResultType::GeeksForGeeks,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&ddg_search_results_clone),
                    Arc::clone(&ddg_search_index_clone),
                    results::ResultType::DdgSearch,
                );

                // log the end of the function
                tracing::info!("Up the results by one successfully and resdisplayed them.");
            });
        }
    });

    // Event for when the button move back all results by one is hit
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
            // clone the necessary ARCs
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
            // get a weak pointer to the main window
            let ui = ui.clone();

            tokio::task::spawn_blocking(move || {
                // try and up the index by one
                index::try_down_index(Arc::clone(&stackoverflow_index_clone));
                index::try_down_index(Arc::clone(&stackexchange_index_clone));
                index::try_down_index(Arc::clone(&github_gist_index_clone));
                index::try_down_index(Arc::clone(&geeksforgeeks_index_clone));
                index::try_down_index(Arc::clone(&ddg_search_index_clone));

                // redisplay results
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&stackoverflow_results_clone),
                    Arc::clone(&stackoverflow_index_clone),
                    results::ResultType::StackOverflow,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&stackexchange_results_clone),
                    Arc::clone(&stackexchange_index_clone),
                    results::ResultType::StackExchange,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&github_gist_results_clone),
                    Arc::clone(&github_gist_index_clone),
                    results::ResultType::GithubGist,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&geeksforgeeks_results_clone),
                    Arc::clone(&geeksforgeeks_index_clone),
                    results::ResultType::GeeksForGeeks,
                );
                display::redisplay_result(
                    ui.clone(),
                    Arc::clone(&ddg_search_results_clone),
                    Arc::clone(&ddg_search_index_clone),
                    results::ResultType::DdgSearch,
                );

                // log the end of the function
                tracing::info!("Down the results by one successfully and resdisplayed them.");
            });
        }
    });

    // setup individual move results buttons
    results::setup_results_btns(
        main_window.as_weak(),
        Arc::clone(&stackoverflow_results),
        Arc::clone(&stackoverflow_index),
        results::ResultType::StackOverflow,
    );
    results::setup_results_btns(
        main_window.as_weak(),
        Arc::clone(&stackexchange_results),
        Arc::clone(&stackexchange_index),
        results::ResultType::StackExchange,
    );
    results::setup_results_btns(
        main_window.as_weak(),
        Arc::clone(&github_gist_results),
        Arc::clone(&github_gist_index),
        results::ResultType::GithubGist,
    );
    results::setup_results_btns(
        main_window.as_weak(),
        Arc::clone(&geeksforgeeks_results),
        Arc::clone(&geeksforgeeks_index),
        results::ResultType::GeeksForGeeks,
    );
    results::setup_results_btns(
        main_window.as_weak(),
        Arc::clone(&ddg_search_results),
        Arc::clone(&ddg_search_index),
        results::ResultType::DdgSearch,
    );

    // setup displaying results content
    dyn_content::setup_content_display(
        main_window.as_weak(),
        Arc::clone(&stackoverflow_results),
        Arc::clone(&stackoverflow_results_awaited),
        Arc::clone(&stackoverflow_index),
        Arc::clone(&stackoverflow_content_index),
        results::ResultType::StackOverflow,
    );
    dyn_content::setup_content_display(
        main_window.as_weak(),
        Arc::clone(&stackexchange_results),
        Arc::clone(&stackexchange_results_awaited),
        Arc::clone(&stackexchange_index),
        Arc::clone(&stackexchange_content_index),
        results::ResultType::StackExchange,
    );
    dyn_content::setup_content_display(
        main_window.as_weak(),
        Arc::clone(&github_gist_results),
        Arc::clone(&github_gist_results_awaited),
        Arc::clone(&github_gist_index),
        Arc::clone(&github_gist_content_index),
        results::ResultType::GithubGist,
    );
    static_content::setup_content_display(
        main_window.as_weak(),
        Arc::clone(&geeksforgeeks_results),
        Arc::clone(&geeksforgeeks_results_awaited),
        Arc::clone(&geeksforgeeks_index),
        results::ResultType::GeeksForGeeks,
    );
    static_content::setup_content_display(
        main_window.as_weak(),
        Arc::clone(&ddg_search_results),
        Arc::clone(&ddg_search_results_awaited),
        Arc::clone(&ddg_search_index),
        results::ResultType::DdgSearch,
    );

    // setup content return button
    main_window.on_content_return_enter({
        let ui = main_window.as_weak();
        move || {
            let ui_clone = ui.clone();
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui = util::get_ui(ui_clone);

                // unwrap is fine since it there is always at least one element
                ui.set_view(MAIN_VIEW);
            }) {
                util::slint_event_loop_panic(err);
            };
        }
    });

    // show the window
    if let Err(err) = main_window.run() {
        tracing::error!("There was an error displaying the window. Error {}", err);
        panic!("There was an error displaying the window. Error {}", err);
    };
}
