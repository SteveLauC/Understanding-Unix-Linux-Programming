use std::os::unix::prelude::CommandExt;
use std::process::Command;

fn main() {
    println!("About to exec ls -l");   

    let mut output: Command = Command::new("ls");
    output.arg("-l");
    output.exec();

    println!("ls is done. bye");
}
