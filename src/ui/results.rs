use super::util;
use super::MainWindow;
use indexmap::IndexMap;
use slint::ComponentHandle;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

type Results<T, E> = Result<IndexMap<String, T>, E>;
type ResultsStaticType<E, F> = Result<IndexMap<String, JoinHandle<Result<String, E>>>, F>;
type ResultsDynType<E, F> = Result<IndexMap<String, JoinHandle<Result<Vec<String>, E>>>, F>;

#[derive(Clone, Copy)]
pub enum ResultType {
    StackOverflow,
    StackExchange,
    GithubGist,
    GeeksForGeeks,
    DdgSearch,
}

pub fn display_first_result<T, E>(
    ui: Weak<MainWindow>,
    results: &Results<T, E>,
    results_type: ResultType,
) where
    E: std::fmt::Display,
{
    match results {
        // unwrap is fine here since it would have been an error if there were no
        // results, so there is at least one
        Ok(results) => {
            let res = results.get_index(0).unwrap().0.to_string();
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui = util::get_ui(ui);

                match results_type {
                    ResultType::StackOverflow => {
                        ui.set_sof_result(res.into());
                        ui.set_is_sof(true);
                        ui.set_is_sof_back(true);
                        ui.set_is_sof_next(true);
                    }
                    ResultType::StackExchange => {
                        ui.set_se_result(res.into());
                        ui.set_is_se(true);
                        ui.set_is_se_back(true);
                        ui.set_is_se_next(true);
                    }
                    ResultType::GithubGist => {
                        ui.set_gg_result(res.into());
                        ui.set_is_gg(true);
                        ui.set_is_gg_back(true);
                        ui.set_is_gg_next(true);
                    }
                    ResultType::GeeksForGeeks => {
                        ui.set_gfg_result(res.into());
                        ui.set_is_gfg(true);
                        ui.set_is_gfg_back(true);
                        ui.set_is_gfg_next(true);
                    }
                    ResultType::DdgSearch => {
                        ui.set_ddg_result(res.into());
                        ui.set_is_ddg(true);
                        ui.set_is_ddg_back(true);
                        ui.set_is_ddg_next(true);
                    }
                }
            }) {
                util::slint_event_loop_panic(err);
            };
        }
        Err(err) => {
            let err = err.to_string();
            match results_type {
                ResultType::StackOverflow => {
                    tracing::warn!("There were no results for StackOverflow. Error {}", err);
                }
                ResultType::StackExchange => {
                    tracing::warn!("There were no results for StackExchange. Error {}", err);
                }
                ResultType::GithubGist => {
                    tracing::warn!("There were no results for GithubGist. Error {}", err);
                }
                ResultType::GeeksForGeeks => {
                    tracing::warn!("There were no results for GeeksForGeeks. Error {}", err);
                }
                ResultType::DdgSearch => {
                    tracing::warn!("There were no results for DdgSearch. Error {}", err);
                }
            }
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui = util::get_ui(ui);

                match results_type {
                    ResultType::StackOverflow => {
                        ui.set_sof_result(err.into());
                    }
                    ResultType::StackExchange => {
                        ui.set_sof_result(err.into());
                    }
                    ResultType::GithubGist => {
                        ui.set_sof_result(err.into());
                    }
                    ResultType::GeeksForGeeks => {
                        ui.set_sof_result(err.into());
                    }
                    ResultType::DdgSearch => {
                        ui.set_sof_result(err.into());
                    }
                }
            }) {
                util::slint_event_loop_panic(err);
            };
        }
    }
}

pub fn redisplay_result<T, E>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<Results<T, E>>>>,
    index: Arc<Mutex<usize>>,
    results_type: ResultType,
) where
    E: std::fmt::Display,
{
    if let Some(Ok(results)) = results.blocking_lock().as_ref() {
        if let Some(res) = results.get_index(*index.blocking_lock()) {
            let res = res.0.to_string();
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui = util::get_ui(ui);

                match results_type {
                    ResultType::StackOverflow => {
                        ui.set_sof_result(res.into());
                    }
                    ResultType::StackExchange => {
                        ui.set_se_result(res.into());
                    }
                    ResultType::GithubGist => {
                        ui.set_gg_result(res.into());
                    }
                    ResultType::GeeksForGeeks => {
                        ui.set_gfg_result(res.into());
                    }
                    ResultType::DdgSearch => {
                        ui.set_ddg_result(res.into());
                    }
                }
            }) {
                util::slint_event_loop_panic(err);
            };
        };
    }
}

pub fn reset_results(ui: Weak<MainWindow>) {
    if let Err(err) = slint::invoke_from_event_loop(move || {
        let ui = util::get_ui(ui);

        let space_string = slint::SharedString::from(" ");

        ui.set_sof_result(space_string.clone());
        ui.set_is_sof(false);
        ui.set_is_sof_back(false);
        ui.set_is_sof_next(false);

        ui.set_se_result(space_string.clone());
        ui.set_is_se(false);
        ui.set_is_se_back(false);
        ui.set_is_se_next(false);

        ui.set_gg_result(space_string.clone());
        ui.set_is_gg(false);
        ui.set_is_gg_back(false);
        ui.set_is_gg_next(false);

        ui.set_gfg_result(space_string.clone());
        ui.set_is_gfg(false);
        ui.set_is_gfg_back(false);
        ui.set_is_gfg_next(false);

        ui.set_ddg_result(space_string.clone());
        ui.set_is_ddg(false);
        ui.set_is_ddg_back(false);
        ui.set_is_ddg_next(false);

        ui.set_is_back(false);
        ui.set_is_next(false);
        ui.set_error(space_string);
    }) {
        util::slint_event_loop_panic(err);
    };
}

pub async fn reset_result_index(index: Arc<Mutex<usize>>) {
    *index.lock().await = 0
}

pub fn disable_search(ui: Weak<MainWindow>) {
    if let Err(err) = slint::invoke_from_event_loop(move || {
        let ui = util::get_ui(ui);

        ui.set_enable_search(false);
    }) {
        util::slint_event_loop_panic(err);
    };
}

pub fn enable_search(ui: Weak<MainWindow>) {
    if let Err(err) = slint::invoke_from_event_loop(move || {
        let ui = util::get_ui(ui);

        ui.set_enable_search(true);
    }) {
        util::slint_event_loop_panic(err);
    };
}

pub fn try_up_index<T, E>(results: Arc<Mutex<Option<Results<T, E>>>>, index: Arc<Mutex<usize>>) {
    if let Some(Ok(results)) = results.blocking_lock().as_ref() {
        let mut index = index.blocking_lock();
        if (*index) < results.len() - 1 {
            *index += 1;
        }
    }
}

pub fn try_down_index(index: Arc<Mutex<usize>>) {
    let mut index = index.blocking_lock();
    *index = index.saturating_sub(1);
}

pub fn setup_results_btns<T, E>(
    ui: Weak<MainWindow>,
    results: Arc<Mutex<Option<Results<T, E>>>>,
    index: Arc<Mutex<usize>>,
    results_type: ResultType,
) where
    E: std::fmt::Display + std::marker::Send + 'static,
    T: 'static + std::marker::Send,
{
    let ui_deref = util::get_ui(ui.clone());
    let ui_clone = ui.clone();

    // events
    let back_event = {
        tracing::info!("On sof back enter event hit.");
        // clone the necessary ARCs
        let results_clone = Arc::clone(&results);
        let index_clone = Arc::clone(&index);

        // actual closure
        move || {
            let results_clone = Arc::clone(&results_clone);
            let index_clone = Arc::clone(&index_clone);
            let ui_clone = ui_clone.clone();
            tokio::task::spawn_blocking(move || {
                // try down the index by one
                try_down_index(Arc::clone(&index_clone));

                // redisplay the result
                redisplay_result(
                    ui_clone,
                    Arc::clone(&results_clone),
                    Arc::clone(&index_clone),
                    results_type,
                );

                // log the end of the function
                tracing::info!("Successfully backed the StaceOverflow results by one.");
            });
        }
    };

    let ui_clone = ui.clone();
    let next_event = {
        tracing::info!("On sof next enter event hit.");
        // clone the necessary ARCs
        let results_clone = Arc::clone(&results);
        let index_clone = Arc::clone(&index);

        // actual closure
        move || {
            let results_clone = Arc::clone(&results_clone);
            let index_clone = Arc::clone(&index_clone);
            let ui_clone = ui_clone.clone();

            tokio::task::spawn_blocking(move || {
                // try down the index by one
                try_up_index(Arc::clone(&results_clone), Arc::clone(&index_clone));

                // redisplay the result
                redisplay_result(
                    ui_clone,
                    Arc::clone(&results_clone),
                    Arc::clone(&index_clone),
                    results_type,
                );

                // log the end of the function
                tracing::info!("Successfully upped the StaceOverflow results by one.");
            });
        }
    };

    match results_type {
        ResultType::StackOverflow => {
            ui_deref.on_sof_back_enter(back_event);
            ui_deref.on_sof_next_enter(next_event);
        }
        ResultType::StackExchange => {
            ui_deref.on_se_back_enter(back_event);
            ui_deref.on_se_next_enter(next_event);
        }
        ResultType::GithubGist => {
            ui_deref.on_gg_back_enter(back_event);
            ui_deref.on_gg_next_enter(next_event);
        }
        ResultType::GeeksForGeeks => {
            ui_deref.on_gfg_back_enter(back_event);
            ui_deref.on_gfg_next_enter(next_event);
        }
        ResultType::DdgSearch => {
            ui_deref.on_ddg_back_enter(back_event);
            ui_deref.on_ddg_next_enter(next_event);
        }
    }
}
