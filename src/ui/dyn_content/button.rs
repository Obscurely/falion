use super::super::results::ResultType;
use super::util;
use super::MainWindow;
use super::ResultsDynType;
use indexmap::IndexMap;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    F: std::marker::Send + 'static,
{
    move || {
        // clone necessary ARCs
        let index_clone = Arc::clone(&index);
        let content_index_clone = Arc::clone(&content_index);
        let results = Arc::clone(&results);
        let results_awaited_clone = Arc::clone(&results_awaited);
        // clone ui weak pointer
        let ui = ui.clone();

        tokio::task::spawn_blocking(move || {
            let index_lock = index_clone.blocking_lock();
            let mut content_index_lock = content_index_clone.blocking_lock();
            // return if the index is already on 0
            if *content_index_lock == 0 {
                return;
            } else {
                *content_index_lock = content_index_lock.saturating_sub(1);
            }
            let results = results.blocking_lock();
            let results_awaited_lock = results_awaited_clone.blocking_lock();

            match results.as_ref() {
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
                                        }) {
                                            util::slint_event_loop_panic(err);
                                        };
                                    }
                                    None => return,
                                }
                            }
                            None => return,
                        },
                        None => return,
                    },
                    Err(err) => return,
                },
                None => return,
            }
        });
    }
}

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
    F: std::marker::Send + 'static,
{
    move || {
        // clone necessary ARCs
        let index_clone = Arc::clone(&index);
        let content_index_clone = Arc::clone(&content_index);
        let results = Arc::clone(&results);
        let results_awaited_clone = Arc::clone(&results_awaited);
        // clone ui weak pointer
        let ui = ui.clone();

        tokio::task::spawn_blocking(move || {
            let index_lock = index_clone.blocking_lock();
            let mut content_index_lock = content_index_clone.blocking_lock();
            let results = results.blocking_lock();
            let results_awaited_lock = results_awaited_clone.blocking_lock();

            match results.as_ref() {
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
                                                })
                                            {
                                                util::slint_event_loop_panic(err);
                                            };
                                        }
                                        None => return,
                                    }
                                }
                            }
                            None => return,
                        },
                        None => return,
                    },
                    Err(err) => return,
                },
                None => return,
            }
        });
    }
}
