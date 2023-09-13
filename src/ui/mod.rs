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
use indexmap::IndexMap;
use results::display;
use results::helper;
use results::index;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

slint::include_modules!();

const MAIN_VIEW: i32 = 0;
const DYN_CONTENT_VIEW: i32 = 1;
const STATIC_CONTENT_VIEW: i32 = 2;

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

type Results<T, E> = Result<IndexMap<String, T>, E>;
type ResultsStaticType<E, F> = Result<IndexMap<String, JoinHandle<Result<String, E>>>, F>;
type ResultsDynType<E, F> = Result<IndexMap<String, JoinHandle<Result<Vec<String>, E>>>, F>;

/// The main ui function that executes the window and sets it up.
#[tracing::instrument(skip_all)]
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

    // make variables to store awaite results
    // create vars
    let stackoverflow_results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(IndexMap::new()));
    let stackexchange_results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(IndexMap::new()));
    let github_gist_results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(IndexMap::new()));
    let geeksforgeeks_results_awaited: Arc<Mutex<IndexMap<String, String>>> =
        Arc::new(Mutex::new(IndexMap::new()));
    let ddg_search_results_awaited: Arc<Mutex<IndexMap<String, String>>> =
        Arc::new(Mutex::new(IndexMap::new()));

    // make variables to store the current index
    let stackoverflow_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let stackexchange_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let github_gist_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let geeksforgeeks_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let ddg_search_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    // make variables to store the current content index
    let stackoverflow_content_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let stackexchange_content_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let github_gist_content_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

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
                let so_res = stackoverflow_clone.get_multiple_questions_content(&text, Some(10));
                let se_res = stackexchange_clone.get_multiple_questions_content(&text, Some(10));
                let gg_res = github_gist_clone.get_multiple_gists_content(&text, Some(10));
                let gfg_res = geeksforgeeks_clone.get_multiple_pages_content(&text, Some(10));
                let ddg_res = ddg_search_clone.get_multiple_pages_content(&text, Some(10));

                // await all results at the same time
                let res = futures::join!(so_res, se_res, gg_res, gfg_res, ddg_res);

                // lock the mutex for the results in oder to update them
                let locked = futures::join!(
                    stackoverflow_results_clone.lock(),
                    stackexchange_results_clone.lock(),
                    github_gist_results_clone.lock(),
                    geeksforgeeks_results_clone.lock(),
                    ddg_search_results_clone.lock(),
                );

                let mut stackoverflow_results_clone_lock = locked.0;
                let mut stackexchange_results_clone_lock = locked.1;
                let mut github_gist_results_clone_lock = locked.2;
                let mut geeksforgeeks_results_clone_lock = locked.3;
                let mut ddg_search_results_clone_lock = locked.4;

                // get the locks for results awaited too, in order to reset them
                let mut locked = futures::join!(
                    stackoverflow_results_awaited_clone.lock(),
                    stackexchange_results_awaited_clone.lock(),
                    github_gist_results_awaited_clone.lock(),
                    geeksforgeeks_results_awaited_clone.lock(),
                    ddg_search_results_awaited_clone.lock(),
                );

                // clear awaited results
                *locked.0 = IndexMap::new();
                *locked.1 = IndexMap::new();
                *locked.2 = IndexMap::new();
                *locked.3 = IndexMap::new();
                *locked.4 = IndexMap::new();

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
