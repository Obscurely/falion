mod button;
use super::results;
use super::results::ResultType;
use super::util;
use super::MainWindow;
use super::ResultsDynType;
use super::DYN_CONTENT_VIEW;
use dashmap::DashMap;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Setup the button on which the result is printed which when is pressed brings you the view
/// content screen.
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
/// * `results` - ARC to the Mutex encapsulation of the Option for the results variable, from the main
/// ui function.
/// * `results_awaited` - ARC to the Mutex of the awaited results variable, from the main ui
/// function.
/// * `index` - ARC to the Mutex of the current results index for this particular resource
/// * `results_type` - the kind of result this is. Ex: StackOverflow.
///
/// # Panics
///
/// It the results type is not made for this function
#[tracing::instrument(skip_all)]
pub fn setup_content_display<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<DashMap<String, Vec<String>>>,
    index: Arc<Mutex<usize>>,
    content_index: Arc<Mutex<usize>>,
    results_type: ResultType,
) where
    E: std::fmt::Display + std::marker::Send + 'static,
    F: std::fmt::Display + std::marker::Send + 'static,
{
    let ui_strong = util::get_ui(ui.clone());

    // disable buttons
    ui_strong.set_enable_content_btns(false);

    // setup enter content
    match results_type {
        ResultType::StackOverflow => ui_strong.on_sof_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
            Arc::clone(&content_index),
            results_type,
        )),
        ResultType::StackExchange => ui_strong.on_se_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
            Arc::clone(&content_index),
            results_type,
        )),
        ResultType::GithubGist => ui_strong.on_gg_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
            Arc::clone(&content_index),
            results_type,
        )),
        _ => {
            tracing::error!("Results type used on a function that doesn't support it.");
            panic!("Results type used on function that doesn't support it. This is a programming error.");
        }
    }
}

/// The callback function for viewing the resource.
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
fn get_resource_enter_fn<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsDynType<E, F>>>>,
    results_awaited: Arc<DashMap<String, Vec<String>>>,
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
        let results_clone = Arc::clone(&results);
        let index_clone = Arc::clone(&index);
        let content_index_clone = Arc::clone(&content_index);
        let results_awaited_clone = Arc::clone(&results_awaited);
        // clone ui weak pointer
        let ui = ui.clone();

        // actual logic
        tokio::spawn(async move {
            // reset content index
            results::index::reset_result_index(Arc::clone(&content_index_clone)).await;
            // get locks
            let locked = futures::join!(
                results_clone.lock(),
                index_clone.lock(),
            );
            let mut results_lock = locked.0;
            let index_lock = locked.1;

            let content = match results_lock.as_mut() {
                Some(results) => match results {
                    Ok(results) => match results.get_mut(*index_lock) {
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
                            match results_awaited_clone.get(&result.0) {
                                Some(result) => result,
                                None => {
                                    let (title, handle) = result;
                                    let awaited = match handle.await {
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
                                    results_awaited_clone.insert(title.to_owned(), awaited);

                                    // unwrap is safe since we just inserted this element
                                    results_awaited_clone.get(title).unwrap()
                                }
                            }
                        }
                        None => {
                            tracing::warn!("User tried accessing a result at a non existen index which shouldn't have happened and it's a programming error if it does");
                            return;
                        }
                    },
                    Err(err) => {
                        tracing::warn!("The results are an error and the user should have not been able to interact with them. Err: {}", err.to_string());
                        return;
                    }
                },
                None => {
                    tracing::warn!("The results are non existen, yet the user still managed to try and access them.");
                    return;
                }
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
            drop(index_lock);
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui_strong = util::get_ui(ui_clone);

                // set dynamic content first tag
                if results_type == ResultType::GithubGist {
                    ui_strong.set_dyn_content_tag("File 1".into());
                } else {
                    ui_strong.set_dyn_content_tag("Question".into());
                }

                // set dyn content
                ui_strong.set_dyn_content(first.into());

                // setup back and next buttons
                // setup back content button
                ui_strong.on_dyn_back_enter(button::get_back_content_fn(
                    ui.clone(),
                    Arc::clone(&results_clone),
                    Arc::clone(&results_awaited_clone),
                    Arc::clone(&index_clone),
                    Arc::clone(&content_index_clone),
                    results_type,
                ));

                // setup next content button
                ui_strong.on_dyn_next_enter(button::get_next_content_fn(
                    ui.clone(),
                    Arc::clone(&results_clone),
                    Arc::clone(&results_awaited_clone),
                    Arc::clone(&index_clone),
                    Arc::clone(&content_index_clone),
                    results_type,
                ));

                // enable btns
                ui_strong.set_enable_content_btns(true);
            }) {
                util::slint_event_loop_panic(err);
            };
        });
    }
}
