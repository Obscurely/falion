mod content;
mod print;
mod util;
use super::search;
use super::search::ddg_search::DdgSearchError;
use super::search::geeksforgeeks::GfgError;
use super::search::github_gist::GithubGistError;
use super::search::stackexchange::SeError;
use super::search::stackoverflow::SofError;
use clap::Parser;
use crossterm::event;
use crossterm::style;
use crossterm::style::Stylize;
use hashbrown::HashMap;
use std::io::Write;

//
// CLI Key mapping
//

// Key press event modifiers
// On windows even though SHIFT + 1 results in ! we still need SHIFT as the modifier
// forward results modifiers
#[cfg(windows)]
const FORWARD_RESOURCE_MODIFIER: event::KeyModifiers = event::KeyModifiers::SHIFT;
#[cfg(not(windows))]
const FORWARD_RESOURCE_MODIFIER: event::KeyModifiers = event::KeyModifiers::NONE;
// backward results modifiers
#[cfg(target_os = "macos")]
const BACKWARD_RESOURCE_MODIFIER: event::KeyModifiers = event::KeyModifiers::NONE;
#[cfg(not(target_os = "macos"))]
const BACKWARD_RESOURCE_MODIFIER: event::KeyModifiers = event::KeyModifiers::ALT;

// Keys for accessing the resources
const ACCESS_RESOURCE_1_CHAR: char = '1';
const ACCESS_RESOURCE_2_CHAR: char = '2';
const ACCESS_RESOURCE_3_CHAR: char = '3';
const ACCESS_RESOURCE_4_CHAR: char = '4';
const ACCESS_RESOURCE_5_CHAR: char = '5';

// Keys for moving the results forward
const FORWARD_RESOURCE_1_CHAR: char = '!';
const FORWARD_RESOURCE_2_CHAR: char = '@';
const FORWARD_RESOURCE_3_CHAR: char = '#';
const FORWARD_RESOURCE_4_CHAR: char = '$';
const FORWARD_RESOURCE_5_CHAR: char = '%';

// Keys for moving results back because on macos using alt with numbers inputs symbols
// first resource
#[cfg(target_os = "macos")]
const BACKWARD_RESOURCE_1_CHAR: char = '¡';
#[cfg(not(target_os = "macos"))]
const BACKWARD_RESOURCE_1_CHAR: char = '1';
// first resource
#[cfg(target_os = "macos")]
const BACKWARD_RESOURCE_2_CHAR: char = '™';
#[cfg(not(target_os = "macos"))]
const BACKWARD_RESOURCE_2_CHAR: char = '2';
// first resource
#[cfg(target_os = "macos")]
const BACKWARD_RESOURCE_3_CHAR: char = '£';
#[cfg(not(target_os = "macos"))]
const BACKWARD_RESOURCE_3_CHAR: char = '3';
// first resource
#[cfg(target_os = "macos")]
const BACKWARD_RESOURCE_4_CHAR: char = '¢';
#[cfg(not(target_os = "macos"))]
const BACKWARD_RESOURCE_4_CHAR: char = '4';
// first resource
#[cfg(target_os = "macos")]
const BACKWARD_RESOURCE_5_CHAR: char = '∞';
#[cfg(not(target_os = "macos"))]
const BACKWARD_RESOURCE_5_CHAR: char = '5';

/// Command line options, cli setup done with clap.
///
/// # Options
///
/// query - mandatory, what to search for
/// verbose - optional, enable debug logging to stdout
/// disable_logs - optional, disable log completely, including writting to files.
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

    /// Run the ui from the cli
    #[arg(short, long)]
    pub ui: bool,

    /// Print the keybinds list
    #[arg(short, long)]
    pub keybinds: bool,
}

/// The main cli function for falion. Show results and lets you scroll through them.
///
/// # Arguments
///
/// `query` - The query to search for.
/// `stackoverflow_results` - stackoverflow results which should be got in the main function and
/// passed entierly (no reference)
/// `stackexchange_results` - stackexchange results which should be got in the main function and
/// passed entierly (no reference)
/// `github_gist_results` - github gist results which should be got in the main function and
/// passed entierly (no reference)
/// `geeksforgeeks_results` - geeksforgeeks results which should be got in the main function and
/// passed entierly (no reference)
/// `ddg_search_results` - ddg results which should be got in the main function and
/// passed entierly (no reference)
#[tracing::instrument(skip_all)]
pub async fn cli() {
    tracing::info!("User chose the cli.");
    // create stdout
    let mut stdout = std::io::stdout();

    // get results
    // setup cli and get query
    let query = match util::setup_cli() {
        Ok(query) => query,
        Err(err) => match err.kind() {
            std::io::ErrorKind::Other => {
                tracing::info!("User chose to run the gui from the cli.");
                return;
            }
            std::io::ErrorKind::NotFound => {
                tracing::error!("Can't continue, user provided a query shorter than 5 characters");
                eprintln!(
                    "Provided query is shorter than 5 characters. Do --help for more information"
                );
                return;
            }
            _ => return,
        },
    };

    // debug log the query
    tracing::debug!("The input query: {}", &query);

    // Make objects
    let client = search::util::client_with_special_settings();
    let stackoverflow = search::stackoverflow::StackOverflow::with_client(client.clone());
    let stackexchange = search::stackexchange::StackExchange::with_client(client.clone());
    let github_gist = search::github_gist::GithubGist::with_client(client.clone());
    let geeksforgeeks = search::geeksforgeeks::GeeksForGeeks::with_client(client.clone());
    let ddg_search = search::ddg_search::DdgSearch::with_client(client.clone());

    // Get results
    let stackoverflow_results = stackoverflow.get_multiple_questions_content(&query, Some(5));
    let stackexchange_results = stackexchange.get_multiple_questions_content(&query, Some(5));
    let github_gist_results = github_gist.get_multiple_gists_content(&query, Some(5));
    let geeksforgeeks_results = geeksforgeeks.get_multiple_pages_content(&query, Some(5));
    let ddg_search_results = ddg_search.get_multiple_pages_content(&query, Some(5));

    // await all results at the same time
    let results_awaited = futures::join!(
        stackoverflow_results,
        stackexchange_results,
        github_gist_results,
        geeksforgeeks_results,
        ddg_search_results
    );

    // transfer the awaited futures back
    let mut stackoverflow_results = results_awaited.0;
    let mut stackexchange_results = results_awaited.1;
    let mut github_gist_results = results_awaited.2;
    let mut geeksforgeeks_results = results_awaited.3;
    let mut ddg_search_results = results_awaited.4;

    // hide the cursor
    if let Err(error) = crossterm::execute!(&mut stdout, crossterm::cursor::Hide) {
        tracing::warn!("Failed to hide terminal cursor. Error: {}", error);
    };

    // create vars
    let mut stackoverflow_results_awaited: HashMap<String, Vec<String>> = HashMap::with_capacity(5);
    let mut stackoverflow_index = 0;
    let mut stackexchange_results_awaited: HashMap<String, Vec<String>> = HashMap::with_capacity(5);
    let mut stackexchange_index = 0;
    let mut github_gist_results_awaited: HashMap<String, Vec<String>> = HashMap::with_capacity(5);
    let mut github_gist_index = 0;
    let mut geeksforgeeks_results_awaited: HashMap<String, String> = HashMap::with_capacity(5);
    let mut geeksforgeeks_index = 0;
    let mut ddg_search_results_awaited: HashMap<String, String> = HashMap::with_capacity(5);
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

        // listen for key presses
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
                code: event::KeyCode::Char(ACCESS_RESOURCE_1_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                tracing::info!(
                    "Accessing content for resource 1 at index: {}",
                    stackoverflow_index
                );
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
                code: event::KeyCode::Char(FORWARD_RESOURCE_1_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: FORWARD_RESOURCE_MODIFIER,
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
                code: event::KeyCode::Char(BACKWARD_RESOURCE_1_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: BACKWARD_RESOURCE_MODIFIER,
                ..
            }) => {
                // stackoverflow back results by one
                stackoverflow_index = stackoverflow_index.saturating_sub(1);
            }

            // enter the menu of the second resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(ACCESS_RESOURCE_2_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                tracing::info!(
                    "Accessing content for resource 2 at index: {}",
                    stackexchange_index
                );
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
                code: event::KeyCode::Char(FORWARD_RESOURCE_2_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: FORWARD_RESOURCE_MODIFIER,
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
                code: event::KeyCode::Char(BACKWARD_RESOURCE_2_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: BACKWARD_RESOURCE_MODIFIER,
                ..
            }) => {
                // stackexchange back results by one
                stackexchange_index = stackexchange_index.saturating_sub(1);
            }

            // enter the menu for the third resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(ACCESS_RESOURCE_3_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                tracing::info!(
                    "Accessing content for resource 3 at index: {}",
                    github_gist_index
                );
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
                code: event::KeyCode::Char(FORWARD_RESOURCE_3_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: FORWARD_RESOURCE_MODIFIER,
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
                code: event::KeyCode::Char(BACKWARD_RESOURCE_3_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: BACKWARD_RESOURCE_MODIFIER,
                ..
            }) => {
                // github gist back results by one
                github_gist_index = github_gist_index.saturating_sub(1);
            }

            // enter the forth resource menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(ACCESS_RESOURCE_4_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                tracing::info!(
                    "Accessing content for resource 4 at index: {}",
                    geeksforgeeks_index
                );
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
                code: event::KeyCode::Char(FORWARD_RESOURCE_4_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: FORWARD_RESOURCE_MODIFIER,
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
                code: event::KeyCode::Char(BACKWARD_RESOURCE_4_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: BACKWARD_RESOURCE_MODIFIER,
                ..
            }) => {
                // geeksforgeeks back results by one
                geeksforgeeks_index = geeksforgeeks_index.saturating_sub(1);
            }

            // enter the fifth resource menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(ACCESS_RESOURCE_5_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                tracing::info!(
                    "Accessing content for resource 5 at index: {}",
                    ddg_search_index
                );
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
                code: event::KeyCode::Char(FORWARD_RESOURCE_5_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: FORWARD_RESOURCE_MODIFIER,
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
                code: event::KeyCode::Char(BACKWARD_RESOURCE_5_CHAR),
                kind: event::KeyEventKind::Press,
                modifiers: BACKWARD_RESOURCE_MODIFIER,
                ..
            }) => {
                // ddg search back results by one
                ddg_search_index = ddg_search_index.saturating_sub(1);
            }

            // move every resource to it's next element in the list, if any more
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::NONE,
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
                code: event::KeyCode::Char('N'),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::SHIFT,
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
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                tracing::info!("Exit app on user command!");
                util::clean(&mut stdout);
                return;
            }
            _ => (),
        }

        util::clear_terminal(&mut stdout);
    }
}
