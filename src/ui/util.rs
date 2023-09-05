use slint::Weak;

pub fn get_ui<T>(ui: Weak<T>) -> T
where
    T: slint::ComponentHandle,
{
    match ui.upgrade() {
        Some(ui) => ui,
        None => {
            tracing::error!("Failed to get ui thread behind weak pointer.");
            // it the pointer to the UI is invalid probably the program is not working anymore.
            panic!("Failed to get pointer to ui.");
        }
    }
}

pub fn slint_event_loop_panic(err: slint::EventLoopError) {
    tracing::error!("Failed to invoke slint from event loop. Error {}", err);
    // if we can't invoke slint from the event loop it's probably right to panic as
    // the program is not responding.
    panic!("Failed to invoke slint from event loop. Error {}", err);
}

pub fn poison_panic<T>(err: std::sync::PoisonError<T>) {
    tracing::error!("Poison error getting the lock on a Mutex. Error {}", err);
    // panick since this should not happen and it's major issue if it doer
    panic!("Failed to get the lock on a Mutex. Erron {}", err);
}
