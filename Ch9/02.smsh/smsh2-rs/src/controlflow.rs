use super::process::process;
use super::process::ProcessRes;
use std::process::exit;

#[derive(PartialEq)]
enum States {
    Nentral,
    WantThen,
    ThenBlock,
}

#[derive(PartialEq)]
enum Result {
    Success,
    Failure,
}

static mut IF_STATE: States = States::Nentral;
static mut IF_RESULT: Result = Result::Success;

pub fn is_control_command(first_cmd: &str) -> bool {
    first_cmd.starts_with("if") || first_cmd.starts_with("then") || first_cmd.starts_with("fi")
}

pub fn do_control_command(args: Vec<String>) -> ProcessRes {
    let rv: ProcessRes;
    match args[0].as_str() {
        "if" => unsafe {
            if IF_STATE != States::Nentral {
                rv = syn_err("if unexpected");
            } else {
                match process(args[1..].to_vec()) {
                    ProcessRes::Success => {
                        IF_RESULT = Result::Success;
                    }
                    ProcessRes::Failure => {
                        IF_RESULT = Result::Failure;
                    }
                }
                IF_STATE = States::WantThen;
                rv = ProcessRes::Success;
            }
        },
        "then" => unsafe {
            if IF_STATE != States::WantThen {
                rv = syn_err("then unexpected");
            } else {
                IF_STATE = States::ThenBlock;
                rv = ProcessRes::Success;
            }
        },
        "fi" => unsafe {
            if IF_STATE != States::ThenBlock {
                rv = syn_err("fi unexpected");
            } else {
                IF_STATE = States::Nentral;
                rv = ProcessRes::Success;
            }
        },
        _ => {
            eprintln!("inernal error processing: {}", args[0].as_str());
            exit(2);
        }
    }

    rv
}

pub fn ok_to_execute() -> bool {
    let rv: bool;
    unsafe {
        match IF_STATE {
            States::Nentral => {
                rv = true;
            }
            States::ThenBlock => match IF_RESULT {
                Result::Success => rv = true,
                Result::Failure => rv = false,
            },
            States::WantThen => {
                syn_err("then unexpected");
                rv = false;
            }
        }
    }

    rv
}

pub fn syn_err(msg: &str) -> ProcessRes {
    eprintln!("syntax error: {}", msg);
    unsafe {
        IF_STATE = States::Nentral;
    }
    ProcessRes::Failure
}
