use std::io::{Read, stdin};

fn main() {
    // for (i, c) in stdin().bytes().map(|x| x.expect("cannot read char from stdin")).enumerate() {
    //     if c != b'Q' {
    //         println!("char {:3} is {} code {}", i, c, c);
    //     }else{
    //         break;
    //     }
    // } 

    stdin().bytes().enumerate().for_each(|(i, item)|{
        if let Ok(c) = item {
            if c == b'Q' {
                std::process::exit(0);
            }else{
                println!("char {:3} is {} code {}", i, char::from(c), c);
            }
        }
    });
}
