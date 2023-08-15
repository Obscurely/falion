pub mod search;
use clap::Parser;
use crossterm::terminal;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Search query
    pub query: Vec<String>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn clean_exit(stdout: &mut std::io::Stdout, err: &str) {
    let _ = terminal::disable_raw_mode();
    let _ = crossterm::execute!(stdout, crossterm::style::ResetColor);
    let _ = crossterm::execute!(stdout, terminal::Clear(terminal::ClearType::All));
    let _ = crossterm::execute!(stdout, terminal::ScrollUp(u16::MAX));
    let _ = crossterm::execute!(stdout, crossterm::cursor::MoveTo(0, 0));

    panic!("Error: {err}");
}
