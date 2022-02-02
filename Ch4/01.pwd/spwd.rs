use std::fs::{metadata, Metadata,read_dir};
use std::os::unix::fs::MetadataExt;
use std::process;
use std::env::set_current_dir;

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

fn inode_to_name(this_inode: u64) -> String {
    if let Ok(rd) = read_dir(".") {
        for item in rd {
            if let Ok(de) = item {
                match de.metadata() {
                    Ok(meta) => {
                        if meta.ino() == this_inode {
                            return de.file_name().into_string().expect("cannot convert this OsStirng to String");
                        }
                    },
                    Err(msg) => {
                        eprintln!("cannot inspect some entries in current dir {:?}", msg);
                    }
                }
            }else{
                eprintln!("cannot inspect some entries in current dir");
            }
        }
    }
    eprintln!("error looking for inode:{}", this_inode);
    process::exit(1);
}

fn print_path_to(inode: u64) {
    let mut my_inode: u64 = 0;
    let mut name: String = String::new();

    if get_inode("..") != inode {
        if set_current_dir("..").is_err() {
            eprintln!("cannot change working directory to the parent directory");
            process::exit(-1);
        }
        name = inode_to_name(inode);
        my_inode = get_inode(".");
        print_path_to(my_inode);
        print!("/{}", name);
    }
}

fn main() {
    print_path_to(get_inode("."));
    println!();
}
