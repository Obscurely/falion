use super::results::ResultType;
use super::util;
use super::MainWindow;
use super::ResultsStaticType;
use super::STATIC_CONTENT_VIEW;
use indexmap::IndexMap;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tracing::instrument(skip_all)]
pub fn setup_content_display<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsStaticType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, String>>>,
    index: Arc<Mutex<usize>>,
    results_type: ResultType,
) where
    E: std::fmt::Display + std::marker::Send + 'static,
    F: std::fmt::Display + std::marker::Send + 'static,
{
    let ui_strong = util::get_ui(ui.clone());

    // disaple buttons
    ui_strong.set_enable_content_btns(false);

    // setup enter content
    match results_type {
        ResultType::GeeksForGeeks => ui_strong.on_gfg_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
        )),
        ResultType::DdgSearch => ui_strong.on_ddg_enter(get_resource_enter_fn(
            ui.clone(),
            Arc::clone(&results),
            Arc::clone(&results_awaited),
            Arc::clone(&index),
        )),
        _ => {
            tracing::error!("Results type used on a function that doesn't support it.");
            panic!("Results type used on function that doesn't support it. This is a programming error.");
        }
    }
}

#[tracing::instrument(skip_all)]
fn get_resource_enter_fn<E, F>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<ResultsStaticType<E, F>>>>,
    results_awaited: Arc<Mutex<IndexMap<String, String>>>,
    index: Arc<Mutex<usize>>,
) -> impl Fn()
where
    E: std::fmt::Display + std::marker::Send + 'static,
    F: std::fmt::Display + std::marker::Send + 'static,
{
    move || {
        // clone necessary ARCs
        let results_clone = Arc::clone(&results);
        let index_clone = Arc::clone(&index);
        let results_awaited_clone = Arc::clone(&results_awaited);
        // clone ui weak pointer
        let ui = ui.clone();

        // actual logic
        tokio::spawn(async move {
            // get locks
            let locked = futures::join!(
                results_clone.lock(),
                index_clone.lock(),
                results_awaited_clone.lock(),
            );
            let mut results_lock = locked.0;
            let index_lock = locked.1;
            let mut results_awaited_lock = locked.2;

            // try get the content
            let content = match results_lock.as_mut() {
                Some(results) => match results {
                    Ok(results) => match results.get_index_mut(*index_lock) {
                        Some(result) => {
                            // show the view static content window
                            let ui_clone = ui.clone();
                            if let Err(err) = slint::invoke_from_event_loop(move || {
                                let ui = util::get_ui(ui_clone);

                                ui.set_static_content("".into());
                                ui.set_view(STATIC_CONTENT_VIEW);
                            }) {
                                util::slint_event_loop_panic(err);
                            };
                            // get content
                            match results_awaited_lock.get(result.0) {
                                Some(result) => result,
                                None => {
                                    let awaited = match result.1.await {
                                        Ok(handled) => match handled {
                                            Ok(content) => content,
                                            Err(error) => {
                                                tracing::error!("There was an error getting the contetn for this a result. Error: {}", error);
                                                format!("There has been an error getting the content for this result. Error: {}", error)
                                            }
                                        },
                                        Err(error) => {
                                            tracing::error!(
                                                "There was an error handeling the future for a result. Error: {}",
                                                error
                                            );
                                            format!("There has been an error handeling the future for this result. Error: {}", error)
                                        }
                                    };

                                    // save already awaited
                                    results_awaited_lock.insert(result.0.to_owned(), awaited);

                                    // unwrap is safe since we just inserted this element
                                    results_awaited_lock.get(result.0).unwrap()
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
            // get owned data for content
            let content = content.to_owned();
            // drop the Mutex locks
            drop(results_lock);
            drop(results_awaited_lock);
            drop(index_lock);
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui_strong = util::get_ui(ui_clone);

                // set content tag
                ui_strong.set_static_content_tag("Page".into());

                // set content
                ui_strong.set_static_content(content.into());

                // enable btns
                ui_strong.set_enable_content_btns(true);

                // log done displaying
                tracing::info!("Displayed static resource.");
            }) {
                util::slint_event_loop_panic(err);
            };
        });
    }
}
