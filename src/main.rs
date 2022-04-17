mod search;
use colored::Colorize;
use crossterm::terminal;
use std::borrow::Borrow;
use std::{env, io};
use indexmap::IndexMap;
use search::stackoverflow::StackOverFlow;
use search::stackexchange::StackExchange;
use search::github_gist::GithubGist;
use search::geeksforgeeks::GeeksForGeeks;
use search::duckduckgo_search::DuckSearch;

#[tokio::main]
async fn main() {
    let term_width = usize::from(match terminal::size() {
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

    // getting args list and making a string from it
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("[115] {}", "You have to provide a search querry, either surronded by \" or the querry as it is after the program's name.".to_string().red());
        std::process::exit(115);
    }
    let mut search_text = args.join(" ");
    search_text = search_text.replace((args[0].to_string() + " ").as_str(), "");

    // getting futures of the resources we want results from
    let stackoverflow_results = StackOverFlow::get_questions_and_content(&search_text, term_width);
    let stackexchange_results = StackExchange::get_questions_and_content(&search_text, term_width);
    let github_gist_results = GithubGist::get_name_and_content(&search_text);
    let geeksforgeeks_results = GeeksForGeeks::get_name_and_content(&search_text, term_width);
    let duck_search_results = DuckSearch::get_name_and_content(&search_text, term_width);

    // awaiting them all at the same time
    let results_awaited = futures::join!(stackoverflow_results, stackexchange_results, github_gist_results, geeksforgeeks_results, duck_search_results);

    // transfer the awaited futures back into the variables
    let mut stackoverflow_results = results_awaited.0;
    let mut stackexchange_results = results_awaited.1;
    let mut github_gist_results = results_awaited.2;
    let mut geeksforgeeks_results = results_awaited.3;
    let mut duck_search_results = results_awaited.4;
    
    // vars to store the awaited results in
    let mut stackoverflow_awaited: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut stackexchange_awaited: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut github_gist_awaited: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut geeksforgeeks_awaited: IndexMap<String, String> = IndexMap::new();
    let mut duck_search_awaited: IndexMap<String, String> = IndexMap::new();

    // main interface loop
    let default_error = String::from("There was an error getting any results!"); 

    let stackoverflow_current = match falion::get_key_at_index_map_with_vec(0, &stackoverflow_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let stackexchange_current = match falion::get_key_at_index_map_with_vec(0, &stackexchange_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let github_gist_current = match falion::get_key_at_index_map_with_vec(0, &github_gist_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let geeksforgeeks_current = match falion::get_key_at_index_map_with_string(0, &geeksforgeeks_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let duck_search_current = match falion::get_key_at_index_map_with_string(0, &duck_search_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    loop {
        // printing search querry
        println!("Your search querry is: {}", search_text.green());
        // printing options
        println!("(1) [  StackOverFlow  ] {}", &stackoverflow_current);
        println!("(2) [  StackExchange  ] {}", &stackexchange_current);
        println!("(3) [   Github Gist   ] {}", &github_gist_current);
        println!("(4) [  GeeksForGeeks  ] {}", &geeksforgeeks_current);
        println!("(5) [DuckDuckGo Search] {}", &duck_search_current);

        // to not exit
        let mut test = String::from("");
        io::stdin().read_line(&mut test);
   }

    // let test = falion::get_index_hash_with_string(0, &mut geeksforgeeks_results, &mut geeksforgeeks_awaited).await.unwrap(); 
    // println!("At key: {}, there is content: \n\n {}", test.0, test.1);

    // let test = falion::get_index_hash_with_string(0, &mut geeksforgeeks_results, &mut geeksforgeeks_awaited).await.unwrap(); 
    // println!("At key: {}, there is content: \n\n {}", test.0, test.1);

    // let mut test = String::from("");
    // io::stdin().read_line(&mut test);
}
