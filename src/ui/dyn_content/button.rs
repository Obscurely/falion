use super::super::results::ResultType;
use super::util;
use super::MainWindow;
use super::ResultsDynType;
use indexmap::IndexMap;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::Mutex;

/// The function that's called when the content back button is hit.
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
/// * `results` - ARC to the Mutex encapsulation of the Option for the results variable, from the main
/// ui function.
/// * `results_awaited` - ARC to the Mutex of the awaited results variable, from the main ui
/// function.
/// * `index` - ARC to the Mutex of the current results index for this particular resource
/// * `content_index` - the index of the item that should be displayed from the result
/// * `results_type` - the kind of result this is. Ex: StackOverflow.
///
/// # Panics
///
/// If slint couldn't be invoked from the event loop.
#[tracing::instrument(skip_all)]
pub fn get_back_content_fn<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>>,
    index: Arc<Mutex<usize>>,
    content_index: Arc<Mutex<usize>>,
    results_type: ResultType,
) -> impl Fn()
where
    E: std::fmt::Display + std::marker::Send + 'static,
    F: std::fmt::Display + std::marker::Send + 'static,
{
    move || {
        // clone necessary ARCs
        let index_clone = Arc::clone(&index);
        let content_index_clone = Arc::clone(&content_index);
        let results_clone = Arc::clone(&results);
        let results_awaited_clone = Arc::clone(&results_awaited);
        // clone ui weak pointer
        let ui = ui.clone();

        tokio::spawn(async move {
            let locked = futures::join!(
                index_clone.lock(),
                content_index_clone.lock(),
                results_clone.lock(),
                results_awaited_clone.lock()
            );
            let index_lock = locked.0;
            let mut content_index_lock = locked.1;
            // return if the index is already on 0
            if *content_index_lock == 0 {
                return;
            } else {
                *content_index_lock = content_index_lock.saturating_sub(1);
            }
            let results_lock = locked.2;
            let results_awaited_lock = locked.3;

            match results_lock.as_ref() {
                Some(results) => match results {
                    Ok(results) => match results.get_index(*index_lock) {
                        Some(result) => match results_awaited_lock.get(result.0) {
                            Some(result) => {
                                match result.get(*content_index_lock) {
                                    Some(content) => {
                                        let ui_clone = ui.clone();
                                        // make content tag
                                        let content_tag = if results_type == ResultType::GithubGist
                                        {
                                            format!("File {}", *content_index_lock + 1)
                                        } else if *content_index_lock == 0 {
                                            "Question".to_string()
                                        } else {
                                            format!("Answer {}", *content_index_lock)
                                        };

                                        // set content
                                        let content = content.to_owned();
                                        if let Err(err) = slint::invoke_from_event_loop(move || {
                                            let ui = util::get_ui(ui_clone);

                                            // set content tag
                                            ui.set_dyn_content_tag(content_tag.into());

                                            // set dynamic content
                                            ui.set_dyn_content(content.into());

                                            // log action
                                            tracing::info!("Successfully displayed previous item in resource result.")
                                        }) {
                                            util::slint_event_loop_panic(err);
                                        };
                                    }
                                    None => {
                                        tracing::warn!("User tried getting content at a non existent index. Programming error.");
                                    }
                                }
                            }
                            None => {
                                tracing::warn!("User tried accessing a result at a non existen index which shouldn't have happened and it's a programming error if it does");
                            }
                        },
                        None => {
                            tracing::warn!("User is on the view content screen, but the result they try to go through is not awaited. Programming error.");
                        }
                    },
                    Err(err) => {
                        tracing::warn!("The results are an error and the user should have not been able to interact with them. Err: {}", err.to_string());
                    }
                },
                None => {
                    tracing::warn!("There are no results for a selected resource, but the user still got to the view content screen. This is a programming error");
                }
            }
        });
    }
}

/// The function that's called when the content next button is hit.
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
/// * `results` - ARC to the Mutex encapsulation of the Option for the results variable, from the main
/// ui function.
/// * `results_awaited` - ARC to the Mutex of the awaited results variable, from the main ui
/// function.
/// * `index` - ARC to the Mutex of the current results index for this particular resource
/// * `content_index` - the index of the item that should be displayed from the result
/// * `results_type` - the kind of result this is. Ex: StackOverflow.
///
/// # Panics
///
/// If slint couldn't be invoked from the event loop.
#[tracing::instrument(skip_all)]
pub fn get_next_content_fn<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>>,
    index: Arc<Mutex<usize>>,
    content_index: Arc<Mutex<usize>>,
    results_type: ResultType,
) -> impl Fn()
where
    E: std::fmt::Display + std::marker::Send + 'static,
    F: std::fmt::Display + std::marker::Send + 'static,
{
    move || {
        // clone necessary ARCs
        let index_clone = Arc::clone(&index);
        let content_index_clone = Arc::clone(&content_index);
        let results_clone = Arc::clone(&results);
        let results_awaited_clone = Arc::clone(&results_awaited);
        // clone ui weak pointer
        let ui = ui.clone();

        tokio::spawn(async move {
            let locked = futures::join!(
                index_clone.lock(),
                content_index_clone.lock(),
                results_clone.lock(),
                results_awaited_clone.lock()
            );
            let index_lock = locked.0;
            let mut content_index_lock = locked.1;
            let results_lock = locked.2;
            let results_awaited_lock = locked.3;

            match results_lock.as_ref() {
                Some(results) => match results {
                    Ok(results) => match results.get_index(*index_lock) {
                        Some(result) => match results_awaited_lock.get(result.0) {
                            Some(result) => {
                                if *content_index_lock < result.len() - 1 {
                                    *content_index_lock += 1;
                                    match result.get(*content_index_lock) {
                                        Some(content) => {
                                            let ui_clone = ui.clone();
                                            // make content tag
                                            let content_tag =
                                                if results_type == ResultType::GithubGist {
                                                    format!("File {}", *content_index_lock + 1)
                                                } else if *content_index_lock == 0 {
                                                    "Question".to_string()
                                                } else {
                                                    format!("Answer {}", *content_index_lock)
                                                };

                                            // set content
                                            let content = content.to_owned();
                                            if let Err(err) =
                                                slint::invoke_from_event_loop(move || {
                                                    let ui = util::get_ui(ui_clone);

                                                    // set content tag
                                                    ui.set_dyn_content_tag(content_tag.into());

                                                    // unwrap is fine since it there is always at least one element
                                                    ui.set_dyn_content(content.into());
                                                    
                                                    // log action
                                                    tracing::info!("Successfully displayed next item in resource result.")
                                                })
                                            {
                                                util::slint_event_loop_panic(err);
                                            };
                                        }
                                        None => {
                                            tracing::warn!("User tried getting content at a non existent index. Programming error.");
                                        }
                                    }
                                }
                            }
                            None => {
                                tracing::warn!("User tried accessing a result at a non existen index which shouldn't have happened and it's a programming error if it does");
                            }
                        },
                        None => {
                            tracing::warn!("User is on the view content screen, but the result they try to go through is not awaited. Programming error.");
                        }
                    },
                    Err(err) => {
                        tracing::warn!("The results are an error and the user should have not been able to interact with them. Err: {}", err.to_string());
                    }
                },
                None => {
                    tracing::warn!("There are no results for a selected resource, but the user still got to the view content screen. This is a programming error");
                }
            }
        });
    }
}
