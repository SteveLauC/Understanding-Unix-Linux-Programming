mod lclnt_funcs1;
use lclnt_funcs1::Client;

fn main() {
    let mut client = Client::setup();
    client.get_ticket();
    if client.ticket.is_none() {
        return;
    }

    client.do_regular_work();
    client.release_ticket();
}
