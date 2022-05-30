mod lserv_funcs1;
use lserv_funcs1::Server;

use std::str;
fn main() {
    let mut server: Server = Server::setup();
    server.free_all_tickets();
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
