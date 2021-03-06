pub mod search;
use colored::Colorize;
use crossterm::event;
use indexmap::IndexMap;
use std::io;
use std::process;

use crate::search::util::Util;

pub fn get_key_at_index_map_with_vec(
    index: usize,
    results: &IndexMap<String, tokio::task::JoinHandle<Vec<String>>>,
) -> Option<String> {
    if results.len() > index {
        let current = match results.get_index(index) {
            Some(value) => value,
            None => return None,
        };

        return Some(current.0.to_string());
    }

    None
}

pub fn get_key_at_index_map_with_string(
    index: usize,
    results: &IndexMap<String, tokio::task::JoinHandle<String>>,
) -> Option<String> {
    if results.len() > index {
        let current = match results.get_index(index) {
            Some(value) => value,
            None => return None,
        };

        return Some(current.0.to_string());
    }

    None
}

pub async fn get_index_map_with_vec(
    index: usize,
    results: &mut IndexMap<String, tokio::task::JoinHandle<Vec<String>>>,
    awaited: &mut IndexMap<String, Vec<String>>,
) -> Option<(String, Vec<String>)> {
    if results.len() > index {
        let current = match results.get_index_mut(index) {
            Some(val) => val,
            None => return None,
        };

        if awaited.contains_key(current.0) {
            return Some((
                current.0.to_string(),
                awaited.get(current.0).unwrap().clone(),
            ));
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

pub async fn get_index_map_with_string(
    index: usize,
    results: &mut IndexMap<String, tokio::task::JoinHandle<String>>,
    awaited: &mut IndexMap<String, String>,
) -> Option<(String, String)> {
    if results.len() > index {
        let current = match results.get_index_mut(index) {
            Some(val) => val,
            None => return None,
        };

        if awaited.contains_key(current.0) {
            return Some((
                current.0.to_string(),
                awaited.get(current.0).unwrap().clone(),
            ));
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

pub async fn get_key_map_with_vec(
    key: &str,
    results: &mut IndexMap<String, tokio::task::JoinHandle<Vec<String>>>,
    awaited: &mut IndexMap<String, Vec<String>>,
    enable_warnings: bool,
) -> Option<Vec<String>> {
    if awaited.contains_key(key) {
        return Some(awaited.get(key).unwrap().clone());
    } else if results.contains_key(key) {
        let current = match results.get_mut(key).unwrap().await {
            Ok(value) => value,
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[535][Warning] There was an error awaiting the value at the specified key, the key existed, the given error is:".yellow(), format!("{}", error).red());
                }
                return None;
            }
        };

        awaited.insert(key.to_string(), current.clone());

        return Some(current);
    }

    None
}

pub async fn get_key_map_with_string(
    key: &str,
    results: &mut IndexMap<String, tokio::task::JoinHandle<String>>,
    awaited: &mut IndexMap<String, String>,
    enable_warnings: bool,
) -> Option<String> {
    if awaited.contains_key(key) {
        return Some(awaited.get(key).unwrap().clone());
    } else if results.contains_key(key) {
        let current = match results.get_mut(key).unwrap().await {
            Ok(value) => value,
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[536][Warning] There was an error awaiting the value at the specified key, the key existed, the given error is:".yellow(), format!("{}", error).red());
                }
                return None;
            }
        };

        awaited.insert(key.to_string(), current.clone());

        return Some(current);
    }

    None
}

pub fn loop_prompt_stacks(content: &Vec<String>, stdout: &mut io::Stdout, enable_warnings: bool) {
    if content.is_empty() {
        if enable_warnings {
            eprintln!("{}", "[537][Warning] The accessed resource did not have anything in it, it normally should at least have a text saying there was an error".yellow());
        }
        println!("Press enter to continue!");
        let mut temp = String::from("");
        std::io::stdin().read_line(&mut temp);
        return;
    }
    let mut current_index = 0;
    loop {
        // printing the content
        if current_index == 0 {
            println!("{}", "Question:".green());
        } else {
            println!(
                "{} {}:",
                "Answer".green(),
                current_index.to_string().green()
            );
        }

        println!("{}", content[current_index]);

        // inform use if it's end of the answers
        if current_index + 1 == content.len() {
            println!("\n{}", "End of answers! Can't go any further!".green());
        }

        Util::enable_terminal_raw_mode(enable_warnings);
        let event_read = match event::read() {
            Ok(ev) => ev,
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[538][Warning] There was an error reading the input event, going to next iteration, if this continues please ctrl+c the program, the given error is:".yellow(), format!("{}", error).red());
                }
                continue;
            }
        };

        // matching the pressed key
        match event_read {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                if content.len() > current_index + 1 {
                    current_index += 1;
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('b'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                if current_index > 0 {
                    current_index -= 1;
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                return;
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(stdout, enable_warnings);
                process::exit(0);
            }
            _ => {
                Util::disable_terminal_raw_mode(enable_warnings);
            }
        }

        // clearing the terminal, since everything needed is gonna be printed again.
        Util::disable_terminal_raw_mode(enable_warnings);
        Util::clear_terminal(stdout, enable_warnings);
    }
}

pub fn loop_prompt_gist(content: &Vec<String>, stdout: &mut io::Stdout, enable_warnings: bool) {
    if content.is_empty() {
        if enable_warnings {
            eprintln!("{}", "[539][Warning] The accessed resource did not have anything in it, it normally should at least have a text saying there was an error".yellow());
        }
        println!("Press enter to continue!");
        let mut temp = String::from("");
        std::io::stdin().read_line(&mut temp);
        return;
    }
    let mut current_index = 0;
    loop {
        // printing the content
        println!(
            "{} {}{}",
            "File ".green(),
            (current_index + 1).to_string().green(),
            ":".green()
        );
        println!("{}", content[current_index]);

        // inform use if it's end of the gist's files
        if current_index + 1 == content.len() {
            println!(
                "\n{}",
                "End of files in gist! Can't go any further!".green()
            );
        }

        Util::enable_terminal_raw_mode(enable_warnings);
        let event_read = match event::read() {
            Ok(ev) => ev,
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[540][Warning] There was an error reading the input event, going to next iteration, if this continues please ctrl+c the program, the given error is:".yellow(), format!("{}", error).red());
                }
                continue;
            }
        };

        // matching the pressed key
        match event_read {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                if content.len() > current_index + 1 {
                    current_index += 1;
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('b'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                if current_index > 0 {
                    current_index -= 1;
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                return;
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(stdout, enable_warnings);
                process::exit(0);
            }
            _ => {
                Util::disable_terminal_raw_mode(enable_warnings);
            }
        }

        // clearing the terminal, since everything needed is gonna be printed again.
        Util::disable_terminal_raw_mode(enable_warnings);
        Util::clear_terminal(stdout, enable_warnings);
    }
}

pub fn loop_prompt_geeksforgeeks(content: &str, stdout: &mut io::Stdout, enable_warnings: bool) {
    if content.is_empty() {
        if enable_warnings {
            eprintln!("{}", "[541][Warning] The accessed resource did not have anything in it, it normally should at least have a text saying there was an error".yellow());
        }
        println!("Press enter to continue!");
        let mut temp = String::from("");
        std::io::stdin().read_line(&mut temp);
        return;
    }
    loop {
        // printing the content
        println!("{}", "GeeksForGeeks: ".green());
        println!("{}", content);

        // inform use if it's only one page
        println!(
            "\n{}",
            "There is only one page! Can't go back or further!".green()
        );

        Util::enable_terminal_raw_mode(enable_warnings);
        let event_read = match event::read() {
            Ok(ev) => ev,
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[542][Warning] There was an error reading the input event, going to next iteration, if this continues please ctrl+c the program, the given error is:".yellow(), format!("{}", error).red());
                }
                continue;
            }
        };

        // matching the pressed key
        match event_read {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                return;
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(stdout, enable_warnings);
                process::exit(0);
            }
            _ => {
                Util::disable_terminal_raw_mode(enable_warnings);
            }
        }

        // clearing the terminal, since everything needed is gonna be printed again.
        Util::disable_terminal_raw_mode(enable_warnings);
        Util::clear_terminal(stdout, enable_warnings);
    }
}

pub fn loop_prompt_duckduckgo(content: &str, stdout: &mut io::Stdout, enable_warnings: bool) {
    if content.is_empty() {
        if enable_warnings {
            eprintln!("{}", "[543][Warning] The accessed resource did not have anything in it, it normally should at least have a text saying there was an error".yellow());
        }
        println!("Press enter to continue!");
        let mut temp = String::from("");
        std::io::stdin().read_line(&mut temp);
        return;
    }
    loop {
        // printing the content
        println!("{}", "DuckDuckGo: ".green());
        println!("{}", content);

        // inform use if it's only one page
        println!(
            "\n{}",
            "There is only one page! Can't go back or further!".green()
        );

        Util::enable_terminal_raw_mode(enable_warnings);
        let event_read = match event::read() {
            Ok(ev) => ev,
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[544][Warning] There was an error reading the input event, going to next iteration, if this continues please ctrl+c the program, the given error is:".yellow(), format!("{}", error).red());
                }
                continue;
            }
        };

        // matching the pressed key
        match event_read {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                return;
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(stdout, enable_warnings);
                process::exit(0);
            }
            _ => {
                Util::disable_terminal_raw_mode(enable_warnings);
            }
        }

        // clearing the terminal, since everything needed is gonna be printed again.
        Util::disable_terminal_raw_mode(enable_warnings);
        Util::clear_terminal(stdout, enable_warnings);
    }
}
