//! builtin.rs: contains the switch and the functions for the built-in commands
//!
//! Currently, the supported built-in commands are as follows:
//!      1. `set`: print all the shell variables
//!      2. `var=value`: initialize a shell variable
//!      3. `export var-name`: make an existed variable global

use crate::process::ProcessRes;
use crate::VarTable;
use std::str::Split;

/// purpose: check if the command is one of the supported built-in commands
///
/// action: check if the command is `set/export` or contains `=`
///
/// arguments:
/// * `cmd`: command
///
/// return: true on built-in commands; otherwise, return false
///
pub fn is_builtin_command(cmd: &str) -> bool {
    cmd == "set" || cmd.contains('=') || cmd.starts_with("export")
}

/// purpose: run the built-in commands
///
/// action:
///
///
/// arguments:
///  * `args`: command words list
///  
/// return: 0 on success, non-zero on failure
///
pub fn builtin_command(args: Vec<&str>, vt: &mut VarTable) -> ProcessRes {
    if args[0] == "set" {
        vt.list();
        return ProcessRes::Success;
    }

    if args[0].contains('=') {
        return assign(args[0], vt);
    }

    if args[0] == "export" {
        if args.len() < 2 {
            return ProcessRes::Failure;
        } else {
            vt.export(args[1]);
            return ProcessRes::Success;
        }
    }

    unreachable!()
}

/// purpose: determine if a string is a legal varable name
///
/// action: a valid variable should be:
///     1. can not start with number
///     2. every char of it should be a letter or a number(0-9) or a underscore
///     3. length of it should be bigger or equal to 1
///     
/// arguments:
///     * `name`: variable name
///    
/// return: if name is valid, return true; otherwise, return false
fn okname(name: &str) -> bool {
    let name_char_vec: Vec<char> = name.chars().collect();

    if name_char_vec.is_empty() || name_char_vec[0].is_digit(10) {
        return false;
    } else {
        for c in name_char_vec.iter() {
            if !(c.is_alphanumeric() || *c == '_') {
                return false;
            }
        }
    }

    true
}

/// purpose: execute name=val AND ensure that name is legal
///
/// action: check the validity of `name` and then call `crate::varlib::VarTable::store`
///
/// argument:
///     * `assignment`: "name=val" assignment statement
///     * `vt`: mutable ref to the variable table
///     
/// return: If name is invalid, return ProcessRes::Failure;
///         Otherwise, return ProcessRes::Success
fn assign(assignment: &str, vt: &mut VarTable) -> ProcessRes {
    let mut it: Split<char> = assignment.split('=');
    let variable: (&str, &str) = (it.next().unwrap(), it.next().unwrap());

    if okname(variable.0) {
        vt.store(variable.0, variable.1);
        ProcessRes::Success
    } else {
        ProcessRes::Failure
    }
}
