use crate::search::duckduckgo::DuckDuckGo;
use colored::Colorize;
use crossterm::terminal;
use html2text;
use std::collections::HashMap;
use urlencoding;

pub struct Util {}

impl Util {
    pub fn get_url_compatible_string(text: String) -> String {
        let comp_string = urlencoding::encode(&text).to_string();

        comp_string
    }

    pub fn beautify_text_in_html(text: &str, term_width: usize) -> String {
        html2text::from_read(text.as_bytes(), term_width)
    }

    pub fn clear_terminal(out: &mut std::io::Stdout) {
        match crossterm::execute!(out, terminal::Clear(terminal::ClearType::All)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[514] Warning, there was an error clearing the terminal, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
        match crossterm::execute!(out, terminal::ScrollUp(u16::MAX)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[515] Warning, there was an error scrolling up the terminal, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
        match crossterm::execute!(out, crossterm::cursor::MoveTo(0, 0)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[516] Warning, there was an error moving the cursor of the terminal, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }

    pub fn move_cursor_beginning(out: &mut std::io::Stdout) {
        match crossterm::execute!(out, crossterm::cursor::MoveTo(0, 0)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[517] Warning, there was an error moving the cursor of the terminal, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }

    pub fn move_cursor_down(out: &mut std::io::Stdout) {
        match crossterm::execute!(out, terminal::ScrollDown(1)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[519] Warning. there was an error moving the cursor down, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }

    pub fn move_cursor_up(out: &mut std::io::Stdout) {
        match crossterm::execute!(out, terminal::ScrollUp(1)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[520] Warning. there was an error moving the cursor up, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }

    pub fn move_cursor_down_5(out: &mut std::io::Stdout) {
        match crossterm::execute!(out, terminal::ScrollDown(5)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[521] Warning. there was an error moving the cursor down 5 lines, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }

    pub fn move_cursor_up_5(out: &mut std::io::Stdout) {
        match crossterm::execute!(out, terminal::ScrollUp(5)) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[522] Warning. there was an error moving the cursor up 5 lines, program may not work as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }

    pub fn disable_terminal_raw_mode() {
        match terminal::disable_raw_mode() {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[510] Warning! There was an error disabling terminal raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }

    pub fn enable_terminal_raw_mode() {
        match terminal::enable_raw_mode() {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[505] Warning, there was an error enabling raw mode, program may not run as expected! the given error is: {}", format!("{}", error).red());
            }
        }
    }
}
