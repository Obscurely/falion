pub mod search;
use clap::Parser;
use crossterm::event;
use crossterm::style;
use crossterm::style::Stylize;
use crossterm::terminal;
use indexmap::IndexMap;
use std::fs;
use std::io::Write;
use std::{fs::File, sync::Arc};
use thiserror::Error;
use tokio::task::{JoinError, JoinHandle};
use tracing_subscriber::{filter, prelude::*};

type ResultsType<T, S> = IndexMap<String, JoinHandle<Result<T, S>>>;
type ResultsStaticType<E> = IndexMap<String, JoinHandle<Result<String, E>>>;
type ResultsDynType<E> = IndexMap<String, JoinHandle<Result<Vec<String>, E>>>;

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

#[derive(Error, Debug)]
pub enum GetResultsError {
    #[error("There was an error awaiting the result for this source. Error: {0}")]
    JoinError(JoinError),
    #[error(
        "There was an error getting the result for the given index. Error: Index out of Bounds"
    )]
    OutOfBounds,
}

#[tracing::instrument(skip_all)]
pub fn clean(stdout: &mut std::io::Stdout) {
    if let Err(error) = crossterm::terminal::disable_raw_mode() {
        tracing::warn!("Failed to disable termial raw mode! Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::style::ResetColor) {
        tracing::warn!("Failed to reset term collor. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::cursor::Show) {
        tracing::warn!("Failed to show back cursor. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::All)) {
        tracing::warn!("Failed to clear terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, terminal::ScrollUp(u16::MAX)) {
        tracing::warn!("Failed to scroll up the terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::cursor::MoveTo(0, 0)) {
        tracing::warn!("Failed to move terminal cursor. Error: {}", error);
    }

    if let Err(error) = stdout.flush() {
        tracing::warn!("Failed to flush stdout. Error: {}", error);
    };
}

#[tracing::instrument(skip_all)]
pub fn clear_terminal(stdout: &mut std::io::Stdout) {
    if let Err(error) = crossterm::queue!(stdout, terminal::Clear(terminal::ClearType::All)) {
        tracing::warn!("Failed to clear terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, terminal::ScrollUp(u16::MAX)) {
        tracing::warn!("Failed to scroll up the terminal. Error: {}", error);
    }
    if let Err(error) = crossterm::queue!(stdout, crossterm::cursor::MoveTo(0, 0)) {
        tracing::warn!("Failed to move terminal cursor. Error: {}", error);
    }

    if let Err(error) = stdout.flush() {
        tracing::warn!("Failed to flush stdout. Error: {}", error);
    }
}

pub fn setup_logs(stdout: &mut std::io::Stdout, verbose: bool) {
    // get/create cache dir
    let cache_dir = match dirs::cache_dir() {
        Some(mut path) => {
            path.push("falion");
            if let Err(error) = fs::create_dir_all(&path) {
                clean(stdout);
                panic!("Failed to create cache dir. Error: {}", error);
            }
            path
        }
        None => {
            clean(stdout);
            panic!("Failed to get cache dir!");
        }
    };
    // move the contents from latest.log (if any) to another file
    let latest_log = cache_dir.join("latest.log");
    let older_log =
        cache_dir.join(chrono::Utc::now().format("%d-%m-%YT%H.%M.%S").to_string() + ".log");
    if let Err(error) = fs::copy(latest_log, older_log) {
        if error.kind() != std::io::ErrorKind::NotFound {
            clean(stdout);
            panic!(
                "Failed to copy latest log to another file. Error: {}",
                error
            );
        }
    }

    // setup tracing subscriber
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    // let latest_log = cache_dir.
    let file = match File::create(cache_dir.join("latest.log")) {
        Ok(file) => file,
        Err(error) => {
            clean(stdout);
            panic!("Failed to create a latest.log file. Error: {:#?}", error)
        }
    };
    let mut debug_log = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(file))
        .with_line_number(true)
        .with_file(true)
        .with_thread_ids(true)
        .with_target(false);
    debug_log.set_ansi(false);

    // register the layers
    if !verbose {
        let filter_debug_log = filter::Targets::new()
            .with_target("falion", filter::LevelFilter::DEBUG)
            .with_target("hyper", filter::LevelFilter::WARN);
        tracing_subscriber::registry()
            .with(
                stdout_log
                    // Add an `INFO` filter to the stdout logging layer
                    .with_filter(filter::LevelFilter::WARN)
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log.with_filter(filter_debug_log)), // Add a filter to *both* layers that rejects spans and
                                                                        // events whose targets start with `metrics`.
            )
            .init();
    } else {
        let filter_debug_log = filter::Targets::new()
            .with_target("falion", filter::LevelFilter::DEBUG)
            .with_target("hyper", filter::LevelFilter::DEBUG);
        let filter_stdout_log = filter::Targets::new()
            .with_target("falion", filter::LevelFilter::DEBUG)
            .with_target("hyper", filter::LevelFilter::WARN);
        tracing_subscriber::registry()
            .with(
                stdout_log
                    // Add an `INFO` filter to the stdout logging layer
                    .with_filter(filter_stdout_log)
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log.with_filter(filter_debug_log)), // Add a filter to *both* layers that rejects spans and
                                                                        // events whose targets start with `metrics`.
            )
            .init();
    }
}

pub fn print_resource<T, S>(
    stdout: &mut std::io::Stdout,
    resource_index: usize,
    resource_print: &str,
    resource_results: &Result<ResultsType<T, S>, S>,
) where
    S: std::string::ToString,
{
    match resource_results {
        Ok(results) => {
            let current_result = match results.get_index(resource_index) {
                Some(res) => res,
                None => {
                    // this should never happen
                    clean(stdout);
                    panic!("This should never have happened. Please create a new issue on github and post latest.log file.")
                }
            };
            if let Err(error) = crossterm::queue!(
                stdout,
                style::PrintStyledContent(
                    (resource_print.to_string() + current_result.0).stylize()
                ),
                style::Print("\n\r")
            ) {
                tracing::warn!("There was an error printing some text. Error: {}", error);
            }
        }
        Err(error) => {
            if let Err(error) = crossterm::queue!(
                stdout,
                style::PrintStyledContent(error.to_string().red()),
                style::Print("\n\r")
            ) {
                tracing::warn!("There was an error printing some text. Error: {}", error);
            }
        }
    }
}

pub fn print_dyn_content(
    stdout: &mut std::io::Stdout,
    content: &[String],
    is_thread: bool,
) -> bool {
    let mut current_index = 0;
    let max_index = content.len() - 1;
    let question_title = if is_thread {
        "Question:".green().bold()
    } else {
        "File 1:".green().bold()
    };
    loop {
        // print content
        let content = match content.get(current_index) {
            Some(content) => content.replace('\n', "\n\r"),
            None => "There has been error getting the contents for this result".to_string(),
        };
        if current_index == 0 {
            if let Err(error) = crossterm::queue!(
                stdout,
                style::PrintStyledContent(question_title),
                style::Print("\n\r\n\r")
            ) {
                tracing::warn!(
                    "There was an error printing the title of thread's current entry. Error: {}",
                    error
                );
            }

            if let Err(error) = crossterm::queue!(stdout, style::Print(content)) {
                tracing::warn!(
                    "There was an error printing a thread's content. Error: {}",
                    error
                );
            }
        } else {
            if let Err(error) = crossterm::queue!(
                stdout,
                style::PrintStyledContent(format!("Answer {}:", current_index).green()),
                style::Print("\n\r\n\r")
            ) {
                tracing::warn!(
                    "There was an error printing the title of thread's current entry. Error: {}",
                    error
                );
            }

            if let Err(error) = crossterm::queue!(stdout, style::Print(content)) {
                tracing::warn!(
                    "There was an error printing a thread's content. Error: {}",
                    error
                );
            }
        }

        if let Err(error) = stdout.flush() {
            tracing::warn!(
                "There was an error flushing stdout in order to print thread's content. Error: {}",
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

        match event_read {
            // go to next content
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                if current_index < max_index {
                    current_index += 1;
                }
            }
            // go to previous content
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('b'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                current_index = current_index.saturating_sub(1);
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                return false;
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                return true;
            }
            _ => (),
        }

        // clear terminal
        clear_terminal(stdout);
    }
}

pub fn print_static_content(stdout: &mut std::io::Stdout, content: &str) -> bool {
    let content = content.replace('\n', "\n\r");
    let page_title = "Page:".green().bold();
    loop {
        // print content
        if let Err(error) = crossterm::queue!(
            stdout,
            style::PrintStyledContent(page_title),
            style::Print("\n\r\n\r")
        ) {
            tracing::warn!(
                "There was an error printing the title of thread's current entry. Error: {}",
                error
            );
        }

        if let Err(error) = crossterm::queue!(stdout, style::Print(&content)) {
            tracing::warn!(
                "There was an error printing a thread's content. Error: {}",
                error
            );
        }

        if let Err(error) = stdout.flush() {
            tracing::warn!(
                "There was an error flushing stdout in order to print thread's content. Error: {}",
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

        match event_read {
            // return to main menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                return false;
            }
            // quit app
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                return true;
            }
            _ => (),
        }

        // clear terminal
        clear_terminal(stdout);
    }
}

pub async fn get_dyn_result_content<'a, E>(
    results_ref: &'a mut Result<ResultsDynType<E>, E>,
    results_awaited_ref: &'a mut IndexMap<String, Vec<String>>,
    results_index: usize,
) -> Option<&'a Vec<String>>
where
    E: std::fmt::Display,
{
    match results_ref {
        Ok(res) => {
            if let Some(unawaited_res) = res.get_index_mut(results_index) {
                if results_awaited_ref.contains_key(unawaited_res.0) {
                    match results_awaited_ref.get(unawaited_res.0) {
                        Some(res) => Some(res),
                        None => None,
                    }
                } else {
                    let awaited = match unawaited_res.1.await {
                        Ok(handled) => match handled {
                            Ok(content) => content,
                            Err(error) => {
                                vec![format!("There has been an error handeling the future for this result. Error: {}", error)]
                            }
                        },
                        Err(error) => {
                            vec![format!("There has been an error handeling the future for this result. Error: {}", error)]
                        }
                    };

                    // save already awaited
                    results_awaited_ref.insert(unawaited_res.0.to_owned(), awaited);

                    // unwrap is safe since we just inserted this element
                    results_awaited_ref.get(unawaited_res.0)
                }
            } else {
                None
            }
        }
        Err(_) => {
            tracing::info!("User tryed accessing a resource that has been deemed unavailable.");
            None
        }
    }
}

pub async fn get_static_result_content<'a, E>(
    results_ref: &'a mut Result<ResultsStaticType<E>, E>,
    results_awaited_ref: &'a mut IndexMap<String, String>,
    results_index: usize,
) -> Option<&'a String>
where
    E: std::fmt::Display,
{
    match results_ref {
        Ok(res) => {
            if let Some(unawaited_res) = res.get_index_mut(results_index) {
                if results_awaited_ref.contains_key(unawaited_res.0) {
                    match results_awaited_ref.get(unawaited_res.0) {
                        Some(res) => Some(res),
                        None => None,
                    }
                } else {
                    let awaited = match unawaited_res.1.await {
                        Ok(handled) => match handled {
                            Ok(content) => content,
                            Err(error) => {
                                format!("There has been an error handeling the future for this result. Error: {}", error)
                            }
                        },
                        Err(error) => {
                            format!("There has been an error handeling the future for this result. Error: {}", error)
                        }
                    };

                    // save already awaited
                    results_awaited_ref.insert(unawaited_res.0.to_owned(), awaited);

                    // unwrap is safe since we just inserted this element
                    results_awaited_ref.get(unawaited_res.0)
                }
            } else {
                None
            }
        }
        Err(_) => {
            tracing::info!("User tryed accessing a resource that has been deemed unavailable.");
            None
        }
    }
}

// pub async fn get_result<'a, T, S>(
//     index: usize,
//     results: &'a mut IntoIter<IndexMap<String, JoinHandle<Result<T, S>>>>,
//     results_awaited: &'a mut IndexMap<String, Result<T, S>>,
// ) -> Result<(&'a String, &'a Result<T, S>), GetResultsError> {
//     match results_awaited.get_index(index) {
//         Some(res) => return Ok(res),
//         None => {
//             match results.get_index_mut(index) {
//                 Some(res) => {
//                     match res.1.await {
//                         Ok(res_awaited) => return Ok((res.0, res_awaited.into())),
//                         Err(error) => return Err(GetResultsError::JoinError(error)),
//                     }
//                 },
//                 None => return Err(GetResultsError::OutOfBounds),
//             }
//         }
//     }
// }
