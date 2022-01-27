use std::env::args;
use std::fs::{metadata, Metadata};
use std::os::unix::fs::MetadataExt;
use std::process;

fn show_stat_info(fname: &str, md: Metadata){
    println!("mode: {:o}", md.mode());
    println!("links: {}", md.nlink());
    println!("user: {}", md.uid());
    println!("group: {}", md.gid());
    println!("size: {}", md.size());
    println!("mtime: {}", md.mtime());
    println!("name: {}", fname);
}

fn main(){
    let av: Vec<String> = args().collect();
    if av.len() == 2 {
        if let Ok(md) = metadata(av[1].as_str()) {
            show_stat_info(av[1].as_str(), md);
        }else{
            eprintln!("cannot obtain metadata of {}", av[1]);
            process::exit(-1);
        }
    }else{
        eprintln!("no arg is supplied");
        process::exit(-1);
    }
}
