mod controlflow;
mod execute;
mod process;
mod splitline;

use process::process;
use splitline::{next_cmd, splitline};

use nix::sys::signal::{signal, SigHandler, SIGINT, SIGQUIT};
use std::io::stdin;

/// purpose: ignore SIGINT and SIGQUIT signal in the parent process
fn set_up() {
    unsafe {
        let _ = signal(SIGINT, SigHandler::SigIgn);
        let _ = signal(SIGQUIT, SigHandler::SigIgn);
    }
}

fn main() {
    set_up();

    while let Some(cmd_line) = next_cmd("> ", stdin()) {
        process(splitline(cmd_line));
    }
}
