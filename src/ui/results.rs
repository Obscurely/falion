use super::util;
use super::MainWindow;
use indexmap::IndexMap;
use slint::Weak;
use std::sync::{Arc, Mutex};

type Results<T, E> = Result<IndexMap<String, T>, E>;

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
    match results.lock() {
        Ok(results) => {
            if let Some(Ok(results)) = results.as_ref() {
                match index.lock() {
                    Ok(index) => {
                        if let Some(res) = results.get_index(*index) {
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
                    Err(err) => {
                        util::poison_panic(err);
                    }
                }
            }
        }
        Err(err) => {
            util::poison_panic(err);
        }
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

pub fn reset_result_index(index: Arc<Mutex<usize>>) {
    match index.lock() {
        Ok(mut index) => {
            *index = 0;
        }
        Err(err) => util::poison_panic(err),
    };
}

pub fn try_up_index<T, E>(results: Arc<Mutex<Option<Results<T, E>>>>, index: Arc<Mutex<usize>>) {
    match results.lock() {
        Ok(results) => {
            if let Some(Ok(results)) = results.as_ref() {
                match index.lock() {
                    Ok(mut index) => {
                        if (*index) < results.len() - 1 {
                            *index += 1;
                        }
                    }
                    Err(err) => {
                        util::poison_panic(err);
                    }
                }
            }
        }
        Err(err) => {
            util::poison_panic(err);
        }
    }
}

pub fn try_down_index(index: Arc<Mutex<usize>>) {
    match index.lock() {
        Ok(mut index) => {
            *index = index.saturating_sub(1);
        }
        Err(err) => {
            util::poison_panic(err);
        }
    }
}
