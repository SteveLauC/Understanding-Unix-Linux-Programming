use std::fs::{metadata, Metadata,read_dir, ReadDir, DirEntry};
use std::os::unix::fs::MetadataExt;
use std::process;
use std::env::set_current_dir;
use std::io;

fn get_inode(filename: &str) -> u64{
    let md: Metadata = match metadata(filename) {
        Ok(md) => md,
        Err(msg) => {
            eprintln!("cannot stat {} {:?}", filename, msg);
            process::exit(-1);
        }
    };
    md.ino()
}

fn inode_to_name(this_inode: u64) -> io::Result<String> {
    let mut name: String = String::new();
    let rd: ReadDir = read_dir(".")?;
    for item in rd {
        let de: DirEntry = item?;
        let md: Metadata = de.metadata()?;
        if md.ino() == this_inode {
           name = de.file_name().into_string().expect("cannot convert OsStrng to String");
        }
    }
    Ok(name)
}

fn print_path_to(inode: u64) {
    let mut my_inode: u64 = 0;
    let mut name: String = String::new();

    if get_inode("..") != inode {
        if set_current_dir("..").is_err() {
            eprintln!("cannot change working directory to the parent directory");
            process::exit(-1);
        }
        name = match inode_to_name(inode) {
            Ok(n) => n,
            Err(msg) => {
                eprintln!("cannot query the filename of inode {} {}", inode, msg);
                process::exit(-1);
            }
        };
        my_inode = get_inode(".");
        print_path_to(my_inode);
        print!("/{}", name);
    }
}

fn main() {
    print_path_to(get_inode("."));
    println!();
}
