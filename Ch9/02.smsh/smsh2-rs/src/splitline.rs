use std::io::{stdout, Stdin, Write};

// hardcode fp to stdin
pub fn next_cmd(prompt: &str, fp: Stdin) -> Option<String> {
    print!("{}", prompt);
    stdout().flush().expect("can not flush stdout");

    let mut buf: String = String::with_capacity(100);
    match fp.read_line(&mut buf) {
        Err(_) => None,
        Ok(0) => None,
        Ok(_) => Some(buf),
    }
}

pub fn splitline(line: String) -> Vec<String> {
    line.split_ascii_whitespace()
        .map(|str| str.to_owned())
        .collect()
}
