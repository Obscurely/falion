pub mod display;
pub mod helper;
pub mod index;
use super::util;
use super::MainWindow;
use super::Results;
use slint::Weak;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Possible results type, pretty self explanatory
#[derive(Clone, Copy, PartialEq)]
pub enum ResultType {
    StackOverflow,
    StackExchange,
    GithubGist,
    GeeksForGeeks,
    DdgSearch,
}

/// Reset the results ui elements. Disabling the buttons and removing any button text.
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui (.as_weak())
///
/// # Panics
///
/// If the slint can't be invoked from the event loop.
#[tracing::instrument(skip_all)]
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

/// Setup the buttons for the results: cycle through them and the results itself
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
/// * `results` - ARC to the Mutex encapsulation of the Option for the results variable, from the main
/// ui function.
/// function.
/// * `index` - ARC to the Mutex of the current results index for this particular resource
/// * `results_type` - the kind of result this is. Ex: StackOverflow.
///
/// # Panics
///
/// If it can't invoke the slint event loop.
#[tracing::instrument(skip_all)]
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
                index::try_down_index(Arc::clone(&index_clone));

                // redisplay the result
                display::redisplay_result(
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
                index::try_up_index(Arc::clone(&results_clone), Arc::clone(&index_clone));

                // redisplay the result
                display::redisplay_result(
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

    // seput events based on the results type and their respective callbacks
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
