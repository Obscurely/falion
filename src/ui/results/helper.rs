use super::util;
use super::MainWindow;
use slint::Weak;

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
