use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

fn main() {
    let total_words: Arc<Mutex<usize>> = Arc::new(Mutex::new(0_usize));

    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        println!("usage: ./twordcount1 file1 file2");
        exit(1);
    }
    let av_clone: Vec<String> = av.clone();

    let ref1 = total_words.clone();
    let ref2 = total_words.clone();
    let t1: JoinHandle<()> = spawn(move || count_words(av[1].clone(), ref1));
    let t2: JoinHandle<()> = spawn(move || count_words(av_clone[2].clone(), ref2));

    t1.join().unwrap();
    t2.join().unwrap();

    println!("{}: total words", total_words.lock().unwrap());
}

fn count_words(file_name: String, counter: Arc<Mutex<usize>>) {
    let mut buf: Vec<u8> = Vec::with_capacity(500);
    let mut reader: BufReader<File> = BufReader::new(File::open(file_name).unwrap());

    reader.read_to_end(&mut buf).unwrap();

    let mut prev: char = '\0';
    for idx in 0..buf.len() {
        if !char::from(buf[idx]).is_ascii_alphanumeric() && prev.is_ascii_alphanumeric() {
            *counter.lock().unwrap() += 1;
        }
        prev = char::from(buf[idx]);
    }
}
