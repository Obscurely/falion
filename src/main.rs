use colored::Colorize;
use crossterm::terminal;
use falion::search::{stackoverflow::StackOverFlow, stackexchange::StackExchange};
use std::collections::HashMap;
use std::{env, io};

#[tokio::main]
async fn main() {
    let term_width = usize::from( match terminal::size() {
        Ok(size) => size.0,
        Err(error) => {
            eprintln!("[520] Warning! There was a problem detecting the terminal width, using 1920 (could use 1milion), the terminal should take care of it, it's just not gonna be as nice, the given error is: {}", format!("{}", error).red());
            1920
        }
    });
    // getting stdout into var in order to manipulate the terminal
    let mut stdout = io::stdout();

    // making sure we start with raw mode disabled on the terminal
    match terminal::disable_raw_mode() {
        Ok(_) => (),
        Err(error) => {
            eprintln!("[512] Warning! There was an error disabling terminal raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
        }
    }

    // getting the search text from the args of the terminal
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("[115] {}", "You have to provide a search querry, either surronded by \" or the querry as it is after the program's name.".to_string().red());
        std::process::exit(115);
    }
    let mut search_text = args.join(" ");
    search_text = search_text.replace((args[0].to_string() + " ").as_str(), "");

    // getting the question links for the provided search querry
    let body = StackExchange::get_questions(search_text.as_str()).await;
    let body_values = body.values();
    let body_keys = body.keys();

    // async getting the content for every link in order to be awaited when needed,
    // this makes it so on a fast connecting you don't even have to wait after seeing the list with questions
    let mut contents = vec![];
    for value in body_values {
        contents.push(tokio::task::spawn(StackOverFlow::get_question_content(
            value.clone(),
        term_width)));
    }

    let mut content_awaited: HashMap<usize, Vec<String>> = HashMap::new(); // storing what was already awaited in a different var
    loop {
        // print the search querry
        println!(
            "{}: {}",
            "Your input search querry".green(),
            (&search_text).to_string().blue()
        );

        // print the questions
        let mut i = 1;
        for key in (&body_keys).clone().into_iter().collect::<Vec<&String>>() {
            println!("{}. {}", i, key);
            i += 1;
        }

        // get user to select a question and get the content for it
        let index = falion::get_valid_question_select(&contents) - 1;
        let selected_question_content =
            falion::get_question_content(&mut contents, &mut content_awaited, index).await;

        falion::loop_through_question(&mut stdout, &selected_question_content);
    }
}
