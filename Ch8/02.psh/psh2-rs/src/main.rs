use std::io::{self, Write};
use std::os::unix::prelude::CommandExt;
use std::process::{exit, Command};
use std::slice::Iter;

use nix::libc::wait;
use nix::unistd::{fork, ForkResult};

const MAXARGS: i32 = 20;

fn main() {
    let mut num_args: i32 = 0;
    let mut buf: String = String::with_capacity(100);
    let mut arg_list: Vec<String> = Vec::new();

    while num_args < MAXARGS {
        // print the prompt
        print!("arg[{}]", num_args);
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut buf).is_err() || buf.starts_with('\n') {
            buf.clear();

            if num_args > 0 {
                // prepare the command
                let mut list: Iter<String> = arg_list.iter();
                let mut cmd: Command = Command::new(list.next().unwrap());
                cmd.args(list);

                // execute
                execute(&mut cmd);

                // clear argument vector
                arg_list.clear();
                // reset num_args
                num_args = 0;
            }
        } else {
            num_args += 1;
            let mut clone: String = buf.clone(); // clone the argument and push it to the arg_list

            assert!(clone.len() >= 1);
            clone.truncate(clone.len() - 1);
            arg_list.push(clone);
            buf.clear();
        }
    }
}

fn execute(cmd: &mut Command) {
    // wait_status buffer
    let mut child_status: i32 = 0;

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            while unsafe { wait(&mut child_status) } != child.as_raw() {}

            println!(
                "child exited with status: {} {}",
                child_status >> 8,
                child_status & 0xff
            );
        }
        Ok(ForkResult::Child) => {
            cmd.exec();
        }
        Err(msg) => {
            eprintln!("can not fork(): {}", msg);
            exit(1);
        }
    }
}
