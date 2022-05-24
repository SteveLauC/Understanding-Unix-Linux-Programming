use crate::popen::{my_pclose, my_popen, Type};
use nix::unistd::dup2;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::os::unix::io::AsRawFd;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub fn cannot_do(stream: &TcpStream) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);

    // header
    buffered_writer
        .write(b"HTTP/1.1 501 Not Implementated\r\n")
        .unwrap();
    buffered_writer
        .write(b"Content-type: text/plain\r\n")
        .unwrap();
    buffered_writer.write(b"\r\n").unwrap();

    // body
    buffered_writer
        .write(b"This is not yet implementated\r\n")
        .unwrap();
}

pub fn header(stream: &TcpStream, content_type: Option<&str>) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    buffered_writer.write(b"HTTP/1.1 200 OK\r\n").unwrap();
    if let Some(content) = content_type {
        buffered_writer
            .write(format!("Content-type: {}\r\n", content).as_bytes())
            .unwrap();
    }
}

pub fn do_404(file_name: &str, stream: &TcpStream) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    buffered_writer
        .write(b"HTTP/1.1 404 Not Found\r\n")
        .unwrap();
    buffered_writer
        .write(b"Content-type: text/plain\r\n")
        .unwrap();
    buffered_writer.write(b"\r\n").unwrap();

    buffered_writer
        .write(format!("The item you requested: {} is not found\r\n", file_name).as_bytes())
        .unwrap();
}

pub fn is_dir(file_name: &str) -> bool {
    Path::new(file_name).is_dir()
}

pub fn not_exist(file_name: &str) -> bool {
    eprintln!("debug: file_name: {}", file_name);
    !Path::new(file_name).exists()
}

pub fn file_type(file_name: &str) -> Option<&OsStr> {
    Path::new(file_name).extension()
}

pub fn ends_in_cgi(file_name: &str) -> bool {
    if let Some(extension) = file_type(file_name) {
        extension == "cgi"
    } else {
        false
    }
}

pub fn do_ls(dir_name: &str, stream: &TcpStream) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    header(stream, Some("text/plain"));
    buffered_writer.write(b"\r\n").unwrap();
    dup2(stream.as_raw_fd(), 1).unwrap();
    dup2(stream.as_raw_fd(), 2).unwrap();

    Command::new("ls").args(["-l", dir_name]).exec();
}

pub fn do_exec(file_name: &str, stream: &TcpStream) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    header(stream, None);
    buffered_writer.write(b"\r\n").unwrap();

    let mut exe_reader: BufReader<File> = my_popen(format!("bash {}", file_name), Type::Read);
    let mut buf: String = String::with_capacity(300);

    exe_reader.read_to_string(&mut buf).unwrap();
    buffered_writer.write(buf.as_bytes()).unwrap();
    my_pclose();
}

pub fn do_cat(file_name: &str, stream: &TcpStream) {
    let mut content = "text/plain";
    if let Some(ty) = file_type(file_name) {
        let ty: &str = ty.to_str().unwrap();
        match ty {
            "html" => content = "text/html",
            "gif" => content = "image/gif",
            "jpg" => content = "image/jpg",
            "jpeg" => content = "image/jpeg",
            _ => (),
        }
    }

    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    let mut file_reader: BufReader<File> = BufReader::new(File::open(file_name).unwrap());
    let mut buf: String = String::with_capacity(300);
    header(stream, Some(content));
    buffered_writer.write(b"\r\n").unwrap();
    file_reader.read_to_string(&mut buf).unwrap();
    buffered_writer.write(buf.as_bytes()).unwrap();
}
