mod search;
use colored::Colorize;
use crossterm::terminal;
use crossterm::event;
use falion::search::util::Util;
use std::borrow::Borrow;
use std::io::Read;
use std::process;
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
            eprintln!("{} {}", "[532][Warning] There was a problem detecting the terminal width, using 50 (could use 1milion), the terminal should take care of it, it's just not gonna be as nice, the given error is:".yellow(), format!("{}", error).red());
            50
        }
    });
    // getting stdout into var in order to manipulate the terminal
    let mut stdout = io::stdout();

    // making sure we start with raw mode disabled on the terminal
    match terminal::disable_raw_mode() {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{} {}", "[533][Warning] There was an error disabling terminal raw mode, program may not run as expected! the given error is:".yellow(), format!("{}", error).red());
        }
    }

    // getting args list and making a string from it
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("{}{}", "[113][Error] ".red(), "You have to provide a search querry, either surronded by \" or the querry as it is after the program's name.".to_string().red());
        std::process::exit(113);
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

    let mut stackoverflow_index = 0;
    let mut stackoverflow_current = match falion::get_key_at_index_map_with_vec(stackoverflow_index, &stackoverflow_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let mut stackexchange_index = 0;
    let mut stackexchange_current = match falion::get_key_at_index_map_with_vec(stackexchange_index, &stackexchange_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let mut github_gist_index = 0;
    let mut github_gist_current = match falion::get_key_at_index_map_with_vec(github_gist_index, &github_gist_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let mut geeksforgeeks_index = 0;
    let mut geeksforgeeks_current = match falion::get_key_at_index_map_with_string(geeksforgeeks_index, &geeksforgeeks_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    let mut duck_search_index = 0;
    let mut duck_search_current = match falion::get_key_at_index_map_with_string(duck_search_index, &duck_search_results) {
        Some(value) => value,
        None => default_error.to_owned(),
    };
    loop {
        // clearing the output in order to start clean.
        Util::clear_terminal(&mut stdout);
        // printing search querry
        println!("Your search querry is: {}", search_text.green());
        // printing options
        println!("(1) [  StackOverFlow  ] {}", &stackoverflow_current);
        println!("(2) [  StackExchange  ] {}", &stackexchange_current);
        println!("(3) [   Github Gist   ] {}", &github_gist_current);
        println!("(4) [  GeeksForGeeks  ] {}", &geeksforgeeks_current);
        println!("(5) [DuckDuckGo Search] {}", &duck_search_current);

        // input key setup
        Util::enable_terminal_raw_mode();
        let event_read = match event::read() {
            Ok(ev) => ev,
            Err(error) => {
                eprintln!("{} {}", "[534][Warning] There was an error reading the input event, going to next iteration, if this continues please ctrl+c the program, the given error is:".yellow(), format!("{}", error).red());
                continue;
            }
        };

        // matching the pressed key
        match event_read {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('1'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    Util::disable_terminal_raw_mode();
                    Util::clear_terminal(&mut stdout);
                    let content = match falion::get_key_map_with_vec(&stackoverflow_current, &mut stackoverflow_results, &mut stackoverflow_awaited).await {
                        Some(content) => content,
                        None => {
                            vec![String::from("[596] Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")]
                        }
                    };

                    falion::loop_prompt_stacks(&content, &mut stdout);
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('!'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    stackoverflow_current = match falion::get_key_at_index_map_with_vec(stackoverflow_index + 1, &stackoverflow_results) {
                        Some(key) => {
                            stackoverflow_index += 1;
                            key
                        },
                        None => stackoverflow_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('1'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                    stackoverflow_current = match falion::get_key_at_index_map_with_vec(stackoverflow_index - 1, &stackoverflow_results) {
                        Some(key) => {
                            stackoverflow_index -= 1;
                            key
                        },
                        None => stackoverflow_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('2'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    Util::disable_terminal_raw_mode();
                    Util::clear_terminal(&mut stdout);
                    let content = match falion::get_key_map_with_vec(&stackexchange_current, &mut stackexchange_results, &mut stackexchange_awaited).await {
                        Some(content) => content,
                        None => {
                            vec![String::from("[596] Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")]
                        }
                    };

                    falion::loop_prompt_stacks(&content, &mut stdout);
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('@'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    stackexchange_current = match falion::get_key_at_index_map_with_vec(stackexchange_index + 1, &stackexchange_results) {
                        Some(key) => {
                            stackexchange_index += 1;
                            key
                        },
                        None => stackexchange_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('2'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                    stackexchange_current = match falion::get_key_at_index_map_with_vec(stackexchange_index - 1, &stackexchange_results) {
                        Some(key) => {
                            stackexchange_index -= 1;
                            key
                        },
                        None => stackexchange_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('3'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    Util::disable_terminal_raw_mode();
                    Util::clear_terminal(&mut stdout);
                    let content = match falion::get_key_map_with_vec(&github_gist_current, &mut github_gist_results, &mut github_gist_awaited).await {
                        Some(content) => content,
                        None => {
                            vec![String::from("[596] Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")]
                        }
                    };

                    falion::loop_prompt_gist(&content, &mut stdout);
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('#'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    github_gist_current = match falion::get_key_at_index_map_with_vec(github_gist_index + 1, &github_gist_results) {
                        Some(key) => {
                            github_gist_index += 1;
                            key
                        },
                        None => github_gist_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('3'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                    github_gist_current = match falion::get_key_at_index_map_with_vec(github_gist_index - 1, &github_gist_results) {
                        Some(key) => {
                            github_gist_index -= 1;
                            key
                        },
                        None => github_gist_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('4'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    Util::disable_terminal_raw_mode();
                    Util::clear_terminal(&mut stdout);
                    let content = match falion::get_key_map_with_string(&geeksforgeeks_current, &mut geeksforgeeks_results, &mut geeksforgeeks_awaited).await {
                        Some(content) => content,
                        None => {
                            String::from("[596] Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")
                        }
                    };

                    falion::loop_prompt_geeksforgeeks(&content, &mut stdout);
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('$'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    geeksforgeeks_current = match falion::get_key_at_index_map_with_string(geeksforgeeks_index + 1, &geeksforgeeks_results) {
                        Some(key) => {
                            geeksforgeeks_index += 1;
                            key
                        },
                        None => geeksforgeeks_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('4'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                    geeksforgeeks_current = match falion::get_key_at_index_map_with_string(geeksforgeeks_index - 1, &geeksforgeeks_results) {
                        Some(key) => {
                            geeksforgeeks_index -= 1;
                            key
                        },
                        None => geeksforgeeks_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('5'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    Util::disable_terminal_raw_mode();
                    Util::clear_terminal(&mut stdout);
                    let content = match falion::get_key_map_with_string(&duck_search_current, &mut duck_search_results, &mut duck_search_awaited).await {
                        Some(content) => content,
                        None => {
                            String::from("[596] Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")
                        }
                    };

                    falion::loop_prompt_duckduckgo(&content, &mut stdout);
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('%'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                    duck_search_current = match falion::get_key_at_index_map_with_string(duck_search_index + 1, &duck_search_results) {
                        Some(key) => {
                            duck_search_index += 1;
                            key
                        },
                        None => duck_search_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('5'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                    duck_search_current = match falion::get_key_at_index_map_with_string(duck_search_index - 1, &duck_search_results) {
                        Some(key) => {
                            duck_search_index -= 1;
                            key
                        },
                        None => duck_search_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                    stackoverflow_current = match falion::get_key_at_index_map_with_vec(stackoverflow_index + 1, &stackoverflow_results) {
                        Some(key) => {
                            stackoverflow_index += 1;
                            key
                        },
                        None => stackoverflow_current,
                    };
                    stackexchange_current = match falion::get_key_at_index_map_with_vec(stackexchange_index + 1, &stackexchange_results) {
                        Some(key) => {
                            stackexchange_index += 1;
                            key
                        },
                        None => stackexchange_current,
                    };
                    github_gist_current = match falion::get_key_at_index_map_with_vec(github_gist_index + 1, &github_gist_results) {
                        Some(key) => {
                            github_gist_index += 1;
                            key
                        },
                        None => github_gist_current,
                    };
                    geeksforgeeks_current = match falion::get_key_at_index_map_with_string(geeksforgeeks_index + 1, &geeksforgeeks_results) {
                        Some(key) => {
                            geeksforgeeks_index += 1;
                            key
                        },
                        None => geeksforgeeks_current,
                    };
                    duck_search_current = match falion::get_key_at_index_map_with_string(duck_search_index + 1, &duck_search_results) {
                        Some(key) => {
                            duck_search_index += 1;
                            key
                        },
                        None => duck_search_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('b'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                    stackoverflow_current = match falion::get_key_at_index_map_with_vec(stackoverflow_index - 1, &stackoverflow_results) {
                        Some(key) => {
                            stackoverflow_index -= 1;
                            key
                        },
                        None => stackoverflow_current,
                    };
                    stackexchange_current = match falion::get_key_at_index_map_with_vec(stackexchange_index - 1, &stackexchange_results) {
                        Some(key) => {
                            stackexchange_index -= 1;
                            key
                        },
                        None => stackexchange_current,
                    };
                    github_gist_current = match falion::get_key_at_index_map_with_vec(github_gist_index - 1, &github_gist_results) {
                        Some(key) => {
                            github_gist_index -= 1;
                            key
                        },
                        None => github_gist_current,
                    };
                    geeksforgeeks_current = match falion::get_key_at_index_map_with_string(geeksforgeeks_index - 1, &geeksforgeeks_results) {
                        Some(key) => {
                            geeksforgeeks_index -= 1;
                            key
                        },
                        None => geeksforgeeks_current,
                    };
                    duck_search_current = match falion::get_key_at_index_map_with_string(duck_search_index - 1, &duck_search_results) {
                        Some(key) => {
                            duck_search_index -= 1;
                            key
                        },
                        None => duck_search_current,
                    };
                }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                    Util::disable_terminal_raw_mode();
                    Util::clear_terminal(&mut stdout);
                    process::exit(0);
                }
            _ => {
                Util::disable_terminal_raw_mode();
            },
        }

        // clearing the terminal, since everything needed is gonna be printed again.
        Util::clear_terminal(&mut stdout);
        Util::disable_terminal_raw_mode();
        // to not exit
        // let mut test = String::from("");
        // io::stdin().read_line(&mut test);
   }

    // let test = falion::get_index_hash_with_string(0, &mut geeksforgeeks_results, &mut geeksforgeeks_awaited).await.unwrap(); 
    // println!("At key: {}, there is content: \n\n {}", test.0, test.1);

    // let test = falion::get_index_hash_with_string(0, &mut geeksforgeeks_results, &mut geeksforgeeks_awaited).await.unwrap(); 
    // println!("At key: {}, there is content: \n\n {}", test.0, test.1);

    // let mut test = String::from("");
    // io::stdin().read_line(&mut test);
}
