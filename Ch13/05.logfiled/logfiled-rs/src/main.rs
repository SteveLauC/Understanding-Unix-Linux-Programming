use chrono::{DateTime, Local};
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SOCKNAME: &str = "/tmp/logfilesock";

fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));
    date.format("%b %e %H:%M:%S %Y").to_string()
}

fn main() {
    let listener: UnixListener = UnixListener::bind(SOCKNAME).unwrap();
    let mut msg_num: u32 = 0;

    for msg in listener.incoming() {
        let mut msg: UnixStream = msg.unwrap();
        msg_num += 1;

        let mut buf: String = String::with_capacity(512);
        msg.read_to_string(&mut buf).unwrap();

        println!(
            "[{:5}] {} {}",
            msg_num,
            show_time(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            buf
        );
    }
}
