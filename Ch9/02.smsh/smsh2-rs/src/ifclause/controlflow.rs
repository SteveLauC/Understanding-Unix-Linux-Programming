//! controlflow.rs: control the flow of command execution

use crate::process::{process, ProcessRes};

/// type to represent states
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

/// global variables to represents current state and if-clause execution result
static mut IF_STATE: States = States::Nentral;
static mut IF_RESULT: Result = Result::Success;

/// purpose: to identify the control command
///
/// action: check if the command starts with `if/then/fi`
///
/// arguments:
///     * `first_cmd`: first item of the command words list constructed by `crate::splitline::splitline()`
///     
/// return: a bool value
pub fn is_control_command(first_cmd: &str) -> bool {
    first_cmd.starts_with("if") || first_cmd.starts_with("then") || first_cmd.starts_with("fi")
}

/// purpose: execute the control command
///
/// action:
///         If it is a `if` clause and the state is `Nentral`, execute the command after if
///         Else, report syntax error
///         
///         If it is a `then` clause and the state is `WantThen`, update the state to `ThenBlock`
///         Else, report syntax error
///         
///         If it is a `fi` clause and the state is `ThenBlock`, update the state to `Nentral`
///         Else, report syntax error
///         
/// arguments: 
///     * `args`: a command words list constructed by `crate::splitline::splitline`
///    
/// return: control command processing result
pub fn do_control_command(args: Vec<&str>) -> ProcessRes {
    match args[0] {
        "if" => unsafe {
            if IF_STATE != States::Nentral {
                syn_err("if unexpected")
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
                ProcessRes::Success
            }
        },
        "then" => unsafe {
            if IF_STATE != States::WantThen {
                syn_err("then unexpected")
            } else {
                IF_STATE = States::ThenBlock;
                ProcessRes::Success
            }
        },
        "fi" => unsafe {
            if IF_STATE != States::ThenBlock {
                syn_err("fi unexpected")
            } else {
                IF_STATE = States::Nentral;
                ProcessRes::Success
            }
        },
        _ => {
            unreachable!("inernal error processing: {}", args[0]);
        }
    }
}


/// purpose: to check if it is able to execute the normal command
/// 
/// action: there are two situations where we can execute the command
///         1. in `Nentral` state
///         2. in `ThenBlock` state and the executing result of `if` clause is success
///         
/// return: if we are in the above two cases, return true; otherwise, return false
pub fn ok_to_execute() -> bool {
    unsafe {
        match IF_STATE {
            States::Nentral => {
                true
            }
            States::ThenBlock => match IF_RESULT {
                Result::Success => true,
                Result::Failure => false,
            },
            States::WantThen => {
                syn_err("then unexpected");
                false
            }
        }
    }
}


/// purpose: dedicated error handling function
/// 
/// action: print the error info to stderr and set the state to `Nentral`, then return `Process::Failure`
/// 
/// arguments:
///     * `msg`: error info
///
/// return: ProcessRes::Failure
pub fn syn_err(msg: &str) -> ProcessRes {
    eprintln!("syntax error: {}", msg);
    unsafe {
        IF_STATE = States::Nentral;
    }
    ProcessRes::Failure
}
