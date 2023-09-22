use crossterm::event;
use crossterm::style;
use crossterm::style::Stylize;
use std::io::Write;
use tokio::task::JoinHandle;

type ResultsType<T, S> = Vec<(String, JoinHandle<Result<T, S>>)>;

/// Print the given print followed by the title of the given index result.
///
/// # Arguments
///
/// `stdout` - std::io::stdout() you should have one in main you reference across functions. It's
/// used to manipulate the terminal.
/// `resource_index` - The index of the given resource to print.
/// `resource_results` - Actual results of the resource you want to print.
#[tracing::instrument(skip_all)]
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
            // get the current result
            let current_result = match results.get(resource_index) {
                Some(res) => res,
                None => {
                    // this should never happen
                    super::util::clean(stdout);
                    tracing::error!("User tried to get content from a resource with a index that is lower than the lenth - 1 of the resource, but for some reason it still failed!");
                    panic!("This should never have happened. Please create a new issue on github and post latest.log file.")
                }
            };
            // display the current result with the given print
            if let Err(error) = crossterm::queue!(
                stdout,
                style::PrintStyledContent(
                    (resource_print.to_string() + &current_result.0).stylize()
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

/// Create a screen similar to the cli one where you go through a content that is iterable.
///
/// # Arguments
///
/// `stdout` - std::io::stdout() you should have one in main you reference across functions. It's
/// used to manipulate the terminal.
/// `content` - the iterable content to display
/// `is_thread` - Specify if the content is thread type, so the first element is gonna be tagged as
/// question and the rest as answers, if not each element is gonna be tagged a file.
#[tracing::instrument(skip_all)]
pub fn print_dyn_content(
    stdout: &mut std::io::Stdout,
    content: &[String],
    is_thread: bool,
) -> bool {
    let mut current_index = 0;
    let max_index = content.len() - 1;
    // depending on is_thread set to either question or file 1 as for the first element.
    let question_title = if is_thread {
        "Question:".green().bold()
    } else {
        "File 1:".green().bold()
    };
    // cli for the given content
    loop {
        // print content
        let content = match content.get(current_index) {
            // replace \n to \n\r because in terminal raw mode a new line doesn't bring you the
            // beginning of the row, it only goes down one line literally.
            Some(content) => content.replace('\n', "\n\r"),
            None => "There has been error getting the contents for this result".to_string(),
        };
        // print first element tag or not
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

        // flush stdout queued commands
        if let Err(error) = stdout.flush() {
            tracing::warn!(
                "There was an error flushing stdout in order to print thread's content. Error: {}",
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

        match event_read {
            // go to next content
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('n'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                if current_index < max_index {
                    current_index += 1;
                }
            }
            // go to previous content
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('N'),
                modifiers: event::KeyModifiers::SHIFT,
                ..
            }) => {
                current_index = current_index.saturating_sub(1);
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }) => {
                return false;
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                tracing::info!("Exit app on user command!");
                return true;
            }
            _ => (),
        }

        // clear terminal
        super::util::clear_terminal(stdout);
    }
}

/// Create a screen similar to the cli one to print static content. Content that is only one page.
///
/// # Arguments
///
/// `stdout` - std::io::stdout() you should have one in main you reference across functions. It's
/// used to manipulate the terminal.
/// `content` - the content to create the cli for.
#[tracing::instrument(skip_all)]
pub fn print_static_content(stdout: &mut std::io::Stdout, content: &str) -> bool {
    // replace \n to \n\r because in terminal raw mode a new line doesn't bring you the
    // beginning of the row, it only goes down one line literally.
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

        // listen to key presses
        match event_read {
            // return to main menu
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::NONE,
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
                tracing::info!("Exit app on user command!");
                return true;
            }
            _ => (),
        }

        // clear terminal
        super::util::clear_terminal(stdout);
    }
}
