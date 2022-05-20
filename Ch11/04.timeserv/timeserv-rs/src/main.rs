use chrono::{DateTime, Local};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));

    date.format("%b %e %H:%M:%S %Y\n").to_string()
}

fn main() {
    for client in TcpListener::bind("127.0.0.1:13000").unwrap().incoming() {
        let mut client: TcpStream = client.unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        client.write(b"Wow! got a call\nThe time here is ").unwrap();
        client.write(show_time(now).as_bytes()).unwrap();
    }
}
