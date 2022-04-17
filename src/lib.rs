pub mod search;
use colored::Colorize;
use indexmap::IndexMap;

pub fn get_key_at_index_map_with_vec(index: usize, results: &IndexMap<String, tokio::task::JoinHandle<Vec<String>>>) -> Option<String> {
    if results.len() > index {
        let current = match results.get_index(index) {
            Some(value) => value,
            None => return None,
        };

        return Some(current.0.to_string());
    }

    None
}

pub fn get_key_at_index_map_with_string(index: usize, results: &IndexMap<String, tokio::task::JoinHandle<String>>) -> Option<String> {
    if results.len() > index {
        let current = match results.get_index(index) {
            Some(value) => value,
            None => return None,
        };

        return Some(current.0.to_string());
    }

    None
}

pub async fn get_index_map_with_vec(index: usize, results: &mut IndexMap<String, tokio::task::JoinHandle<Vec<String>>>, awaited: &mut IndexMap<String, Vec<String>>) -> Option<(String, Vec<String>)> {
    if results.len() > index {
        let current = match results.get_index_mut(index) {
            Some(val) => val,
            None => return None,
        };

        if awaited.contains_key(current.0) {
            return Some((current.0.to_string(), awaited.get(current.0).unwrap().clone()));
        } else {
            let content_awaited = match current.1.await {
                Ok(value) => value,
                Err(_) => return None,
            };
            
            awaited.insert(current.0.to_string(), content_awaited.clone());

            return Some((current.0.to_string(), content_awaited));
        }
    } 

    None
}

pub async fn get_index_map_with_string(index: usize, results: &mut IndexMap<String, tokio::task::JoinHandle<String>>, awaited: &mut IndexMap<String, String>) -> Option<(String, String)> {
    if results.len() > index {
        let current = match results.get_index_mut(index) {
            Some(val) => val,
            None => return None,
        };

        if awaited.contains_key(current.0) {
            return Some((current.0.to_string(), awaited.get(current.0).unwrap().clone()));
        } else {
            let content_awaited = match current.1.await {
                Ok(value) => value,
                Err(_) => return None,
            };
            
            awaited.insert(current.0.to_string(), content_awaited.clone());

            return Some((current.0.to_string(), content_awaited));
        }
    } 

    None
}


pub async fn get_key_map_with_vec(key: &str, results: &mut IndexMap<String, tokio::task::JoinHandle<Vec<String>>>, awaited: &mut IndexMap<String, Vec<String>>) -> Option<Vec<String>> {
    if awaited.contains_key(key) {
        return Some(awaited.get(key).unwrap().clone());
    } else if results.contains_key(key) {
        let current = match results.get_mut(key).unwrap().await {
            Ok(value) => value,
            Err(error) => {
                eprintln!("[580] Warning! There was an error awaiting the value at the specified key, the key existed, the given error is: {}", format!("{}", error).red());
                return None;
            }
        };

        awaited.insert(key.to_string(), current.clone());

        return Some(current);
    }

    None
}

pub async fn get_key_map_with_string(key: &str, results: &mut IndexMap<String, tokio::task::JoinHandle<String>>, awaited: &mut IndexMap<String, String>) -> Option<String> {
    if awaited.contains_key(key) {
        return Some(awaited.get(key).unwrap().clone());
    } else if results.contains_key(key) {
        let current = match results.get_mut(key).unwrap().await {
            Ok(value) => value,
            Err(error) => {
                eprintln!("[580] Warning! There was an error awaiting the value at the specified key, the key existed, the given error is: {}", format!("{}", error).red());
                return None;
            }
        };

        awaited.insert(key.to_string(), current.clone());

        return Some(current);
    }

    None
}
