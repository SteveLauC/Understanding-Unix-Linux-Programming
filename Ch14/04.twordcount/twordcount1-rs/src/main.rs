use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;
use std::thread::{spawn, JoinHandle};

static mut TOTAL_WORDS: usize = 0;

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        println!("usage: ./twordcount1 file1 file2");
        exit(1);
    }
    let av_clone: Vec<String> = av.clone();

    let t1: JoinHandle<()> = spawn(move || count_words(av[1].as_str()));
    let t2: JoinHandle<()> = spawn(move || count_words(av_clone[2].as_str()));

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe{
        println!("{}: total words", TOTAL_WORDS+1);
    }
}

fn count_words(file_name: &str) {
    let mut buf: Vec<u8> = Vec::with_capacity(500);
    let mut reader: BufReader<File> = BufReader::new(File::open(file_name).unwrap());

    reader.read_to_end(&mut buf).unwrap();

    unsafe {
        TOTAL_WORDS += buf.iter().filter(|item| **item == b' ').count();
    }
}
