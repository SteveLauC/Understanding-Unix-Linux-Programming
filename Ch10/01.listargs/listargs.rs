use std::env::args;


fn main() {
    args().enumerate().for_each(|(idx, arg)| println!("args[{}] {}", idx, arg));

    eprintln!("This message is sent to stderr");
}
