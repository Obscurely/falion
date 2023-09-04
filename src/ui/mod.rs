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
    let main_window = MainWindow::new().unwrap();

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
    let stackoverflow_index = Arc::new(Mutex::new(0));
    let stackexchange_index = Arc::new(Mutex::new(0));
    let github_gist_index = Arc::new(Mutex::new(0));
    let geeksforgeeks_index = Arc::new(Mutex::new(0));
    let ddg_search_index = Arc::new(Mutex::new(0));

    // on query enter callback
    main_window.on_query_enter({
        tracing::info!("Query enter event hit.");
        // get weak pointer the ui in order to use it in an event loop
        let ui_thread = main_window.as_weak();
        // main_window.set_sof_result("test".into());
        // let stackoverflow_clone = Arc::clone(&stackoverflow);
        // let stackexchange_clone = Arc::clone(&stackexchange);
        // let github_gist_clone = Arc::clone(&github_gist);
        // let geeksforgeeks_clone = Arc::clone(&geeksforgeeks);
        // let ddg_search_clone = Arc::clone(&ddg_search);
        let stackoverflow_results_clone = Arc::clone(&stackoverflow_results);
        // let stackexchange_results_clone = Arc::clone(&stackexchange_results);
        // let github_gist_results_res_clone = Arc::clone(&github_gist_results);
        // let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results);
        // let ddg_search_results_clone = Arc::clone(&ddg_search_results);
        let stackoverflow_index_clone = Arc::clone(&stackoverflow_index);
        move |text| {
            // clone the weak pointer in order to use it in event loops
            let ui_thread = ui_thread.clone();
            // check it the query is longer than 5 characters.
            if text.len() < 5 {
                tracing::info!("User tried searching for content with a query with less than 5 chars. Query: {}", text);
                if let Err(err) = slint::invoke_from_event_loop(move || {
                    let ui = match ui_thread.upgrade() {
                        Some(ui) => ui,
                        None => {
                            tracing::error!("Failed to get ui thread behind weak pointer.");
                            // it the pointer to the UI is invalid probably the program is not working anymore.
                            panic!("Failed to get pointer to ui.");
                        }
                    };
                    ui.set_error("Error: Query is shorter than 5 characters, please provide a longer one.".into());
                }) {
                    tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                    // if we can't invoke slint from the event loop it's probably right to panic as
                    // the program is not responding.
                    panic!("Failed to invoke slint from event loop. Error {}", err);
                };
                return;
            }
            // reset results
            let ui_thread_clone = ui_thread.clone();
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui = match ui_thread_clone.upgrade() {
                    Some(ui) => ui,
                    None => {
                        tracing::error!("Failed to get ui thread behind weak pointer.");
                        // it the pointer to the UI is invalid probably the program is not working anymore.
                        panic!("Failed to get pointer to ui.");
                    }
                };

                let space_string = slint::SharedString::from(" ");

                ui.set_sof_result(space_string.clone());
                ui.set_is_sof(false);
                ui.set_se_result(space_string.clone());
                ui.set_is_se(false);
                ui.set_gg_result(space_string.clone());
                ui.set_is_gg(false);
                ui.set_gfg_result(space_string.clone());
                ui.set_is_gfg(false);
                ui.set_ddg_result(space_string.clone());
                ui.set_is_ddg(false);
                ui.set_is_back(false);
                ui.set_is_next(false);
                ui.set_error(space_string);
            }) {
                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                // if we can't invoke slint from the event loop it's probably right to panic as
                // the program is not responding.
                panic!("Failed to invoke slint from event loop. Error {}", err);
            };
            // clone any ARCs we need
            let stackoverflow_clone = Arc::clone(&stackoverflow);
            let stackexchange_clone = Arc::clone(&stackexchange);
            let github_gist_clone = Arc::clone(&github_gist);
            let geeksforgeeks_clone = Arc::clone(&geeksforgeeks);
            let ddg_search_clone = Arc::clone(&ddg_search);
            let stackoverflow_results_clone = Arc::clone(&stackoverflow_results_clone);
            let stackexchange_results_clone = Arc::clone(&stackexchange_results);
            let github_gist_results_clone = Arc::clone(&github_gist_results);
            let geeksforgeeks_results_clone = Arc::clone(&geeksforgeeks_results);
            let ddg_search_results_clone = Arc::clone(&ddg_search_results);
            let stackoverflow_index_clone = Arc::clone(&stackoverflow_index_clone);
            let stackexchange_index_clone = Arc::clone(&stackexchange_index);
            let github_gist_index_clone = Arc::clone(&github_gist_index);
            let geeksforgeeks_index_clone = Arc::clone(&geeksforgeeks_index);
            let ddg_search_index_clone = Arc::clone(&ddg_search_index);

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
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                let mut stackexchange_results_clone_lock = match stackexchange_results_clone.lock()
                {
                    Ok(res) => res,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                let mut github_gist_results_clone_lock = match github_gist_results_clone.lock() {
                    Ok(res) => res,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                let mut geeksforgeeks_results_clone_lock = match geeksforgeeks_results_clone.lock()
                {
                    Ok(res) => res,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                let mut ddg_search_results_clone_lock = match ddg_search_results_clone.lock() {
                    Ok(res) => res,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };

                // resest index to 0
                match stackoverflow_index_clone.lock() {
                    Ok(mut index) => *index = 0,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                match stackexchange_index_clone.lock() {
                    Ok(mut index) => *index = 0,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                match github_gist_index_clone.lock() {
                    Ok(mut index) => *index = 0,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                match geeksforgeeks_index_clone.lock() {
                    Ok(mut index) => *index = 0,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
                match ddg_search_index_clone.lock() {
                    Ok(mut index) => *index = 0,
                    Err(err) => {
                        tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
                        // panick since this should not happen and it's major issue if it doer
                        panic!("Failed to get the lock on a Mutex. Erron {}", err);
                    },
                };
 
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
                    match results {
                        // unwrap is fine here since it would have been an error if there were no
                        // results, so there is at least one
                        Ok(results) => {
                            let res = results.get_index(0).unwrap().0.to_string();
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_sof_result(res.into());
                                ui.set_is_sof(true);
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },
                        Err(err) => {
                            let err = err.to_string();
                            tracing::warn!("There were no results for StackOverflow using search query {}. Error {}", &text, err);
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_sof_result(err.into());
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },              
                    }
                }
                // stackexchange
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = stackexchange_results_clone_lock.as_ref() {
                    match results {
                        // unwrap is fine here since it would have been an error if there were no
                        // results, so there is at least one
                        Ok(results) => {
                            let res = results.get_index(0).unwrap().0.to_string();
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_se_result(res.into());
                                ui.set_is_se(true);
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },
                        Err(err) => {
                            let err = err.to_string();
                            tracing::warn!("There were no results for StackExchange using search query {}. Error {}", &text, err);
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_se_result(err.into());
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },              
                    }
                }
                // github gist
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = github_gist_results_clone_lock.as_ref() {
                    match results {
                        // unwrap is fine here since it would have been an error if there were no
                        // results, so there is at least one
                        Ok(results) => {
                            let res = results.get_index(0).unwrap().0.to_string();
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_gg_result(res.into());
                                ui.set_is_gg(true);
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },
                        Err(err) => {
                            let err = err.to_string();
                            tracing::warn!("There were no results for GitHub Gist using search query {}. Error {}", &text, err);
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_gg_result(err.into());
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },              
                    }
                }
                // GeeksForGeeks
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = geeksforgeeks_results_clone_lock.as_ref() {
                    match results {
                        // unwrap is fine here since it would have been an error if there were no
                        // results, so there is at least one
                        Ok(results) => {
                            let res = results.get_index(0).unwrap().0.to_string();
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_gfg_result(res.into());
                                ui.set_is_gfg(true);
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },
                        Err(err) => {
                            let err = err.to_string();
                            tracing::warn!("There were no results for GeeksForGeeks using search query {}. Error {}", &text, err);
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_gfg_result(err.into());
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },              
                    }
                }
                // Ddg Search
                let ui_thread_clone = ui_thread.clone();
                if let Some(results) = ddg_search_results_clone_lock.as_ref() {
                    match results {
                        // unwrap is fine here since it would have been an error if there were no
                        // results, so there is at least one
                        Ok(results) => {
                            let res = results.get_index(0).unwrap().0.to_string();
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_ddg_result(res.into());
                                ui.set_is_ddg(true);
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },
                        Err(err) => {
                            let err = err.to_string();
                            tracing::warn!("There were no results for Ddg Search using search query {}. Error {}", &text, err);
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = match ui_thread_clone.upgrade() {
                                    Some(ui) => ui,
                                    None => {
                                        tracing::error!("Failed to get ui thread behind weak pointer.");
                                        // it the pointer to the UI is invalid probably the program is not working anymore.
                                        panic!("Failed to get pointer to ui.");
                                    }
                                };
                                ui.set_ddg_result(err.into());
                            }) {
                                tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                                // if we can't invoke slint from the event loop it's probably right to panic as
                                // the program is not responding.
                                panic!("Failed to invoke slint from event loop. Error {}", err);
                            };
                        },              
                    }
                }
                // Enable the next and bach buttons aswell
                let ui_thread_clone = ui_thread.clone();
                if let Err(err) = slint::invoke_from_event_loop(move || {
                    let ui = match ui_thread_clone.upgrade() {
                        Some(ui) => ui,
                        None => {
                            tracing::error!("Failed to get ui thread behind weak pointer.");
                            // it the pointer to the UI is invalid probably the program is not working anymore.
                            panic!("Failed to get pointer to ui.");
                        }
                    };
                    ui.set_is_next(true);
                    ui.set_is_back(true);
                }) {
                    tracing::error!("Failed to invoke slint from event loop. Error {}", err);
                    // if we can't invoke slint from the event loop it's probably right to panic as
                    // the program is not responding.
                    panic!("Failed to invoke slint from event loop. Error {}", err);
                };
            });
        }
    });

    // show the window
    main_window.run().unwrap();
}
