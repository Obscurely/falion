mod search;
use colored::Colorize;
use search::stackoverflow::StackOverFlow;
use std::{borrow::BorrowMut, future};
use std::{env, process};

use crate::search::util::Util;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut search_text = args.join(" ");
    search_text = search_text.replace((args[0].to_string() + " ").as_str(), "");
    println!("{}", search_text);
    let body = StackOverFlow::get_questions(search_text.as_str()).await;

    let mut i = 1;
    let mut contents = vec![];
    for (key, value) in body {
        contents.push(tokio::task::spawn(StackOverFlow::get_question_content(
            value,
        )));
        println!("{}. {}", i, key);
        i += 1;
    }

    let mut input = String::from("");
    std::io::stdin().read_line(&mut input);
    let mut selected_question: usize = 1; 

    let mut valid_input = false;
    while !valid_input {
        selected_question = match input.trim().parse() {
            Ok(input_int) => {
                if input_int > contents.len() {
                    println!("Input number was too high, please try again: ");
                    input.clear();
                    continue;
                }

                valid_input = true;
                input_int
            },
            Err(error) => {
                eprintln!(
                    "There was an error parsing user input, the given error is: {}",
                    format!("{}", error).red()
                );
                println!("Please try again:");
                
                input.clear();
                std::io::stdin().read_line(&mut input);
                continue;
            }
        };
    }

    let index = selected_question - 1;
    // let x = contents.remove(index);
    // let x = x.await;
    let start = std::time::Instant::now();
    let selected_question_content = match contents.get_mut(index) {
        Some(x) => match x.await {
            Ok(x) => x,
            Err(error) => {
                eprintln!("There was an error awaiting for the response for the chosen question, the given error is: {}", format!("{}", error).red());
                process::exit(111);
            }
        },
        None => {
            eprintln!("There was an error getting the question content for the chosen question.");
            process::exit(110);
        }
    };

    let dur = std::time::Instant::now() - start;
    println!("The duration to await ms: {}", dur.as_millis());

    Util::clear_terminal();
    println!("{}", selected_question_content[0]);
    Util::move_cursor_beginning();
    std::io::stdin().read_line(&mut input);
}

