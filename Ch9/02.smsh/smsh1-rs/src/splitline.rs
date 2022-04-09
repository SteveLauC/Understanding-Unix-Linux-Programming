use std::io::{Stdin, stdout, Write};
use std::process::Command;

// hardcode fp to stdin
pub fn next_cmd(prompt: &str, fp: Stdin) -> Option<String>{
    print!("{}", prompt);
    stdout().flush().expect("can not flush stdout");

    let mut buf: String = String::with_capacity(100);
    match fp.read_line(&mut buf) {
        Err(_) => None,
        Ok(0) => None,
        Ok(_) => Some(buf),
    }
}


pub fn splitline(line: String) -> Option<Command> {
    let mut args_iter = line.split_ascii_whitespace();
    let cmd = args_iter.next()?;
    let mut res = Command::new(cmd);
    res.args(args_iter);

    Some(res)
}