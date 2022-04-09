pub mod search;
use crate::search::util::Util;
use colored::Colorize;
use crossterm::event;
use crossterm::terminal;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::{io, process};

pub fn get_valid_question_select(contents: &Vec<JoinHandle<Vec<String>>>) -> usize {
    println!("\n{}", "Input your choice: ".green());
    let mut input = String::from("");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("[113] There was an error reading the input, can't continue, the given error is: {}", format!("{}", error).red());
            process::exit(113);
        }
    };
    let mut selected_question: usize = 1;

    let mut valid_input = false;
    while !valid_input {
        selected_question = match input.trim().parse() {
            Ok(input_int) => {
                if input_int > contents.len() {
                    println!("\nInput number was too high, please try again: ");
                    input.clear();
                    continue;
                }

                valid_input = true;
                input_int
            }
            Err(error) => {
                eprintln!(
                    "[504] There was an error parsing user input, the given error is: {}",
                    format!("{}", error).red()
                );
                println!("\nPlease try again:");

                input.clear();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("[113] There was an error reading the input, can't continue, the given error is: {}", format!("{}", error).red());
                        process::exit(113);
                    }
                };
                continue;
            }
        };
    }

    selected_question
}

pub async fn get_question_content(contents: &mut Vec<JoinHandle<Vec<String>>>, content_awaited: &mut HashMap<usize, Vec<String>>, index: usize) -> Vec<String> {
    let mut selected_question_content = vec![String::from("")];
    if content_awaited.contains_key(&index) {
        selected_question_content = match content_awaited.get_mut(&index) {
            Some(val) => val.to_owned(),
            None => {
                eprintln!(
                    "[112] There was an retrieving an already awaited response, can't continue"
                );
                process::exit(112);
            }
        }
    } else {
        selected_question_content = match contents.get_mut(index) {
            Some(x) => match x.await {
                Ok(x) => {
                    content_awaited.insert(index, x.clone());
                    x
                }
                Err(error) => {
                    eprintln!("[111] There was an error awaiting for the response for the chosen question, the given error is: {}", format!("{}", error).red());
                    process::exit(111);
                }
            },
            None => {
                eprintln!("[110] There was an error getting the question content for the chosen question.");
                process::exit(110);
            }
        };
    }

    selected_question_content
}

pub fn loop_through_question(stdout: &mut io::Stdout, selected_question_content: &Vec<String>) {
    // clear the terminal
    Util::clear_terminal(stdout);

    // print the question content
    let mut r = 0;
    println!("{}", "Question/Answer:\n".green());
    println!("{}", selected_question_content[r]);
    Util::move_cursor_beginning(stdout);

    // enable raw mode in order to be able to use keybings
    match terminal::enable_raw_mode() {
        Ok(_) => (),
        Err(error) => {
            eprintln!("[505] Warning, there was an error enabling raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
        }
    }
    loop {
        Util::move_cursor_beginning(stdout);
        //matching the key
        match event::read().unwrap() {
            //i think this speaks for itself
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                match terminal::disable_raw_mode() {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("[506] Warning! There was an error disabling terminal raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
                    }
                }
                if r < selected_question_content.len() - 1 {
                    r += 1;
                    Util::clear_terminal(stdout);
                    println!("{}", "Question/Answer:\n".green());
                    println!("{}", selected_question_content[r]);
                    Util::move_cursor_beginning(stdout);
                }
                match terminal::enable_raw_mode() {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("[507] Warning, there was an error enabling raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
                    }
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('b'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                match terminal::disable_raw_mode() {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("[508] Warning! There was an error disabling terminal raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
                    }
                }
                if r > 0 {
                    r -= 1;
                    Util::clear_terminal(stdout);
                    println!("{}", "Question/Answer:\n".green());
                    println!("{}", selected_question_content[r]);
                    Util::move_cursor_beginning(stdout);
                }
                match terminal::enable_raw_mode() {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("[509] Warning, there was an error enabling raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
                    }
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                match terminal::disable_raw_mode() {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("[510] Warning! There was an error disabling terminal raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
                    }
                }
                Util::clear_terminal(stdout);

                break;
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                match terminal::disable_raw_mode() {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("[511] Warning! There was an error disabling terminal raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
                    }
                }
                std::process::exit(0);
            }
            _ => (),
        }
    }
}
