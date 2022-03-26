use nix::unistd::fork;
use std::process;

fn main() {
    println!("my pid is {}", process::id());
    unsafe{
        let _ = fork();
        let _ = fork();
        let _ = fork();
    }
    println!("my pid is {}", process::id());
}
