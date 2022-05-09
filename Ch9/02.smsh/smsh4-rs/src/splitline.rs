//! splitline.rs: command reading and spliting

use std::io::{stdout, Stdin, Write};

/// purpose: read a line from stdin
///
/// action: prompt user and read a line from stdin.
///
/// arguments:
///     * `prompt`: user prompt
///     * `fp`: file ptr, hardcode to Stdin here
///
/// return:
///     If read successfully, return Some(Command)
///     Otherwise, retrun None
pub fn next_cmd(prompt: &str, fp: Stdin) -> Option<String> {
    // print the command prompt
    print!("{}", prompt);
    stdout().flush().expect("can not flush stdout");

    let mut buf: String = String::with_capacity(100);
    match fp.read_line(&mut buf) {
        Err(_) => None,
        Ok(0) => None,
        Ok(_) => Some(buf),
    }
}

/// purpose: tokenize the command line using spaces
///
/// action: split the line into words using space
///
/// arguments:
///     * `line`: command line
///    
/// return: a list of words
pub fn splitline(line: &str) -> Vec<&str> {
    line.split_ascii_whitespace().collect()
}
