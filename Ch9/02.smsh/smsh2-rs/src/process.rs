//! process.rs: commmand processing, front-end for all kinds of commands

use crate::execute::execute;
use crate::ifclause::controlflow::{do_control_command, is_control_command, ok_to_execute};
use std::process::Command;

/// type to represent the processing result
pub enum ProcessRes {
    Success,
    Failure,
}

/// purpose: process the command
///
/// action:
///     If the command words list is empty, just return ProcessRes::Failure
///     If the command is a control command, pass it to `crate::ifclause::controlflow::do_control_command`
///     If the command is a normal command, execute it
///
/// arguments:
///     * `args`: a command words list constructed by `crate::splitline::splitline`
///     
/// return: ProcessRes
pub fn process(args: Vec<&str>) -> ProcessRes {
    if args.is_empty() {
        ProcessRes::Failure
    } else if is_control_command(args[0]) {
        do_control_command(args)
    } else if ok_to_execute() {
        let mut cmd = Command::new(args[0]);
        cmd.args(&args[1..]);
        execute(cmd)
    } else {
        ProcessRes::Failure
    }
}
