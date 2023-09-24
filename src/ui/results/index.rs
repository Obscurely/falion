use super::Results;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Reset usize variable to 0, in this case an index
///
/// # Arguments
///
/// * `index` - ARC to RwLock of a usize variable
#[tracing::instrument(skip_all)]
pub async fn reset_result_index(index: Arc<RwLock<usize>>) {
    *index.write().await = 0
}

/// Try up the index based on if its smaller than the results max index
///
/// # Arguments
///
/// * `results` - ARC to the RwLock encapsulation of the Option for the results variable, from the main
/// ui function.
/// function.
/// * `index` - ARC to the RwLock of the current results index for this particular resource
///
/// # Panics
///
/// If blocking lock can't be called on the RwLock which would happen in a context where blocking is
/// not acceptable, like an async function or the callback for a button.
#[tracing::instrument(skip_all)]
pub fn try_up_index<T, E>(results: Arc<RwLock<Option<Results<T, E>>>>, index: Arc<RwLock<usize>>) {
    if let Some(Ok(results)) = results.blocking_read().as_ref() {
        let mut index = index.blocking_write();
        if (*index) < results.len() - 1 {
            *index += 1;
        }
    }
}

/// Try substract one from a usize, in this case an index, untill it reaches 0
///
/// # Arguments
///
/// * `index` - ARC to RwLock of a usize variable
///
/// # Panics
///
/// If blocking lock can't be called on the RwLock which would happen in a context where blocking is
/// not acceptable, like an async function or the callback for a button.
#[tracing::instrument(skip_all)]
pub fn try_down_index(index: Arc<RwLock<usize>>) {
    let mut index = index.blocking_write();
    // substract one untill we reach the minimum supported by the data type, in our case usize,
    // which is 0
    *index = index.saturating_sub(1);
}
