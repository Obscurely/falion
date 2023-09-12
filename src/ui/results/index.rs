use super::Results;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tracing::instrument(skip_all)]
pub async fn reset_result_index(index: Arc<Mutex<usize>>) {
    *index.lock().await = 0
}

#[tracing::instrument(skip_all)]
pub fn try_up_index<T, E>(results: Arc<Mutex<Option<Results<T, E>>>>, index: Arc<Mutex<usize>>) {
    if let Some(Ok(results)) = results.blocking_lock().as_ref() {
        let mut index = index.blocking_lock();
        if (*index) < results.len() - 1 {
            *index += 1;
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn try_down_index(index: Arc<Mutex<usize>>) {
    let mut index = index.blocking_lock();
    *index = index.saturating_sub(1);
}
