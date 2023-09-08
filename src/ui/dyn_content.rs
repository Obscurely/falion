use super::results;
use super::results::ResultType;
use super::util;
use super::MainWindow;
use super::ResultsDynType;
use super::DYN_CONTENT_VIEW;
use indexmap::IndexMap;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn setup_content_display<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>>,
    index: Arc<Mutex<usize>>,
    content_index: Arc<Mutex<usize>>,
    results_type: ResultType,
) where
    E: std::fmt::Display + std::marker::Send + 'static,
    F: std::marker::Send + 'static,
{
    let ui_strong = util::get_ui(ui.clone());

    // setup enter content
    match results_type {
        ResultType::StackOverflow => ui_strong.on_sof_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
            Arc::clone(&content_index),
        )),
        ResultType::StackExchange => ui_strong.on_se_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
            Arc::clone(&content_index),
        )),
        ResultType::GithubGist => ui_strong.on_gg_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
            Arc::clone(&content_index),
        )),
        _ => return,
    } 
}

fn get_resource_enter_fn<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>>,
    index: Arc<Mutex<usize>>,
    content_index: Arc<Mutex<usize>>,
) -> impl Fn()
where
    E: std::fmt::Display + std::marker::Send + 'static,
    F: std::marker::Send + 'static,
{
    move || {
        // clone necessary ARCs
        let results_clone = Arc::clone(&results);
        let index_clone = Arc::clone(&index);
        let content_index_clone = Arc::clone(&content_index);
        let results_awaited_clone = Arc::clone(&results_awaited);
        // clone ui weak pointer
        let ui = ui.clone();

        // actual logic
        tokio::spawn(async move {
            // reset content index
            results::reset_result_index(Arc::clone(&content_index_clone)).await;
            // get locks
            let locked = futures::join!(
                results_clone.lock(),
                index_clone.lock(),
                results_awaited_clone.lock(),
            );
            let mut results_lock = locked.0;
            let index_lock = locked.1;
            let mut results_awaited_lock = locked.2;

            let content = match results_lock.as_mut() {
                Some(results) => match results {
                    Ok(results) => match results.get_index_mut(*index_lock) {
                        Some(result) => {
                            // show the view dynamic content window
                            let ui_clone = ui.clone();
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = util::get_ui(ui_clone);

                                ui.set_dyn_content("".into());
                                ui.set_view(DYN_CONTENT_VIEW);
                            }) {
                                util::slint_event_loop_panic(err);
                            };
                            // get content
                            match results_awaited_lock.get_index(*index_lock) {
                                Some(result) => result.1,
                                None => {
                                    let awaited = match result.1.await {
                                        Ok(handled) => match handled {
                                            Ok(content) => content,
                                            Err(error) => {
                                                tracing::error!("There was an error getting the contetn for this a result. Error: {}", error);
                                                vec![format!("There has been an error getting the content for this result. Error: {}", error)]
                                            }
                                        },
                                        Err(error) => {
                                            tracing::error!(
                                                "There was an error handeling the future for a result. Error: {}",
                                                error
                                            );
                                            vec![format!("There has been an error handeling the future for this result. Error: {}", error)]
                                        }
                                    };

                                    // save already awaited
                                    results_awaited_lock.insert(result.0.to_owned(), awaited);

                                    // unwrap is safe since we just inserted this element
                                    results_awaited_lock.get(result.0).unwrap()
                                }
                            }
                        }
                        None => return,
                    },
                    Err(err) => return,
                },
                None => return,
            }; 

            // set the first element
            let ui_clone = ui.clone();
            // clone necessary ARCs
            let results_clone = Arc::clone(&results_clone);
            let index_clone = Arc::clone(&index_clone);
            let content_index_clone = Arc::clone(&content_index_clone);
            let results_awaited_clone = Arc::clone(&results_awaited_clone);
            // get first element
            let first = content.first().unwrap().to_owned();
            // drop the Mutex locks
            drop(results_lock);
            drop(results_awaited_lock);
            drop(index_lock);
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui_strong = util::get_ui(ui_clone);

                // unwrap is fine since it there is always at least one element
                ui_strong.set_dyn_content(first.into());

                // setup back and next buttons
                // setup back content button
                ui_strong.on_dyn_back_enter(get_back_content_fn(
                    ui.clone(),
                    Arc::clone(&results_clone),
                    Arc::clone(&results_awaited_clone),
                    Arc::clone(&index_clone),
                    Arc::clone(&content_index_clone),
                ));

                // setup next content button
                ui_strong.on_dyn_next_enter(get_next_content_fn(
                    ui.clone(),
                    Arc::clone(&results_clone),
                    Arc::clone(&results_awaited_clone),
                    Arc::clone(&index_clone),
                    Arc::clone(&content_index_clone),
                ));       
            }) {
                util::slint_event_loop_panic(err);
            };
        });
    }
}

fn get_back_content_fn<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>>,
    index: Arc<Mutex<usize>>,
    content_index: Arc<Mutex<usize>>,
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
                                        let content = content.to_owned();
                                        if let Err(err) = slint::invoke_from_event_loop(move || {
                                            let ui = util::get_ui(ui_clone);

                                            // unwrap is fine since it there is always at least one element
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

fn get_next_content_fn<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, Vec<String>>>>,
    index: Arc<Mutex<usize>>,
    content_index: Arc<Mutex<usize>>,
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
                                            let content = content.to_owned();
                                            if let Err(err) =
                                                slint::invoke_from_event_loop(move || {
                                                    let ui = util::get_ui(ui_clone);

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
