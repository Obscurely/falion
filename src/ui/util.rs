use slint::Weak;

/// Get the strong pointer for the slint UI.
///
/// # Arguments
///
/// * `ui` - weak pointes to the ui (.as_weak())
///
/// # Panics
///
/// If it can't upgrade the pointes to a strong one because it ussually means that the ui is not
/// usable.
#[tracing::instrument(skip_all)]
pub fn get_ui<T>(ui: Weak<T>) -> T
where
    T: slint::ComponentHandle,
{
    match ui.upgrade() {
        Some(ui) => ui,
        None => {
            tracing::error!("Failed to get ui thread behind weak pointer.");
            // if the pointer to the UI is invalid probably the program is not working anymore.
            panic!("Failed to get pointer to ui.");
        }
    }
}

/// Code to run when needing to panic because slint couldn't be invoked from the event loop which
/// usualy means the ui is not usable.
///
/// # Panics
///
/// When run
#[tracing::instrument(skip_all)]
pub fn slint_event_loop_panic(err: slint::EventLoopError) {
    tracing::error!("Failed to invoke slint from event loop. Error {}", err);
    // if we can't invoke slint from the event loop it's probably right to panic as
    // the program is not responding.
    panic!("Failed to invoke slint from event loop. Error {}", err);
}
