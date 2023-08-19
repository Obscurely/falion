use indexmap::IndexMap;
use tokio::task::JoinHandle;

type ResultsStaticType<E> = IndexMap<String, JoinHandle<Result<String, E>>>;
type ResultsDynType<E> = IndexMap<String, JoinHandle<Result<Vec<String>, E>>>;

pub async fn get_dyn_result_content<'a, E>(
    results_ref: &'a mut Result<ResultsDynType<E>, E>,
    results_awaited_ref: &'a mut IndexMap<String, Vec<String>>,
    results_index: usize,
) -> Option<&'a Vec<String>>
where
    E: std::fmt::Display,
{
    match results_ref {
        Ok(res) => {
            if let Some(unawaited_res) = res.get_index_mut(results_index) {
                if results_awaited_ref.contains_key(unawaited_res.0) {
                    match results_awaited_ref.get(unawaited_res.0) {
                        Some(res) => Some(res),
                        None => None,
                    }
                } else {
                    let awaited = match unawaited_res.1.await {
                        Ok(handled) => match handled {
                            Ok(content) => content,
                            Err(error) => {
                                vec![format!("There has been an error handeling the future for this result. Error: {}", error)]
                            }
                        },
                        Err(error) => {
                            vec![format!("There has been an error handeling the future for this result. Error: {}", error)]
                        }
                    };

                    // save already awaited
                    results_awaited_ref.insert(unawaited_res.0.to_owned(), awaited);

                    // unwrap is safe since we just inserted this element
                    results_awaited_ref.get(unawaited_res.0)
                }
            } else {
                None
            }
        }
        Err(_) => {
            tracing::info!("User tryed accessing a resource that has been deemed unavailable.");
            None
        }
    }
}

pub async fn get_static_result_content<'a, E>(
    results_ref: &'a mut Result<ResultsStaticType<E>, E>,
    results_awaited_ref: &'a mut IndexMap<String, String>,
    results_index: usize,
) -> Option<&'a String>
where
    E: std::fmt::Display,
{
    match results_ref {
        Ok(res) => {
            if let Some(unawaited_res) = res.get_index_mut(results_index) {
                if results_awaited_ref.contains_key(unawaited_res.0) {
                    match results_awaited_ref.get(unawaited_res.0) {
                        Some(res) => Some(res),
                        None => None,
                    }
                } else {
                    let awaited = match unawaited_res.1.await {
                        Ok(handled) => match handled {
                            Ok(content) => content,
                            Err(error) => {
                                format!("There has been an error handeling the future for this result. Error: {}", error)
                            }
                        },
                        Err(error) => {
                            format!("There has been an error handeling the future for this result. Error: {}", error)
                        }
                    };

                    // save already awaited
                    results_awaited_ref.insert(unawaited_res.0.to_owned(), awaited);

                    // unwrap is safe since we just inserted this element
                    results_awaited_ref.get(unawaited_res.0)
                }
            } else {
                None
            }
        }
        Err(_) => {
            tracing::info!("User tryed accessing a resource that has been deemed unavailable.");
            None
        }
    }
}
