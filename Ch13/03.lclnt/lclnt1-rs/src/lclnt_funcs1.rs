use nix::unistd::gethostname;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::process::id;
use std::str;
use std::thread::sleep;
use std::time::Duration;

const SERVER_PORT: u16 = 2020;
const MSG_LEN: usize = 128;

pub struct Client {
    pub pid: u32,
    pub sd: UdpSocket,
    pub server_addr: SocketAddr,
    pub ticket: Option<String>,
}

impl Client {
    pub fn setup() -> Self {
        let mut hostname: [u8; 64] = [0; 64];
        let hostname: &str = gethostname(&mut hostname).unwrap().to_str().unwrap();
        let addrs: Vec<SocketAddr> = format!("{}:{}", hostname, SERVER_PORT)
            .to_socket_addrs()
            .unwrap()
            .collect();
        assert!(!addrs.is_empty());

        Self {
            pid: id(),
            sd: UdpSocket::bind("0.0.0.0:0").unwrap(),
            server_addr: addrs[0],
            ticket: None,
        }
    }

    pub fn do_transaction(&self, msg: String) -> String {
        eprintln!("debug: sendto={}|", msg);
        self.sd.send_to(msg.as_bytes(), self.server_addr).unwrap();

        let mut response: [u8; MSG_LEN] = [0; 128];
        let n = self.sd.recv(&mut response).unwrap();
        str::from_utf8(&response[..n]).unwrap().to_owned()
    }

    pub fn get_ticket(&mut self) {
        let response: String = self.do_transaction(format!("HELO {}", self.pid));
        if response.starts_with("TICK") {
            eprintln!("got ticket {}", &response[5..]);
            self.ticket = Some(response[5..].to_owned());
        } else if response.starts_with("FAIL") {
            eprintln!("could not get ticket {}", &response[5..]);
        } else {
            eprintln!("unknown message: {}", &response[5..]);
        }
    }

    pub fn release_ticket(&self) {
        if self.ticket.is_none() {
            return;
        }
        let response = self.do_transaction(format!("GBYE {}", self.ticket.as_ref().unwrap()));
        if response.starts_with("THNX") {
            eprintln!("released ticket ok");
        } else if response.starts_with("FAIL") {
            eprintln!("release failed {}", &response[5..]);
        } else {
            eprintln!("unknown message {}", &response[5..]);
        }
    }

    pub fn do_regular_work(&self) {
        println!("SuperSleep version 1.0 Running-Licensed Software");
        sleep(Duration::from_secs(10));
    }
}
