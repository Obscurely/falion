use super::util;
use super::MainWindow;
use slint::Weak;

/// Disable the search text line edit in the ui
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
///
/// # Panics
/// 
/// If it can't invoke the slint event loop.
#[tracing::instrument(skip_all)]
pub fn disable_search(ui: Weak<MainWindow>) {
    if let Err(err) = slint::invoke_from_event_loop(move || {
        let ui = util::get_ui(ui);

        ui.set_enable_search(false);
    }) {
        util::slint_event_loop_panic(err);
    };
}

/// Enable the search text line edit in the ui
///
/// # Arguments
///
/// * `ui` - weak pointer to the slint ui
///
/// # Panics
/// 
/// If it can't invoke the slint event loop.
#[tracing::instrument(skip_all)]
pub fn enable_search(ui: Weak<MainWindow>) {
    if let Err(err) = slint::invoke_from_event_loop(move || {
        let ui = util::get_ui(ui);

        ui.set_enable_search(true);
    }) {
        util::slint_event_loop_panic(err);
    };
}
