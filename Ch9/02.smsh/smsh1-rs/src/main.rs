mod execute;
mod splitline;

use splitline::{next_cmd, splitline};
use execute::execute;

use nix::sys::signal::{signal, SigHandler, SIGINT, SIGQUIT};
use std::io::stdin;

fn set_up() {
    unsafe{
        let _ = signal(SIGINT, SigHandler::SigIgn);
        let _ = signal(SIGQUIT, SigHandler::SigIgn);
    }
}
fn main() {
    set_up();

    while let Some(cmd_line) = next_cmd("> ", stdin()) {
        if let Some(cmd) = splitline(cmd_line) {
            execute(cmd);
        }
    }
}
