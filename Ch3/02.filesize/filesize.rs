use std::fs::{metadata, Metadata};

fn main(){
    let md: Metadata = match metadata("/etc/passwd") {
        Ok(m) => m,
        Err(msg) => {
            eprintln!("cannot get size of passwd {}", msg);
            std::process::exit(-1);
        }
    };
    println!("The size of /etc/passwd is {}", md.len());
}
