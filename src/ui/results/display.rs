use super::util;
use super::MainWindow;
use super::ResultType;
use super::Results;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Display the first resould for the provided resource
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
/// * `results` - ARC to the RwLock encapsulation of the Option for the results variable, from the main
/// ui function.
/// * `results_type` - the kind of result this is. Ex: StackOverflow.
///
/// # Panics
///
/// If it can't invoke the slint event loop.
#[tracing::instrument(skip_all)]
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
            let (title, _) = results.get(0).unwrap();
            let res = slint::SharedString::from(title);
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui = util::get_ui(ui);

                // display the result based on the results type
                // set the text, enable button, and the cycle buttons
                match results_type {
                    ResultType::StackOverflow => {
                        ui.set_sof_result(res);
                        ui.set_is_sof(true);
                        ui.set_is_sof_back(true);
                        ui.set_is_sof_next(true);
                    }
                    ResultType::StackExchange => {
                        ui.set_se_result(res);
                        ui.set_is_se(true);
                        ui.set_is_se_back(true);
                        ui.set_is_se_next(true);
                    }
                    ResultType::GithubGist => {
                        ui.set_gg_result(res);
                        ui.set_is_gg(true);
                        ui.set_is_gg_back(true);
                        ui.set_is_gg_next(true);
                    }
                    ResultType::GeeksForGeeks => {
                        ui.set_gfg_result(res);
                        ui.set_is_gfg(true);
                        ui.set_is_gfg_back(true);
                        ui.set_is_gfg_next(true);
                    }
                    ResultType::DdgSearch => {
                        ui.set_ddg_result(res);
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
            let err = slint::SharedString::from(err.to_string());
            // error depending on the results type
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

                // error depending on the results type
                match results_type {
                    ResultType::StackOverflow => {
                        ui.set_sof_result(err);
                    }
                    ResultType::StackExchange => {
                        ui.set_sof_result(err);
                    }
                    ResultType::GithubGist => {
                        ui.set_sof_result(err);
                    }
                    ResultType::GeeksForGeeks => {
                        ui.set_sof_result(err);
                    }
                    ResultType::DdgSearch => {
                        ui.set_sof_result(err);
                    }
                }
            }) {
                util::slint_event_loop_panic(err);
            };
        }
    }
}

/// Redisplay the result for the provide resource
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
/// * `results` - ARC to the RwLock encapsulation of the Option for the results variable, from the main
/// ui function.
/// function.
/// * `index` - ARC to the RwLock of the current results index for this particular resource
/// * `results_type` - the kind of result this is. Ex: StackOverflow.
///
/// # Panics
///
/// If it can't invoke the slint event loop.
#[tracing::instrument(skip_all)]
pub fn redisplay_result<T, E>(
    ui: Weak<MainWindow>,
    results: Arc<RwLock<Option<Results<T, E>>>>,
    index: Arc<RwLock<usize>>,
    results_type: ResultType,
) where
    E: std::fmt::Display,
{
    if let Some(Ok(results)) = results.blocking_read().as_ref() {
        if let Some(res) = results.get(*index.blocking_read()) {
            let (title, _) = res;
            let res = slint::SharedString::from(title);
            if let Err(err) = slint::invoke_from_event_loop(move || {
                let ui = util::get_ui(ui);

                // redisplay results based on their type
                match results_type {
                    ResultType::StackOverflow => {
                        ui.set_sof_result(res);
                    }
                    ResultType::StackExchange => {
                        ui.set_se_result(res);
                    }
                    ResultType::GithubGist => {
                        ui.set_gg_result(res);
                    }
                    ResultType::GeeksForGeeks => {
                        ui.set_gfg_result(res);
                    }
                    ResultType::DdgSearch => {
                        ui.set_ddg_result(res);
                    }
                }
            }) {
                util::slint_event_loop_panic(err);
            };
        };
    }
}
