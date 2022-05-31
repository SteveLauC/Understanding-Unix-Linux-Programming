use crossbeam::scope;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

struct ArgSet<'a> {
    file_name: &'a str,
    count: usize,
}

impl<'a> ArgSet<'a> {
    fn new(file_name: &'a str) -> Self {
        Self {
            file_name,
            count: 0,
        }
    }
}

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        println!("usage: ./twordcount3-rs file1 file2");
        exit(1);
    }

    let mut arg1: ArgSet<'_> = ArgSet::new(av[1].as_str());
    let mut arg2: ArgSet<'_> = ArgSet::new(av[2].as_str());

    scope(|s| {
        s.spawn(|_| {
            word_count(&mut arg1);
        });
    })
    .unwrap();

    scope(|s| {
        s.spawn(|_| {
            word_count(&mut arg2);
        });
    })
    .unwrap();

    println!("{}: {}", arg1.count, arg1.file_name);
    println!("{}: {}", arg2.count, arg2.file_name);
    println!("{}: total words", arg1.count + arg2.count);
}

fn word_count(arg: &mut ArgSet<'_>) {
    let mut buf: Vec<u8> = Vec::with_capacity(500);
    let mut reader: BufReader<File> = BufReader::new(File::open(arg.file_name).unwrap());

    reader.read_to_end(&mut buf).unwrap();

    let mut prev: char = '\0';
    for idx in 0..buf.len() {
        if !char::from(buf[idx]).is_ascii_alphanumeric() && prev.is_ascii_alphanumeric() {
            arg.count += 1;
        }
        prev = char::from(buf[idx]);
    }
}
