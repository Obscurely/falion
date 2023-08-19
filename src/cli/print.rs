use crossterm::event;
use crossterm::style;
use crossterm::style::Stylize;
use indexmap::IndexMap;
use std::io::Write;
use tokio::task::JoinHandle;

type ResultsType<T, S> = IndexMap<String, JoinHandle<Result<T, S>>>;

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
                    super::util::clean(stdout);
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
        super::util::clear_terminal(stdout);
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
        super::util::clear_terminal(stdout);
    }
}
