use hashbrown::HashMap;
use tokio::task::JoinHandle;

type ResultsStaticType<E> = Vec<(String, JoinHandle<Result<String, E>>)>;
type ResultsDynType<E> = Vec<(String, JoinHandle<Result<Vec<String>, E>>)>;

/// Get dynamic type content (in form of a vector). Either await it if it wasn't already, it it was
/// get it from the awaited list.
///
/// # Arguments
///
/// `results_ref` - mutable reference to results.
/// `results_awaited_ref` - mutable reference to the object keeping already awaited resources.
/// `results_index` - which result to get back, it's index.
#[tracing::instrument(skip_all)]
pub async fn get_dyn_result_content<'a, E>(
    results_ref: &'a mut Result<ResultsDynType<E>, E>,
    results_awaited_ref: &'a mut HashMap<String, Vec<String>>,
    results_index: usize,
) -> Option<&'a Vec<String>>
where
    E: std::fmt::Display,
{
    match results_ref {
        Ok(res) => {
            if let Some(unawaited_res) = res.get_mut(results_index) {
                let (title, handle) = unawaited_res;
                if results_awaited_ref.contains_key(title) {
                    match results_awaited_ref.get(title) {
                        Some(res) => Some(res),
                        None => None,
                    }
                } else {
                    let awaited = match handle.await {
                        Ok(handled) => match handled {
                            Ok(content) => content,
                            Err(error) => {
                                tracing::error!("There was an error getting the contetn for this a result. Error: {}", error);
                                vec![format!("There has been an error getting the content for this result. Error: {}", error)]
                            }
                        },
                        Err(error) => {
                            tracing::error!(
                                "There was an error handeling the future for a result. Error: {}",
                                error
                            );
                            vec![format!("There has been an error handeling the future for this result. Error: {}", error)]
                        }
                    };

                    // save already awaited
                    results_awaited_ref.insert(title.to_owned(), awaited);

                    // unwrap is safe since we just inserted this element
                    results_awaited_ref.get(title)
                }
            } else {
                None
            }
        }
        Err(error) => {
            tracing::info!(
                "User tryed accessing a resource that has been deemed unavailable. Error: {}",
                error
            );
            None
        }
    }
}

/// Get the static content for a result (String type). Either await it or get it from the list of
/// already awaited.
///
/// # Arguments
///
/// `results_ref` - mutable reference to results.
/// `results_awaited_ref` - mutable reference to the object keeping already awaited resources.
/// `results_index` - which result to get back, it's index.
#[tracing::instrument(skip_all)]
pub async fn get_static_result_content<'a, E>(
    results_ref: &'a mut Result<ResultsStaticType<E>, E>,
    results_awaited_ref: &'a mut HashMap<String, String>,
    results_index: usize,
) -> Option<&'a String>
where
    E: std::fmt::Display,
{
    match results_ref {
        Ok(res) => {
            if let Some(unawaited_res) = res.get_mut(results_index) {
                let (title, handle) = unawaited_res;
                if results_awaited_ref.contains_key(title) {
                    match results_awaited_ref.get(title) {
                        Some(res) => Some(res),
                        None => None,
                    }
                } else {
                    let awaited = match handle.await {
                        Ok(handled) => match handled {
                            Ok(content) => content,
                            Err(error) => {
                                tracing::error!("There was an error getting the contetn for this a result. Error: {}", error);
                                format!("There has been an error getting the content for this result. Error: {}", error)
                            }
                        },
                        Err(error) => {
                            tracing::error!(
                                "There was an error handeling the future for a result. Error: {}",
                                error
                            );
                            format!("There has been an error handeling the future for this result. Error: {}", error)
                        }
                    };

                    // save already awaited
                    results_awaited_ref.insert(title.to_owned(), awaited);

                    // unwrap is safe since we just inserted this element
                    results_awaited_ref.get(title)
                }
            } else {
                None
            }
        }
        Err(error) => {
            tracing::info!(
                "User tryed accessing a resource that has been deemed unavailable. Error: {}",
                error
            );
            None
        }
    }
}
