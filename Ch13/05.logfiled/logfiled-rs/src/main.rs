use chrono::{DateTime, Local};
use std::os::unix::net::UnixDatagram;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SOCKNAME: &str = "/tmp/logfilesock";
const MSG_LEN: usize = 512;

fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));
    date.format("%b %e %H:%M:%S %Y").to_string()
}

fn main() {
    let server: UnixDatagram = UnixDatagram::bind(SOCKNAME).unwrap();
    let mut msg_num: u32 = 0;

    loop {
        let mut buf: [u8; MSG_LEN] = [0; MSG_LEN];
        let n: usize = server.recv(&mut buf).unwrap();

        println!(
            "[{:5}] {} {}",
            msg_num,
            show_time(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            std::str::from_utf8(&buf[..n]).unwrap()
        );
        msg_num += 1;
    }
}
