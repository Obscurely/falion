mod search;
use colored::Colorize;
use search::stackoverflow::StackOverFlow;
use std::{borrow::BorrowMut, future};
use std::{env, process};

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
    let input: usize = match input.trim().parse() {
        Ok(input) => input,
        Err(error) => {
            eprintln!(
                "There was an error parsing user input, the given error is: {}",
                format!("{}", error).red()
            );
            process::exit(109);
        }
    };

    let index = input - 1;
    // let x = contents.remove(index);
    // let x = x.await;
    let start = std::time::Instant::now();
    let x = match contents.get_mut(index) {
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

    for content in x {
        println!("\n\nQuestion/Answer:\n {}", &content);
    }
}
