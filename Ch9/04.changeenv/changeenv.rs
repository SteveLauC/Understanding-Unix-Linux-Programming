use std::env::{remove_var, set_var, vars};
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    for (key, _) in vars() {
        remove_var(key);
    }

    set_var("TERM", "vt100");
    set_var("HOME", "/on/the/range");


    Command::new("env").exec();    
}
