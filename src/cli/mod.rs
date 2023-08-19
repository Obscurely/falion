mod content;
mod print;
mod util;
use super::search::ddg_search::DdgSearchError;
use super::search::geeksforgeeks::GfgError;
use super::search::github_gist::GithubGistError;
use super::search::stackexchange::SeError;
use super::search::stackoverflow::SofError;
use clap::Parser;
use crossterm::event;
use crossterm::style;
use crossterm::style::Stylize;
use indexmap::IndexMap;
use std::io::Write;
use tokio::task::JoinHandle;
pub use util::setup_cli;

type StackOverflowResults =
    Result<IndexMap<String, JoinHandle<Result<Vec<String>, SofError>>>, SofError>;
type StackExchangeResults =
    Result<IndexMap<String, JoinHandle<Result<Vec<String>, SeError>>>, SeError>;
type GithubGistResults =
    Result<IndexMap<String, JoinHandle<Result<Vec<String>, GithubGistError>>>, GithubGistError>;
type GeeksForGeeksResults =
    Result<IndexMap<String, JoinHandle<Result<String, GfgError>>>, GfgError>;
type DdgSearchResults =
    Result<IndexMap<String, JoinHandle<Result<String, DdgSearchError>>>, DdgSearchError>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Search query
    pub query: Vec<String>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable logs
    #[arg(short, long)]
    pub disable_logs: bool,
}

pub async fn cli(
    query: String,
    mut stackoverflow_results: StackOverflowResults,
    mut stackexchange_results: StackExchangeResults,
    mut github_gist_results: GithubGistResults,
    mut geeksforgeeks_results: GeeksForGeeksResults,
    mut ddg_search_results: DdgSearchResults,
) {
    // create stdout
    let mut stdout = std::io::stdout();

    // create vars
    let mut stackoverflow_results_awaited: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut stackoverflow_index = 0;
    let mut stackexchange_results_awaited: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut stackexchange_index = 0;
    let mut github_gist_results_awaited: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut github_gist_index = 0;
    let mut geeksforgeeks_results_awaited: IndexMap<String, String> = IndexMap::new();
    let mut geeksforgeeks_index = 0;
    let mut ddg_search_results_awaited: IndexMap<String, String> = IndexMap::new();
    let mut ddg_search_index = 0;
    // actual cli
    // reusable prints
    let query_print = format!("{} {}", "Your search query is:".green(), query.blue());
    let sof_print = format!("{} {} ", "(1)".green(), "[  StackOverFlow  ]".yellow());
    let se_print = format!("{} {} ", "(2)".green(), "[  StackExchange  ]".yellow());
    let gg_print = format!("{} {} ", "(3)".green(), "[   Github Gist   ]".yellow());
    let gfg_print = format!("{} {} ", "(4)".green(), "[  GeeksForGeeks  ]".yellow());
    let ddg_print = format!("{} {} ", "(5)".green(), "[DuckDuckGo Search]".yellow());
    // clear terminal
    util::clear_terminal(&mut stdout);

    loop {
        // save values as mutable references in loop in order to mitigate nll borrow checker false
        // positives when in a loop
        let stackoverflow_results_ref = &mut stackoverflow_results;
        let stackexchange_results_ref = &mut stackexchange_results;
        let github_gist_results_ref = &mut github_gist_results;
        let geeksforgeeks_results_ref = &mut geeksforgeeks_results;
        let ddg_search_results_ref = &mut ddg_search_results;
        // display query
        if let Err(error) = crossterm::queue!(
            &mut stdout,
            style::PrintStyledContent(query_print.as_str().stylize()),
            style::Print("\n\r")
        ) {
            tracing::warn!("There was an error printing some text. Error: {}", error);
        };

        // display resources
        print::print_resource::<Vec<String>, SofError>(
            &mut stdout,
            stackoverflow_index,
            &sof_print,
            stackoverflow_results_ref,
        );
        print::print_resource::<Vec<String>, SeError>(
            &mut stdout,
            stackexchange_index,
            &se_print,
            stackexchange_results_ref,
        );
        print::print_resource::<Vec<String>, GithubGistError>(
            &mut stdout,
            github_gist_index,
            &gg_print,
            github_gist_results_ref,
        );
        print::print_resource::<String, GfgError>(
            &mut stdout,
            geeksforgeeks_index,
            &gfg_print,
            geeksforgeeks_results_ref,
        );
        print::print_resource::<String, DdgSearchError>(
            &mut stdout,
            ddg_search_index,
            &ddg_print,
            ddg_search_results_ref,
        );

        // flush in order to print content
        if let Err(error) = stdout.flush() {
            tracing::warn!(
                "There was an error flushing stdout in order to print resources. Error: {}",
                error
            );
        }

        let event_read = match event::read() {
            Ok(ev) => ev,
            Err(error) => {
                tracing::warn!("There was an error reading the input event... going to the next iteration. If this continue please post an issue on github with the specific log file. Error: {}", error);
                continue;
            }
        };

        // matching the pressed key
        match event_read {
            // enter the menu for first resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('1'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                match content::get_dyn_result_content(
                    stackoverflow_results_ref,
                    &mut stackoverflow_results_awaited,
                    stackoverflow_index,
                )
                .await
                {
                    Some(content) => {
                        util::clear_terminal(&mut stdout);
                        if print::print_dyn_content(&mut stdout, content, true) {
                            util::clean(&mut stdout);
                            return;
                        }
                    }
                    None => {
                        tracing::warn!(
                            "User tried accessing stackoverflow which has been deemed unavailable."
                        );
                    }
                }
            }
            // go to next element in the first resource (using ! because of terminal limitations)
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('!'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // stackoverflow next result
                match &stackoverflow_results {
                    Ok(res) => {
                        if stackoverflow_index < res.len() - 1 {
                            stackoverflow_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
            }
            // go to the previous element in the first resource (using alt instead of ctrl because of terminal limitations)
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('1'),
                modifiers: event::KeyModifiers::ALT,
                ..
            }) => {
                // stackoverflow back results by one
                stackoverflow_index = stackoverflow_index.saturating_sub(1);
            }

            // enter the menu of the second resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('2'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // stackexchange current result content
                match content::get_dyn_result_content(
                    stackexchange_results_ref,
                    &mut stackexchange_results_awaited,
                    stackexchange_index,
                )
                .await
                {
                    Some(content) => {
                        util::clear_terminal(&mut stdout);
                        if print::print_dyn_content(&mut stdout, content, true) {
                            util::clean(&mut stdout);
                            return;
                        }
                    }
                    None => {
                        tracing::warn!(
                            "User tried accessing stackexchange which has been deemed unavailable."
                        );
                    }
                }
            }
            // go to the next element in the second resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('@'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // stackexchange next result
                match stackexchange_results_ref {
                    Ok(res) => {
                        if stackexchange_index < res.len() - 1 {
                            stackexchange_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
            }
            // go to previous element in the second resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('2'),
                modifiers: event::KeyModifiers::ALT,
                ..
            }) => {
                // stackexchange back results by one
                stackexchange_index = stackexchange_index.saturating_sub(1);
            }

            // enter the menu for the third resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('3'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // github_gist show current result content
                match content::get_dyn_result_content(
                    github_gist_results_ref,
                    &mut github_gist_results_awaited,
                    github_gist_index,
                )
                .await
                {
                    Some(content) => {
                        util::clear_terminal(&mut stdout);
                        if print::print_dyn_content(&mut stdout, content, false) {
                            util::clean(&mut stdout);
                            return;
                        }
                    }
                    None => {
                        tracing::warn!(
                            "User tried accessing github gist which has been deemed unavailable."
                        );
                    }
                }
            }
            // go to the next element in the third resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('#'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // github gist next result
                match github_gist_results_ref {
                    Ok(res) => {
                        if github_gist_index < res.len() - 1 {
                            github_gist_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
            }
            // go to the previous element in the third resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('3'),
                modifiers: event::KeyModifiers::ALT,
                ..
            }) => {
                // github gist back results by one
                github_gist_index = github_gist_index.saturating_sub(1);
            }

            // enter the forth resource menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('4'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // geeksforgeeks show content for current result
                match content::get_static_result_content(
                    geeksforgeeks_results_ref,
                    &mut geeksforgeeks_results_awaited,
                    geeksforgeeks_index,
                )
                .await
                {
                    Some(content) => {
                        util::clear_terminal(&mut stdout);
                        if print::print_static_content(&mut stdout, content) {
                            util::clean(&mut stdout);
                            return;
                        }
                    }
                    None => {
                        tracing::warn!(
                            "User tried accessing geeksforgeeks which has been deemed unavailable."
                        );
                    }
                }
            }
            // go to the next element in the forth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('$'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // geeksforgeeks next result
                match geeksforgeeks_results_ref {
                    Ok(res) => {
                        if geeksforgeeks_index < res.len() - 1 {
                            geeksforgeeks_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
            }
            // go to the previous element in the forth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('4'),
                modifiers: event::KeyModifiers::ALT,
                ..
            }) => {
                // geeksforgeeks back results by one
                geeksforgeeks_index = geeksforgeeks_index.saturating_sub(1);
            }

            // enter the fifth resource menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('5'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // ddg search show content for current result
                match content::get_static_result_content(
                    ddg_search_results_ref,
                    &mut ddg_search_results_awaited,
                    ddg_search_index,
                )
                .await
                {
                    Some(content) => {
                        util::clear_terminal(&mut stdout);
                        if print::print_static_content(&mut stdout, content) {
                            util::clean(&mut stdout);
                            return;
                        }
                    }
                    None => {
                        tracing::warn!(
                            "User tried accessing ddg search which has been deemed unavailable."
                        );
                    }
                }
            }
            // go to the next element in the fifth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('%'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                // ddg search next result
                match ddg_search_results_ref {
                    Ok(res) => {
                        if ddg_search_index < res.len() - 1 {
                            ddg_search_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
            }
            // go to the previous element in the fifth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('5'),
                modifiers: event::KeyModifiers::ALT,
                ..
            }) => {
                // ddg search back results by one
                ddg_search_index = ddg_search_index.saturating_sub(1);
            }

            // move every resource to it's next element in the list, if any more
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                // move all resources to the next element
                match stackoverflow_results_ref {
                    Ok(res) => {
                        if stackoverflow_index < res.len() - 1 {
                            stackoverflow_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
                match stackexchange_results_ref {
                    Ok(res) => {
                        if stackexchange_index < res.len() - 1 {
                            stackexchange_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
                match github_gist_results_ref {
                    Ok(res) => {
                        if github_gist_index < res.len() - 1 {
                            github_gist_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
                match geeksforgeeks_results_ref {
                    Ok(res) => {
                        if geeksforgeeks_index < res.len() - 1 {
                            geeksforgeeks_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
                match ddg_search_results_ref {
                    Ok(res) => {
                        if ddg_search_index < res.len() - 1 {
                            ddg_search_index += 1;
                        }
                    }
                    // we already handled the error
                    Err(_) => (),
                }
            }
            // move to the previous element in the list of every resource, if any more
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('b'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                // move all the resources to the previous element
                stackoverflow_index = stackoverflow_index.saturating_sub(1);
                stackexchange_index = stackexchange_index.saturating_sub(1);
                github_gist_index = github_gist_index.saturating_sub(1);
                geeksforgeeks_index = geeksforgeeks_index.saturating_sub(1);
                ddg_search_index = ddg_search_index.saturating_sub(1);
            }

            // clear the terminal and exit the program
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                util::clean(&mut stdout);
                return;
            }
            _ => (),
        }

        util::clear_terminal(&mut stdout);
    }
}
