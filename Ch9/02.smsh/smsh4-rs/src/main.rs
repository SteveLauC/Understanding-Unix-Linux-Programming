mod builtin;
mod execute;
mod ifclause;
mod process;
mod splitline;
mod varlib;

use process::process;
use splitline::{next_cmd, splitline};
use varlib::VarTable;

use nix::sys::signal::{signal, SigHandler, SIGINT, SIGQUIT};
use std::io::stdin;

/// purpose: ignore SIGINT and SIGQUIT signal in the parent process
///
/// action: disable SIGINT and SIGQUIT singal handling using signal(2) syscall
fn set_up() {
    unsafe {
        let _ = signal(SIGINT, SigHandler::SigIgn);
        let _ = signal(SIGQUIT, SigHandler::SigIgn);
    }
}

fn main() {
    set_up();
    let mut vt: VarTable = VarTable::new();
    vt.environ2table();

    while let Some(cmd_line) = next_cmd("> ", stdin()) {
        process(splitline(&cmd_line), &mut vt);
    }
}
