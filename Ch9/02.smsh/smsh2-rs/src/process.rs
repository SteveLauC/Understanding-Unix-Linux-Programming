use crate::controlflow::{do_control_command, is_control_command, ok_to_execute};
use crate::execute::execute;
use std::process::Command;

pub enum ProcessRes {
    Success,
    Failure,
}

pub fn process(args: Vec<String>) -> ProcessRes {
    if args.is_empty() {
        ProcessRes::Failure
    } else if is_control_command(args[0].as_str()) {
        do_control_command(args)
    } else if ok_to_execute() {
        let mut cmd = Command::new(args[0].as_str());
        cmd.args(&args[1..]);
        execute(cmd)
    } else {
        ProcessRes::Failure
    }
}
