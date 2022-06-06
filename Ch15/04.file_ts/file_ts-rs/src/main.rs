use chrono::{DateTime, Local};
use file_lock::{FileLock, FileOptions};
use std::env::args;
use std::io::Write;
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let av: Vec<String> = args().collect();
    if av.len() != 2 {
        eprintln!("usage: ./file_ts-rs filename");
        exit(1);
    }

    loop {
        let opt: FileOptions = FileOptions::new().write(true).truncate(true).create(true);

        if let Ok(mut lock) = FileLock::lock(av[1].as_str(), true, opt) {
            lock.file
                .write(
                    show_time(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    )
                    .as_bytes(),
                )
                .unwrap();

            lock.unlock().unwrap();
        }
    }
}
fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));
    date.format("%b %e %H:%M:%S %Y").to_string()
}
