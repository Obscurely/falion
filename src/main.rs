mod search;
use colored::Colorize;
use crossterm::terminal;
use std::{env, io};
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
    let stackoverflow_results = results_awaited.0;
    let stackexchange_results = results_awaited.1;
    let github_gist_results = results_awaited.2;
    let geeksforgeeks_results = results_awaited.3;
    let duck_search_results = results_awaited.4;

    // println!("Stackoverflow: ");
    // for l in stackoverflow_results {
    //     println!("{}", l.0);
    // }
    // println!("Stackexchange: ");
    // for l in stackexchange_results {
    //     println!("{}", l.0);
    // }
    // println!("Github Gist: ");
    // for l in github_gist_results {
    //     println!("{}", l.0);
    // }
    // println!("GeeksForGeeks: ");
    // for l in geeksforgeeks_results {
    //     println!("{}", l.0);
    // }
    // println!("Duck search: ");
    // for l in duck_search_results {
    //     println!("{}", l.0);
    // }

    // let mut tt = String::from("");
    // io::stdin().read_line(&mut tt);

    // let links = GeeksForGeeks::get_links(&search_text).await;

    // println!("Done");
    // println!("{}", links.len());

    // let mut s = vec![];
    // for t in links {
    //     println!("{}: {}", t.0, t.1);
    //     s.push(tokio::task::spawn(GeeksForGeeks::get_page_content(t.1, term_width)));
    //     // println!("{}", content);

    //     // if true {
    //     //     break;
    //     // }
    // }

    // let s = futures::future::join_all(s).await;
}
