mod mode;
use mode::*;

use std::env::{args, Args};
use std::fs::{read_dir, DirEntry, Metadata};
use std::os::unix::prelude::MetadataExt;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Local};
use libc::mode_t;
use users::{get_group_by_gid, get_user_by_uid};

fn do_ls(dirname: &str) {
    match read_dir(dirname) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(e) = entry {
                    do_stat(e);
                } else {
                    eprintln!(
                        "something goes wrong while getting direntry in dir: {}",
                        dirname
                    );
                }
            }
        }
        Err(msg) => {
            eprintln!("cannot read dir {}: {}", dirname, msg);
        }
    }
}

fn do_stat(entry: DirEntry) {
    if let Ok(md) = entry.metadata() {
        show_file_info(entry.file_name().into_string().unwrap(), md);
    } else {
        eprintln!("can not get metadata of {:?}", entry.file_name());
    }
}

fn show_time(seconds: i64) -> String {
    let date: DateTime<Local> =
        DateTime::from(UNIX_EPOCH + Duration::from_secs(u64::try_from(seconds).unwrap()));
    date.format("%b %e %H:%M:%S %Y").to_string()
}

fn show_file_info(filename: String, md: Metadata) {
    print!("{:<10} ", mode_to_letter(md.mode()));
    print!("{:<4} ", md.nlink());
    print!("{:<8} ", uid_to_name(md.uid()));
    print!("{:<8} ", gid_to_name(md.gid()));
    print!("{:<12}   ", show_time(md.mtime()));
    println!("{}", filename);
}

// String in rust does not support index
// so I use an array of char here.
// and construct the final string from it.
fn mode_to_letter(mode: mode_t) -> String {
    let mut mode_str: [char; 10] = ['-'; 10];
    // file type
    
    // another rusty way to get filetype on UNIX is to use `std::os::unix::fs::FileTypeExt`
    // but honestly, it's not suitable here for the reason that `mode_to_letter()` should
    // only have one arg of type mode_t
    // If u wanna use FileTypeExt, you need to get the Metadata, call file_type() on that...
    // which will break my code arch.
    if is_dir(mode) {
        mode_str[0] = 'd';
    }
    if is_chr(mode) {
        mode_str[0] = 'c';
    }
    if is_blk(mode) {
        mode_str[0] = 'b';
    }
    if is_lnk(mode) {
        mode_str[0] = 'l';
    }
    if is_sock(mode) {
        mode_str[0] = 's';
    }
    if is_fifo(mode) {
        mode_str[0] = 'p';
    }

    // permission
    if irusr(mode) {
        mode_str[1] = 'r';
    }
    if iwusr(mode) {
        mode_str[2] = 'w';
    }
    if ixusr(mode) {
        mode_str[3] = 'x';
    }
    if irgrp(mode) {
        mode_str[4] = 'r';
    }
    if iwgrp(mode) {
        mode_str[5] = 'w';
    }
    if ixgrp(mode) {
        mode_str[6] = 'x';
    }
    if iroth(mode) {
        mode_str[7] = 'r';
    }
    if iwoth(mode) {
        mode_str[8] = 'w';
    }
    if ixoth(mode) {
        mode_str[9] = 'x';
    }
    mode_str.iter().collect::<String>()
}

fn uid_to_name(uid: u32) -> String {
    if let Some(u) = get_user_by_uid(uid) {
        u.name().to_str().unwrap().to_owned()
    } else {
        uid.to_string()
    }
}

fn gid_to_name(gid: u32) -> String {
    if let Some(g) = get_group_by_gid(gid) {
        g.name().to_str().unwrap().to_owned()
    } else {
        gid.to_string()
    }
}

fn main() {
    let mut av: Args = args();
    if av.len() == 1 {
        do_ls(".");
    } else {
        av.next();
        for file in av {
            do_ls(file.as_str());
        }
    }
}
