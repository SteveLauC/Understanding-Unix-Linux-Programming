use std::process;
use std::io::{self, stdin, Read, Write};

#[derive(PartialEq)]
enum Answer{
    YES,
    NO,
}

fn get_response() -> Answer {
    print!("Do you want another transaction(y/n)");
    io::stdout().flush().unwrap();

    for n in stdin().bytes().map(|x|x.expect("Can not read from stdin")) {
        if n == 'y' as u32 as u8 || n == 'Y' as u32 as u8 {
            return Answer::YES;
        }else if n=='n' as u32 as u8 || n=='N' as u32 as u8 {
            return Answer::NO;
        }else{
            continue;
        }
    }
    unreachable!()
}


fn main() {
    if get_response() == Answer::NO {
        process::exit(1);
    }
}