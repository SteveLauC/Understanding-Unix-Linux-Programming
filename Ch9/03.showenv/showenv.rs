use std::env::vars;

fn main() {
    for (key, val) in vars() {
        println!("{}={}", key, val);
    }
}
