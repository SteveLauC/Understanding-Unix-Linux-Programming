//! The rust implementation of lserv2 can not be done for
//! the reason that the interface exposed by `signal` strictly
//! request its second argument to be of type `extern "c" fn(i32)`
//!

mod lserv_funcs1;
use lserv_funcs1::Server;
use std::str;

// use nix::sys::signal::Signal::SIGALRM;
/* use nix::sys::signal::{signal, SigHandler};
use nix::unistd::alarm; */

fn main() {
    let mut server: Server = Server::setup();
    server.free_all_tickets();

    /* signal(SIGALRM, SigHandler::Handler(Server::ticket_reclaim));
    alarm(); */

    loop {
        let mut buf: [u8; 128] = [0; 128];
        if let Ok((size, addr)) = server.server_sock.recv_from(&mut buf) {
            eprintln!(
                "\t\tSERVER GOT: {} {:?}",
                str::from_utf8(&buf[..size]).unwrap(),
                addr
            );
            server.handle_request(str::from_utf8(&buf[..size]).unwrap().into(), addr);
        }
    }
}
