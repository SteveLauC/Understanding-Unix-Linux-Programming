use nix::errno::Errno;
use nix::sys::signal::kill;
use nix::unistd::{gethostname, Pid};
use std::net::{SocketAddr, UdpSocket};

const MAX_USER: usize = 3;
const TICKET_AVAIL: u32 = 0;
const SERVER_PORT: u32 = 2020;

pub struct Server {
    pub ticket_array: [u32; MAX_USER],
    pub num_ticket_out: usize,
    pub server_sock: UdpSocket,
}

impl Server {
    pub fn setup() -> Self {
        Self {
            ticket_array: [TICKET_AVAIL; MAX_USER],
            num_ticket_out: 0,
            server_sock: UdpSocket::bind(format!("{}:{}", hostname(), SERVER_PORT)).unwrap(),
        }
    }

    pub fn free_all_tickets(&mut self) {
        self.ticket_array.fill(0);
    }

    pub fn do_hello(&mut self, msg: String) -> String {
        let pid: u32 = msg
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        if self.num_ticket_out >= MAX_USER {
            return "FAIL no tickets available".into();
        }

        let mut index_of_ticket: usize = self.ticket_array.len();
        for idx in 0..self.ticket_array.len() {
            if self.ticket_array[idx] == TICKET_AVAIL {
                self.ticket_array[idx] = pid;
                index_of_ticket = idx;
                break;
            }
        }
        assert_ne!(index_of_ticket, self.ticket_array.len());
        self.num_ticket_out += 1;
        format!("TICK {} {}", pid, index_of_ticket)
    }

    pub fn do_goodbye(&mut self, msg: String) -> String {
        let index_of_ticket: usize = msg
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(2)
            .unwrap()
            .parse()
            .unwrap();

        self.ticket_array[index_of_ticket] = TICKET_AVAIL;
        self.num_ticket_out -= 1;

        "THNX see ya!".into()
    }
    pub fn handle_request(&mut self, req: String, client_addr: SocketAddr) {
        let response: String;
        if req.starts_with("HELO") {
            response = self.do_hello(req);
        } else if req.starts_with("GBYE") {
            response = self.do_goodbye(req);
        } else {
            response = "FAIL invalid request".to_owned();
        }

        eprint!("\t\tSERVER SAID: {}", response.as_str());
        eprintln!("({})", client_addr);

        self.server_sock
            .send_to(response.as_bytes(), client_addr)
            .unwrap();
    }

    pub fn ticket_reclaim(&mut self) {
        self.ticket_array
            .iter_mut()
            .find(|item| **item == TICKET_AVAIL && is_pid_dead(**item as i32))
            .and_then::<(), _>(|item| {
                *item = TICKET_AVAIL;
                self.num_ticket_out -= 1;
                None
            });
    }
}

fn hostname() -> String {
    let mut hostname: [u8; 64] = [0; 64];
    gethostname(&mut hostname)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn is_pid_dead(pid: i32) -> bool {
    Err(Errno::ESRCH) == kill(Pid::from_raw(pid), None)
}
