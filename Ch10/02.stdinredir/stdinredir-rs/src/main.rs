use nix::unistd::close;
use std::fs::File;
use std::io::stdin;

fn main() {
    close(0).unwrap();
    let _pw: File = File::open("/etc/passwd").unwrap();
    let mut buf: String = String::with_capacity(100);

    stdin().read_line(&mut buf).unwrap();
    print!("{}", buf);
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    print!("{}", buf);
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    println!("{}", buf);
    buf.clear();
}
