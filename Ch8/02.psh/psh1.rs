use std::io::{self, Write};
use std::os::unix::prelude::CommandExt;
use std::process::Command;
use std::slice::Iter;

const MAXARGS: i32 = 20;

fn main() {
    let mut num_args: i32 = 0;
    let mut buf: String = String::with_capacity(100);
    let mut arg_list: Vec<String> = Vec::new();

    while num_args < MAXARGS {
        print!("arg[{}]", num_args);
		io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut buf).is_err() || buf.starts_with('\n') {
            // prepare the command
            let mut list: Iter<String> = arg_list.iter();
            let mut cmd: Command = Command::new(list.next().unwrap());
            cmd.args(list);

            // execute
            execute(&mut cmd);
        } else {
            num_args += 1;
            let mut clone: String = buf.clone();
            clone.truncate(clone.len() - 1);
            arg_list.push(clone);
            buf.clear();
        }
    }
}

fn execute(cmd: &mut Command) {
    cmd.exec();
}
