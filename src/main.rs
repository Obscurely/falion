use std::io::{stdout, IsTerminal};
mod cli;
mod search;
mod ui;
mod util;

#[cfg(windows)]
use winapi::um::processthreadsapi::GetStartupInfoW;

#[cfg(windows)]
fn is_launched_from_terminal() -> bool {
    unsafe {
        let mut si = std::mem::zeroed();
        GetStartupInfoW(&mut si);
        
        // Check if the STARTF_USESHOWWINDOW flag is set
        if si.dwFlags & winapi::um::winbase::STARTF_USESHOWWINDOW != 0 {
            // If it's set, it may be launched from a terminal
            return si.wShowWindow == winapi::um::winuser::SW_SHOW as u16;
        }
    }
    
    // If the flag is not set, it's likely not launched from a terminal
    false
}

/// Main Falion execution
#[tokio::main]
async fn main() {
    if cfg!(windows) {
        #[cfg(windows)]
        if is_launched_from_terminal() {
            cli::cli().await;
        } else {
            ui::ui();
        }
        
    } else {
        if stdout().is_terminal() {
            cli::cli().await;
        } else {
            ui::ui();
        }
    }
}
