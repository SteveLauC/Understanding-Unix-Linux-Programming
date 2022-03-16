use std::fs::read_dir;
use std::path::Path;
use std::env::{args,Args};

fn do_ls<P: AsRef<Path>>(dirname: P) {
    match read_dir(dirname) {
        Ok(entries) => {
            for entry in entries{
                if entry.is_ok() {
                    match entry.unwrap().file_name().into_string() {
                        Ok(s) => println!("{}", s),
                        Err(os_str) => println!("{:?}", os_str),
                    }
                }
            }
        },
        Err(msg) => {
            eprintln!("can not read directory {}", msg);
        }
    }
}


fn main(){
    let mut av: Args = args();
    if av.len() == 1 {
        do_ls(".");
    }else{
        av.next();
        for dir in av {
            println!("{}: ", dir.as_str());
            do_ls(dir);
        }
    }
}
