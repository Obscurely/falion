mod search;
use colored::Colorize;
use crossterm::event;
use crossterm::terminal;
use falion::search::util::Util;
use indexmap::IndexMap;
use search::duckduckgo_search::DuckSearch;
use search::geeksforgeeks::GeeksForGeeks;
use search::github_gist::GithubGist;
use search::stackexchange::StackExchange;
use search::stackoverflow::StackOverFlow;
use std::io;
use std::process;

#[tokio::main]
async fn main() {
    // arguments input setup
    let mut enable_warnings = false;
    let mut print_key_binds = false;
    let mut search_text = vec![String::from("")];
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("An open source, privacy focused tool for getting programming resources fast, efficient and asynchronous from the terminal. By the time you see the results most of the pages are fully loaded. All the searches are done through DuckDuckGO. For more information about the program and key binds, please read the README.md on github: insert link here.");
        ap.refer(&mut enable_warnings).add_option(
            &["-v", "--verbose"],
            argparse::StoreTrue,
            "Print the warnings.",
        );
        ap.refer(&mut print_key_binds).add_option(
            &["-k", "--key-binds"],
            argparse::StoreTrue,
            "Print list of the key binds and exit",
        );
        ap.refer(&mut search_text).required().add_argument("", argparse::ParseList, "Your query, inside quotation marks (\") or not, either way it works as long as there isn't any argument in between when not using quotation marks.");
        ap.parse_args_or_exit();
    }

    // if user chose to print the key binds of the program then exit
    if print_key_binds {
        // first informative line
        println!("{}", "Key Binds list for falion!".green());
        println!("{}", "Note: where '..' is used it means from that to that like '1..5' would mean from 1 to 5.".green());

        // main menu binds
        println!("{}", "\nMain menu:".blue());
        println!("[1..5]         = Access that resource.");
        println!("SHIFT + [1..5] = Go to the next element in the list of that resource.");
        println!("ALT + [1..5]   = Go to the previous element in the list of that resource.");
        println!("CTRL + n       = Move to the next element in the list of every resource.");
        println!(
            "CTRL + b       = Move back to the previous element in the list of every resource."
        );
        println!("CTRL + c       = Clear terminal and exit.");

        // sub menus binds
        println!("{}", "\nSub menus for the resources:".blue());
        println!("CTRL + n       = Move to the next element in the content list (like questions & answers).");
        println!("CTRL + b       = Move back to the previous element in the content list.");
        println!("CTRL + q       = Go back to the main menu.");
        println!("CTRL + c       = Clear terminal and exit.");

        // end of key binds print
        println!(
            "{}",
            "\nThese were all the key binds, enjoy using Falion!".green()
        );
        process::exit(202);
    }

    // parse the input search query, if it's empty inform user and exit
    let search_text = search_text.join(" ");
    if search_text.is_empty() {
        println!("{}", "You haven't provided a search query!".red());
        println!("Run falion -h for more information!");
        process::exit(201);
    }

    // getting terminal width in order to make the pages print prettier
    let term_width = usize::from(match terminal::size() {
        Ok(size) => size.0,
        Err(error) => {
            if enable_warnings {
                eprintln!("{} {}", "[532][Warning] There was a problem detecting the terminal width, using 50 (could use 1million), the terminal should take care of it, it's just not gonna be as nice, the given error is:".yellow(), format!("{}", error).red());
            }
            50
        }
    });

    // getting stdout into var in order to manipulate the terminal
    let mut stdout = io::stdout();

    // making sure we start with raw mode disabled on the terminal
    match terminal::disable_raw_mode() {
        Ok(_) => (),
        Err(error) => {
            if enable_warnings {
                eprintln!("{} {}", "[533][Warning] There was an error disabling terminal raw mode, program may not run as expected! the given error is:".yellow(), format!("{}", error).red());
            }
        }
    }

    // getting futures of the resources we want results from
    let stackoverflow_results =
        StackOverFlow::get_questions_and_content(&search_text, term_width, enable_warnings);
    let stackexchange_results =
        StackExchange::get_questions_and_content(&search_text, term_width, enable_warnings);
    let github_gist_results = GithubGist::get_name_and_content(&search_text, enable_warnings);
    let geeksforgeeks_results =
        GeeksForGeeks::get_name_and_content(&search_text, term_width, enable_warnings);
    let duck_search_results =
        DuckSearch::get_name_and_content(&search_text, term_width, enable_warnings);

    // awaiting them all at the same time
    let results_awaited = futures::join!(
        stackoverflow_results,
        stackexchange_results,
        github_gist_results,
        geeksforgeeks_results,
        duck_search_results
    );

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
    let mut stackoverflow_current =
        match falion::get_key_at_index_map_with_vec(stackoverflow_index, &stackoverflow_results) {
            Some(value) => value,
            None => default_error.to_owned(),
        };
    let mut stackexchange_index = 0;
    let mut stackexchange_current =
        match falion::get_key_at_index_map_with_vec(stackexchange_index, &stackexchange_results) {
            Some(value) => value,
            None => default_error.to_owned(),
        };
    let mut github_gist_index = 0;
    let mut github_gist_current =
        match falion::get_key_at_index_map_with_vec(github_gist_index, &github_gist_results) {
            Some(value) => value,
            None => default_error.to_owned(),
        };
    let mut geeksforgeeks_index = 0;
    let mut geeksforgeeks_current =
        match falion::get_key_at_index_map_with_string(geeksforgeeks_index, &geeksforgeeks_results)
        {
            Some(value) => value,
            None => default_error.to_owned(),
        };
    let mut duck_search_index = 0;
    let mut duck_search_current =
        match falion::get_key_at_index_map_with_string(duck_search_index, &duck_search_results) {
            Some(value) => value,
            None => default_error.to_owned(),
        };
    loop {
        // clearing the output in order to start clean.
        Util::clear_terminal(&mut stdout, enable_warnings);
        // printing search query
        println!(
            "{} {}\n",
            "Your search query is:".green(),
            search_text.blue()
        );
        // printing options
        println!(
            "{} {} {}",
            "(1)".green(),
            "[  StackOverFlow  ]".yellow(),
            &stackoverflow_current.blue()
        );
        println!(
            "{} {} {}",
            "(2)".green(),
            "[  StackExchange  ]".yellow(),
            &stackexchange_current.blue()
        );
        println!(
            "{} {} {}",
            "(3)".green(),
            "[   Github Gist   ]".yellow(),
            &github_gist_current.blue()
        );
        println!(
            "{} {} {}",
            "(4)".green(),
            "[  GeeksForGeeks  ]".yellow(),
            &geeksforgeeks_current.blue()
        );
        println!(
            "{} {} {}",
            "(5)".green(),
            "[DuckDuckGo Search]".yellow(),
            &duck_search_current.blue()
        );

        // input key setup
        Util::enable_terminal_raw_mode(enable_warnings);
        let event_read = match event::read() {
            Ok(ev) => ev,
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[534][Warning] There was an error reading the input event, going to next iteration, if this continues please ctrl+c the program, the given error is:".yellow(), format!("{}", error).red());
                }
                continue;
            }
        };

        // matching the pressed key
        match event_read {
            // enter the menu for first resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('1'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(&mut stdout, enable_warnings);
                let content = match falion::get_key_map_with_vec(
                    &stackoverflow_current,
                    &mut stackoverflow_results,
                    &mut stackoverflow_awaited,
                    enable_warnings,
                )
                .await
                {
                    Some(content) => content,
                    None => {
                        vec![String::from("Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")]
                    }
                };

                falion::loop_prompt_stacks(&content, &mut stdout, enable_warnings);
            }
            // go to next element in the first resource (using ! because of terminal limitations)
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('!'),
                modifiers: event::KeyModifiers::NONE,
            }) => {
                stackoverflow_current = match falion::get_key_at_index_map_with_vec(
                    stackoverflow_index + 1,
                    &stackoverflow_results,
                ) {
                    Some(key) => {
                        stackoverflow_index += 1;
                        key
                    }
                    None => stackoverflow_current,
                };
            }
            // go to the previous element in the first resource (using alt instead of ctrl because of terminal limitations)
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('1'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                stackoverflow_current = match falion::get_key_at_index_map_with_vec(
                    stackoverflow_index - 1,
                    &stackoverflow_results,
                ) {
                    Some(key) => {
                        stackoverflow_index -= 1;
                        key
                    }
                    None => stackoverflow_current,
                };
            }
            // enter the menu of the second resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('2'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(&mut stdout, enable_warnings);
                let content = match falion::get_key_map_with_vec(
                    &stackexchange_current,
                    &mut stackexchange_results,
                    &mut stackexchange_awaited,
                    enable_warnings,
                )
                .await
                {
                    Some(content) => content,
                    None => {
                        vec![String::from("Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")]
                    }
                };

                falion::loop_prompt_stacks(&content, &mut stdout, enable_warnings);
            }
            // go to the next element in the second resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('@'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                stackexchange_current = match falion::get_key_at_index_map_with_vec(
                    stackexchange_index + 1,
                    &stackexchange_results,
                ) {
                    Some(key) => {
                        stackexchange_index += 1;
                        key
                    }
                    None => stackexchange_current,
                };
            }
            // go to previous element in the second resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('2'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                stackexchange_current = match falion::get_key_at_index_map_with_vec(
                    stackexchange_index - 1,
                    &stackexchange_results,
                ) {
                    Some(key) => {
                        stackexchange_index -= 1;
                        key
                    }
                    None => stackexchange_current,
                };
            }
            // enter the menu for the third resource
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('3'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(&mut stdout, enable_warnings);
                let content = match falion::get_key_map_with_vec(
                    &github_gist_current,
                    &mut github_gist_results,
                    &mut github_gist_awaited,
                    enable_warnings,
                )
                .await
                {
                    Some(content) => content,
                    None => {
                        vec![String::from(" Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")]
                    }
                };

                falion::loop_prompt_gist(&content, &mut stdout, enable_warnings);
            }
            // go to the next element in the third resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('#'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                github_gist_current = match falion::get_key_at_index_map_with_vec(
                    github_gist_index + 1,
                    &github_gist_results,
                ) {
                    Some(key) => {
                        github_gist_index += 1;
                        key
                    }
                    None => github_gist_current,
                };
            }
            // go to the previous element in the third resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('3'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                github_gist_current = match falion::get_key_at_index_map_with_vec(
                    github_gist_index - 1,
                    &github_gist_results,
                ) {
                    Some(key) => {
                        github_gist_index -= 1;
                        key
                    }
                    None => github_gist_current,
                };
            }
            // enter the forth resource menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('4'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(&mut stdout, enable_warnings);
                let content = match falion::get_key_map_with_string(&geeksforgeeks_current, &mut geeksforgeeks_results, &mut geeksforgeeks_awaited, enable_warnings).await {
                        Some(content) => content,
                        None => {
                            String::from("Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")
                        }
                    };

                falion::loop_prompt_geeksforgeeks(&content, &mut stdout, enable_warnings);
            }
            // go to the next element in the forth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('$'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                geeksforgeeks_current = match falion::get_key_at_index_map_with_string(
                    geeksforgeeks_index + 1,
                    &geeksforgeeks_results,
                ) {
                    Some(key) => {
                        geeksforgeeks_index += 1;
                        key
                    }
                    None => geeksforgeeks_current,
                };
            }
            // go to the previous element in the forth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('4'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                geeksforgeeks_current = match falion::get_key_at_index_map_with_string(
                    geeksforgeeks_index - 1,
                    &geeksforgeeks_results,
                ) {
                    Some(key) => {
                        geeksforgeeks_index -= 1;
                        key
                    }
                    None => geeksforgeeks_current,
                };
            }
            // enter the fifth resource menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('5'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(&mut stdout, enable_warnings);
                let content = match falion::get_key_map_with_string(&duck_search_current, &mut duck_search_results, &mut duck_search_awaited, enable_warnings).await {
                        Some(content) => content,
                        None => {
                            String::from("Warning! There was an error getting the content for the specified key.\nPress [Ctrl + q] to go back!")
                        }
                    };

                falion::loop_prompt_duckduckgo(&content, &mut stdout, enable_warnings);
            }
            // go to the next element in the fifth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('%'),
                modifiers: event::KeyModifiers::NONE,
                //clearing the screen and printing our message
            }) => {
                duck_search_current = match falion::get_key_at_index_map_with_string(
                    duck_search_index + 1,
                    &duck_search_results,
                ) {
                    Some(key) => {
                        duck_search_index += 1;
                        key
                    }
                    None => duck_search_current,
                };
            }
            // go to the previous element in the fifth resource list
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('5'),
                modifiers: event::KeyModifiers::ALT,
                //clearing the screen and printing our message
            }) => {
                duck_search_current = match falion::get_key_at_index_map_with_string(
                    duck_search_index - 1,
                    &duck_search_results,
                ) {
                    Some(key) => {
                        duck_search_index -= 1;
                        key
                    }
                    None => duck_search_current,
                };
            }
            // move every resource to it's next element in the list, if any more
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                stackoverflow_current = match falion::get_key_at_index_map_with_vec(
                    stackoverflow_index + 1,
                    &stackoverflow_results,
                ) {
                    Some(key) => {
                        stackoverflow_index += 1;
                        key
                    }
                    None => stackoverflow_current,
                };
                stackexchange_current = match falion::get_key_at_index_map_with_vec(
                    stackexchange_index + 1,
                    &stackexchange_results,
                ) {
                    Some(key) => {
                        stackexchange_index += 1;
                        key
                    }
                    None => stackexchange_current,
                };
                github_gist_current = match falion::get_key_at_index_map_with_vec(
                    github_gist_index + 1,
                    &github_gist_results,
                ) {
                    Some(key) => {
                        github_gist_index += 1;
                        key
                    }
                    None => github_gist_current,
                };
                geeksforgeeks_current = match falion::get_key_at_index_map_with_string(
                    geeksforgeeks_index + 1,
                    &geeksforgeeks_results,
                ) {
                    Some(key) => {
                        geeksforgeeks_index += 1;
                        key
                    }
                    None => geeksforgeeks_current,
                };
                duck_search_current = match falion::get_key_at_index_map_with_string(
                    duck_search_index + 1,
                    &duck_search_results,
                ) {
                    Some(key) => {
                        duck_search_index += 1;
                        key
                    }
                    None => duck_search_current,
                };
            }
            // move to the previous element in the list of every resource, if any more
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('b'),
                modifiers: event::KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => {
                stackoverflow_current = match falion::get_key_at_index_map_with_vec(
                    stackoverflow_index - 1,
                    &stackoverflow_results,
                ) {
                    Some(key) => {
                        stackoverflow_index -= 1;
                        key
                    }
                    None => stackoverflow_current,
                };
                stackexchange_current = match falion::get_key_at_index_map_with_vec(
                    stackexchange_index - 1,
                    &stackexchange_results,
                ) {
                    Some(key) => {
                        stackexchange_index -= 1;
                        key
                    }
                    None => stackexchange_current,
                };
                github_gist_current = match falion::get_key_at_index_map_with_vec(
                    github_gist_index - 1,
                    &github_gist_results,
                ) {
                    Some(key) => {
                        github_gist_index -= 1;
                        key
                    }
                    None => github_gist_current,
                };
                geeksforgeeks_current = match falion::get_key_at_index_map_with_string(
                    geeksforgeeks_index - 1,
                    &geeksforgeeks_results,
                ) {
                    Some(key) => {
                        geeksforgeeks_index -= 1;
                        key
                    }
                    None => geeksforgeeks_current,
                };
                duck_search_current = match falion::get_key_at_index_map_with_string(
                    duck_search_index - 1,
                    &duck_search_results,
                ) {
                    Some(key) => {
                        duck_search_index -= 1;
                        key
                    }
                    None => duck_search_current,
                };
            }
            // clear the terminal and exit the program
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            }) => {
                Util::disable_terminal_raw_mode(enable_warnings);
                Util::clear_terminal(&mut stdout, enable_warnings);
                process::exit(0);
            }
            _ => {
                Util::disable_terminal_raw_mode(enable_warnings);
            }
        }

        // clearing the terminal, since everything needed is gonna be printed again.
        Util::clear_terminal(&mut stdout, enable_warnings);
        Util::disable_terminal_raw_mode(enable_warnings);
    }
}
